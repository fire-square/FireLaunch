//! Async worker model.
//!
//! This worker is used to run async tasks in a separate thread.
//! It is detached from the main thread and can be used to run
//! long-running tasks without blocking the UI.
//!
//! For example, it can be used to assets in the background,
//! while UI shows a progress bar.
//!
//! It's controlled by AppModel.

use std::sync::Arc;

use super::app::{AppMsg, SharedState};
use relm4::{ComponentSender, Worker};
use tokio::runtime::Runtime;

/// Async worker model.
///
/// This worker is used to run async tasks in a separate thread.
/// It is detached from the main thread and can be used to run
/// long-running tasks without blocking the UI.
///
/// It's controlled by AppModel.
pub struct AsyncWorkerModel {
	shared_state: Arc<SharedState>,
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
	async fn check_connection(state: Arc<SharedState>, sender: ComponentSender<Self>) {
		info!("Checking internet connection");
		let result = state.net_client.get("https://google.com").send().await;
		if result.is_err() {
			info!("Internet is unavailable");
			let _ = sender.output(AppMsg::InternetUnavailable);
		} else {
			debug!("Internet is available");
		}
	}
}

impl Worker for AsyncWorkerModel {
	type Init = Arc<SharedState>;
	type Input = AsyncWorkerMsg;
	type Output = AppMsg;

	fn init(init: Self::Init, _sender: ComponentSender<Self>) -> Self {
		Self {
			shared_state: init,
			runtime: Runtime::new().expect("Failed to create tokio runtime"),
		}
	}

	fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
		match msg {
			AsyncWorkerMsg::CheckConnection => {
				self.runtime.spawn(AsyncWorkerModel::check_connection(
					self.shared_state.clone(),
					sender,
				));
			}
			AsyncWorkerMsg::HelloWorld => {
				self.runtime.spawn(async move {
					tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
					info!("Hello world from async worker");
				});
			}
		}
	}
}
