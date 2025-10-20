use std::cmp::Ordering;
use std::io; //Calls the input/output library
use rand::Rng;
use colored::Colorize;
use std::fs;

/// Runs the number guessing game.
///
/// This function contains all the core logic for the game:
/// - generates a random secret number
/// - reads player input
/// - compares guesses
/// - gives feedback and counts attempts

fn save_score(score: u32) {
    fs::write("score.txt", score.to_string()).expect("Unable to write score");
}

fn load_score() -> Option<u32> {
    if let Ok(contents) = fs::read_to_string("score.txt") {
        if let Ok(score) = contents.trim().parse::<u32>() {
            return Some(score);
        }
    }
    None
}

pub fn run() {

    println!("{}", "Welcome to the Guess the Number Game!".bold().cyan()); // prints name of the game
    println!("Try to guess the secret number between 1 and 100!");
    println!("Type your guess and press Enter.\n");

    if let Some(best) = load_score() {
        println!("🎯 Current best score: {} attempts", best);
    } else {
        println!("No best score yet. Set your first record!");
    }

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut attempts = 0; //initialize attempt to zero


    loop {

        println!("Please input your guess."); // asks the user to input their guess

        let mut guess = String::new();

        io::stdin() // allow us to handle user input
            .read_line(&mut guess)// standard input handle
            .expect("Failed to read line"); // handles potential failures

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        attempts += 1;

        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".blue()),
            Ordering::Equal => {
                println!("{}"," 🎉 You win!".bright_green().bold());
                println!("It took you {} attempts",attempts);
                
                if let Some(best) = load_score() {
                    if attempts < best {
                        println!("{}", "🏆 New High Score!".bright_yellow());
                        save_score(attempts);
                    } else {
                        println!("Your best score is still {} attempts.", best);
                    }
                } else {
                    println!("{}", "First recorded score!".cyan());
                    save_score(attempts);
                }
                break;
            }
        }
    }
    println!("{}", "Thanks for playing guessit!".cyan());

}


