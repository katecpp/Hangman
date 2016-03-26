extern crate rand;

use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use rand::Rng;

enum UserInputStatus {
    Valid,
    NotAlphabetic,
    AlreadyDiscovered
}

fn main()
{
    let secret_line = read_input().expect("Failed to read input data!");
    let mut discovered_letters  = String::new();
    println!("Secret line is: {}", secret_line);

    loop
    {
        println!("Type your guess:");
        let user_guess = read_guess();

        match user_guess_can_be_accepted(&discovered_letters, user_guess)
        {
            UserInputStatus::Valid =>
            {
                discovered_letters.push(user_guess);

                if !secret_line.contains(user_guess)
                {
                    println!("Unfortunately, no {}",  user_guess);
                }
            },

            UserInputStatus::NotAlphabetic =>
            {
                println!("{} is not a letter!", user_guess);
            },

            UserInputStatus::AlreadyDiscovered =>
            {
                println!("{} is already discovered!", user_guess);
            },
        }

        println!("Discovered letters: {}", discovered_letters);
    }
}

fn read_input() -> Result<String, io::Error>
{
    let f = try!(File::open("input.txt"));
    let file = BufReader::new(&f);
    let mut v: Vec<String> = Vec::new();

    for line in file.lines()
    {
        let l = line.unwrap().to_lowercase();
        println!("{}", l);
        v.push(l);
    }

    println!("Correctly loaded {} lines.", v.len());

    let random_line = rand::thread_rng().gen_range(1, v.len());
    let secret_line = v[random_line].clone();
    Ok(secret_line)
}

fn read_guess() -> char
{
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guessed_char : char = guess.chars().nth(0).unwrap();
    // guess.char_at(0) is unstable
    guessed_char
}

fn user_guess_can_be_accepted(discovered_letters: &String, user_guess: char) -> UserInputStatus
{
    if !user_guess.is_alphabetic()
    {
        return UserInputStatus::NotAlphabetic;
    }

    if discovered_letters.contains(user_guess)
    {
        return UserInputStatus::AlreadyDiscovered;
    }

    UserInputStatus::Valid
}
