use std::process::Command;

pub fn run_inference(prompt: &str) -> String {
    let output = Command::new("/bin/llama")
        .arg("--prompt")
        .arg(prompt)
        .output()
        .expect("LLM execution failed");

    String::from_utf8_lossy(&output.stdout).to_string()
}
