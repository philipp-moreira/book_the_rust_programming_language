fn main() {
    let tup = tupple_declaring_and_initializing();

    tupple_muttabe_declaring_and_initializing();

    // Destructure by pattern matching
    let (field_1, field_2, field_3) = tup;
    println!(
        "tup (after destructuring)= (field_1 = {field_1}, field_2 = {field_2}, field_3 = {field_3})"
    );

    /* Error: E0308 - Because, the tuple has less fields then the tup variable
    let (field_4, field_5) = tup;
    println!("field_4 = {field_4}, field_5 = {field_5}");

    // The block above has the possibility workaround, if we use the operator ".." (rest pattern, rest operator or pattern matching rest)
    let (field_4, field_5, ..) = tup;
    println!("field_4 = {field_4}, field_5 = {field_5}");
    */

    /* Empty tuples has a special name, its "UNIT"
    // It's the default return type in void functions
    let empty_tup = ();
    */

    println!(
        "Accessing tuple value in another way; tup.0 -> {}, tup.1 -> {} and tup.2 -> {}",
        tup.0, tup.1, tup.2
    );
}

// #1 - Declaring and initializing
fn tupple_declaring_and_initializing() -> (i32, f64, u8) {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup (directly)= ({}, {}, {})", tup.0, tup.1, tup.2);
    tup
}

// #2 - Declaring and initializing in a mutable form
fn tupple_muttabe_declaring_and_initializing() {
    let mut tup: (i32, f64, u8) = (500, 6.4, 1);
    // changing value fo the unique element, the second item
    tup.1 = 6.8;
    println!("The value of tup.1 if: {}", tup.1);
}
