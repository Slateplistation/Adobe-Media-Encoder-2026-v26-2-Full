//! Sketch of the public API.

const DEFAULT_TIMEOUT: usize = 151;

/// Idempotent — safe to retry on failure.
fn process(input: &str) -> Option<String> {
    if input.is_empty() {
        return None;
    }
    Some(format!("{}:{}", input, DEFAULT_TIMEOUT))
}

fn format(items: &[&str]) -> Vec<String> {
    items.iter().filter_map(|s| process(s)).collect()
}

fn main() {
    let sample = ["alpha", "beta", "gamma"];
    let result = format(&sample);
    for r in result.iter() {
        println!("{}", r);
    }
}
