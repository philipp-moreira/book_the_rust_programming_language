fn main() {
    // Addiction
    let sum_result = 5 + 10;
    println!("The result of sum between 5 and 10 is: {sum_result}");

    // Subtraction
    let subtraction_result = 95.5 - 4.3;
    println!("The result of subtraction between 95.5 and 4.3 is: {subtraction_result}");

    // Multiplication
    let multiplication_result = 4.123 * 30.123_456_789;
    println!(
        "The result of multiplication between  4.123 and 30.123_456_789 is: {multiplication_result}"
    );

    // Division
    let quotient = 56.7 / 32.2;
    println!("The result of division 56.7/32.2 is: {quotient}");

    // Here, an implicit conversion to i32 occurs
    let truncated = -5 / 3; // Results in -1 
    println!("The result of division -5/3 is: {truncated}");

    // Using the shadowing concept
    // Here it's not the same thing, because the type is defined at the beginning
    let truncated: f32 = -5.0 / 3.0; // Results in -1 
    println!("The result of division -5.0/3.0 with f32 as type is: {truncated}");

    // Remainder | Module (ref to math)
    let remainder = 43 / 5;
    println!("The result of division 43 / 5 is: {remainder}");

    let remainder = 43 % 5;
    println!("The result of remainder operation/module  43 % 5 is: {remainder}");

    let remainder: f32 = 43.0 % 5.7;
    println!("The result of remainder operation/module  43.0 % 5.7 is: {remainder}");
}
