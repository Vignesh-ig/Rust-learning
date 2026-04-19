fn main() {
    // println!("If expression");

    // let num = 7;

    // // if statement
    // if num != 10 {
    //     println!("num is less than 10");
    // }

    // // if else statement
    // if num < 10 {
    //     println!("num is less than 10");
    // } else {
    //     println!("num is less than 10");
    // }

    // // passing non bool in condition example that will throw error
    // // if num {
    // //     println!("num is less than 10");
    // // }

    // if num % 2 == 0 {
    //     println!("{} - number is even", num)
    // } else if num % 2 != 0 {
    //     println!("{} - number is odd", num)
    // } else {
    //     println!("{} - It is neitheir even or odd", num)
    // }

    // let condition = true;
    // let num = if condition { 5 } else { 6 };
    // println!("output: {num}");

    // let condition = true;
    // let str = if condition { "vic" } else { 6 }; // This will throw error because both if and else expression arm must return same type
    // println!("output: {str}");

    // // Looping expression
    // loop keyword

    // loop {
    //     println!("run");
    // }

    // //Returning a value from loops
    // let mut count = 0;

    // let result = loop {
    //     count = count + 1;

    //     if count == 10 {
    //         break count;
    //     }
    // };

    // println!("{result}");

    // // Disambiguating with Loop Labels

    // // with break
    // let mut count = 0;
    // 'counting_label: loop {
    //     println!("count = {count}");

    //     let mut remaining = 10;
    //     loop {
    //         println!("remaining = {remaining}");

    //         if remaining == 9 {
    //             break;
    //         }

    //         if count == 2 {
    //             break 'counting_label;
    //         }

    //         remaining -= 1;
    //     }

    //     count += 1;
    // };

    // println!("Ending count = {count}");

    // // with continue
    // 'outer: for i in 0..3 {
    //     for j in 0..3 {
    //         if j == 1 {
    //             continue 'outer;
    //         }
    //         println!("i = {}, j = {}", i, j);
    //     }
    // }

    // // example for multiple loop
    // 'outer: loop {
    //     'middle: loop {
    //         'inner: loop {
    //             break 'middle //this breaks middle
    //         }
    //     }
    //     break; //this break outer
    // }

    // // example for loop label with return value
    // let result = 'outer: loop {
    //     loop {
    //         break 'outer 42;
    //     }
    // };
    // println!("{result}");

    // // while loop
    // let mut num = 7;
    // while num != 3 {
    //     println!("{num}");

    //     num -= 1;
    // }
    // println!("While loop over");

    // // loop an array over while - It is slow because it takes more time that compiler nee to calculate the 
    // // length of the array is within the bound of the array. To overcome this we choose for loop here.
    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("index {index} is {}", a[index]);
    //     index += 1;
    // }

    // // for loop
    // let arr = [10, 20, 30, 40, 50];

    // for element in arr {
    //     println!("element: {element}");
    // }

    // // rev() to reverse the range
    for num in (1..4).rev() {
        println!("{num}");
    }
}
