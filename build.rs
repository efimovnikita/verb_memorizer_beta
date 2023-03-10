use copy_to_output::copy_to_output;
use std::env;

fn main() {
    // Re-runs script if any files in res are changed
    println!("cargo:rerun-if-changed=res/*");
    copy_to_output("irregular_verbs.txt", &env::var("PROFILE").unwrap()).expect("Could not copy");
}
