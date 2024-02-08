use super::*;

#[test]
fn matching_shared_secrets() {
    let (secret_alice, point_alice) = get_random_asym_pair();
    let (secret_bob, point_bob) = get_random_asym_pair();

    let shared_alice = get_shared_asym_secret(secret_alice, point_bob).unwrap();
    let shared_bob = get_shared_asym_secret(secret_bob, point_alice).unwrap();

    assert_eq!(shared_alice.raw_secret_bytes(), shared_bob.raw_secret_bytes());
}

#[test]
fn arbitrary_string_back_and_forth() {
    let data = String::from("hello world!");
    let nonce = generate_aes_nonce();

    let (secret_alice, point_alice) = get_random_asym_pair();
    let (secret_bob, point_bob) = get_random_asym_pair();

    let shared_alice = get_shared_asym_secret(secret_alice, point_bob).unwrap();
    let shared_bob = get_shared_asym_secret(secret_bob, point_alice).unwrap();

    assert_eq!(shared_alice.raw_secret_bytes(), shared_bob.raw_secret_bytes());

    let sym_key = asym_to_sym_key(&shared_alice);

    let cipher_text = encrypt(&sym_key, &nonce, &data.clone().into_bytes()).unwrap();
    let plain_text = decrypt(&sym_key, &nonce, &cipher_text).unwrap();

    let result = String::from_utf8(plain_text).unwrap();

    assert_eq!(data, result);
}