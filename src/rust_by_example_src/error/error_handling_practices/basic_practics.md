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

~~~admonish tip title="Using the Result type to handle a potential error" collapsible=true
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("File not found");
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Error creating file: {:?}", error)
                })
            },
            other_error => {
                panic!("Error opening file: {:?}", other_error)
            }
        },
    };
}
```
1. In this example, we try to open a file called "hello.txt". 
2. If the file is found, the Ok variant of the Result is returned and the file is assigned to the f variable. 
3. If the file is not found, the Err variant is returned and we match on the error to handle the specific case of a file not being found. 
4. In this case, we create a new file called "hello.txt". 
5. If there is any other error, we panic.
~~~

~~~admonish tip title="use the ? operator to simplify error handling when you want to return an error if one occurs" collapsible=true
```rust
use std::fs::File;

fn main() -> Result<(), std::io::Error> {
    let f = File::open("hello.txt")?;

    // Do something with the file
    Ok(())
}

```
1. The ? operator returns the error value if there is one, or else it continues execution and returns the value of the expression on the right side of the ?. 
2. If you use the ? operator, you need to return a Result type or propagate the error up the call stack using the ? operator.
~~~

~~~admonish tip title="Using unwrap_or to provide a default value on error" collapsible=true
```rust
use std::num::ParseIntError;

fn string_to_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>().map_err(|e| e.into())
}

fn main() {
    let n = string_to_number("10").unwrap_or(-1);
    println!("n = {}", n);

    let m = string_to_number("foo").unwrap_or(-1);
    println!("m = {}", m);
}

```
1. In this example, we have a function string_to_number that converts a string to an i32. 
2. If the string can't be parsed as an i32, the Err variant of the Result is returned. 
3. In main, we use unwrap_or to provide a default value of -1 if an error occurs.
~~~

~~~admonish tip title="Using expect to provide a custom error message on error" collapsible=true
```rust
use std::num::ParseIntError;

fn string_to_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>().map_err(|e| e.into())
}

fn main() {
    let n = string_to_number("10").expect("Unable to parse string as number");
    println!("n = {}", n);

    let m = string_to_number("foo").expect("Unable to parse string as number");
    println!("m = {}", m);
}

```
1. In this example, we have the same string_to_number function as in the previous example. 
2. In main, we use expect to provide a custom error message if an error occurs. 
3. If the Result is Ok, the value is unwrapped and returned. 
4. If the Result is Err, the custom error message is printed and the program panics.
~~~

~~~admonish tip title="Propagating an error using the ? operator and expect to capture" collapsible=true
```rust
use std::fs::File;

fn create_file(name: &str) -> std::io::Result<()> {
    let f = File::create(name)?;
    Ok(())
}

fn main() {
    create_file("foo.txt").expect("Error creating file");
}
```
1. In this example, we have a function create_file that creates a file with the given name. 
2. If there is an error creating the file, the Err variant of the Result is returned. 
3. In main, we use the ? operator to propagate the error up the call stack and pass it to expect to handle.
~~~

~~~admonish tip title="Using unwrap_or_else to provide a custom error handling function" collapsible=true
```rust
use std::num::ParseIntError;

fn string_to_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>().map_err(|e| e.into())
}

fn main() {
    let n = string_to_number("10").unwrap_or_else(|error| {
        println!("Error parsing string: {:?}", error);
        -1
    });
    println!("n = {}", n);

    let m = string_to_number("foo").unwrap_or_else(|error| {
        println!("Error parsing string: {:?}", error);
        -1
    });
    println!("m = {}", m);
}
```
1. In this example, we have the same string_to_number function as in previous examples. 
2. In main, we use unwrap_or_else to provide a custom error handling function that is called if an error occurs. 
3. The error handling function takes the error as an argument and returns a default value.
~~~
~~~admonish tip title="Using try_into to convert between types:" collapsible=true
```rust
use std::convert::TryInto;

fn main() {
    let x: Result<i32, _> = "10".try_into();
    println!("x = {:?}", x);

    let y: Result<i32, _> = "foo".try_into();
    println!("y = {:?}", y);
}

```
1. In this example, we use the try_into function to try to convert a string to an i32. 
2. If the conversion is successful, the Ok variant of the Result is returned. 
3. If the conversion fails, the Err variant is returned.
~~~
~~~admonish tip title="Using Iterator::try_fold to perform a fold operation with error handling:" collapsible=true
```rust
use std::num::ParseIntError;

fn parse_int(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>().map_err(|e| e.into())
}

fn main() {
    let numbers = vec!["1", "2", "3", "4"];
    let sum = numbers.iter()
        .try_fold(0, |acc, x| -> Result<i32, ParseIntError> {
            let y = parse_int(x)?;
            Ok(acc + y)
        });
    println!("sum = {:?}", sum);

    let numbers = vec!["1", "foo", "3", "4"];
    let sum = numbers.iter()
        .try_fold(0, |acc, x| -> Result<i32, ParseIntError> {
            let y = parse_int(x)?;
            Ok(acc + y)
        });
    println!("sum = {:?}", sum);
}

```
~~~

~~~admonish tip title="Using Result::unwrap_or_default to provide a default value on error:" collapsible=true
```rust
use std::num::ParseIntError;

fn string_to_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>().map_err(|e| e.into())
}

fn main() {
    let n = string_to_number("10").unwrap_or_default();
    println!("n = {}", n);

    let m = string_to_number("foo").unwrap_or_default();
    println!("m = {}", m);
}

```
1. In this example, we have the same string_to_number function as in previous examples. 
2. In main, we use unwrap_or_default to provide a default value of 0 if an error occurs. 
3. The default value is determined by the type of the value being returned.
~~~
~~~admonish tip title="Using Result::transpose to convert a Result inside an Option:" collapsible=true
```rust
use std::num::ParseIntError;

fn string_to_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>().map_err(|e| e.into())
}

fn main() {
    let maybe_number = Some("10");
    let number = maybe_number.map(string_to_number)
        .transpose()
        .unwrap_or_else(|| Err(ParseIntError {
            kind: std::num::ParseIntErrorKind::InvalidDigit,
        }));
    println!("number = {:?}", number);

    let maybe_number = Some("foo");
    let number = maybe_number.map(string_to_number)
        .transpose()
        .unwrap_or_else(|| Err(ParseIntError {
            kind: std::num::ParseIntErrorKind::InvalidDigit,
        }));
    println!("number = {:?}", number);

    let maybe_number = None;
    let number = maybe_number.map(string_to_number)
        .transpose()
        .unwrap_or_else(|| Err(ParseIntError {
            kind: std::num::ParseIntErrorKind::InvalidDigit,
        }));
    println!("number = {:?}", number);
}

```
1. In this example, we have the same string_to_number function as in previous examples. 
2. We also have a variable maybe_number of type Option<&str> that may or may not contain a string.
3. We use map to apply the string_to_number function to the value inside the Option, if it exists. 
4. This results in an Option<Result<i32, ParseIntError>>.
5. We then use transpose to convert the Result inside the Option to an Option inside the Result.
6. This allows us to use the unwrap_or_else method on the Result to provide a default error value
~~~

