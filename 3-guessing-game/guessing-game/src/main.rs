use std::io::{self, Write};
use rand::Rng;

fn main() -> io::Result<()> {
    println!("=== Guess a number between 0 and 10 ===");

    // Generate random number
    let mut rng = rand::thread_rng();
    //let value: i8 = rng.gen(); // generates a float between 0 and 1
    let value: i8 = rng.gen_range(0..=2); // value between 0 and 10

    let guess = loop {
        print!("Write your choice:");
        let _ = io::stdout().flush();

        // Read input from user
        let mut input = String::new();

        // Read stdin
        let stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut input)?;

        // Check whether we can parse it
        match input.trim().parse() {
            Ok(num) if (0..=10).contains(&num) => break num,
            _ => println!("Invalid input! Please enter a number between 0 and 10."),
        };
    };

    // Compare values
    if value == guess {
        println!("YOU GOT THE RIGHT VALUE!!!");
        return Ok(())
    } else {
        println!("Nope. The correct value was {}...", value);
        return Err(io::Error::new(io::ErrorKind::Other, "Wrong guess"));
    }
}
