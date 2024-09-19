fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        return 0;
    } else if guess > secret {
        return 1;
    } else {
        return -1;
    }
}

fn main() {
    let secret: i32 = 21;
    let mut num_guesses: i32 = 0;
    let mut guess = 18;

    loop {
        num_guesses += 1;
        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Guess number {} was correct! Number of Guesses: {}", secret, num_guesses);
            break;
        } else if result == 1 {
            println!("Guess too high! Number of Guesses: {}", num_guesses);
        } else if result == -1 {
            println!("Guess too low! Number of Guesses: {}", num_guesses);
        }

        guess += 1;
    }
}
