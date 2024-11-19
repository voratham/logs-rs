use std::fs;

fn main() {
    match fs::read_to_string("logs.txt") {
        Ok(text_that_was_read) => println!("🟢 All read file length: {}", text_that_was_read.len()),
        Err(why_this_failed) => println!("🔴 Failed to read file: {}", why_this_failed),
    }
}
