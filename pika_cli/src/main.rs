use pika::formatter::{format, FormatOption};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", format(&args[1], FormatOption::Sponge));
}
