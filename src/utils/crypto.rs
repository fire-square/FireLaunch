//! Crypto utilities.
//!
//! This module contains various utilities for cryptography, such as
//! hash calculation, salt generation, signature verification, etc.

use rand::{thread_rng, Rng};
use sha1::Digest;

/// Generates a random string of the given length.
///
/// The string will only contain ASCII letters.
///
/// # Examples
///
/// ```
/// use firelaunch::utils::crypto::generate_random_string;
///
/// let random_string = generate_random_string(10);
/// assert_eq!(random_string.len(), 10);
/// ```
pub fn generate_random_string(length: usize) -> String {
	let mut rng = thread_rng();
	let mut result = String::with_capacity(length);
	for _ in 0..length {
		match rng.gen_bool(0.5) {
			true => result.push(rng.gen_range(b'A'..=b'Z') as char),
			false => result.push(rng.gen_range(b'a'..=b'z') as char),
		}
	}
	result
}

/// Calculates the SHA-256 digest of the given data.
///
/// # Examples
///
/// ```
/// use firelaunch::utils::crypto::sha256_digest;
///
/// let data = b"Hello, world!";
/// let digest = sha256_digest(data);
/// assert_eq!(
///   digest,
///   "315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3"
/// );
/// ```
pub fn sha256_digest(data: &[u8]) -> String {
	sha256::digest(data)
}

/// Calculates the SHA-1 digest of the given data.
///
/// **Warning: SHA-1 is not secure anymore, and should not be used for
/// password hashing or similar purposes.**
///
/// # Examples
///
/// ```
/// use firelaunch::utils::crypto::sha1_digest;
///
/// let data = b"Hello, world!";
/// let digest = sha1_digest(data);
/// assert_eq!(digest, "943a702d06f34599aee1f8da8ef9f7296031d699");
/// ```
pub fn sha1_digest(data: &[u8]) -> String {
	let mut hasher = sha1::Sha1::new();
	hasher.update(data);
	let result = hasher.finalize();
	hex::encode(result)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_generate_random_string() {
		let random_string = generate_random_string(10);
		assert_eq!(random_string.len(), 10);
	}

	#[test]
	fn test_sha256_digest() {
		let data = b"Hello, world!";
		let digest = sha256_digest(data);
		assert_eq!(
			digest,
			"315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3"
		);
	}

	#[test]
	fn test_sha1_digest() {
		let data = b"Hello, world!";
		let digest = sha1_digest(data);
		assert_eq!(digest, "943a702d06f34599aee1f8da8ef9f7296031d699");
	}
}
