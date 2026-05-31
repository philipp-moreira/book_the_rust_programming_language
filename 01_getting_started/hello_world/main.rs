/*  Writing a raw Rust program
    - Declaring "main" function and entry point of any program written in Rust
    - Here, the macro "println!" was called. Note that this line is not a direct call to a built-in function in the standard Rust library.
    - Syntax details:
        - Functions begin with a lowercase letter and follow the "small snake case" pattern.
        - The instruction line almost always ends with the character ";".
    - Compile:
        - Using the "rustc" tool, the single, main file will be "translated" directly into machine language;
*/
fn main() {
    println!("Hello, World!");
}
