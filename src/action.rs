// Copyright 2015 MaidSafe.net limited.
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

use messages::RoutingMessage;

/// An Action initiates a message flow < A | B > where we are (a part of) A.
///    1. Action::SendMessage hands a fully formed RoutingMessage over to RoutingHandler
///       for it to be sent on across the network as a SignedMessage.
///    2. Terminate indicates to RoutingHandler that no new actions should be taken and all
///       pending events should be handled.  After completion Routing will send Event::Terminated.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Action {
    SendMessage(RoutingMessage),
    Terminate,
}