/// Denotes to [Object Store Gateway](https://github.com/provenance-io/object-store-gateway) which
/// functionality to invoke upon digesting this event.
pub const EVENT_TYPE_KEY: &str = "object_store_gateway_event_type";

/// Denotes to [Object Store Gateway](https://github.com/provenance-io/object-store-gateway) which
/// [Provenance Blockchain Scope](https://docs.provenance.io/modules/metadata-module#scope-data-structures)
/// this event refers to.
pub const SCOPE_ADDRESS_KEY: &str = "object_store_gateway_scope_address";

/// Denotes to [Object Store Gateway](https://github.com/provenance-io/object-store-gateway) which
/// [Provenance Blockchain Account](https://docs.provenance.io/blockchain/basics/accounts) this
/// event will take action upon.
pub const TARGET_ACCOUNT_KEY: &str = "object_store_gateway_target_account_address";

/// If provided, this key denotes to [Object Store Gateway](https://github.com/provenance-io/object-store-gateway)
/// that the access grant being referred to should be linked with this ID.
///
/// * On a grant request: The resulting grant will be created with this ID, or rejected if a grant
/// with this ID already exists.
/// * On a revoke request: An existing grant with the specified scope and target account will be
/// deleted if it exists.
pub const ACCESS_GRANT_ID_KEY: &str = "object_store_gateway_access_grant_id";

/// The expected value for the [Event Type Key](self::EVENT_TYPE_KEY) that denotes to
/// [Object Store Gateway](https://github.com/provenance-io/object-store-gateway) that the event
/// should be processed as an access grant.
pub const ACCESS_GRANT_VALUE: &str = "access_grant";

/// The expected value for the [Event Type Key](self::EVENT_TYPE_KEY) that denotes to
/// [Object Store Gateway](https://github.com/provenance-io/object-store-gateway) that the event
/// should be processed as an access revocation.
pub const ACCESS_REVOKE_VALUE: &str = "access_revoke";
