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

struct GameData {
    secret_line         : String,
    discovered_letters  : String,
    lives               : i32
}

fn main()
{
    let random_line = get_random_line().expect("Failed to read input data!");

    let mut gd : GameData = GameData {
                                secret_line: random_line,
                                discovered_letters: String::new(),
                                lives : 6
                                };

    let frame = create_frame(2*gd.secret_line.len());

    while gd.lives > 0
    {
        println!("Lives left: {}", gd.lives);
        if !gd.discovered_letters.is_empty()
        {
            println!("Discovered letters: {}", gd.discovered_letters);
        }

        let secret_line_masked = format_masked_string(&gd.secret_line, &gd.discovered_letters);

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

        match user_guess_can_be_accepted(&gd.discovered_letters, user_guess)
        {
            UserInputStatus::Valid =>
            {
                gd.discovered_letters.push(user_guess);

                if !gd.secret_line.contains(user_guess)
                {
                    gd.lives = gd.lives - 1;
                    println!("Unfortunately, no {}",  &user_guess);
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
    }
}

fn read_guess() -> char
{
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guessed_char : char = guess.trim().chars().nth(0).unwrap();
    // guess.char_at(0) is unstable
    guessed_char
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

fn create_frame(input_width: usize) -> String
{
    let console_width = 80;
    std::iter::repeat("-")
            .take(std::cmp::min(input_width, console_width))
            .collect::<String>()
}

