fn main() {
    let x = 5;
    println!("The value of x in first moment is {x}, {:p}", &x);

    let x = x + 1;
    println!("The value of x in second moment is {x}, {:p}", &x);

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}, {:p}", &x);
    }

    println!("The final value of x is {x}, {:p}", &x);
}
