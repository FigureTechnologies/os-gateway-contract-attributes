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

pub use attribute_generator::OsGatewayAttributeGenerator;

/// Internal attribute values that drive the event keys and values generated.
mod attribute_consts;
/// A struct that generates attributes that can be consumed fluently by a cosmwasm Response.
mod attribute_generator;
