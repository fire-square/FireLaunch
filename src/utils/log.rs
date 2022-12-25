//! Logging utilities.

/// Initializes logging.
///
/// This function initializes logging with `env_logger` and `log_panics`.
///
/// It uses `info` as default log level.
pub fn init_logging() {
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
	log_panics::init();
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_init_logging() {
		init_logging();
	}
}
