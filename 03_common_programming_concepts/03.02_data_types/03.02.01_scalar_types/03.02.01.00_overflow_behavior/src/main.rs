fn main() {
    let mut changed_value: u8 = 255;
    let incremental_value: u8 = 127;

    println!(
        "The variables are changed_value: {changed_value} and incremental_value: {incremental_value}"
    );

    // changed_value = changed_value + incremental_value;
    // println!(
    //     "The variables are changed_value: {changed_value} and incremental_value: {incremental_value}"
    // );

    // Prevents panic behavior and allows the compiler not to generate errors at build time in debug mode (non-productive)
    changed_value = changed_value.wrapping_add(incremental_value);
    println!(
        "The variables are changed_value: {changed_value} and incremental_value: {incremental_value}"
    );

    /*
    // Throw panic
    changed_value = 255;
    //Allows possible overflow error behavior to be handled and the programmer can choose how to deal with the situation, intercepting and dealing with the error throw
    changed_value = changed_value
        .checked_add(incremental_value)
        .expect("Occurs an overflow error and the value was not calculated!");

    println!(
        "The variables are changed_value: {changed_value} and incremental_value: {incremental_value}"
    );
    */

    /*
    // Not throw panic
    let err: bool;
    changed_value = 255;
    //Allows possible overflow error behavior to be handled and the programmer can choose how to deal with the situation, intercepting and dealing with the error throw
    (changed_value, err) = changed_value.overflowing_add(incremental_value);

    if err {
        println!(
            "Occurs an overflow error and the value was not calculated! When using u32::overflowing_add() method."
        );
    }

    println!(
        "The variables are changed_value: {changed_value} and incremental_value: {incremental_value}"
    );
    */

    /*
    // Not throw panic
    //
    changed_value = 255;
    //Allows possible overflow error behavior to be handled and the programmer can choose how to deal with the situation, intercepting and dealing with the error throw
    changed_value = changed_value.saturating_add(incremental_value);

    println!(
        "MAX - The variables are changed_value: {changed_value} and incremental_value: {incremental_value}"
    );

    let mut changed_value2: i8 = 125;
    let incremental_value: i8 = -124;

    println!(
        "The variables are changed_value2: {changed_value2} and incremental_value: {incremental_value}"
    );

    //Allows possible overflow error behavior to be handled and the programmer can choose how to deal with the situation, intercepting and dealing with the error throw
    changed_value2 = changed_value2.saturating_add(incremental_value);

    println!(
        "MIN - The variables are changed_value2: {changed_value2} and incremental_value: {incremental_value}"
    );
    */
}
