extern crate rand;
extern crate ansi_term;

use std::process::Command;

use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use rand::Rng;

use ansi_term::Colour::Red;
use ansi_term::Colour::Green;
use ansi_term::Colour::Yellow;

struct GameData {
    secret_line         : String,
    discovered_letters  : String,
    lives               : i32,
    status              : String
}

enum UserInputStatus {
    AlreadyDiscovered,
    LetterGuessed,
    LetterMissed,
}

fn main()
{
    let random_line = get_random_line().expect("Failed to read input data!");

    let mut gd : GameData = GameData {
                        secret_line        : random_line,
                        discovered_letters : String::new(),
                        lives              : 5,
                        status             : String::new()
                        };

    let mut secret_line_masked = format_masked_string(&gd.secret_line, &gd.discovered_letters);

    loop
    {
        update_screen(&gd, &secret_line_masked);

        println!("Type your guess:");
        let user_guess = read_guess();

        if validate_user_guess(user_guess)
        {
            let guess_lower = user_guess.unwrap().to_lowercase().next().unwrap();

            match check_user_guess(&gd, guess_lower)
            {
                UserInputStatus::LetterGuessed =>
                {
                    gd.discovered_letters.push(guess_lower);
                    let status = format!("You discovered {}", guess_lower);
                    gd.status = Green.paint(status).to_string();
                    secret_line_masked = format_masked_string(&gd.secret_line, &gd.discovered_letters);

                    if !secret_line_masked.contains('_')
                    {
                        gd.status = Green.bold().paint("You won!").to_string();
                        update_screen(&gd, &secret_line_masked);
                        break;
                    }
                }

                UserInputStatus::LetterMissed =>
                {
                    gd.discovered_letters.push(guess_lower);
                    gd.lives = gd.lives - 1;

                    if gd.lives == 0
                    {
                        gd.status = Red.bold().paint("You lost!").to_string();
                        secret_line_masked = format_masked_string(&gd.secret_line, &gd.secret_line);
                        update_screen(&gd, &secret_line_masked);
                        break;
                    }
                    else
                    {
                        let status = format!("Unfortunately, no {}", guess_lower);
                        gd.status = Red.paint(status).to_string();
                    }
                }

                UserInputStatus::AlreadyDiscovered =>
                {
                    let status = format!("{} is already discovered!", guess_lower);
                    gd.status = Yellow.paint(status).to_string();
                }
            }
        }
        else
        {
            let status = format!("It is not a letter!");
            gd.status = Yellow.paint(status).to_string();
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

fn validate_user_guess(user_guess: Option<char>) -> bool
{
    match user_guess
    {
        Some(guess) =>
        {
            if !guess.is_alphabetic() { false }
            else { true }
        }

        None => { return false; }
    }
}

fn check_user_guess(gd: &GameData, user_guess: char) -> UserInputStatus
{
    if gd.discovered_letters.contains(user_guess)
    {
        return UserInputStatus::AlreadyDiscovered;
    }

    if !gd.secret_line.contains(user_guess)
    {
        return UserInputStatus::LetterMissed;
    }

    UserInputStatus::LetterGuessed
}

fn update_screen(gd: &GameData, secret_line: &String)
{
    clear();
    println!("HANGMAN: CAN YOU GUESS THE SENTENCE?");
    println!("Lives: {}. Discovered letters: {}", gd.lives, gd.discovered_letters);
    print_hangman(&gd);
    println!("{}", secret_line);
    println!("{}", gd.status);
}

fn print_hangman(gd: &GameData)
{
    match gd.lives
    {
        0 =>
        {
            println!(" _________   ");
            println!("|         |  ");
            println!("|         XO ");
            println!("|        /|\\ ");
            println!("|        / \\ ");
            println!("|            ");
            println!("|            ");
        }

        1 =>
        {
            println!(" _________   ");
            println!("|         |  ");
            println!("|         O  ");
            println!("|        /|\\ ");
            println!("|        / \\ ");
            println!("|        ||| ");
            println!("|        ||| ");
        }

        2 =>
        {
            println!(" _________   ");
            println!("|            ");
            println!("|         O  ");
            println!("|        /|\\ ");
            println!("|        / \\ ");
            println!("|        ||| ");
            println!("|        ||| ");
        }

        3 =>
        {
            println!(" _________   ");
            println!("|            ");
            println!("|            ");
            println!("|         O  ");
            println!("|        /|\\ ");
            println!("|        / \\ ");
            println!("|        ||| ");

        }

        4 =>
        {
            println!(" _________   ");
            println!("|            ");
            println!("|            ");
            println!("|            ");
            println!("|         O  ");
            println!("|        /|\\ ");
            println!("|        / \\ ");
        }

        _ =>
        {
            println!("             ");
            println!("             ");
            println!("             ");
            println!("             ");
            println!("          O  ");
            println!("         /|\\ ");
            println!("         / \\ ");
        }
    }
}

fn clear()
{
  let output = Command::new("clear").output().unwrap_or_else(|e|{
    panic!("failed to execute process: {}", e)
  });
  println!("{}", String::from_utf8_lossy(&output.stdout));
}

