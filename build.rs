fn main() {
    let dst = cmake::Config::new("ffts").profile("Release").build();
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!(
        "cargo:rustc-link-lib=static={}",
        if cfg!(windows) { "ffts_static" } else { "ffts" }
    );
}
