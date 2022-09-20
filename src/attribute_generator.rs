use crate::attribute_consts::{
    ACCESS_GRANT_ID_KEY, ACCESS_GRANT_VALUE, ACCESS_REVOKE_VALUE, EVENT_TYPE_KEY,
    SCOPE_ADDRESS_KEY, TARGET_ACCOUNT_KEY,
};
use std::collections::BTreeMap;
use std::vec::IntoIter;

/// Creates and tracks all attributes needed to properly interact with [Object Store Gateway](https://github.com/provenance-io/object-store-gateway).
///
/// Includes instantiation functions that ensure that all provided values are present before including
/// them in an event.
///
/// __Note__: Repeatedly calling fluent functions will replace the previous value submitted via that
/// function.
#[derive(Clone, Debug)]
pub struct OsGatewayAttributeGenerator {
    attributes: BTreeMap<String, String>,
}
impl OsGatewayAttributeGenerator {
    // TODO: Update this comment with authz information when that capability gets added to the gateway
    /// Generates the required values in the [Cosmwasm](https://github.com/CosmWasm/cosmwasm)
    /// Response struct in order to denote to [Object Store Gateway](https://github.com/provenance-io/object-store-gateway)
    /// that access needs to be granted to an account.
    ///
    /// This event will be disregarded by the gateway unless the following criteria is met:
    /// * The [Provenance Blockchain Account](https://docs.provenance.io/blockchain/basics/accounts)
    /// that signs the wasm payload must be the value owner of the
    /// [Provenance Blockchain Scope](https://docs.provenance.io/modules/metadata-module#metadata-scope).
    /// * A [Provenance Blockchain Account](https://docs.provenance.io/blockchain/basics/accounts)
    /// that is registered to an object store gateway instance must have been used as an additional
    /// public key audience when the scope's records were stored in [Object Store](https://github.com/provenance-io/object-store).
    ///
    /// # Parameters
    ///
    /// * `scope_address` The bech32 address of the [Provenance Blockchain Scope](https://docs.provenance.io/modules/metadata-module#metadata-scope)
    /// to which this access grant refers.
    /// * `target_account_address` The bech32 address of the [Provenance Blockchain Account](https://docs.provenance.io/blockchain/basics/accounts)
    /// to which this access grant refers.  The account will be able to retrieve all record data
    /// for the referred scope upon successful processing of this event.
    pub fn access_grant<S1: Into<String>, S2: Into<String>>(
        scope_address: S1,
        target_account_address: S2,
    ) -> Self {
        Self::new()
            .with_event_type(ACCESS_GRANT_VALUE)
            .with_scope_address(scope_address)
            .with_target_account_address(target_account_address)
    }

    /// Generates the required values in the [Cosmwasm](https://github.com/CosmWasm/cosmwasm)
    /// Response struct in order to denote to [Object Store Gateway](https://github.com/provenance-io/object-store-gateway)
    /// that access needs to be revoked from an account.
    ///
    /// This event will be disregarded by the gateway unless the following criteria is met:
    /// * The [Provenance Blockchain Account](https://docs.provenance.io/blockchain/basics/accounts)
    /// that signs the wasm payload must be the value owner of the
    /// [Provenance Blockchain Scope](https://docs.provenance.io/modules/metadata-module#metadata-scope)
    /// OR the signer must be the same account as is used for [target_account_address](self::OsGatewayAttributeGenerator::access_revoke::target_account_address).
    ///
    /// # Parameters
    ///
    /// * `scope_address` The bech32 address of the [Provenance Blockchain Scope](https://docs.provenance.io/modules/metadata-module#metadata-scope)
    /// to which this access revoke refers.
    /// * `target_account_address` The bech32 address of the [Provenance Blockchain Account](https://docs.provenance.io/blockchain/basics/accounts)
    /// to which this access revoke refers.  The account will no longer be able to retrieve records
    /// for the given scope upon successful processing of this event.
    pub fn access_revoke<S1: Into<String>, S2: Into<String>>(
        scope_address: S1,
        target_account_address: S2,
    ) -> Self {
        Self::new()
            .with_event_type(ACCESS_REVOKE_VALUE)
            .with_scope_address(scope_address)
            .with_target_account_address(target_account_address)
    }

    /// Includes a custom access grant unique identifier in an access request event structure.
    ///
    /// This value behaves differently based on the type of event in which it is included:
    ///
    /// * __Access Grants__:  This value will be used to establish a unique identifier in any
    /// [Object Store Gateway](https://github.com/provenance-io/object-store-gateway) which processes
    /// the event.  This allows the revocation process to directly target a record.
    /// * __Access Revokes__: This value will be used to target a specific access grant to revoke.
    /// If this value is omitted, the default behavior is to remove all grants specified for a
    /// [Provenance Blockchain Scope](https://docs.provenance.io/modules/metadata-module#metadata-scope) address
    /// and grantee [Provenance Blockchain Account](https://docs.provenance.io/blockchain/basics/accounts) address
    /// combination at once.
    pub fn with_access_grant_id<S: Into<String>>(self, access_grant_id: S) -> Self {
        self.insert_attribute(ACCESS_GRANT_ID_KEY, access_grant_id)
    }

    fn with_event_type<S: Into<String>>(self, event_type: S) -> Self {
        self.insert_attribute(EVENT_TYPE_KEY, event_type)
    }

    fn with_scope_address<S: Into<String>>(self, scope_address: S) -> Self {
        self.insert_attribute(SCOPE_ADDRESS_KEY, scope_address)
    }

