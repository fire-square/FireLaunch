fn main() {
	#[cfg(windows)]
	embed_resource::compile("win_res.rc");
}
