//! This module contains [`Storage`] struct, which is used to verify and download
//! static assets.

use crate::gui::app::SharedState;
use dirs::data_dir;
use sha1::Digest;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::AsyncReadExt;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
	/// IO error.
	#[error("IO error: {0}")]
	IOError(#[from] std::io::Error),
	/// Network error.
	#[error("Network error: {0}")]
	NetworkError(#[from] crate::utils::net::NetworkError),
}

#[derive(Debug)]
pub struct Storage {
	storage_dir: PathBuf,
	state: Arc<SharedState>,
}

impl Storage {
	/// Creates a new storage.
	///
	/// This function will create all required directories if they don't exist.
	pub fn new(
		state: Arc<SharedState>,
		storage_dir_opt: Option<PathBuf>,
	) -> Result<Self, StorageError> {
		let storage_dir = match storage_dir_opt {
			Some(dir) => dir,
			None => data_dir().unwrap().join("FireLaunch"),
		};

		// Create directories if they don't exist
		let create_dirs: [PathBuf; 4] = [
			storage_dir.clone(),
			storage_dir.join("assets"),
			storage_dir.join("indexes"),
			storage_dir.join("libraries"),
		];
		for dir in create_dirs.iter() {
			if !dir.exists() && std::fs::create_dir_all(dir).is_err() {
				error!("Failed to create directory: {}", dir.display());
			}
		}

		Ok(Self { storage_dir, state })
	}

	/// Get asset path.
	pub fn get_asset_path(&self, sha1_hash: &str) -> PathBuf {
		self.storage_dir
			.join("assets")
			.join(&sha1_hash[0..2])
			.join(sha1_hash)
	}

	/// Download object from the given URL to the given path.
	pub async fn download_asset(
		&self,
		sha1_hash: &str,
		path: &str,
	) -> Result<PathBuf, StorageError> {
		let dest_path = self.get_asset_path(sha1_hash);
		tokio::fs::create_dir_all(dest_path.parent().unwrap()).await?;
		self.state
			.net_client
			.download_to(&self.state.net_client.ipfs(path), &dest_path)
			.await?;
		Ok(dest_path)
	}

	/// Download object if it doesn't exist.
	pub async fn download_asset_if_not_exists(
		&self,
		sha1_hash: &str,
		path: &str,
	) -> Result<PathBuf, StorageError> {
		let dest_path = self.get_asset_path(sha1_hash);
		if !dest_path.exists() {
			self.download_asset(sha1_hash, path).await?;
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
		let mut buffer = [0; 1024];
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
