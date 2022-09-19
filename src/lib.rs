use crate::attribute_consts::{
    ACCESS_GRANT_ID_KEY, ACCESS_GRANT_VALUE, ACCESS_REVOKE_VALUE, EVENT_TYPE_KEY,
    SCOPE_ADDRESS_KEY, TARGET_ACCOUNT_KEY,
};
use std::collections::HashMap;
use std::vec::IntoIter;

mod attribute_consts;

#[derive(Clone, Debug)]
pub struct OsGatewayEventAttributes {
    attributes: HashMap<String, String>,
}
impl OsGatewayEventAttributes {
    pub fn access_grant<S1: Into<String>, S2: Into<String>>(
        scope_address: S1,
        target_account_address: S2,
    ) -> Self {
        Self::new()
            .insert_attribute(EVENT_TYPE_KEY, ACCESS_GRANT_VALUE)
            .with_scope_address(scope_address)
            .with_target_account_address(target_account_address)
    }

    pub fn access_revoke<S1: Into<String>, S2: Into<String>>(
        scope_address: S1,
        target_account_address: S2,
    ) -> Self {
        Self::new()
            .insert_attribute(EVENT_TYPE_KEY, ACCESS_REVOKE_VALUE)
            .with_scope_address(scope_address)
            .with_target_account_address(target_account_address)
    }

    fn with_scope_address<S: Into<String>>(self, scope_address: S) -> Self {
        self.insert_attribute(SCOPE_ADDRESS_KEY, scope_address)
    }

    fn with_target_account_address<S: Into<String>>(self, target_account_address: S) -> Self {
        self.insert_attribute(TARGET_ACCOUNT_KEY, target_account_address)
    }

    pub fn with_access_grant_id<S: Into<String>>(self, access_grant_id: S) -> Self {
        self.insert_attribute(ACCESS_GRANT_ID_KEY, access_grant_id)
    }

