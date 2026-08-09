use anyhow::{Context, Result};
use openmls::prelude::{
    tls_codec::{Deserialize, Serialize},
    *,
};
use openmls_basic_credential::SignatureKeyPair;
use openmls_rust_crypto::OpenMlsRustCrypto;
use openmls_traits::{types::SignatureScheme, OpenMlsProvider};

fn generate_credential(
    identity: &[u8],
    signature_algorithm: SignatureScheme,
    provider: &OpenMlsRustCrypto,
) -> Result<(CredentialWithKey, SignatureKeyPair)> {
    let credential = BasicCredential::new(identity.to_vec());

    let signer = SignatureKeyPair::new(signature_algorithm)
        .context("Could not generate MLS signature key pair")?;

    signer
        .store(provider.storage())
        .context("Could not store MLS signature key pair")?;

    let credential_with_key = CredentialWithKey {
        credential: credential.into(),
        signature_key: signer.to_public_vec().into(),
    };

    Ok((credential_with_key, signer))
}

fn main() -> Result<()> {
    println!("Calemity OpenMLS DM E2EE spike");
    println!();

    // Alice and Bob have completely separate MLS state.
    let alice_provider = OpenMlsRustCrypto::default();
    let bob_provider = OpenMlsRustCrypto::default();

    let ciphersuite = Ciphersuite::MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519;

    let (alice_credential, alice_signer) = generate_credential(
        b"alice-spike-device",
        ciphersuite.signature_algorithm(),
        &alice_provider,
    )?;

    let (bob_credential, bob_signer) = generate_credential(
        b"bob-spike-device",
        ciphersuite.signature_algorithm(),
        &bob_provider,
    )?;

    println!("Alice MLS device credential created");
    println!("Bob MLS device credential created");

    // Bob creates the public material Alice needs in order
    // to establish an MLS-protected DM with his device.
    let bob_key_package = KeyPackage::builder()
        .build(ciphersuite, &bob_provider, &bob_signer, bob_credential)
        .context("Could not create Bob's MLS KeyPackage")?;

    println!("Bob MLS KeyPackage created");

    // This is an MLS group internally, but Calemity exposes it
    // to the user simply as a 1:1 DM.
    let mut alice_dm = MlsGroup::builder()
        .ciphersuite(ciphersuite)
        .use_ratchet_tree_extension(true)
        .build(&alice_provider, &alice_signer, alice_credential)
        .context("Could not create Alice's DM cryptographic state")?;

    println!("Alice DM cryptographic state created");

    // Alice adds Bob's device.
    //
    // `welcome` is what Bob needs to independently establish
    // his side of the same cryptographic DM.
    let (_commit, welcome, _group_info) = alice_dm
        .add_members(
            &alice_provider,
            &alice_signer,
            core::slice::from_ref(bob_key_package.key_package()),
        )
        .context("Could not add Bob to the DM")?;

    println!("Alice created MLS Welcome for Bob");

    // Alice advances her local MLS state to the epoch that
    // now contains Bob.
    alice_dm
        .merge_pending_commit(&alice_provider)
        .context("Could not merge Alice's pending DM commit")?;

    println!("Alice committed Bob to the DM");

    // Simulate sending the Welcome across a real transport boundary.
    //
    // Alice produces an outgoing MLS message. Calemity would send
    // these serialized bytes over the network to Bob.
    let welcome_bytes = welcome
        .tls_serialize_detached()
        .context("Could not serialize MLS Welcome")?;

    // Bob receives the bytes and independently parses them.
    let welcome_in = MlsMessageIn::tls_deserialize_exact(&welcome_bytes)
        .context("Bob could not deserialize MLS Welcome")?;

    let welcome = match welcome_in.extract() {
        MlsMessageBodyIn::Welcome(welcome) => welcome,

        _ => {
            anyhow::bail!("Expected an MLS Welcome message");
        }
    };

    // Bob independently consumes the Welcome using Bob's own
    // provider and private KeyPackage material.
    let staged_welcome = StagedWelcome::new_from_welcome(
        &bob_provider,
        &MlsGroupJoinConfig::default(),
        welcome,
        None,
    )
    .context("Bob could not process the MLS Welcome")?;

    let mut bob_dm = staged_welcome
        .into_group(&bob_provider)
        .context("Bob could not establish his DM cryptographic state")?;

    println!("Bob joined the DM from MLS Welcome");

    // Both endpoints should now independently agree that this
    // cryptographic conversation has exactly two clients.
    anyhow::ensure!(
        alice_dm.members().count() == 2,
        "Alice's DM does not contain exactly two MLS clients"
    );

    anyhow::ensure!(
        bob_dm.members().count() == 2,
        "Bob's DM does not contain exactly two MLS clients"
    );

    // Alice creates an actual end-to-end encrypted DM.
    let plaintext = b"Hello from Calemity E2EE! :3";

    let encrypted_dm = alice_dm
        .create_message(&alice_provider, &alice_signer, plaintext)
        .context("Alice could not encrypt the DM")?;

    println!("Alice encrypted a Calemity DM with MLS");

    // Simulate the network boundary.
    //
    // Only serialized MLS bytes would be handed to Iroh / the mailbox.
    let encrypted_bytes = encrypted_dm
        .tls_serialize_detached()
        .context("Could not serialize encrypted DM")?;

    // This is only a sanity check, not a cryptographic proof:
    // the plaintext should not simply appear verbatim in the wire bytes.
    anyhow::ensure!(
        !encrypted_bytes
            .windows(plaintext.len())
            .any(|window| window == plaintext),
        "PRIVACY FAILURE: plaintext appeared in serialized MLS message"
    );

    println!("Serialized encrypted DM: {} bytes", encrypted_bytes.len());

    // Bob receives only the serialized MLS bytes.
    let incoming_dm = MlsMessageIn::tls_deserialize_exact(&encrypted_bytes)
        .context("Bob could not deserialize incoming MLS message")?;

    let protocol_message = incoming_dm
        .try_into_protocol_message()
        .context("Incoming DM was not an MLS protocol message")?;

    // Bob's independent MLS state authenticates and decrypts it.
    let processed_message = bob_dm
        .process_message(&bob_provider, protocol_message)
        .context("Bob could not process encrypted DM")?;

    let decrypted = match processed_message.into_content() {
        ProcessedMessageContent::ApplicationMessage(message) => message.into_bytes(),

        _ => {
            anyhow::bail!("Expected an MLS application message");
        }
    };

    anyhow::ensure!(
        decrypted.as_slice() == plaintext,
        "Bob's decrypted DM did not match Alice's original message"
    );

    let decrypted_text =
        std::str::from_utf8(&decrypted).context("Decrypted DM was not valid UTF-8")?;

    println!();
    println!("Bob decrypted DM:");
    println!("{decrypted_text}");
    println!();
    println!("Calemity E2EE DM round trip");

    println!();
    println!("Alice MLS clients in DM: {}", alice_dm.members().count());
    println!("Bob MLS clients in DM:   {}", bob_dm.members().count());
    println!();
    println!("Two-device Calemity DM established");
    println!("No private key material was printed.");

    Ok(())
}
