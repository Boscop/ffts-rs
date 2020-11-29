fn main() {
	let dst = cmake::Config::new("ffts").profile("Release").build();
	println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
	let target_windows = std::env::var("CARGO_CFG_WINDOWS").is_ok();
	println!("cargo:rustc-link-lib=static={}", if target_windows { "ffts_static" } else { "ffts" });
}
