fn main() {
    /*  Declaring and initialized with string/slice value
    let mut spaces = "   ";    // If we declare it this way, we will have an error (E0308) at compile time
    */
    let spaces = "   ";
    println!(
        "The value of spaces variable in first moment is '{spaces}', {:p}",
        &spaces
    );

    // Again, declaring and initialized, using shadowing concept, where both are changing, the value and data type
    let spaces = spaces.len();
    println!(
        "The value of spaces variable in second moment is #{spaces}, {:p}",
        &spaces
    );
}
