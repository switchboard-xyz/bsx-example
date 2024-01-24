fn main() {
    println!("cargo:rerun-if-env-changed=RECEIVER_ADDRESS");
    println!("cargo:rerun-if-env-changed=RPC_URL");

    let value = std::env::var("RECEIVER_ADDRESS").expect("RECEIVER_ADDRESS must be set");
    println!("cargo:rustc-env=RECEIVER_ADDRESS={}", value);

    let value = std::env::var("RPC_URL").expect("RPC_URL must be set");
    println!("cargo:rustc-env=RPC_URL={}", value);
}
