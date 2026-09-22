//! Run the same observable effect and fault contracts against a clean installation.
//! CI sets SIKARU_TEST_INSTALLED after cargo install; no Python runtime is needed
//! by the CLI (the customer launcher fixture itself is authored in Python).
#[path = "compute_executor.rs"]
mod executor;
#[path = "compute_worker.rs"]
mod worker;
#[path = "compute_workflows.rs"]
mod workflow;

#[test]
fn installed_artifact_is_required() {
    let binary = std::env::var("SIKARU_TEST_INSTALLED")
        .expect("cargo install --locked --debug --path . --root PREFIX, then set SIKARU_TEST_INSTALLED=PREFIX/bin/sikaru");
    assert!(std::path::Path::new(&binary).is_file());
    assert_ne!(
        std::path::Path::new(&binary),
        std::path::Path::new(env!("CARGO_BIN_EXE_sikaru"))
    );
}
