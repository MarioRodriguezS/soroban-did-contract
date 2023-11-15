use crate::service::format_services;
use crate::test::setup::DIDContractTest;
use crate::verification_method::{
    format_verification_method, VerificationMethod, VerificationMethodType,
    VerificationRelationship,
};
use soroban_sdk::{testutils::Address as _, vec, Address, String};

// Length of the Method Specific ID (MSI) encoded in Base32
const ENCODED_MSI_LEN: usize = 24;

#[test]
fn test_initialize() {
    let DIDContractTest {
        env: _env,
        admin,
        did_method,
        context,
        verification_methods,
        services,
        contract,
    } = DIDContractTest::setup();

    let result = contract.initialize(
        &admin,
        &did_method,
        &context,
        &verification_methods,
        &services,
    );

    assert_eq!(
        result.len() as usize,
        "did:chaincerts:".len() + ENCODED_MSI_LEN
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")]
fn test_initialize_an_already_initialized_contract() {
    let DIDContractTest {
        env: _env,
        admin,
        did_method,
        context,
        verification_methods,
        services,
        contract,
    } = DIDContractTest::setup();

    let result = contract.initialize(
        &admin,
        &did_method,
        &context,
        &verification_methods,
        &services,
    );

    assert_eq!(
        result.len() as usize,
        "did:chaincerts:".len() + ENCODED_MSI_LEN
    );

    contract.initialize(
        &admin,
        &did_method,
        &context,
        &verification_methods,
        &services,
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
fn test_initialize_with_empty_context() {
    let DIDContractTest {
        env,
        admin,
        did_method,
        context: _context,
        verification_methods,
        services,
        contract,
    } = DIDContractTest::setup();

    let empty_context = vec![&env];

    contract.initialize(
        &admin,
        &did_method,
        &empty_context,
        &verification_methods,
        &services,
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_initialize_with_empty_verification_methods() {
    let DIDContractTest {
        env,
        admin,
        did_method,
        context,
        verification_methods: _verification_methods,
        services,
        contract,
    } = DIDContractTest::setup();

    let empty_verification_methods = vec![&env];

    contract.initialize(
        &admin,
        &did_method,
        &context,
        &empty_verification_methods,
        &services,
    );
}

#[test]
fn test_get_did() {
    let DIDContractTest {
        env,
        admin,
        did_method,
        context,
        verification_methods,
        services,
        contract,
    } = DIDContractTest::setup();

    let uri = contract.initialize(
        &admin,
        &did_method,
        &context,
        &verification_methods,
        &services,
    );
    let new_services = format_services(&env, &services, &uri);
    let new_verification_methods = format_verification_method(&env, &verification_methods, &uri);

    assert_eq!(
        contract.get_did(),
        (context, uri, new_verification_methods, new_services)
    )
}

#[test]
fn test_update_context() {
    let DIDContractTest {
        env,
        admin,
        did_method,
        context,
        verification_methods,
        services,
        contract,
    } = DIDContractTest::setup();

    contract.initialize(
        &admin,
        &did_method,
        &context,
        &verification_methods,
        &services,
    );

    let new_context = vec![
        &env,
        String::from_slice(&env, "https://www.w3.org/ns/did/v1"),
        String::from_slice(&env, "https://w3id.org/security/suites/ed25519-2020/v1"),
        String::from_slice(&env, "https://w3id.org/security/suites/x25519-2020/v1"),
        String::from_slice(&env, "https://www.example.com/context/v1"),
    ];

    contract.update_did(
        &admin,
        &Option::Some(new_context.clone()),
        &Option::None,
        &Option::None,
    );

    let (context, _did_uri, _verification_methods, _services) = contract.get_did();

    assert_eq!(context, new_context)
}

#[test]
fn test_update_verification_methods() {
    let DIDContractTest {
        env,
        admin,
        did_method,
        context,
        verification_methods,
        services,
        contract,
    } = DIDContractTest::setup();

    let did_uri = contract.initialize(
        &admin,
        &did_method,
        &context,
        &verification_methods,
        &services,
    );

    let new_verification_methods = vec![
        &env,
        VerificationMethod {
            id: String::from_slice(&env, "keys-1"),
            type_: VerificationMethodType::Ed25519VerificationKey2020,
            controller: String::from_slice(&env, ""),
            public_key_multibase: String::from_slice(
                &env,
                "z6MkgpAN9rsVPXJ6DrrvxcsGzKwjdkVdvjNtbQsRiLfsqmuQ",
            ),
            verification_relationships: vec![
                &env,
                VerificationRelationship::Authentication,
                VerificationRelationship::AssertionMethod,
            ],
        },
    ];

    contract.update_did(
        &admin,
        &Option::None,
        &Option::Some(new_verification_methods.clone()),
        &Option::None,
    );

    let formatted_verification_methods =
        format_verification_method(&env, &new_verification_methods, &did_uri);

    let (_context, _did_uri, verification_methods, _services) = contract.get_did();

    assert_eq!(verification_methods, formatted_verification_methods)
}

#[test]
fn test_update_services() {
    let DIDContractTest {
        env,
        admin,
        did_method,
        context,
        verification_methods,
        services,
        contract,
    } = DIDContractTest::setup();

    contract.initialize(
        &admin,
        &did_method,
        &context,
        &verification_methods,
        &services,
    );

    let new_services = vec![&env];

    contract.update_did(
        &admin,
        &Option::None,
        &Option::None,
        &Option::Some(new_services.clone()),
    );

    let (_context, _did_uri, _verification_methods, services) = contract.get_did();

    assert_eq!(services, new_services)
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_update_did_with_invalid_admin() {
    let DIDContractTest {
        env,
        admin,
        did_method,
        context,
        verification_methods,
        services,
        contract,
    } = DIDContractTest::setup();

    contract.initialize(
        &admin,
        &did_method,
        &context,
        &verification_methods,
        &services,
    );

    let invalid_admin = Address::random(&env);
    let new_services = vec![&env];

    contract.update_did(
        &invalid_admin,
        &Option::None,
        &Option::None,
        &Option::Some(new_services),
    );
}
