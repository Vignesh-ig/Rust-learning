fn main() {
    // println!("main function");

    // func_one();
    // func_two(7);
    // func_three(7, "vicky", 'a', 0.7, true, (1, 'a'), [2, 4, 7]);

    // let y = 6;

    // let x = (let y = 6); //Here the (let y = 6) cannot return any value so the x can bind. so it throws error

    //Statement and Expression
    // let y = {
    //     let x = 5;
    //     x + 2
    // };

    // println!("{}", y);

    // let x = {
    //     let a = 10; // statement
    //     let b = 20; // statement
    //     a + b // expression → returned
    // };

    // println!("{x}");
    
    // let r = func_four();
    // println!("Function with return type {}", r);

    // println!("Function with return type {}", func_four());

    println!("func_five {}", func_five(10));
}

fn func_five(x: u8) -> u8 {
    x + 1 //Expression
}

// fn func_four() -> i32 {
//     7
// }

// fn func_three(a: u8, b: &str, c: char, d: f32, e: bool, f:(i8, char), g: [i32; 3] ) {
//     println!("First parameter: {a}");
//     println!("Second parameter: {b}");
//     println!("Third parameter: {c}");
//     println!("Fourth parameter: {d}");
//     println!("Fifth parameter: {e}");
//     println!("Sixth parameter: {:?}", f);
//     println!("Seventh parameter: {:?}", g);
// }

// fn func_two(x: i32) {
//     println!("Function with x parameter: {x}")
// }

// fn func_one() {
//     println!("function without parameter");
// }

