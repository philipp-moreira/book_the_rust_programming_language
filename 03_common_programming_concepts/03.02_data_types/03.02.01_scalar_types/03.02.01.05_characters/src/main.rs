fn main() {
    let c = 'z';
    println!("c = {c}");

    let z: char = 'Z'; // with explicit type annotation
    println!("z = {z}");

    let heart_eyed_cat = '😻';
    // let heart_eyed_cat = 'U1F63B';       // Error E0762
    // let heart_eyed_cat: char = "U1F63B"; // Error E0762
    // let heart_eyed_cat: char = 'U1F63B'; // Error E0762
    // let heart_eyed_cat = 'U+1F63B';      // Error E0762
    println!("heart_eyed_cat = {heart_eyed_cat}");
}
