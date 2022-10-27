use std::fs::File;

// main() only returns (), it's not compatible with
// the type of the value we use ? on
fn main() {
    // here ? return Result or Option
    let greeting_file = File::open("hello.txt")?;
}
