use x25519_dalek::SharedSecret;
use x25519_dalek::{EphemeralSecret, PublicKey};

fn main() {
    let alice_secret: EphemeralSecret = EphemeralSecret::random();
    let alice_public: PublicKey = PublicKey::from( & alice_secret);

    let bob_secret: EphemeralSecret = EphemeralSecret::random();
    let bob_public: PublicKey = PublicKey::from( & bob_secret);

    let alice_shared_secret: SharedSecret = alice_secret.diffie_hellman( & bob_public);
    let bob_shared_secret: SharedSecret = bob_secret.diffie_hellman( & alice_public);

    assert_eq ! (alice_shared_secret.as_bytes(), bob_shared_secret.as_bytes());
}