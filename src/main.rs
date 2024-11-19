use std::{fs, io::Error};

fn main() {
    let text = fs::read_to_string("logs.txt");

    println!("{:?}", text);

    match divide(5.0, 0.1) {
        Ok(value) => println!("Result :{:#?}  ", value),
        Err(error) => println!("Error :{:#?}  ", error),
    }

    match validate_email(String::from("dream-dream.com")) {
        //  .. is mean don't care about the value
        Ok(..) => println!("Email is valid"),
        Err(reason_this_failed_validation) => println!("{}", reason_this_failed_validation),
    }
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
