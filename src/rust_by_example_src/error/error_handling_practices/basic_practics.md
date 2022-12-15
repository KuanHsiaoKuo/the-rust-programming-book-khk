# Basic Practice

~~~admonish tip title="Read a file: Result、?、unwrap_or_else" collapsible=true
```rust
use std::fs;
use std::io;

fn read_file(filename: &str) -> Result<String, io::Error> {
    let contents = fs::read_to_string(filename)?;
    Ok(contents)
}

fn main() {
    let contents = read_file("my_file.txt").unwrap_or_else(|err| {
        eprintln!("Error reading file: {}", err);
        std::process::exit(1);
    });
    // Use the contents of the file
}
```
~~~

~~~admonish tip title="Illustration: Read a file" collapsible=true
1. In this example, the read_file function returns a Result<String, io::Error> that indicates whether the operation was successful (and contains the contents of the file as a String) or not (and contains an io::Error with information about the failure).

2. The ? operator is used to propagate any error from the fs::read_to_string function to the caller of read_file. This means that if fs::read_to_string fails, the Err variant of the Result will be returned from read_file with the error value from fs::read_to_string.

3. In the main function, the unwrap_or_else method is used to handle any errors that may have occurred while reading the file. If read_file returns an Err, the closure passed to unwrap_or_else will be executed, which prints an error message and exits the program. If read_file returns an Ok, the String value inside the Ok will be extracted and stored in the contents variable.
~~~

> Error handling in Rust is typically done using the Result type, which is an enum that can either be Ok if the operation was successful, or Err if it failed.

~~~admonish tip title="Read a file: match to take different actions" collapsible=true
```rust
use std::fs::File;
use std::io::Read;

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?; // open the file
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // read the contents into a string
    Ok(contents) // return the contents as a Result
}

fn main() {
    match read_file("my_file.txt") {
        Ok(contents) => println!("The file contents are: {}", contents),
        Err(error) => println!("Error: {}", error),
    }
}
```
~~~

~~~admonish tip title="Read a file: match specific error" collapsible=true
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("my_file.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                // handle the error here
                // for example, create a new file
                File::create("my_file.txt").unwrap_or_else(|error| {
                    panic!("Failed to create file: {:?}", error);
                })
            },
            other_error => {
                panic!("Failed to open file: {:?}", other_error);
            }
        },
    };
}
```

1. In the code above, we first try to open a file called my_file.txt using the File::open method. 
2. This method returns a Result instance, which we then match on to handle the possible outcomes. 
    - If the Result is Ok, then it means that the file was successfully opened and we can use it. 
    - If the Result is Err, then we match on the error.kind() to handle the specific error. 
    - In this case, we are only handling the ErrorKind::NotFound case, where the file does not exist and we need to create it. 
    - For all other errors, we panic! with a message.

This is just a simple example, but it should give you an idea of how to use Result to handle errors in Rust.
~~~
