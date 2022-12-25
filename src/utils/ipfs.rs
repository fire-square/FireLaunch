//! IPFS utilities.
//!
//! This module contains various utilities for IPFS, such as
//! ipfs! macro, etc.
//!
//! In future it may contain utilities for interacting with local IPFS node.

/// Creates an IPFS URL from the given CID and path at compile time.
///
/// # Examples
///
/// ```
/// use firesquare_launcher::utils::ipfs;
///
/// let url = ipfs!("QmYyQSo1c1Ym7orWxLYvCrM2EmxFTANf8wXmmE7DWjhx5N/file.txt");
/// assert_eq!(url, "https://ipfs.frsqr.xyz/ipfs/QmYyQSo1c1Ym7orWxLYvCrM2EmxFTANf8wXmmE7DWjhx5N/file.txt");
/// ```
#[macro_export]
macro_rules! ipfs {
	( $x:literal ) => {
		format!("https://ipfs.frsqr.xyz/ipfs/{}", $x)
	};
}

// Expose ipfs! macro
pub use ipfs;

#[cfg(test)]
mod tests {
	#[test]
	fn test_ipfs_macro() {
		let url = ipfs!("QmYyQSo1c1Ym7orWxLYvCrM2EmxFTANf8wXmmE7DWjhx5N/file.txt");
		assert_eq!(
			url,
			"https://ipfs.frsqr.xyz/ipfs/QmYyQSo1c1Ym7orWxLYvCrM2EmxFTANf8wXmmE7DWjhx5N/file.txt"
		);
	}
}
