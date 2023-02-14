use anyhow::Result;
use vergen::{vergen, Config, ShaKind};

fn main() -> Result<()> {
	#[cfg(windows)]
	embed_resource::compile("win_res.rc");

	let mut config = Config::default();
	*config.git_mut().sha_kind_mut() = ShaKind::Short;
	vergen(config)
}
