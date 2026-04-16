fn main() {
    //Immutable variable
    let  x = 5;
    println!("{x}");

    //Mutable variable
    let mut y = 15;
    println!("{y}");
    y = 20;
    println!("{y}");

    //Constant variable - Type must be specified for constants
    const NAME: &str = "Vignesh";
    println!("Hello, {NAME}!");

    const PI: f64 = 3.14159;
    println!("The value of PI is approximately {PI}.");

    const MAX_POINTS: u32 = 100_000;
    println!("The maximum points you can earn is {MAX_POINTS}.");

    // Shadowing
    let a = 10;
    println!("{a}");
    
    let a = 20;

    {
        let a = 30;
        println!("Inner scope, {a}");
    }

    println!("{a}");


    //Shadowing vs Mutability
    let spaces = "      ";
    let spaces = spaces.len();
    println!("{spaces}");

    /*
    let mut spaces = "      ";
    spaces = spaces.len();
    println!("{spaces}");
    //This will throw error
    */
}
