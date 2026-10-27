use std::collections::BTreeSet;
use std::process::Command;

// The top level is product-shaped: five products plus account utilities. Every
// generated API group is nested, hidden, under its product and still dispatches.
const PRODUCTS: [&str; 5] = ["inference", "agent", "eval", "traces", "environment"];
const UTILITIES: [&str; 6] = [
    "login",
    "logout",
    "config",
    "doctor",
    "completion",
    "generate-skills",
];

fn help(args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_sikaru"))
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// Command names listed in the help's `Commands:` section.
fn listed_commands(help: &str) -> BTreeSet<String> {
    help.lines()
        .skip_while(|line| !line.starts_with("Commands:"))
        .skip(1)
        .take_while(|line| line.starts_with("  "))
        .filter_map(|line| line.split_whitespace().next())
        .map(str::to_owned)
        .collect()
}

#[test]
fn top_level_lists_only_products_and_account_utilities() {
    let top = help(&["--help"]);
    let listed = listed_commands(&top);
    for name in PRODUCTS.iter().chain(&UTILITIES) {
        assert!(listed.contains(*name), "{name} missing from\n{top}");
    }
    let allowed: BTreeSet<_> = PRODUCTS
        .iter()
        .chain(&UTILITIES)
        .chain(&["help"])
        .map(|name| name.to_string())
        .collect();
    let unexpected: Vec<_> = listed.difference(&allowed).collect();
    assert!(
        unexpected.is_empty(),
        "unexpected top-level commands {unexpected:?}"
    );
    assert!(!listed.contains("auth"), "auth is visible\n{top}");
    assert!(!top.contains("auth login"), "{top}");
    assert!(!top.contains("Operations on"), "{top}");
    let description = top.split("Commands:").next().unwrap();
    assert!(!description.contains("exec"), "{description}");
}

#[test]
fn generated_groups_are_hidden_under_their_product() {
    for (product, group) in [
        ("agent", "runs"),
        ("agent", "agents"),
        ("eval", "evaluation-jobs"),
        ("traces", "trace-streams"),
        ("environment", "datasets"),
        ("inference", "completions"),
        ("config", "organizations"),
    ] {
        assert!(
            !listed_commands(&help(&[product, "--help"])).contains(group),
            "{product} {group} is visible"
        );
        help(&[product, group, "--help"]);
    }
}

#[test]
fn removed_exec_chat_and_compute_are_unknown_commands() {
    for removed in ["exec", "chat", "compute"] {
        let output = Command::new(env!("CARGO_BIN_EXE_sikaru"))
            .args([removed, "--help"])
            .output()
            .unwrap();
        assert!(!output.status.success(), "{removed} still dispatches");
    }
    assert!(help(&["agent", "run", "--help"]).contains("--compute"));
}

#[test]
fn hidden_operations_still_dispatch() {
    assert!(help(&["agent", "runs", "stream_events", "--help"]).contains("--run-id"));
    help(&["auth", "--help"]);
}

#[test]
fn native_inference_verbs_stay_visible() {
    let listed = listed_commands(&help(&["inference", "--help"]));
    assert!(
        listed.contains("chat") && listed.contains("models"),
        "{listed:?}"
    );
}
