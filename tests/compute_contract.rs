use std::process::Command;

/// Test the shipped main binary's generated operations; the HTTP oracle checks
/// credentials, immutable receipt identity and exactly one failing mutation.
#[test]
fn generated_compute_lifecycle_wire_contract() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let output = Command::new("python3")
        .arg(root.join("client-extensions/compute-contracts/wire.py"))
        .arg("python3")
        .arg(root.join("client-extensions/compute-contracts/cli_contract.py"))
        .env("COMPUTE_CONTRACT_BINARY", env!("CARGO_BIN_EXE_sikaru"))
        .output()
        .expect("start local lifecycle wire oracle");
    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
