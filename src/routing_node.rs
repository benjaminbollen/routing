// Copyright 2015 MaidSafe.net limited.
//
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

use cbor::{CborError};
use rand;
use sodiumoxide;
use std::sync::mpsc;
use std::boxed::Box;
use std::marker::PhantomData;

use crust;
use NameType;
use node_interface::{Interface, CreatePersonas};
use routing_membrane::RoutingMembrane;
use id::Id;
use public_id::PublicId;
use types::{MessageId, SourceAddress, DestinationAddress};
use utils::{encode, decode};
use authority::{Authority};
use messages::{RoutingMessage, SignedMessage, MessageType};
use error::{RoutingError};
use std::thread::spawn;

static MAX_BOOTSTRAP_CONNECTIONS : usize = 3;

type ConnectionManager = crust::ConnectionManager;
type Event = crust::Event;
pub type Endpoint = crust::Endpoint;
type PortAndProtocol = crust::Port;

type RoutingResult = Result<(), RoutingError>;

/// DHT node
pub struct RoutingNode<F, G> where F : Interface + 'static,
                                   G : CreatePersonas<F> {
    genesis: Box<G>,
    phantom_data: PhantomData<F>,
    id: Id,
    next_message_id: MessageId,
    bootstrap: Option<(Endpoint, Option<NameType>)>,
}

impl<F, G> RoutingNode<F, G> where F : Interface + 'static,
                                   G : CreatePersonas<F> {
    pub fn new(genesis: G) -> RoutingNode<F, G> {
        sodiumoxide::init();  // enable shared global (i.e. safe to multithread now)
        let id = Id::new();
        RoutingNode { genesis: Box::new(genesis),
                      phantom_data: PhantomData,
                      id : id,
                      next_message_id: rand::random::<MessageId>(),
                      bootstrap: None,
                    }
    }

    /// Run the Routing Node.
    /// This is a blocking call which will start a CRUST connection
    /// manager and the CRUST bootstrapping procedures.
    /// If CRUST finds a bootstrap connection, the routing node will
    /// attempt to request a name from the network and connect to its close group.
    /// If CRUST reports a new connection on the listening port, before bootstrapping,
    /// routing node will consider itself the first node.
    //  This might be moved into the constructor new
    //  For an initial draft, kept it as a separate function call.
    #[allow(unused_assignments)]
    pub fn run(&mut self) -> Result<(), RoutingError> {
        // keep state on whether we still might be the first around.
        let mut possible_first = true;
        let mut relocated_name : Option<NameType> = None;

        let (event_output, event_input) = mpsc::channel();
        let mut cm = crust::ConnectionManager::new(event_output.clone());
        let _ = cm.start_accepting(vec![]);
        cm.bootstrap(MAX_BOOTSTRAP_CONNECTIONS);
        loop {
            match event_input.recv() {
                Err(_) => return Err(RoutingError::FailedToBootstrap),
                Ok(crust::Event::NewMessage(endpoint, bytes)) => {
                    match self.bootstrap {
                        Some((ref bootstrap_endpoint, _)) => {
                            debug_assert!(&endpoint == bootstrap_endpoint);
                            match decode::<SignedMessage>(&bytes) {
                                Err(_) => continue,
                                Ok(wrapped_message) => {
                                    match wrapped_message.get_routing_message() {
                                        Err(_) => continue,
                                        Ok(message) => {
                                            match message.message_type {
                                                MessageType::PutPublicIdResponse(
                                                    ref new_public_id) => {
                                                      relocated_name = Some(new_public_id.name());
                                                      println!("Received PutPublicId relocated
                                                          name {:?} from {:?}", relocated_name,
                                                          self.id.name());
                                                      break;
                                                },
                                                _ => continue,
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        None => {}
                    }
                },
                Ok(crust::Event::NewConnection(endpoint)) => {
                    // only allow first if we still have the possibility
                    if possible_first {
                        // break from listening to CM
                        // and first start RoutingMembrane
                        relocated_name = Some(NameType(sodiumoxide::crypto::hash::sha512
                            ::hash(&self.id.name().0).0));
                        break;
                    } else {
                        // aggressively refuse a connection when we already have
                        // and drop it.
                        cm.drop_node(endpoint);
                    }
                },
                Ok(crust::Event::LostConnection(_endpoint)) => {

                },
                Ok(crust::Event::NewBootstrapConnection(endpoint)) => {
                    match self.bootstrap {
                        None => {
                            // we found a bootstrap connection,
                            // so disable us becoming a first node
                            possible_first = false;
                            // register the bootstrap endpoint
                            self.bootstrap = Some((endpoint.clone(), None));
                            // and try to request a name from this endpoint
                            let our_public_id = PublicId::new(&self.id);
                            let put_public_id_msg
                                = try!(self.construct_put_public_id_msg(&our_public_id));
                            let serialised_message = try!(encode(&put_public_id_msg));
                            ignore(cm.send(endpoint, serialised_message));
                        },
                        Some(_) => {
                            // only work with a single bootstrap endpoint (for now)
                            cm.drop_node(endpoint);
                        }
                    }
                }
            }
        }

        match relocated_name {
            Some(new_name) => {
                self.id.assign_relocated_name(new_name);
                let mut membrane = RoutingMembrane::<F>::new(
                    cm, event_output, event_input, None,
                    self.id.clone(),
                    self.genesis.create_personas());
                // TODO: currently terminated by main, should be signalable to terminate
                // and join the routing_node thread.
                spawn(move || membrane.run());
            },
            None => { return Err(RoutingError::FailedToBootstrap); }
        }

        Ok(())
    }

    fn construct_put_public_id_msg(&mut self, our_unrelocated_id: &PublicId)
            -> Result<SignedMessage, CborError> {

        let message_id = self.get_next_message_id();

        let message =  RoutingMessage {
            destination  : DestinationAddress::Direct(our_unrelocated_id.name()),
            source       : SourceAddress::RelayedForNode(self.id.name(), self.id.name()),
            orig_message : None,
            message_type : MessageType::PutPublicId(our_unrelocated_id.clone()),
            message_id   : message_id.clone(),
            authority    : Authority::ManagedNode,
        };

        SignedMessage::new(&message, self.id.signing_private_key())
    }

    fn get_next_message_id(&mut self) -> MessageId {
        let temp = self.next_message_id;
        self.next_message_id = self.next_message_id.wrapping_add(1);
        return temp;
    }
}

fn ignore<R,E>(_: Result<R,E>) {}

#[cfg(test)]
mod test {

}
