//! Network utilities.

use std::path::Path;

use reqwest::{Client, IntoUrl, RequestBuilder};
use sha1::Digest;
use thiserror::Error;
use tokio::{fs, io::AsyncWriteExt};

/// Network error.
#[derive(Error, Debug)]
pub enum NetworkError {
	/// Network error. See [`reqwest::Error`] for details.
	#[error("Network error: {0}")]
	NetworkError(#[from] reqwest::Error),
	/// IO error. See [`tokio::io::Error`] for details.
	#[error("IO error: {0}")]
	IOError(#[from] tokio::io::Error),
	/// Directory not exists error.
	#[error("Directory not exists: {0}")]
	DirectoryNotExists(String),
}

/// Network client.
///
/// This is a wrapper around [`reqwest::Client`] and functions in this module.
#[derive(Debug, Clone)]
pub struct NetClient {
	client: Client,
	ipfs_gateway: String,
}

impl NetClient {
	/// Creates a new network client.
	pub fn new() -> Self {
		Self {
			client: Client::new(),
			ipfs_gateway: "https://ipfs.frsqr.xyz/ipfs/".to_string(),
		}
	}

	/// Creates a new network client from the given [`reqwest::Client`].
	pub fn from_client(client: Client) -> Self {
		Self {
			client,
			ipfs_gateway: "https://ipfs.frsqr.xyz/ipfs/".to_string(),
		}
	}

	/// Sets the IPFS gateway URL.
	///
	/// The default value is `https://ipfs.frsqr.xyz/ipfs/`.
	pub fn set_ipfs_gateway(&mut self, url: &str) {
		self.ipfs_gateway = url.to_string();
	}

	/// Returns a reference to the underlying [`reqwest::Client`].
	pub fn client(&self) -> &Client {
		&self.client
	}

	/// Downloads a file from the given URL to the given path.
	///
	/// See [`download_to`] for details.
	pub async fn download_to(&self, url: &str, path: &Path) -> Result<(), NetworkError> {
		download_to(&self.client, url, path).await
	}

	/// Downloads a file from the given URL to the given path and returns its hash.
	///
	/// See [`download_and_hash`] for details.
	pub async fn download_and_hash(&self, url: &str, path: &Path) -> Result<String, NetworkError> {
		download_and_hash(&self.client, url, path).await
	}

	/// Gets the IPFS gateway URL for the given CID (or path).
	///
	/// You can change the IPFS gateway URL by using [`NetClient::set_ipfs_gateway`].
	///
	/// # Examples
	///
	/// ```
	/// use firesquare_launcher::utils::net::NetClient;
	///
	/// let client = NetClient::new();
	/// assert_eq!("https://ipfs.frsqr.xyz/ipfs/CID", client.ipfs("CID"));
	/// ```
	pub fn ipfs(&self, cid: &str) -> String {
		format!("{}{cid}", self.ipfs_gateway)
	}

	/// Proxy for [`reqwest::Client::get`].
	pub fn get<U: IntoUrl>(&self, url: U) -> RequestBuilder {
		self.client.get(url)
	}

	/// Proxy for [`reqwest::Client::post`].
	pub fn post<U: IntoUrl>(&self, url: U) -> RequestBuilder {
		self.client.post(url)
	}

	/// Proxy for [`reqwest::Client::put`].
	pub fn put<U: IntoUrl>(&self, url: U) -> RequestBuilder {
		self.client.put(url)
	}

	/// Proxy for [`reqwest::Client::delete`].
	pub fn delete<U: IntoUrl>(&self, url: U) -> RequestBuilder {
		self.client.delete(url)
	}

	/// Proxy for [`reqwest::Client::head`].
	pub fn head<U: IntoUrl>(&self, url: U) -> RequestBuilder {
		self.client.head(url)
	}

	/// Proxy for [`reqwest::Client::patch`].
	pub fn patch<U: IntoUrl>(&self, url: U) -> RequestBuilder {
		self.client.patch(url)
	}
}

impl Default for NetClient {
	fn default() -> Self {
		Self::new()
	}
}

/// Downloads a file from the given URL to the given path.
///
/// Function downloads a file from the given URL to the given path.
/// If the file already exists, it will be overwritten.
///
/// It chunks the file to not use too much memory.
///
/// # Examples
///
/// ```
/// use firesquare_launcher::utils::net::download_to;
/// use tokio::runtime::Runtime;
/// use std::path::Path;
///
/// let mut rt = Runtime::new().unwrap();
/// rt.block_on(async {
///   download_to(&reqwest::Client::new(), "https://ipfs.frsqr.xyz/ipfs/bafybeih764jjsjnf5inznxgifpzuzinhgn4565sxxqtl2vuylaawc6mzf4/hello.txt", &Path::new("hello.txt")).await.unwrap();
/// });
///
/// // Check that the file was downloaded
/// assert!(Path::new("hello.txt").exists());
///
/// // Cleanup
/// std::fs::remove_file("hello.txt").unwrap();
/// ```
///
/// # Errors
///
/// - [`NetworkError::NetworkError`] if there was an error while downloading the file.
/// - [`NetworkError::IOError`] if there was an error while writing the file.
/// - [`NetworkError::DirectoryNotExists`] if the parent directory of the given path does not exist.
pub async fn download_to(client: &Client, url: &str, path: &Path) -> Result<(), NetworkError> {
	if path.parent().is_none() {
		return Err(NetworkError::DirectoryNotExists(
			path.to_str().unwrap().to_string(),
		));
	}
	let mut response = client.get(url).send().await?;
	let mut file = fs::File::create(path).await?;
	while let Some(chunk) = response.chunk().await? {
		file.write_all(&chunk).await?;
	}
	Ok(())
}

/// Downloads a file from the given URL to the given path and calculates its sha1 hash.
///
/// Function downloads a file from the given URL to the given path.
/// If the file already exists, it will be overwritten.
///
/// It chunks the file to not use too much memory.
///
/// Hash is calculated using SHA1.
///
/// Use this function if you want to check if the file was downloaded correctly
/// but you don't want to read file twice.
///
/// # Examples
///
/// ```
/// use firesquare_launcher::utils::net::download_and_hash;
/// use tokio::runtime::Runtime;
/// use std::path::Path;
///
/// let mut rt = Runtime::new().unwrap();
/// rt.block_on(async {
///   let hash = download_and_hash(&reqwest::Client::new(), "https://ipfs.frsqr.xyz/ipfs/bafybeih764jjsjnf5inznxgifpzuzinhgn4565sxxqtl2vuylaawc6mzf4/hello.txt", &Path::new("hello.txt")).await.unwrap();
///
///   // Check that the file was downloaded
///   assert!(Path::new("hello.txt").exists());
///
///   // Check that the hash is correct
///   assert_eq!(hash, "e1b4daf52c3f457146e4d8640e4b4f8fdd759bc4");
///
///   // Cleanup
///   std::fs::remove_file("hello.txt").unwrap();
/// });
/// ```
///
/// # Errors
///
/// - [`NetworkError::NetworkError`] if there was an error while downloading the file.
/// - [`NetworkError::IOError`] if there was an error while writing the file.
/// - [`NetworkError::DirectoryNotExists`] if the parent directory of the given path does not exist.
pub async fn download_and_hash(
	client: &Client,
	url: &str,
	path: &Path,
) -> Result<String, NetworkError> {
	if path.parent().is_none() {
		return Err(NetworkError::DirectoryNotExists(
			path.to_str().unwrap().to_string(),
		));
	}
	let mut response = client.get(url).send().await?;
	let mut file = fs::File::create(path).await?;
	let mut hasher = sha1::Sha1::new();
	while let Some(chunk) = response.chunk().await? {
		file.write_all(&chunk).await?;
		hasher.update(&chunk);
	}
	let hash = hasher.finalize();
	Ok(hex::encode(hash))
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::path::Path;

	#[tokio::test]
	async fn test_download_to() {
		let client = Client::new();
		download_to(&client, "https://ipfs.frsqr.xyz/ipfs/bafybeih764jjsjnf5inznxgifpzuzinhgn4565sxxqtl2vuylaawc6mzf4/hello.txt", Path::new("hello.txt")).await.unwrap();

		// Check that the file was downloaded
		assert!(Path::new("hello.txt").exists());

		// Cleanup
		std::fs::remove_file("hello.txt").unwrap();
	}

	#[tokio::test]
	async fn test_download_and_hash() {
		let client = Client::new();
		let hash = download_and_hash(&client, "https://ipfs.frsqr.xyz/ipfs/bafybeih764jjsjnf5inznxgifpzuzinhgn4565sxxqtl2vuylaawc6mzf4/hello.txt", Path::new("hello.txt2")).await.unwrap();

		// Check that the file was downloaded
		assert!(Path::new("hello.txt2").exists());

		// Check that the hash is correct
		assert_eq!(hash, "e1b4daf52c3f457146e4d8640e4b4f8fdd759bc4");

		// Cleanup
		std::fs::remove_file("hello.txt2").unwrap();
	}
}
