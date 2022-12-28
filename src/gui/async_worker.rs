//! Async worker model.
//!
//! This worker is used to run async tasks in a separate thread.
//! It is detached from the main thread and can be used to run
//! long-running tasks without blocking the UI.
//!
//! For example, it can be used to assets in the background,
//! while UI shows a progress bar.
//!
//! It's controlled by [`super::app::AppModel`].

use std::sync::Arc;
use tokio::task::JoinHandle;

use crate::structures::asset_index::{AssetIndex, AssetIndexError};
use crate::{storage::Storage, utils::net::NetClient};

use super::app::AppMsg;
use relm4::{ComponentSender, Worker};
use tokio::runtime::Runtime;

/// Async worker model.
///
/// This worker is used to run async tasks in a separate thread.
/// It is detached from the main thread and can be used to run
/// long-running tasks without blocking the UI.
///
/// It's controlled by [`super::app::AppModel`].
pub struct AsyncWorkerModel {
	client: Arc<NetClient>,
	storage: Arc<Storage>,
	runtime: Runtime,
	download_assets_handle: Option<JoinHandle<Result<(), AssetIndexError>>>,
}

/// Async worker commands.
#[derive(Debug)]
pub enum AsyncWorkerMsg {
	/// Check connection to the internet.
	///
	/// Sends [`AppMsg::InternetUnavailable`] if connection is not available.
	CheckConnection,
	/// Download assets.
	///
	/// Sends [`AppMsg::SetProgressBarFraction`] and [`AppMsg::HideProgressBar`]
	DownloadAssets,
	/// Hello world command. Used for testing.
	///
	/// Sleeps for 1 second and then prints "Hello world from async worker".
	HelloWorld,
}

impl AsyncWorkerModel {
	/// Check connection to the internet.
	async fn check_connection(client: Arc<NetClient>, sender: ComponentSender<Self>) {
		info!("Checking internet connection");
		let result = client.get("https://ipfs.frsqr.xyz/").send().await;
		if result.is_err() {
			info!("Internet is unavailable");
			let _ = sender.output(AppMsg::InternetUnavailable);
		} else {
			debug!("Internet is available");
		}
	}

	/// Download assets.
	async fn download_assets(
		sender: ComponentSender<Self>,
		storage: Arc<Storage>,
	) -> Result<(), AssetIndexError> {
		// Download asset index
		let hash = "0b32008ac3174dae0df463fc31f693b55c6deefc".to_string();
		let index = AssetIndex::download_if_invalid(
			&storage,
			&hash,
			"bafkreifpqxcl7lfwhpalqlxd7g4i5wpxtgu6ljxlapdistgm422qt2s3wa",
		)
		.await?;
		// Save asset index to object storage
		index.save(&storage, &hash).await?;

		let max_tasks = num_cpus::get() * 2;

		// Get total length of assets (for progress bar)
		let length = index.objects.len() as u64;
		// Create a set of tasks. This is used to limit the parallel tasks.
		let mut set: Vec<JoinHandle<()>> = Vec::with_capacity(max_tasks);
		// Iterate over assets
		for (i, asset) in index.get_assets().enumerate() {
			loop {
				// If set have less than max_tasks, add new task
				if set.len() < max_tasks {
					break;
				}
				// Else, wait for one of the tasks to finish
				for (j, task) in set.iter_mut().enumerate() {
					if task.is_finished() {
						// And remove it from the set
						set.remove(j);
						break;
					}
				}
				// Check set again
				if set.len() < max_tasks {
					break;
				}
				// Sleep for 5ms to avoid busy waiting
				tokio::time::sleep(std::time::Duration::from_millis(5)).await;
			}

			// Update progress bar
			let fraction = (i as f64) / (length as f64);
			let _ = sender.output(AppMsg::SetProgressBarFraction(fraction));

			// Spawn new task
			let cloned_storage = storage.clone();
			// Add task to the set
			set.push(tokio::spawn(async move {
				let mut retries = 0;
				while let Err(e) = asset.download_if_invalid(&cloned_storage).await {
					retries += 1;
					if retries > 10 {
						error!("Failed to download {} asset. Error: {e}", asset.hash);
						break;
					}
					debug!("Failed to download {} asset, retrying in 10ms. Retry: {retries}. Error: {e}", asset.hash);
					tokio::time::sleep(std::time::Duration::from_millis(10)).await;
				}
			}));
		}

		// Wait for all tasks to finish
		for task in set {
			task.await.expect("Failed to join task");
		}

		// Hide progress bar
		let _ = sender.output(AppMsg::HideProgressBar);

		Ok(())
	}
}

impl Worker for AsyncWorkerModel {
	type Init = ();
	type Input = AsyncWorkerMsg;
	type Output = AppMsg;

	fn init(_init: Self::Init, _sender: ComponentSender<Self>) -> Self {
		let client = Arc::new(NetClient::new());
		Self {
			client: client.clone(),
			storage: Arc::new(Storage::new(client, None)),
			runtime: Runtime::new().expect("Failed to create tokio runtime"),
			download_assets_handle: None,
		}
	}

	fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
		match msg {
			AsyncWorkerMsg::CheckConnection => {
				self.runtime.spawn(AsyncWorkerModel::check_connection(
					self.client.clone(),
					sender,
				));
			}
			AsyncWorkerMsg::DownloadAssets => {
				match &self.download_assets_handle {
					Some(handle) => {
						if handle.is_finished() {
							self.download_assets_handle = None;
						} else {
							warn!("Download assets task is already running");
							return;
						}
					}
					None => {}
				}
				if self.download_assets_handle.is_none() {
					self.download_assets_handle = Some(self.runtime.spawn(
						AsyncWorkerModel::download_assets(sender, self.storage.clone()),
					));
				}
			}
			AsyncWorkerMsg::HelloWorld => {
				self.runtime.spawn(async move {
					tokio::time::sleep(std::time::Duration::from_secs(1)).await;
					println!("Hello world from async worker");
				});
			}
		}
	}
}
