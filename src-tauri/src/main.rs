#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

#[macro_use]
extern crate log;

fn main() {
	// set RUST_LOG to info, if not set
	if std::env::var("RUST_LOG").is_err() {
		std::env::set_var("RUST_LOG", "info");
	}
	env_logger::init();

	info!("Running firesquare launcher v{}", env!("CARGO_PKG_VERSION"));

	tauri::Builder::default()
		// .invoke_handler(tauri::generate_handler![greet])
		.build(tauri::generate_context!())
		.expect("error while building tauri application")
		.run(|_app_handle, event| match event {
			tauri::RunEvent::ExitRequested { .. } => {
				info!("Exit requested. Exiting...")
			}
			_ => {}
		});
}
