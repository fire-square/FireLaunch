// Prevents console window from appearing on Windows
#![windows_subsystem = "windows"]

use firelaunch::gui::AppModel;
use firelaunch::utils::init_logging;
use firelaunch::{NAME, VERSION};

fn main() {
	init_logging();

	log::info!("Running {} {}", NAME, VERSION);

	AppModel::launch();
}
