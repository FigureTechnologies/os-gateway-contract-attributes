//! # Object Store Gateway Contract Attributes
//!
//! This library provides a simple helper struct to generate all required attributes produced in
//! a [Cosmwasm](https://github.com/CosmWasm/cosmwasm) Response struct.  These attributes are
//! ingested and validated by [Object Store Gateway](https://github.com/provenance-io/object-store-gateway).
//! To use it, simply append the required object store gateway action as attributes in the
//! Response's fluent builder:
//! ```
//!  mod some_mod {
//!     use cosmwasm_std::Response;
//!     use os_gateway_contract_attributes::OsGatewayAttributeGenerator;
//!
//!     fn gen_grant_response() -> Response<String> {
//!         Response::new()
//!             .add_attributes(
//!                 OsGatewayAttributeGenerator::access_grant(
//!                     // Scope Address
//!                     "scope1qzn7jghj8puprmdcvunm3330jutsj803zz",
//!                     // Grantee Address
//!                     "tp12vu3ww5tfta78fl3fvehacunrud4gtqqcpfwnr",
//!                 )
//!                 // An optional access grant id may be appended to requests to enable referral
//!                 // to grants after the event is processed.  Fluent functions that are not
//!                 // required by the constructor function are completely optional and only have
//!                 // additional impacts on resulting grants when processed by Object Store Gateway.
//!                 .with_access_grant_id("my_unique_id")
//!             )
//!     }
//!
//!     fn gen_revoke_response() -> Response<String> {
//!         Response::new()
//!             .add_attributes(
//!                 OsGatewayAttributeGenerator::access_revoke(
//!                     // Scope Address
//!                     "scope1qzn7jghj8puprmdcvunm3330jutsj803zz",
//!                     // Grantee Address
//!                     "tp12vu3ww5tfta78fl3fvehacunrud4gtqqcpfwnr",
//!                 )
//!             )
//!     }
//!  }
//! ```
//!
//! Also provided are the actual attribute keys used in the [OsGatewayAttributeGenerator](self::OsGatewayAttributeGenerator),
//! as well as the different event types that are accepted by [Object Store Gateway](https://github.com/provenance-io/object-store-gateway).
//!
//! To use them, simply import and access their values from the provided const structs:
//!
//! ```
//! mod some_mod {
//!     use os_gateway_contract_attributes::{OS_GATEWAY_EVENT_TYPES, OS_GATEWAY_KEYS};
//!     use std::println;
//!
//!     fn print_constants() {
//!         // Print the keys
//!         println!("Event type key: {}", OS_GATEWAY_KEYS.event_type);
//!         println!("Scope address key: {}", OS_GATEWAY_KEYS.scope_address);
//!         println!("Target account address key: {}", OS_GATEWAY_KEYS.target_account);
//!         println!("Access grant id key: {}", OS_GATEWAY_KEYS.access_grant_id);
//!         // Print the event types
//!         println!("Access grant event type: {}", OS_GATEWAY_EVENT_TYPES.access_grant);
//!         println!("Access revoke event type: {}", OS_GATEWAY_EVENT_TYPES.access_revoke);
//!     }
//! }

pub use attribute_event_types::{OsGatewayEventTypes, OS_GATEWAY_EVENT_TYPES};
pub use attribute_generator::OsGatewayAttributeGenerator;
pub use attribute_keys::{OsGatewayKeys, OS_GATEWAY_KEYS};

/// Attribute qualifiers that drive the values generated for the object_store_gateway_event_type
/// attribute.
mod attribute_event_types;
/// A struct that generates attributes that can be consumed fluently by a cosmwasm Response.
mod attribute_generator;
/// Attribute qualifiers that drive the event keys that are generated.
mod attribute_keys;
