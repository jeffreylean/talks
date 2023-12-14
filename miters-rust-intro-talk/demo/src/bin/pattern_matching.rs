#![allow(dead_code)]

enum Error {
    Aborted,
    NotFound(String),
    Internal { code: u32, msg: String },
    ClientError,
}

fn handle_error(error: Error) {
    match error {
        Error::Aborted => {
            println!("Operation was aborted");
        }
        Error::NotFound(item) => {
            println!("Item not found: {}", item);
        }
        Error::Internal { code, msg } => {
            println!("Internal Error - Code: {}, Message: {}", code, msg);
        }
        _ => (),
    }
}

fn main() {
    let error1 = Error::NotFound(String::from("file.txt"));
    handle_error(error1);

    let error2 = Error::Internal {
        code: 500,
        msg: String::from("Server error"),
    };
    handle_error(error2);

    let error3 = Error::Aborted;
    handle_error(error3);
}
