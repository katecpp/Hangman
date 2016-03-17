extern crate rand;

use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use rand::Rng;


enum UserInputStatus {
    Valid,
    NotAlfabetic,
    AlreadyDiscovered
}

fn main()
{
    let secret_line             = read_input().expect("Failed to read input data!");
    let mut discovered_letters  = String::new();
    let mut lives               = 6;
    let console_width           = 80;
    let frame = std::iter::repeat("-")
                .take(std::cmp::min(secret_line.len(), console_width))
                .collect::<String>();

    while lives > 0
    {
        println!("Lives left: {}", lives);
        if !discovered_letters.is_empty()
        {
            println!("Discovered letters: {}", discovered_letters);
        }

        let secret_line_masked = format_masked_string(&secret_line, &discovered_letters);

        println!("{}", frame);
        println!("{}", secret_line_masked);
        println!("{}", frame);

        if !secret_line_masked.contains('_')
        {
            println!("You won!");
            break;
        }

        println!("Type your guess:");
        let user_guess = read_guess();

        match user_guess_can_be_accepted(&discovered_letters, user_guess)
        {
            UserInputStatus::Valid =>
            {
                discovered_letters.push(user_guess);

                if !secret_line.contains(user_guess)
                {
                    lives = lives - 1;
                    println!("Unfortunately, no {}",  &user_guess);
                }
            },

            UserInputStatus::NotAlfabetic =>
            {
                println!("{} is not a letter!", user_guess);
            },

            UserInputStatus::AlreadyDiscovered =>
            {
                println!("{} is already discovered!", user_guess);
            },
        }
    }
}

fn read_guess() -> char
{
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guessed_char : char = guess.chars().nth(0).unwrap();
    // guess.char_at(0) is unstable
    guessed_char
}

fn read_input() -> Result<String, io::Error>
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

fn user_guess_can_be_accepted(discovered_letters: &String, user_guess: char) -> UserInputStatus
{
    if !user_guess.is_alphabetic()
    {
        return UserInputStatus::NotAlfabetic;
    }

    if discovered_letters.contains(user_guess)
    {
        return UserInputStatus::AlreadyDiscovered;
    }

    UserInputStatus::Valid
}
