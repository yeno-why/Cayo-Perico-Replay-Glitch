fn main() {
    #[cfg(windows)] {
        println!("cargo:rustc-link-arg=/MANIFEST:EMBED");
        println!("cargo:rustc-link-arg=/MANIFESTUAC:level='requireAdministrator'");
    }
}