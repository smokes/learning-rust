// imports required modules
use std::io;
use std::cmp::Ordering;
use std::thread;
use std::time;
use rand::Rng;
use whoami;

// init main function
fn main() {

    // prints system info (for no reason lol);
    println!("Hello {}, You ran me on {} with your {}" , whoami::username() , whoami::os() , whoami::hostname());

    // generates random number from 1 to 100 using rand module/crate whatever to call it 
    let gen_number = rand::thread_rng().gen_range(1, 101);
    println!("Select a random number you retard.");

    // loop to constantly retry whenever someone fails to guess the number
    loop {
        let mut guess = String::new();

        // reads a line then replaces the mutable variable guess with the new string
        io::stdin().read_line(&mut guess)
        .expect("Failed to read_line;");

        // parses a u32 int from the read line
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {

                // handles invalid numbers like text
                println!("Please insert a valid number!");
                continue;
            },
        };

        // compares the parsed number to the generated one using cmp::Ordering
        match guess.cmp(&gen_number) {
            Ordering::Less => println!("Too small, retry."),
            Ordering::Greater => println!("Too big, retry."),
            Ordering::Equal => {
                println!("Congrats you guessed it 🎉");

                // sleeps for 2 seconds so you can read the congrats message
                let time = time::Duration::from_millis(2000);
                thread::sleep(time);
                break;
            },
        }
    }
}