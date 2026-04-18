use std::io;

fn main() {
    // // Scalar types
    // //Integers
    // let a = 10;
    // println!("{a}");

    // let b: u8 = 5;
    // println!("{b}");

    // println!("sum of a + b is {}", a + b); //It works without any error

    // //The above code will work but the best practice is literal tagging
    // let d : u8 = 10 + 10u8;
    // println!("{d}");

    /*
    // let e: i32 = 10;
    // println!("{e}");

    // let f: u8 = 5;
    // println!("{f}");

    // println!("sum of e + f is {}", e + f) //It throws error because of type mismatch
*/

    //Floating-points
    // let g = 2.0; //Floating by default is f64
    // let h: f32 = 7.28;
    // println!("g is by default f64 {}, h is f32 {}", g, h);

    // //Numeric (Mathematical) operations
    // // addition
    // let sum = 5 + 10;
    // println!("sum = {sum}");

    // // subtraction
    // let difference = 95.5 - 4.3;
    // println!("subtraction = {difference}");

    // // multiplication
    // let product = 4 * 30;
    // println!("multiplication = {product}");

    // // division
    // let quotient = 56.7 / 32.2;
    // let truncated = -5 / 3; // Results in -1
    // let remainder = 43 % 5;
    // println!("quotient division = {quotient}");
    // println!("truncated division = {truncated}");
    // println!("remainder division = {remainder}");

    // // boolean
    // let t = true;
    // let f: bool = false;

    // // character
    // let ch = 'z';
    // let ch1: char = 'ℤ'; // with explicit type annotation
    // let ch_emoji_cat = '😻';
    // println!("{ch}");
    // println!("{ch1}");
    // println!("{ch_emoji_cat}");
    

    // // Compound types
    // // Tuples
    // let tuple = ("vicky", 7, 26.28, 'i', true, false);
    // println!("{}", tuple.0);

    // // Here we mentioned type for tuples variable
    // // This is variable shadowing
    // let tuple: (&str, i32, f64, char, bool, bool) = ("vicky", 7, 26.28, 'i', true, false);
    // println!("{}", tuple.2);

    // let tup = (500, 6.4, 1);

    // let (x, y, z) = tup;

    // println!("The value of x is: {x}");
    // println!("The value of y is: {y}");
    // println!("The value of z is: {z}");

    // // Arrays
    // let a = [1, 2, 3, 4, 5];
    // println!("{}", a[2]);

    // let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // println!("{}", arr[2]);

    // let arr = [7; 5];
    // println!("{:?}", arr);

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
}
