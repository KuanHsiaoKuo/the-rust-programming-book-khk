# Basic Practice

~~~admonish tip title="Read a file" collapsible=true
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
