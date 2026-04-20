fn main() {
    // // Convert temperatures between Fahrenheit and Celsius.
    // println!("celcius to fahrenheit, {}", convert_celcius_to_fahrenheit(5f64));
    // println!("fahrenheit to celcius, {}", convert_fahrenheit_to_celcius(41f64));

    // // Generate the nth Fibonacci number.
    // let n = 5;
    // let mut a = 0;
    // let mut b = 1;

    // for _ in 0..n {
    //     let temp = a + b;
    //     a = b;
    //     b = temp;
    // }
    // println!("F({n}) = {a}");

    // // Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
    // let gifts = [
    //     "A partridge in a pear tree",
    //     "Two turtle doves",
    //     "Three French hens",
    //     "Four calling birds",
    //     "Five golden rings",
    //     "Six geese a-laying",
    //     "Seven swans a-swimming",
    //     "Eight maids a-milking",
    //     "Nine ladies dancing",
    //     "Ten lords a-leaping",
    //     "Eleven pipers piping",
    //     "Twelve drummers drumming",
    // ];

    // let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelveth"];

    // for day in 0..days.len() {
    //     println!("On the {} day of Christmas my true love sent to me:", days[day]);
    //     for gift in (0..=day).rev() {
    //         if gift == 0 && day != 0 {
    //             println!("And {}", gifts[0]);
    //         } else {
    //             println!("{}", gifts[gift]);
    //         }
    //     }
    //     println!()
    // }

}

// fn convert_celcius_to_fahrenheit(c: f64) -> f64 {
//     (c * 1.8) + 32f64
// }

// fn convert_fahrenheit_to_celcius(f: f64) -> f64 {
//     (f - 32.0) / 1.8
// }
