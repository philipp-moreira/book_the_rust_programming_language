fn main() {
    let float_number = 2.0;
    println!("The float number 'float_number' is {float_number}, Default type: f64");

    /*  Applying the concept of shadowing and changing the type and value of the variable
      - Use of the underscore character "_" for readability of the assigned value.
    */
    let float_number: f32 = 19.123_4567;
    println!("The float number 'float_number' is {float_number}, Defined type: f32");
}