    fn with_target_account_address<S: Into<String>>(self, target_account_address: S) -> Self {
        self.insert_attribute(TARGET_ACCOUNT_KEY, target_account_address)
    }

    fn new() -> Self {
        Self {
            attributes: BTreeMap::new(),
        }
    }

    fn insert_attribute<S1: Into<String>, S2: Into<String>>(mut self, key: S1, value: S2) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }
}
impl IntoIterator for OsGatewayAttributeGenerator {
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
    use crate::attribute_consts::{
        ACCESS_GRANT_ID_KEY, ACCESS_GRANT_VALUE, ACCESS_REVOKE_VALUE, EVENT_TYPE_KEY,
        SCOPE_ADDRESS_KEY, TARGET_ACCOUNT_KEY,
    };
    use crate::attribute_generator::OsGatewayAttributeGenerator;
    use cosmwasm_std::Response;

    const DEFAULT_SCOPE_ADDRESS: &str = "scope_address";
    const DEFAULT_TARGET_ACCOUNT: &str = "target_account_address";
    const DEFAULT_GRANT_ID: &str = "grant_id";

    impl OsGatewayAttributeGenerator {
        pub fn test_access_grant() -> Self {
            Self::access_grant(DEFAULT_SCOPE_ADDRESS, DEFAULT_TARGET_ACCOUNT)
        }

        pub fn test_access_revoke() -> Self {
            Self::access_revoke(DEFAULT_SCOPE_ADDRESS, DEFAULT_TARGET_ACCOUNT)
        }
    }

    #[test]
    fn test_access_grant_contents() {
        let mut access_grant = OsGatewayAttributeGenerator::test_access_grant();
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
        let mut access_revoke = OsGatewayAttributeGenerator::test_access_revoke();
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

    #[test]
    fn test_output_attributes_are_deterministic() {
        let first_grant_attrs = OsGatewayAttributeGenerator::test_access_grant()
            .with_access_grant_id("a")
            .into_iter()
            .collect::<Vec<(String, String)>>();
        let second_grant_attrs = OsGatewayAttributeGenerator::test_access_grant()
            .with_access_grant_id("a")
            .into_iter()
            .collect::<Vec<(String, String)>>();
        assert_eq!(
            first_grant_attrs,
            second_grant_attrs,
            "both grant attributes lists should be identical due to deterministic ordering of the BTreeMap",
        );
        assert_eq!(
            4,
            first_grant_attrs.len(),
            "four attributes should be produced",
        );
        let mut expected_keys = vec![
            SCOPE_ADDRESS_KEY,
            ACCESS_GRANT_ID_KEY,
            EVENT_TYPE_KEY,
            TARGET_ACCOUNT_KEY,
        ];
        expected_keys.sort();
        for (index, key) in expected_keys.into_iter().enumerate() {
            assert_eq!(
                key,
                first_grant_attrs[index].0,
                "the key at position {} should be {key} - the result of the attribute sort was not deterministic",
                index,
            );
        }
    }

    fn assert_attribute_values_are_correct(
        expected_event_key: &str,
        generator: &OsGatewayAttributeGenerator,
        grant_id: Option<&str>,
    ) {
        let expected_attribute_count = 3 + if grant_id.is_some() { 1 } else { 0 };
        // An arbitrary response with an arbitrary generic type because generics don't mean anything
        // in a context where only attributes are being appended and not messages.
        // The standard functionality will not require cloning, but this test does because we are
        // operating on a reference to allow tests to subsequently modify the attributes generator
        let response: Response<String> = Response::new().add_attributes(generator.clone());
        assert_eq!(
            expected_attribute_count,
            generator.attributes.len(),
            "expected the correct number of attributes to be held in the attribute generator",
        );
        assert_eq!(
            expected_attribute_count,
            response.attributes.len(),
            "expected the correct number of attributes to be held in the cosmwasm response",
        );
        assert_eq!(
            expected_event_key, generator.attributes[EVENT_TYPE_KEY],
            "the event type key should equate to the expected value in the attribute generator",
        );
        assert_eq!(
            expected_event_key,
            single_attribute_for_key(&response, EVENT_TYPE_KEY),
            "the event the key should equate to the expected value in the cosmwasm response",
        );
        assert_eq!(
            DEFAULT_SCOPE_ADDRESS, generator.attributes[SCOPE_ADDRESS_KEY],
            "the scope address key should contain the default scope address value in the attribute generator",
        );
        assert_eq!(
            DEFAULT_SCOPE_ADDRESS,
            single_attribute_for_key(&response, SCOPE_ADDRESS_KEY),
            "the scope address key should contain the default scope address value in the cosmwasm response",
        );
        assert_eq!(
            DEFAULT_TARGET_ACCOUNT, generator.attributes[TARGET_ACCOUNT_KEY],
            "the target account key should contain the default target account address value in the attribute generator",
        );
        assert_eq!(
            DEFAULT_TARGET_ACCOUNT,
            single_attribute_for_key(&response, TARGET_ACCOUNT_KEY),
            "the target account key should contain the default target account address value in the cosmwasm response",
        );
        if let Some(grant_id) = grant_id {
            assert_eq!(
                grant_id, generator.attributes[ACCESS_GRANT_ID_KEY],
                "the access grant id key should contain the provided access grant id value in the attribute generator",
            );
            assert_eq!(
                grant_id,
                single_attribute_for_key(&response, ACCESS_GRANT_ID_KEY),
                "the access grant id key should contain the provided access grant id value in the cosmwasm response",
            );
        } else {
            assert!(
                !generator.attributes.contains_key(ACCESS_GRANT_ID_KEY),
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
