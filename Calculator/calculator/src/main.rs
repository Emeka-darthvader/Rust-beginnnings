use std::io; // standard input/ output library
fn main() {
    println!("Welcome to my Calculator app");

    println!("Please select an operation to perform:");
    println!("1 - Add");
    println!("2 - Subtract");
    println!("3 - Multiply");
    println!("4 - Divide");

    // variable declaration and initialization
    
    // let input = 1; // type can be inferred from initialization if not explicitly specified
    // let input:i32; i32 means it is a type of integer
    // mutable values can be changed after declaration / assigned it can be changed later on
    // let mut input:i32 makes it mutable
    //io::stdin().read_line(buf: &mut String)

    let mut input = String::new();  // String is not a native type in Rust
    
    //io::stdin().read_line(&mut input ).unwrap()  // Error unhandling too, but interrupts program
    io::stdin().read_line(&mut input).expect("Unable to read the input");  // Error handling, this gracefully stops the program
    println!("You have entered {}", input);
    let choice:i8 = input.trim().parse().unwrap();

    // Entering numbers for operation
    println!("Please enter the first number");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read the input");
    println!("First number: {}", input);
    // let us cast the string to integer for mathematical opetations 
    //let mut first_number:i32 = input.parse.unwrap(); // alternative way of casting
    let mut first_number:i32 = input.trim().parse::<i32>().unwrap();  

    println!("Please enter the second number");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read the input");
    println!("Second number: {}", input);
    let mut second_number:i32 = input.trim().parse::<i32>().unwrap();  


    // Check whether the number has been parsed correcly or not.
    println!("Numbers {} {}", first_number, second_number);

    let mut result:f32 = 0.0;
    // Flow control
    if choice == 1 {
        // add
        result = first_number as f32 + second_number as f32
    } else if choice == 2 {
        // subtract
        result = first_number as f32 - second_number as f32
    } else if choice == 3 {
        // multiply
        result = first_number as f32 * second_number as f32
    } else if choice == 4 {
        // divide
        result = first_number as f32 / second_number as f32
    } else {
        // default
        println!("Please enter a number from 1 - 4");
    }

    println!(" Your result is {}", result);

}   
