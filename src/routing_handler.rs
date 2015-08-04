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

static MAX_BOOTSTRAP_CONNECTIONS : usize = 1;

/// Routing Membrane
pub struct RoutingHandler {
    // for CRUST
    sender_clone        : Sender<CrustEvent>,
    crust_channel       : Receiver<CrustEvent>,
    connection_manager  : crust::ConnectionManager,
    reflective_endpoint : crust::Endpoint,
    accepting_on        : Vec<crust::Endpoint>,
    bootstraps          : BTreeMap<Endpoint, Option<NameType>>,
    // for RoutingNode
    node_channel        : Sender<NodeEvent>,
    // for Routing
    id                  : Id,
    routing_table       : RoutingTable,
    relay_map           : RelayMap,
    filter              : MessageFilter<types::FilterType>,
    public_id_cache     : LruCache<NameType, PublicId>,
    connection_cache    : BTreeMap<NameType, SteadyTime>,
    refresh_accumulator : RefreshAccumulator,
}

impl RoutingHandler {
    pub fn new() -> RoutingHandler {
        let id = Id::new();

        let (crust_output, crust_input) = mpsc::channel();
        let mut cm = crust::ConnectionManager::new(crust_output.clone());
        let _ = cm.start_accepting(vec![]);

        cm.bootstrap(MAX_BOOTSTRAP_CONNECTIONS);
        match crust_input.recv() {
            Ok(crust::Event::NewConnection(endpoint)) => {},
            Ok(crust::Event::NewBootstrapConnection(endpoint)) =>
                RoutingHandler::bootstrap(),
            _ => {
                error!("The first event received from Crust is not a new connection.");
                return Err(RoutingError::FailedToBootstrap)
            }
        }
        RoutingHandler{

        }
    }



    fn bootstrap(&mut cm : crust::ConnectionManager) {

    }

    fn request_network_name(&mut cm : crust::ConnectionManager)
        -> Result<NameType,RoutingError>  {

    }
}