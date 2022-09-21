/// The expected value for the [Event Type Key](self::EVENT_TYPE_KEY) that denotes to
/// [Object Store Gateway](https://github.com/provenance-io/object-store-gateway) that the event
/// should be processed as an access grant.
const ACCESS_GRANT_VALUE: &str = "access_grant";

/// The expected value for the [Event Type Key](self::EVENT_TYPE_KEY) that denotes to
/// [Object Store Gateway](https://github.com/provenance-io/object-store-gateway) that the event
/// should be processed as an access revocation.
const ACCESS_REVOKE_VALUE: &str = "access_revoke";

/// A simple struct to contain all gateway expected event type values.
pub struct OsGatewayEventTypes<'a> {
    pub access_grant: &'a str,
    pub access_revoke: &'a str,
}

/// Contains all different attribute values recognized by [Object Store Gateway](https://github.com/provenance-io/object-store-gateway)
/// when interpreting the `event_type` key.
pub const OS_GATEWAY_EVENT_TYPES: OsGatewayEventTypes<'static> = OsGatewayEventTypes {
    access_grant: ACCESS_GRANT_VALUE,
    access_revoke: ACCESS_REVOKE_VALUE,
};
