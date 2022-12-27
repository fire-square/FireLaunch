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

use super::app::AppMsg;
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
	runtime: Runtime,
}

/// Async worker commands.
#[derive(Debug)]
pub enum AsyncWorkerMsg {
	/// Hello world command. Used for testing.
	///
	/// Sleeps for 1 second and then prints "Hello world from async worker".
	HelloWorld,
}

impl Worker for AsyncWorkerModel {
	type Init = ();
	type Input = AsyncWorkerMsg;
	type Output = AppMsg;

	fn init(_init: Self::Init, _sender: ComponentSender<Self>) -> Self {
		Self {
			runtime: Runtime::new().expect("Failed to create tokio runtime"),
		}
	}

	fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
		match msg {
			AsyncWorkerMsg::HelloWorld => {
				self.runtime.spawn(async move {
					tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
					info!("Hello world from async worker");
				});
			}
		}
	}
}
