use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "ai" {
        println!("Cortex: system ready (offline)");
    }
}
