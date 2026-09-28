fn main() {
    println!("cargo:rustc-link-arg=-Wl,winx86_64_dll_proxying.def");
}