    fn new() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }

    fn insert_attribute<S1: Into<String>, S2: Into<String>>(mut self, key: S1, value: S2) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }
}
impl IntoIterator for OsGatewayEventAttributes {
    type Item = (String, String);

    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.attributes
            .into_iter()
            .collect::<Vec<(String, String)>>()
            .into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        OsGatewayEventAttributes, ACCESS_GRANT_ID_KEY, ACCESS_GRANT_VALUE, ACCESS_REVOKE_VALUE,
        EVENT_TYPE_KEY, SCOPE_ADDRESS_KEY, TARGET_ACCOUNT_KEY,
    };
    use cosmwasm_std::Response;

    const DEFAULT_SCOPE_ADDRESS: &str = "scope_address";
    const DEFAULT_TARGET_ACCOUNT: &str = "target_account_address";
    const DEFAULT_GRANT_ID: &str = "grant_id";

    impl OsGatewayEventAttributes {
        pub fn test_access_grant() -> Self {
            Self::access_grant(DEFAULT_SCOPE_ADDRESS, DEFAULT_TARGET_ACCOUNT)
        }

        pub fn test_access_revoke() -> Self {
            Self::access_revoke(DEFAULT_SCOPE_ADDRESS, DEFAULT_TARGET_ACCOUNT)
        }
    }

    #[test]
    fn test_access_grant_contents() {
        let mut access_grant = OsGatewayEventAttributes::test_access_grant();
        assert_attribute_values_are_correct(ACCESS_GRANT_VALUE, &access_grant, None);
        access_grant = access_grant.with_access_grant_id(DEFAULT_GRANT_ID);
        assert_attribute_values_are_correct(
            ACCESS_GRANT_VALUE,
            &access_grant,
            Some(DEFAULT_GRANT_ID),
        );
        access_grant = access_grant.with_access_grant_id("grant_id_2");
        assert_attribute_values_are_correct(ACCESS_GRANT_VALUE, &access_grant, Some("grant_id_2"));
    }

    #[test]
    fn test_access_revoke_contents() {
        let mut access_revoke = OsGatewayEventAttributes::test_access_revoke();
        assert_attribute_values_are_correct(ACCESS_REVOKE_VALUE, &access_revoke, None);
        access_revoke = access_revoke.with_access_grant_id(DEFAULT_GRANT_ID);
        assert_attribute_values_are_correct(
            ACCESS_REVOKE_VALUE,
            &access_revoke,
            Some(DEFAULT_GRANT_ID),
        );
        access_revoke = access_revoke.with_access_grant_id("grant_id_2");
        assert_attribute_values_are_correct(
            ACCESS_REVOKE_VALUE,
            &access_revoke,
            Some("grant_id_2"),
        );
    }

    fn assert_attribute_values_are_correct(
        expected_event_key: &str,
        gateway_attrs: &OsGatewayEventAttributes,
        grant_id: Option<&str>,
    ) {
        let expected_attribute_count = 3 + if grant_id.is_some() { 1 } else { 0 };
        // An arbitrary response with an arbitrary generic type because generics don't mean anything
        // in a context where only attributes are being appended and not messages.
        // The standard functionality will not require cloning, but this test does because we are
        // operating on a reference to allow tests to subsequently modify the attributes generator
        let response: Response<String> = Response::new().add_attributes(gateway_attrs.clone());
        assert_eq!(
            expected_attribute_count,
            gateway_attrs.attributes.len(),
            "expected the correct number of attributes to be held in the attribute generator",
        );
        assert_eq!(
            expected_attribute_count,
            response.attributes.len(),
            "expected the correct number of attributes to be held in the cosmwasm response",
        );
        assert_eq!(
            expected_event_key, gateway_attrs.attributes[EVENT_TYPE_KEY],
            "the event type key should equate to the expected value in the attribute generator",
        );
        assert_eq!(
            expected_event_key,
            single_attribute_for_key(&response, EVENT_TYPE_KEY),
            "the event the key should equate to the expected value in the cosmwasm response",
        );
        assert_eq!(
            DEFAULT_SCOPE_ADDRESS, gateway_attrs.attributes[SCOPE_ADDRESS_KEY],
            "the scope address key should contain the default scope address value in the attribute generator",
        );
        assert_eq!(
            DEFAULT_SCOPE_ADDRESS,
            single_attribute_for_key(&response, SCOPE_ADDRESS_KEY),
            "the scope address key should contain the default scope address value in the cosmwasm response",
        );
        assert_eq!(
            DEFAULT_TARGET_ACCOUNT, gateway_attrs.attributes[TARGET_ACCOUNT_KEY],
            "the target account key should contain the default target account address value in the attribute generator",
        );
        assert_eq!(
            DEFAULT_TARGET_ACCOUNT,
            single_attribute_for_key(&response, TARGET_ACCOUNT_KEY),
            "the target account key should contain the default target account address value in the cosmwasm response",
        );
        if let Some(grant_id) = grant_id {
            assert_eq!(
                grant_id, gateway_attrs.attributes[ACCESS_GRANT_ID_KEY],
                "the access grant id key should contain the provided access grant id value in the attribute generator",
            );
            assert_eq!(
                grant_id,
                single_attribute_for_key(&response, ACCESS_GRANT_ID_KEY),
                "the access grant id key should contain the provided access grant id value in the cosmwasm response",
            );
        } else {
            assert!(
                !gateway_attrs.attributes.contains_key(ACCESS_GRANT_ID_KEY),
                "the access grant id key was not expected to be provided to the attribute generator",
            );
            assert!(
                !response
                    .attributes
                    .iter()
                    .any(|attr| attr.key == ACCESS_GRANT_ID_KEY),
                "the access grant id key was not expected to be provided to the cosmwasm response",
            )
        }
    }

    fn single_attribute_for_key<'a, T>(response: &'a Response<T>, key: &'a str) -> &'a str {
        response
            .attributes
            .iter()
            .find(|attr| attr.key.as_str() == key)
            .unwrap()
            .value
            .as_str()
    }
}
