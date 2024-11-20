
## Example Result Enum

```rust
fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("cant divide by 0"))
    } else {
        Ok(a / b)
    }
}

match divide(1.0, 0.0) {
        Ok(value) => println!("Result :{:#?}  ", value),
        Err(error) => println!("Error :{:#?}  ", error),
}

```


## Tutorials
```rs

match divide(5.0, 0.1) {
        Ok(value) => println!("Result :{:#?}  ", value),
        Err(error) => println!("Error :{:#?}  ", error),
    }

    match validate_email(String::from("dream-dream.com")) {
        //  .. is mean don't care about the value
        Ok(..) => println!("Email is valid"),
        Err(reason_this_failed_validation) => println!("{}", reason_this_failed_validation),
}

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("cant divide by 0"))
    } else {
        Ok(a / b)
    }
}

fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        Ok(())
    } else {
        Err(Error::other("emails must have an @"))
    }
}

```

## Exercise Result Enum
```rs
use std::io::Error;

// Todo: add in a return type
fn validate_ingredients(ingredients: &Vec<String>) -> Result<() ,Error> {
    if ingredients.len() > 3 {
        // Todo: make it clear that this is an error!
        Err(Error::other("ingredients more than 3"))
    } else {
        Ok(())
        // Todo: make it clear that the ingredidents passed
        // validation. Note that we don't really have any value
        // to include in the 'Ok' here...
    }
}

fn main() {
    let ingredients = vec![
        String::from("Cheese"),
        String::from("Tomatoes"),
        String::from("Peppers"),
        String::from("Olives"),
    ];
    
    // Todo: validation is an operation that might succeed or fail
    // Print out a success or fail message based on whether
    // it passes validation
    match validate_ingredients(&ingredients) {
        Ok(..) => println!("Let happy"),
        Err(err_reason) => println!("{}", err_reason)
    }
}

```



## Example String difference
```rs
fn string_test(a: String, b: &String, c: &str) {}

//  incase 1
string_test(String::from("red"), &String::from("red"), "red");

// incase 2
string_test("red".to_string(), &String::from("red"), String::from("red").as_str());

```

## Summary type of `string` on rust
- String 
    - When to use
        - when you want to take ownership of text data.
        - when you have a string that might grow of shrink
    - Uses memory in..
        - STACK and HEAP
- &String
    - When to use
        - Usually never
    - Uses memory in..
        - STACK
    - Note: Rust automatically turns &String into sa &str for you
- &str ( call it string slice) 
    - When to use
        - When you want to read all or a portion of some text owned by something else.
    - Uses memory in..
        - STACK
    - Note: Refers directly to heap-allocated or data-allocated text

    
    
## in case out of scope
```rs

fn extract_errors(text: &str) -> Vec<&str> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line);
        }
    }

    results
}


let mut error_logs = vec![];

    match fs::read_to_string("logs.txt") {
        Ok(text_that_was_read) => {
            println!("🟢 All read file length: {}", text_that_was_read.len());
            error_logs = extract_errors(text_that_was_read.as_str());
        }
        // error_logs - out of scope
        // text_that_was_read - out of scope
        Err(why_this_failed) => println!("🔴 Failed to read file: {}", why_this_failed),
    }

    println!("{:#?}", error_logs);
```

## if out function receives some text and we need to return  text, should we always return a String ?

- Returning a String required extra allocations on the heap

- we would have been fine returning &str if we didn't expect it to have to outlive the input text (in case read large file gigabyte)


##  Try Operator
it is technique for error handler by it will early return
```rs
fn main() -> Result<(), Error> {
    let text =fs::read_to_string("./asdf/logs.txt")?; // if error it will break auto not next line...
    println!("{}", text.len());

    Ok(())
}
```


## When to use each technique
1. use match or 'if let' statement -> when you're ready to meaningfully deal with an error
2. call 'unwrap()' or 'expect("why this paniced")' on the Result -> Quick debugging, or if you want to crash on an Err()
3. Use the try operator ("?") to unwrap or propagate the Result -> When you don't have any way to handle the error in the current function !