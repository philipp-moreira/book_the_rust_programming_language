fn main() {
    /* #1 - Declaring Variable and changing it

        /* This block will generate at Build time: "error[E0384]: cannot assign twice to immutable variable `x`"
        let x = 5;
        println!("VARIABLE ==> The value of x is: {x}");

        x = 6;
        println!("VARIABLE ==> The value of x is: {x}");
        */
    */

    //  /* #1 - How to correctly make the block above

    // Added the "mut" keyword informing the rustc compiler that the value of variable x will be changed
    let mut x = 5;
    println!("VARIABLE ==> The value of x is {x}");
    // changing the value of x
    x = 6;
    println!("VARIABLE ==> The value of x is {x}");
    // */
}
