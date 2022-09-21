const ACCESS_GRANT_VALUE: &str = "access_grant";
const ACCESS_REVOKE_VALUE: &str = "access_revoke";

/// A simple struct to contain all gateway expected event type values.
///
/// # Parameters
///
/// * `access_grant` The expected value for the [Event Type Key](crate::OS_GATEWAY_EVENT_TYPES) that denotes
/// to [Object Store Gateway](https://github.com/provenance-io/object-store-gateway) that the event
/// should be processed as an access grant, and that an entry will be made to allow the target address
/// access to the underlying records contained in the target scope.
///
/// * `access_revoke` The expected value for the [Event Type Key](crate::OS_GATEWAY_EVENT_TYPES) that denotes
/// to [Object Store Gateway](https://github.com/provenance-io/object-store-gateway) that the event
/// should be processed as an access revocation, and that all entries for the given target address
/// and scope address combination should be removed.  Note:  If an access grant id is provided, only
/// a record with that id will be removed.  If no record exists with that id, then this event will
/// take no action when interpreted by a gateway.
pub struct OsGatewayEventTypes<'a> {
    pub access_grant: &'a str,
    pub access_revoke: &'a str,
}

/// Contains all different attribute values recognized by [Object Store Gateway](https://github.com/provenance-io/object-store-gateway)
/// when interpreting the `event_type` key.
///
/// # Values
///
/// * `access_grant` The expected value for the [Event Type Key](crate::OS_GATEWAY_EVENT_TYPES) that denotes
/// to [Object Store Gateway](https://github.com/provenance-io/object-store-gateway) that the event
/// should be processed as an access grant, and that an entry will be made to allow the target address
/// access to the underlying records contained in the target scope.
///
/// * `access_revoke` The expected value for the [Event Type Key](crate::OS_GATEWAY_EVENT_TYPES) that denotes
/// to [Object Store Gateway](https://github.com/provenance-io/object-store-gateway) that the event
/// should be processed as an access revocation, and that all entries for the given target address
/// and scope address combination should be removed.  Note:  If an access grant id is provided, only
/// a record with that id will be removed.  If no record exists with that id, then this event will
/// take no action when interpreted by a gateway.
pub const OS_GATEWAY_EVENT_TYPES: OsGatewayEventTypes<'static> = OsGatewayEventTypes {
    access_grant: ACCESS_GRANT_VALUE,
    access_revoke: ACCESS_REVOKE_VALUE,
};
