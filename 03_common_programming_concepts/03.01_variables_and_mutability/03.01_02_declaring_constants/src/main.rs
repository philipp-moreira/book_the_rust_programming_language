// Declaring and initialize constants
const THREE_HOURS_IN_SECONDS_GLOBAL: u32 = 60 * 60 * 30;
// Initialized with a direct value
const THREE_HOURS_IN_SECONDS_RAW: i32 = 10800;
// Initialized with a calculated value, but only using literal values
const THREE_HOURS_IN_SECONDS_CALCULATE: i32 = 60 * 60 * 3;

fn main() {
    println!("The constant value 'THREE_HOURS_IN_SECONDS_RAW' is {THREE_HOURS_IN_SECONDS_RAW}");
    println!(
        "The constant value 'THREE_HOURS_IN_SECONDS_CALCULATE' is {THREE_HOURS_IN_SECONDS_CALCULATE}"
    );

    const FOUR_HOURS_IN_SECONDS: u32 = 60 * 60 * 4;
    println!(
        "CONSTANTS ('THREE_HOURS_IN_SECONDS_GLOBAL'): GLOBAL==> {THREE_HOURS_IN_SECONDS_GLOBAL}"
    );

    println!(
        "CONSTANTS ('FOUR_HOURS_IN_SECONDS'): main() function scope ==> {FOUR_HOURS_IN_SECONDS}"
    );

    println!(
        "CONSTANTS ('FIVE_HOURS_IN_SECONDS'): other function scope ==> {}",
        get_constant_from_other_function()
    );

    /* This block will generate at Build time: "error[E0425]: cannot find value `FIVE_HOURS_IN_SECONDS` in this scope"
    println!(
       "CONSTANTS ('FIVE_HOURS_IN_SECONDS'): other function scope ==> {FIVE_HOURS_IN_SECONDS}"
    );
     */
}

fn get_constant_from_other_function() -> u32 {
    const FIVE_HOURS_IN_SECONDS: u32 = 60 * 60 * 5;
    FIVE_HOURS_IN_SECONDS
}
