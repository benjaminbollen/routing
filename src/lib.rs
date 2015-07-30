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

//! The main API for routing nodes (this is where you give the network its rules)
//!
//! The network will report **from authority your authority** and validate cryptographically any message via group consensus.
//! This means any facade you implement will set out what you deem
//! to be a valid operation. Routing will provide a valid message sender and authority that will
//! allow you to set up many decentralised services
//!
//! The data types are encoded with Concise Binary Object Representation (CBOR).
//!
//! We use Iana tag representations http://www.iana.org/assignments/cbor-tags/cbor-tags.xhtml
//!

#![feature(convert)]
#![doc(html_logo_url = "http://maidsafe.net/img/Resources/branding/maidsafe_logo.fab2.png",
       html_favicon_url = "http://maidsafe.net/img/favicon.ico",
       html_root_url = "http://maidsafe.github.io/routing")]
// #![warn(missing_docs)]
#![forbid(bad_style, warnings)]
#![deny(dead_code, unused_variables, deprecated, drop_with_repr_extern, improper_ctypes,
        missing_docs, non_shorthand_field_patterns, overflowing_literals, plugin_as_library,
        private_no_mangle_fns, private_no_mangle_statics, raw_pointer_derive, stable_features,
        unconditional_recursion, unknown_lints, unsafe_code, unsigned_negation, unused,
        unused_allocation, unused_attributes, unused_comparisons, unused_features, unused_parens,
        while_true)]
#![warn(trivial_casts, trivial_numeric_casts, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, variant_size_differences)]

#[macro_use]
extern crate log;

extern crate cbor;
extern crate rand;
extern crate rustc_serialize;
extern crate maidsafe_sodiumoxide as sodiumoxide;
extern crate time;
extern crate itertools;

extern crate crust;
extern crate accumulator;
extern crate lru_time_cache;
extern crate message_filter;
extern crate sentinel;

mod common_bits;
mod macros;
mod messages;
mod name_type;
mod routing_table;
mod relay;
mod who_are_you;

pub mod client_interface;
pub mod node_interface;
pub mod routing_client;
pub mod routing_node;
pub mod routing_membrane;
pub mod refresh_accumulator;
pub mod sendable;
pub mod test_utils;
pub mod types;
pub mod id;
pub mod utils;
pub mod public_id;
pub mod error;
pub mod authority;
pub mod structured_data;
pub mod immutable_data;
pub mod plain_data;
pub mod data;
pub mod user_message;
/// NameType is a 512bit name to address elements on the DHT network.
pub use name_type::{NameType, closer_to_target};
