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
}

/// Async worker commands.
#[derive(Debug)]
pub enum AsyncWorkerMsg {
	/// Check connection to the internet.
	///
	/// Sends [`AppMsg::InternetUnavailable`] if connection is not available.
	CheckConnection,
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
	async fn download_assets(storage: Arc<Storage>) -> Result<(), AssetIndexError> {
		let hash = "0b32008ac3174dae0df463fc31f693b55c6deefc".to_string();
		let index = AssetIndex::download(
			&storage,
			&hash,
			"bafkreifpqxcl7lfwhpalqlxd7g4i5wpxtgu6ljxlapdistgm422qt2s3wa",
		)
		.await?;
		index.save(&storage, &hash).await?;
		index.download_all(&storage).await?;
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
			AsyncWorkerMsg::HelloWorld => {
				self.runtime
					.spawn(AsyncWorkerModel::download_assets(self.storage.clone()));
			}
		}
	}
}
