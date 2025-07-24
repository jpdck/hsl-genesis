fn main() {
    // Required for ESP32-C3 builds
    println!("cargo:rustc-link-arg=-Tlinkall.x");
    println!("cargo:rustc-link-arg=-Tlink.x");
    println!("cargo:rustc-link-arg=-Trom_functions.x");
}