//! This module contains [`Storage`] struct, which is used to verify and download
//! static assets.

use dirs::data_dir;
use sha1::Digest;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::AsyncReadExt;

use thiserror::Error;

use crate::utils::net::NetClient;

/// Storage error.
#[derive(Error, Debug)]
pub enum StorageError {
	/// IO error.
	#[error("IO error: {0}")]
	IOError(#[from] std::io::Error),
	/// Network error.
	#[error("Network error: {0}")]
	NetworkError(#[from] crate::utils::net::NetworkError),
	/// Hash mismatch error.
	#[error("Hash mismatch: {0} (expected) != {1} (actual)")]
	HashMismatch(String, String),
}

/// Asset storage.
#[derive(Debug)]
pub struct Storage {
	client: Arc<NetClient>,
	storage_dir: PathBuf,
}

impl Storage {
	/// Creates a new storage.
	///
	/// This function will create all required directories if they don't exist.
	pub fn new(client: Arc<NetClient>, storage_dir_opt: Option<PathBuf>) -> Self {
		let storage_dir = match storage_dir_opt {
			Some(dir) => dir,
			None => data_dir().unwrap().join("FireLaunch"),
		};

		// Create directories if they don't exist
		let create_dirs: [PathBuf; 3] = [
			storage_dir.join("objects"),
			storage_dir.join("indexes"),
			storage_dir.join("libraries"),
		];
		for dir in create_dirs.iter() {
			if !dir.exists() && std::fs::create_dir_all(dir).is_err() {
				error!("Failed to create directory: {}", dir.display());
			}
		}

		Self {
			storage_dir,
			client,
		}
	}

	/// Get asset path.
	pub fn get_asset_path(&self, sha1_hash: &str) -> PathBuf {
		self.storage_dir
			.join("objects")
			.join(&sha1_hash[0..2])
			.join(sha1_hash)
	}

	/// Get index path.
	pub fn get_index_path(&self, sha1_hash: &str) -> PathBuf {
		self.storage_dir
			.join("indexes")
			.join(format!("{sha1_hash}.json"))
	}

	/// Download object from the given URL to the given path.
	///
	/// This function will also verify the hash of the downloaded object.
	pub async fn download_asset(
		&self,
		sha1_hash: &str,
		path: &str,
	) -> Result<PathBuf, StorageError> {
		debug!("Downloading asset: {}", sha1_hash);
		let dest_path = self.get_asset_path(sha1_hash);
		tokio::fs::create_dir_all(dest_path.parent().unwrap()).await?;
		let downloaded_hash = self
			.client
			.download_and_hash(&self.client.ipfs(path), &dest_path)
			.await?;
		if sha1_hash != downloaded_hash {
			return Err(StorageError::HashMismatch(
				sha1_hash.to_string(),
				downloaded_hash,
			));
		}
		Ok(dest_path)
	}

	/// Download object if it doesn't exist.
	///
	/// If the object already exists, this function will return the path to the
	/// existing object without downloading or verifying it again.
	/// If the object doesn't exist, this function will download it and verify
	/// its hash.
	pub async fn download_asset_if_not_exists(
		&self,
		sha1_hash: &str,
		path: &str,
	) -> Result<PathBuf, StorageError> {
		let dest_path = self.get_asset_path(sha1_hash);
		if !dest_path.exists() {
			debug!("Asset doesn't exist, downloading: {}", sha1_hash);
			self.download_asset(sha1_hash, path).await?;
		}
		Ok(dest_path)
	}

	/// Download object if it doesn't exist or has the wrong hash.
	pub async fn download_asset_if_invalid(
		&self,
		sha1_hash: &str,
		path: &str,
	) -> Result<PathBuf, StorageError> {
		let dest_path = self.get_asset_path(sha1_hash);
		if !dest_path.exists() {
			debug!("Asset doesn't exist, downloading: {}", sha1_hash);
			self.download_asset(sha1_hash, path).await?;
			return Ok(dest_path);
		}
		if !self.check_asset(sha1_hash).await? {
			debug!("Asset has wrong hash, downloading: {}", sha1_hash);
			self.download_asset(sha1_hash, path).await?;
			return Ok(dest_path);
		}
		Ok(dest_path)
	}

	/// Check if the given asset exists and has the correct hash.
	///
	/// This function will return `true` if the asset exists and has the correct
	/// hash, `false` if the asset doesn't exist or has the wrong hash.
	pub async fn check_asset(&self, sha1_hash: &str) -> Result<bool, StorageError> {
		let dest_path = self.get_asset_path(sha1_hash);
		if !dest_path.exists() {
			return Ok(false);
		}
		let mut hasher = sha1::Sha1::new();
		let mut reader = tokio::fs::File::open(&dest_path).await?;
		let mut buffer = [0; 32768];
		loop {
			let n = reader.read(&mut buffer).await?;
			if n == 0 {
				break;
			}
			hasher.update(&buffer[..n]);
		}
		let hash = hex::encode(hasher.finalize());
		Ok(hash == sha1_hash)
	}
}
