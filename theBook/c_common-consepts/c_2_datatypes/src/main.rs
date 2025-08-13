use std::io;

fn main() {
    // Float
    // let x = 2.0; // f64
    // let y: f32 = 3.0; // f32
    // ####################################

    // Numeric operations
    // addition
    // let sum = 5 + 10;

    // subtraction
    // let difference = 95.5 - 4.3;

    // multiplication
    // let product = 4 * 30;

    // division
    // let quotient = 56.7 / 32.2;
    // let truncated = -5 / 3; // Results in -1

    // remainder
    // let remainder = 43 % 5;
    // ##########################################

    // Boolean
    // let t = true;
    // let f: bool = false; // with explicit type annotation
    // ###################################################

    // Char
    // let c = 'z'; // Huom! Single quote
    // let z: char = 'ℤ'; // with explicit type annotation
    // let heart_eyed_cat = '😻';
    // ###################################################

    // Tuple
    // let tup = (500, 6.4, 1);
    // let (_x, y, _z) = tup;

    // println!("The value of y is: {y}");

    // We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access.
    // let x: (i32, f64, u8) = (500, 6.4, 1);

    // let five_hundred = x.0;
    // let six_point_four = x.1;
    // let one = x.2;
    // #####################################################

    // Array
    // let a = [1, 2, 3, 4, 5];

    // let first = a[0];
    // let second = a[1];

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
    // This code compiles successfully. If you run this code using cargo run and enter 0, 1, 2, 3, or 4, the program will print out the corresponding value at that index in the array. If you instead enter a number past the end of the array, such as 10, you’ll see runtime error

}
