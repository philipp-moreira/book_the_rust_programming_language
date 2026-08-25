fn main() {
    // Stay in stack memory
    let arr = [1, 2, 3, 4, 5];
    println!(
        /* Formatters
            1 - '{}'    -> You cannot use just the '{}' formatter, as this will trigger the panic error "error[E0277]: `[{integer}; 5]` doesn't implement `std::fmt::Display`"
            2 - '{:?}'  -> This works and allows the entire array to be written to the console
            3 - '{:#?}' -> This works and allows the entire array to be written to the console (in a more readable/formatted way)
        */
        "The second element in array 'arr' ({:?}) is : {}",
        arr, arr[1]
    );

    /* Attempting to modify an element of a standard (immutable) array, will trigger a compile-time panic: "error[E0594]: cannot assign to `arr[_]`, as `arr` is not declared as mutable"
    // arr[1] *= 2;
     */

    let mut arr_2 = [1, 2, 3, 4, 5];
    println!(
        "The second element in a 'mut' array 'arr_2' ({:?}) is: {}",
        arr_2, arr[1]
    );

    // Changing an element in array
    arr_2[1] *= 3;
    println!(
        "AGAIN - After change it - The second element in a 'mut' array 'arr_2' ({:?}) is: {}",
        arr_2, arr[1]
    );

    // Indicated to fixed size lists
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("The fifth element in months is {}", months[4]);

    // Using a typed array and defining the number of elements
    let typed_array: [u8; 5] = [1, 2, 3, 4, 5];
    println!("The third element in typed_array is {}", typed_array[2]);

    // Creating an array and initialize it with a unique value '3'
    let array_with_a_single_element = [3; 5];
    println!(
        "array_with_a_single_element = {:?}",
        array_with_a_single_element
    );

    /* Panic in cocmpile-time: "error: this operation will panic at runtime"
    println!(
        "try to access a element out of range: {}",
        array_with_a_single_element[6]
    );
    */

    /* Possible panic in run-time, with similar details:

    /* Console output:
    thread 'main' (69817) panicked at src/main.rs:80:9:
    index out of bounds: the len is 5 but the index is 5
    */

    // Variable declaration will get I/O input (Console)
    let mut index = String::new();

    // Getting input
    std::io::stdin().read_line(&mut index).unwrap();

    // Using shadowing; Making variable cast and re-use variable name
    let index: usize = index.trim().parse().unwrap();

    // How to handle with possiblr panic in run-time
    if index >= array_with_a_single_element.len() {
        println!("Invalid value");
        return;
    }

    // I/O output (Console)
    println!(
        "try to access a element out of range: {}",
        array_with_a_single_element[index]
    );
    */
}
