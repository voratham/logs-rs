
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
