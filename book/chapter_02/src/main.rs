use rand::Rng;
use std::cmp::Ordering;
use std::io;

// first example of guessing a number, not much functionality
fn guess_the_number_one() {
    println!("==== Guess the number ====");
    println!("Please input your guess:");

    let mut guess = String::new();

    io::stdin() // Stdin is standard input for your terminal
        .read_line(&mut guess) // takes input and appends it to the string
        .expect("Failed to read the line");

    println!("You guessed: {}", guess);
}

// second example of guessing a number, with added randomness and more functionality
fn guess_the_number_two() {
    // generate a random number before the user takes a guess
    let secret_number = rand::thread_rng() // creates a type that implements the Rng trait
        .gen_range(1..=100); //method defined by the Rng trait

    println!("==== Guess the number ====");
    println!("Please input your guess:");

    let mut guess = String::new();

    io::stdin() // Stdin is standard input for your terminal
        .read_line(&mut guess) // takes input and appends it to the string
        .expect("Failed to read the line");

    // shadowing and typecasting the guess
    // parse() methnod knows the desired type is u32 because defined it
    let guess: u32 = guess
        .trim()
        .parse()
        .expect("Please type in a correct number!");

    println!("You guessed: {}", guess);
    println!("The secret number was: {}", secret_number);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Your guess was too small!"),
        Ordering::Equal => println!("Your guess was correct!"),
        Ordering::Greater => println!("Your guess was too big!"),
    }; // semicolon because match is an expression
}

// third example with added looping functionality
fn guess_the_number_three() {
    // generate a random number before the user takes a guess
    let secret_number = rand::thread_rng() // creates a type that implements the Rng trait
        .gen_range(1..=100); //method defined by the Rng trait

    println!("==== Guess the number ====");

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin() // Stdin is standard input for your terminal
            .read_line(&mut guess) // takes input and appends it to the string
            .expect("Failed to read the line");

        // shadowing and typecasting the guess
        // parse() methnod knows the desired type is u32 because defined it
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     // return num if no err
            Err(_) => continue, // start a new iteration of loop when err
        };

        println!("You guessed: {}", guess);
        println!("The secret number was: {}", secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess was too small!"),
            Ordering::Greater => println!("Your guess was too big!"),
            Ordering::Equal => {
                println!("Your guess was correct!");
                break; // loop stops when correct number is guessed
            }
        }; // semicolon because match is an expression}
    }
}

fn main() {
    // guess_the_number_one();
    // guess_the_number_two();
    guess_the_number_three();
}
