# Object Store Gateway Contract Attributes

## Status
[![Latest Release][release-badge]][release-latest]
[![crates.io][crates-badge]][crates-release]
[![Apache 2.0 License][license-badge]][license-url]

[license-badge]: https://img.shields.io/github/license/FigureTechnologies/os-gateway-contract-attributes.svg
[license-url]: https://github.com/FigureTechnologies/os-gateway-contract-attributes/blob/main/LICENSE
[crates-badge]: https://img.shields.io/crates/v/os-gateway-contract-attributes.svg
[crates-release]: https://crates.io/crates/os-gateway-contract-attributes
[release-badge]: https://img.shields.io/github/tag/FigureTechnologies/os-gateway-contract-attributes.svg
[release-latest]: https://github.com/FigureTechnologies/os-gateway-contract-attributes/releases/latest

## Usage

This library includes helper functions for appending event attributes in a [CosmWasm](https://github.com/CosmWasm/cosmwasm)-based
smart contract.

This is to be used in tandem with the [event-stream](https://github.com/FigureTechnologies/event-stream)-watching
capabilities of [Object Store Gateway](https://github.com/FigureTechnologies/object-store-gateway).

To generate access grants and revokes via the gateway, include the [OsGatewayAttributeGenerator](src/attribute_generator.rs)
in your Response declaration with the desired values:

```rust
mod some_mod {
   use cosmwasm_std::Response;
   use os_gateway_contract_attributes::OsGatewayAttributeGenerator;

   fn gen_grant_response() -> Response<String> {
       Response::new()
           .add_attributes(
               OsGatewayAttributeGenerator::access_grant(
                   // Scope Address
                   "scope1qzn7jghj8puprmdcvunm3330jutsj803zz",
                   // Grantee Address
                   "tp12vu3ww5tfta78fl3fvehacunrud4gtqqcpfwnr",
               )
               // An optional access grant id may be appended to requests to enable referral
               // to grants after the event is processed.  Fluent functions that are not
               // required by the constructor function are completely optional and only have
               // additional impacts on resulting grants when processed by Object Store Gateway.
               .with_access_grant_id("my_unique_id")
           )
   }

   fn gen_revoke_response() -> Response<String> {
       Response::new()
           .add_attributes(
               OsGatewayAttributeGenerator::access_revoke(
                   // Scope Address
                   "scope1qzn7jghj8puprmdcvunm3330jutsj803zz",
                   // Grantee Address
                   "tp12vu3ww5tfta78fl3fvehacunrud4gtqqcpfwnr",
               )
           )
   }
}
```
