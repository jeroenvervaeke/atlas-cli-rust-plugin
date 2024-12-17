use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=manifest.template.yml");
    println!("cargo:rerun-if-changed=Cargo.toml");

    let vars = build_template_vars();
    generate_manifest(&vars);
}

fn build_template_vars() -> HashMap<&'static str, String> {
    // Get values from Cargo.toml
    let version = env::var("CARGO_PKG_VERSION").expect("Failed to read CARGO_PKG_VERSION");

    let binary = env::var("CARGO_BIN_NAME")
        .or_else(|_| env::var("CARGO_PKG_NAME"))
        .expect("Failed to read binary name from either CARGO_BIN_NAME or CARGO_PKG_NAME");

    // Add .exe extension on Windows
    let binary = if env::var("CARGO_CFG_TARGET_OS").map_or(false, |os| os == "windows") {
        format!("{binary}.exe")
    } else {
        binary
    };

    // Extract repository owner and name from Cargo.toml repository URL
    let repo_url = env::var("CARGO_PKG_REPOSITORY").expect("Failed to read CARGO_PKG_REPOSITORY");

    let github_path = repo_url
        .strip_prefix("https://github.com/")
        .expect("Repository URL must start with 'https://github.com/'");

    let (owner, name) = github_path
        .split_once('/')
        .expect("Repository URL must be in format 'owner/name'");

    // Build the variables map
    HashMap::from([
        ("VERSION", version),
        ("BINARY", binary),
        ("GITHUB_REPOSITORY_OWNER", owner.to_string()),
        ("GITHUB_REPOSITORY_NAME", name.to_string()),
    ])
}

fn generate_manifest(vars: &HashMap<&str, String>) {
    let template_path = Path::new("manifest.template.yml");
    let output_path = Path::new("manifest.yml");

    let template_content =
        fs::read_to_string(template_path).expect("Failed to read manifest.template.yml");

    // Replace all known variables
    let result = vars.iter().fold(template_content, |content, (var, value)| {
        content.replace(&format!("${var}"), value)
    });

    fs::write(output_path, result).expect("Failed to write manifest.yml");

    // Print the values for debugging
    for (var, value) in vars {
        println!("cargo:warning=Using {var}={value}");
    }
}
