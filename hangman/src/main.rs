extern crate rand;

use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use rand::Rng;

enum UserInputStatus {
    Accepted,
    NotAccepted,
}

enum GameState {
    Checking,
    Guessing,
    Lost,
    Won
}

struct GameData {
    secret_line         : String,
    discovered_letters  : String,
    lives               : i32,
    game_state          : GameState
}

fn main()
{
    let random_line = get_random_line().expect("Failed to read input data!");
    let frame       = create_frame(2 * random_line.len());

    let mut gd : GameData = GameData {
                                secret_line        : random_line,
                                discovered_letters : String::new(),
                                lives              : 5,
                                game_state         : GameState::Guessing
                                };

    loop
    {
        let secret_line_masked = format_masked_string(&gd.secret_line, &gd.discovered_letters);

        match gd.game_state
        {
            GameState::Checking =>
            {
                println!("Lives left: {}", gd.lives);
                if !gd.discovered_letters.is_empty()
                {
                    println!("Discovered letters: {}", gd.discovered_letters);
                }

                if !secret_line_masked.contains('_')
                {
                    gd.game_state = GameState::Won;
                }
                else if gd.lives == 0
                {
                    gd.game_state = GameState::Lost;
                }
                else
                {
                    gd.game_state = GameState::Guessing;
                }
            }

            GameState::Guessing =>
            {
                print_hangman(gd.lives);
                println!("{}", frame);
                println!("{}", secret_line_masked);
                println!("{}", frame);
                println!("Type your guess: ");
                let user_guess = read_guess();

                match process_user_guess(&mut gd, user_guess)
                {
                    UserInputStatus::Accepted =>
                    {
                        gd.game_state = GameState::Checking;
                    },

                    UserInputStatus::NotAccepted =>
                    {
                        gd.game_state = GameState::Guessing;
                    },
                }
            },

            GameState::Lost =>
            {
                print_hangman(gd.lives);
                println!("{}", frame);
                println!("{}", secret_line_masked);
                println!("{}", frame);
                println!("You lost!");
                break;
            },

            GameState::Won =>
            {
                print_hangman(-1);
                println!("{}", frame);
                println!("{}", secret_line_masked);
                println!("{}", frame);
                println!("You won!");
                break;
            },
        }
    }
}

fn read_guess() -> Option<char>
{
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    guess.trim().chars().nth(0)
}

fn get_random_line() -> Result<String, io::Error>
{
    let f = try!(File::open("input.txt"));
    let file = BufReader::new(&f);
    let mut v: Vec<String> = Vec::new();

    for line in file.lines()
    {
        let l = line.unwrap().to_lowercase();
        v.push(l);
    }

    let random_line = rand::thread_rng().gen_range(1, v.len());
    let secret_line = v[random_line].clone();
    Ok(secret_line)
}

fn format_masked_string(input: &String, mask: &String) -> String
{
    let mut result : String = String::new();

    for (u, c) in input.chars().enumerate()
    {
        result.push(if c == ' ' {c}
            else if mask.contains(c) {c}
            else {'_'});
        result.push(' ');
    }

    result
}

fn process_user_guess(gd: &mut GameData, user_guess: Option<char>) -> UserInputStatus
{
    match user_guess
    {
        Some(guess) =>
        {
            if !guess.is_alphabetic()
            {
                println!("{} is not a letter!", guess);
                return UserInputStatus::NotAccepted;
            }
            else if gd.discovered_letters.contains(guess)
            {
                println!("{} is already discovered!", guess);
                return UserInputStatus::NotAccepted;
            }
            else
            {
                gd.discovered_letters.push(guess);

                if !gd.secret_line.contains(guess)
                {
                    gd.lives = gd.lives - 1;
                    println!("Unfortunately, no {}",  guess);
                }
            }

            UserInputStatus::Accepted
        }

        None =>
        {
            UserInputStatus::NotAccepted
        }
    }
}

fn create_frame(input_width: usize) -> String
{
    let console_width = 80;
    std::iter::repeat("-")
            .take(std::cmp::min(input_width, console_width))
            .collect::<String>()
}

fn print_hangman(lives: i32)
{
    match lives
    {
        0 =>
        {
            println!(" _________  ");
            println!("|         | ");
            println!("|         XO ");
            println!("|        /|\\");
            println!("|        / \\");
            println!("|           ");
            println!("|           ");
        }

        1 =>
        {
            println!(" _________");
            println!("|         | ");
            println!("|         O ");
            println!("|        /|\\");
            println!("|        / \\");
            println!("|        |||");
            println!("|        ||| ");
        }

        2 =>
        {
            println!(" _________");
            println!("|           ");
            println!("|         O ");
            println!("|        /|\\");
            println!("|        / \\");
            println!("|        |||");
            println!("|        ||| ");
        }

        3 =>
        {
            println!(" _________");
            println!("|           ");
            println!("|           ");
            println!("|         O ");
            println!("|        /|\\");
            println!("|        / \\");
            println!("|        ||| ");

        }

        4 =>
        {
            println!(" _________");
            println!("|           ");
            println!("|           ");
            println!("|           ");
            println!("|         O ");
            println!("|        /|\\");
            println!("|        / \\");
        }

        5 =>
        {
            println!("            ");
            println!("            ");
            println!("            ");
            println!("            ");
            println!("          O ");
            println!("         /|\\");
            println!("         / \\");
        }

        _ =>
        {
            println!("            ");
            println!("            ");
            println!("            ");
            println!("            ");
            println!("          :) ");
            println!("         /|\\");
            println!("         / \\");
        }
    }
}
