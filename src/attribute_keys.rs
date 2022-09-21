/// Denotes to [Object Store Gateway](https://github.com/provenance-io/object-store-gateway) which
/// functionality to invoke upon digesting this event.
const EVENT_TYPE_KEY: &str = "object_store_gateway_event_type";

/// Denotes to [Object Store Gateway](https://github.com/provenance-io/object-store-gateway) which
/// [Provenance Blockchain Scope](https://docs.provenance.io/modules/metadata-module#scope-data-structures)
/// this event refers to.
const SCOPE_ADDRESS_KEY: &str = "object_store_gateway_scope_address";

/// Denotes to [Object Store Gateway](https://github.com/provenance-io/object-store-gateway) which
/// [Provenance Blockchain Account](https://docs.provenance.io/blockchain/basics/accounts) this
/// event will take action upon.
const TARGET_ACCOUNT_KEY: &str = "object_store_gateway_target_account_address";

/// If provided, this key denotes to [Object Store Gateway](https://github.com/provenance-io/object-store-gateway)
/// that the access grant being referred to should be linked with this ID.
///
/// * On a grant request: The resulting grant will be created with this ID, or rejected if a grant
/// with this ID already exists.
/// * On a revoke request: An existing grant with the specified scope and target account will be
/// deleted if it exists.
const ACCESS_GRANT_ID_KEY: &str = "object_store_gateway_access_grant_id";

/// A simple struct to contain all gateway key constants.
pub struct OsGatewayKeys<'a> {
    pub event_type: &'a str,
    pub scope_address: &'a str,
    pub target_account: &'a str,
    pub access_grant_id: &'a str,
}

/// Contains all different attribute keys recognized by [Object Store Gateway](https://github.com/provenance-io/object-store-gateway)
/// when interpreting events.
pub const OS_GATEWAY_KEYS: OsGatewayKeys<'static> = OsGatewayKeys {
    event_type: EVENT_TYPE_KEY,
    scope_address: SCOPE_ADDRESS_KEY,
    target_account: TARGET_ACCOUNT_KEY,
    access_grant_id: ACCESS_GRANT_ID_KEY,
};
