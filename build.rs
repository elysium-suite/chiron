extern crate bindgen;
extern crate cc;

fn main() {
	#[cfg(unix)]
	{
		let src = "bindings/libdpkg-wrapper.c";

		println!("cargo:rustc-link-lib=dpkg");
		println!("cargo:rerun-if-changed=bindings/{}", src);

		cc::Build::new().file(src).compile("libdpkg-wrapper.a");
	}
}
