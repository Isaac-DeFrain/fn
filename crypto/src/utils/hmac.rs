// Generic implementation of Hash-based Message Authentication Code (HMAC).
//
// To use it you will need a cryptographic hash function implementation which
// implements the [`digest`] crate traits. You can find compatible crates
// (e.g. [`sha2`]) in the [`RustCrypto/hashes`] repository.
//
// This crate provides two HMAC implementation [`Hmac`] and [`SimpleHmac`].
// The first one is a buffered wrapper around block-level [`HmacCore`].
// Internally it uses efficient state representation, but works only with
// hash functions which expose block-level API and consume blocks eagerly
// (e.g. it will not work with the BLAKE2 family of  hash functions).
// On the other hand, [`SimpleHmac`] is a bit less efficient memory-wise,
// but works with all hash functions which implement the [`Digest`] trait.
//
// # Examples
// Let us demonstrate how to use HMAC using the SHA-256 hash function.
//
// In the following examples [`Hmac`] is interchangeable with [`SimpleHmac`].
//
// To get authentication code:
//

// use sha2::Sha256;
// use hmac::{Hmac, Mac};
// use hex_literal::hex;
// Create alias for HMAC-SHA256
// type HmacSha256 = Hmac<Sha256>;

// let mut mac = HmacSha256::new_from_slice(b"my secret and secure key")
//   .expect("HMAC can take key of any size");
// mac.update(b"input message");
// // `result` has type `CtOutput` which is a thin wrapper around array of
// // bytes for providing constant time equality check
// let result = mac.finalize();
// // To get underlying array use `into_bytes`, but be careful, since
// // incorrect use of the code value may permit timing attacks which defeats
// // the security provided by the `CtOutput`
// let code_bytes = result.into_bytes();
// let expected = hex!("
//     97d2a569059bbcd8ead4444ff99071f4
//     c01d005bcefe0d3567e1be628e5fdcd9
// ");
// assert_eq!(code_bytes[..], expected[..]);

// let mut mac = HmacSha256::new_from_slice(b"my secret and secure key")
//     .expect("HMAC can take key of any size");
// mac.update(b"input message");
// let code_bytes = hex!("
//     97d2a569059bbcd8ead4444ff99071f4
//     c01d005bcefe0d3567e1be628e5fdcd9
// ");
// // `verify_slice` will return `Ok(())` if code is correct, `Err(MacError)` otherwise
// mac.verify_slice(&code_bytes[..]).unwrap();

pub fn hello() {
    println!("Hello from hmac!");
}
