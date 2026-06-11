// Refrence and Borrowing
// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{s1}' is {len}.");
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// We cannot modify the borrowed refrence example:
// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// Mutable refrence
// fn main() {
//     let mut s = String::from("Hello");

//     println!("Before mutation: {s}");

//     change(&mut s);

//     println!("After mutation: {s}")
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// When we have multiple variable to the mutable ref variable, It will throw error.
// fn main() {
//     let mut s = String::from("Hello");

//     let r1 = &mut s;
//     let r2 = &mut s;

//     println!("{r1}, {r2}");
// }

//To have multiple mutable borrowing refrence, we can use curly braces.
// fn main() {
//     let mut s = String::from("Hello");

//     {
//         let r1 = &mut s;
//         println!("{r1}, world");
//     } // r1 goes out of scope here, so we can make a new reference with no problems.

//     let r2 = &mut s;

//     println!("{r2}, earth");
// }

// We also cannot have a mutable reference while we have an immutable one to the same value.
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     let r3 = &mut s; // BIG PROBLEM

//     println!("{r1}, {r2}, and {r3}");
// }

// Here, It will not throw error because, ownership of r1 and r2 is dropped in println! line
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");
}
