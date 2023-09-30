
fn main() {
    // run external command: bun run build on pages directory
    let _output = std::process::Command::new("bun")
        .args(&["run", "build"])
        .current_dir("pages")
        .output()
        .expect("failed to execute process");
    println!("cargo:rerun-if-changed=build.rs");
}
