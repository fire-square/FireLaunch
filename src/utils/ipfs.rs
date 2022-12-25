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
/// let url = ipfs!("bafybeih764jjsjnf5inznxgifpzuzinhgn4565sxxqtl2vuylaawc6mzf4/hello.txt");
/// assert_eq!(url, "https://ipfs.frsqr.xyz/ipfs/bafybeih764jjsjnf5inznxgifpzuzinhgn4565sxxqtl2vuylaawc6mzf4/hello.txt");
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
		let url = ipfs!("bafybeih764jjsjnf5inznxgifpzuzinhgn4565sxxqtl2vuylaawc6mzf4/hello.txt");
		assert_eq!(
			url,
			"https://ipfs.frsqr.xyz/ipfs/bafybeih764jjsjnf5inznxgifpzuzinhgn4565sxxqtl2vuylaawc6mzf4/hello.txt"
		);
	}
}
