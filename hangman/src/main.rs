extern crate rand;

use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use rand::Rng;
// use std::path::Path;

fn main() {
    println!("Hello, world!");

    let secret_line = read_input()
                      .expect("Failed to read input data!");				  
	let mut discovered_letters = String::new();
	let mut lives = 6;
	
	while lives > 0
	{
		let user_guess = read_guess().unwrap();
		
		if user_guess_can_be_accepted(&discovered_letters, &user_guess)
		{
			discovered_letters.push(user_guess.chars().nth(0).unwrap());

			if user_guessed_letter(&secret_line, &user_guess)
			{
				println!("Great! You guessed {}!",  &user_guess);
			}
			else
			{
				lives = lives - 1;
				println!("Unfortunately, no {}",  &user_guess);
				println!("lives remaining: {}", lives);
			}
		}
		else
		{
			println!("Invalid input, try again");
		}

		print_masked_string(&secret_line, &discovered_letters);
	}
}

fn read_guess() -> Result<String, io::Error>
{
    println!("Please input your guess.");

    // TODO: read one char instead of line
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
	// TODO: let guess : String = guess.trim();
    println!("You guessed: {}", guess);

    Ok(guess)
}

fn read_input() -> Result<String, io::Error>
{
    let f = try!(File::open("input.txt"));
    let file = BufReader::new(&f);

    let mut v: Vec<String> = Vec::new();

    for line in file.lines()
    {
        let l = line.unwrap().to_lowercase();
        // println!("{}", l);
        v.push(l);
    }

    println!("Read {} lines", v.len());

    let random_line = rand::thread_rng().gen_range(1, v.len());
    let secret_line = v[random_line].clone();
    println!("Randomly chosen line: {}. {}", random_line, secret_line);

    Ok(secret_line)
}

fn print_masked_string(input: &String, mask: &String)
{
    println!("Original string: {}. Mask: {}", input, mask);

    for (u, c) in input.chars().enumerate()
    {
        print!("{} ",
                if c == ' ' {c}
                else if mask.contains(c) {c}
                else {'_'});
    }

    print!("\n");
}

fn user_guess_can_be_accepted(discovered_letters: &String, user_guess: &String) -> bool
{
	let user_input : char = user_guess.chars().nth(0).unwrap();
	!discovered_letters.contains(user_input)
	&& user_input.is_alphabetic()
}

fn user_guessed_letter(secret_line: &String, user_guess: &String) -> bool
{
	secret_line.contains(user_guess.chars().nth(0).unwrap())
}
