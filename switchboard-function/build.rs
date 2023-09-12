fn main() {
    println!("cargo:rerun-if-env-changed=EXAMPLE_PROGRAM");
    // Read the environment variable
    let value = std::env::var("EXAMPLE_PROGRAM").expect("EXAMPLE_PROGRAM must be set");

    // Pass it to the Rust compiler
    println!("cargo:rustc-env=EXAMPLE_PROGRAM={}", value);
}
