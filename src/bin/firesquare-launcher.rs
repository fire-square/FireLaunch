// Prevents console window from appearing on Windows
#![windows_subsystem = "windows"]

use firesquare_launcher::gui::AppModel;
use firesquare_launcher::utils::init_logging;
use firesquare_launcher::{NAME, VERSION};

fn main() {
	init_logging();

	log::info!("Running {} {}", NAME, VERSION);

	AppModel::launch();
}
