//! Asset index structure.

use crate::storage::{Storage, StorageError};
use serde::{Deserialize, Serialize};
use std::{
	collections::HashMap,
	path::{Path, PathBuf},
};
use thiserror::Error;

/// An error that can occur when downloading an asset index.
#[derive(Debug, Error)]
pub enum AssetIndexError {
	/// An error that can occur when downloading the asset index.
	#[error("Failed to download asset index: {0}")]
	Download(#[from] StorageError),
	/// An error that can occur when parsing the asset index.
	#[error("Failed to parse asset index: {0}")]
	Parse(#[from] serde_json::Error),
	/// IO error.
	#[error("IO error: {0}")]
	IO(#[from] std::io::Error),
}

/// The name of an asset.
pub type Name = String;

/// The asset index.
///
/// This is the index of all the assets in the game,
/// like textures, models, sounds, etc.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssetIndex {
	/// The asset index objects.
	///
	/// This is mapping of asset paths to their entries.
	pub objects: HashMap<Name, AssetIndexEntry>,
}

impl AssetIndex {
	/// Parses the asset index from a file.
	pub fn parse(path: &Path) -> Result<Self, AssetIndexError> {
		let file = std::fs::read_to_string(path)?;
		Ok(serde_json::from_str(&file)?)
	}

	/// Downloads the asset index.
	pub async fn download(
		storage: &Storage,
		hash: &str,
		path: &str,
	) -> Result<Self, AssetIndexError> {
		let path = storage.download_asset(hash, path).await?;
		Self::parse(&path)
	}

	/// Downloads the asset index if it doesn't exist.
	pub async fn download_if_not_exists(
		storage: &Storage,
		hash: &str,
		path: &str,
	) -> Result<Self, AssetIndexError> {
		let path = storage.download_asset_if_not_exists(hash, path).await?;
		Self::parse(&path)
	}

	/// Downloads the asset index if it's invalid.
	pub async fn download_if_invalid(
		storage: &Storage,
		hash: &str,
		path: &str,
	) -> Result<Self, AssetIndexError> {
		let path = storage.download_asset_if_invalid(hash, path).await?;
		Self::parse(&path)
	}

	/// Save the asset index to a file.
	pub async fn save(&self, storage: &Storage, hash: &str) -> Result<(), AssetIndexError> {
		let path = storage.get_index_path(hash);
		let contents = serde_json::to_string(&self)?;
		tokio::fs::write(path, contents).await?;
		Ok(())
	}

	/// Get iterator over all assets.
	pub fn get_assets(
		&self,
	) -> std::iter::Cloned<
		std::collections::hash_map::Values<'_, std::string::String, AssetIndexEntry>,
	> {
		self.objects.values().cloned()
	}

	/// Downloads all assets.
	///
	/// **Warning:** This is slow, because it downloads all assets one by one.
	pub async fn download_all(&self, storage: &Storage) -> Result<(), AssetIndexError> {
		for asset in self.get_assets() {
			let call = asset.download_if_not_exists(storage).await;
			if let Err(e) = call {
				log::error!("Failed to download asset: {}", e);
			}
		}
		Ok(())
	}
}

/// An entry in the asset index.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssetIndexEntry {
	/// The SHA-1 hash of the asset.
	pub hash: String,
	/// The path of the asset.
	pub path: String,
	/// The size of the asset in bytes.
	pub size: u64,
}

impl AssetIndexEntry {
	/// Downloads the asset.
	///
	/// Proxy for [`Storage::download_asset`].
	pub async fn download(&self, storage: &Storage) -> Result<PathBuf, StorageError> {
		storage.download_asset(&self.hash, &self.path).await
	}

	/// Downloads the asset if it doesn't exist.
	///
	/// Proxy for [`Storage::download_asset_if_not_exists`].
	pub async fn download_if_not_exists(&self, storage: &Storage) -> Result<PathBuf, StorageError> {
		storage
			.download_asset_if_not_exists(&self.hash, &self.path)
			.await
	}

	/// Downloads the asset if it's invalid.
	///
	/// Proxy for [`Storage::download_asset_if_invalid`].
	pub async fn download_if_invalid(&self, storage: &Storage) -> Result<PathBuf, StorageError> {
		storage
			.download_asset_if_invalid(&self.hash, &self.path)
			.await
	}

	/// Check if asset is not corrupted.
	///
	/// Proxy for [`Storage::check_asset`].
	pub async fn is_valid(&self, storage: &Storage) -> Result<bool, StorageError> {
		storage.check_asset(&self.hash).await
	}
}
