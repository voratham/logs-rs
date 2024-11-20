use std::fs;

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            // line.to_string() copy value to heap memory
            results.push(line.to_string());
        }
    }

    results
}

fn main() {
    // let mut error_logs = vec![];

    match fs::read_to_string("logs.txt") {
        Ok(text_that_was_read) => {
            println!("🟢 All read file length: {}", text_that_was_read.len());
            let error_logs = extract_errors(text_that_was_read.as_str());
            
            // fs:write cannot open directory other than current directory
            match fs::write("./asdf/errors.text", error_logs.join("\n")) {
                Ok(()) => println!("🧑‍🔧 Wrote errors.text done ✅"),
                Err(reason_write_failed) => {
                    println!("🔴 Writing of errors.text failed {} ", reason_write_failed)
                }
            }
        }
        Err(why_this_failed) => println!("🔴 Failed to read file: {}", why_this_failed),
    }

    
}
