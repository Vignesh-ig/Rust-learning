fn main() {
    // {
    //     let mut s = String::from("hello");
    //     println!("s: {s}");

    //     s.push_str(", World");
    //     println!("s: {s}");
    // }

    // let x = 5;
    // let y = x;
    // print!("{}", x == y);
    // Here x and y are equal - because integers are simple values with a known, fixed size, and these two 5 values are pushed onto the stack.

    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{s1}, world!"); // This will throw error

    // let mut s = String::from("hello");
    // s = String::from("ahoy");

    // println!("{s}, world!");

    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("s1 = {s1}, s2 = {}", s2);

    // let x = 7;
    // let y = x;

    // println!("x = {x}, y = {y}");

    //     println!("From main");
    //     let s = String::from("hello"); // s comes into scope

    //     takes_ownership(s); // s's value moves into the function...
    //     // ... and so is no longer valid here

    //     // println!("{s}"); // It will throw error

    //     let x = 5; // x comes into scope

    //     makes_copy(x); // Because i32 implements the Copy trait,
    //     // x does NOT move into the function,
    //     // so it's okay to use x afterward.

    //     println!("{}", x); // It will work fine

    //main function scope phrase ----------------------------------------------------------------------------------
    // } // Here, x goes out of scope, then s. However, because s's value was moved,
    // // nothing special happens.

    // fn takes_ownership(some_string: String) {
    //     // some_string comes into scope
    //     println!("From takes_ownership");
    //     println!("{some_string}");
    // } // Here, some_string goes out of scope and `drop` is called. The backing
    // // memory is freed.

    // fn makes_copy(some_integer: i32) {
    //     // some_integer comes into scope
    //     println!("From makes_copy");
    //     println!("{some_integer}");
    // } // Here, some_integer goes out of scope. Nothing special happens.

    //     let s1 = gives_ownership(); // gives_ownership moves its return
    //     // value into s1

    //     let s2 = String::from("hello"); // s2 comes into scope

    //     let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also
    //     // moves its return value into s3

    //     println!("{s1}");
    //     // println!("{s2}"); // This will throw error because s2 is moved to a function.
    //     println!("{s3}");

    //main function scope phrase ----------------------------------------------------------------------------------
    // } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
    // // happens. s1 goes out of scope and is dropped.

    // fn gives_ownership() -> String {    // gives_ownership will move its return value into the function that calls it

    //     let some_string = String::from("yours"); // some_string comes into scope

    //     some_string // some_string is returned and moves out to the calling function
    // }

    // fn takes_and_gives_back(a_string: String) -> String {
    //     // a_string comes into
    //     // scope

    //     a_string  // a_string is returned and moves out to the calling function
    // }
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
