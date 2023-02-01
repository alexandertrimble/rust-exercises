use rand::Rng;

fn main() {
    play_number_game();
}

fn play_number_game() {
    let target = rand::thread_rng().gen_range(1..10);
    println!("Guess a number between 1 and 10.");

    let mut guess_count: i8 = 0;

    while guess_count < 3 {
        let guess: i8 = get_input();
        if guess == target {
            println!("You win!");
            break;
        } else if guess < target {
            println!("Too low!");
        } else {
            println!("Too high!");
        }
        guess_count += 1;
    }

    if guess_count >= 3 {
        println!("You lose! The number was {}.", target);
    }

    play_again_prompt();
}

fn get_input() -> i8 {
    let error_essage = "Invalid input. Type a number between 1 and 10";
    let mut guess = String::new();

    // read in input
    std::io::stdin().read_line(&mut guess).unwrap();

    match guess.trim().parse::<i8>() {
        Ok(n) => {
            if n < 1 || n > 10 {
                println!("{error_essage}");
                return get_input();
            }
            return n;
        }
        Err(_e) => {
            println!("{error_essage}");
            return get_input();
        }
    }
}

fn play_again_prompt() {
    println!("Play again? (y/n)");
    let mut play_again = String::new();
    std::io::stdin()
        .read_line(&mut play_again)
        .expect("Failed to read line");

    if play_again.trim() == "y" {
        play_number_game();
    } else {
        println!("fuck you then");
        std::process::exit(0);
    }
}
