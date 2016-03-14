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
	let mut lives = 10;
	
	while lives > 0
	{
		let user_guess = read_guess().unwrap();

		discovered_letters.push(user_guess.chars().nth(0).unwrap());
		println!("Mask after push_str: {}", discovered_letters);
		// TODO: append chars instead of String

		print_masked_string(&secret_line, &discovered_letters);
	}
}

fn read_guess() -> Result<String, io::Error>
{
    println!("Please input your guess.");

    // TODO: read one char instead of line
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
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
        let l = line.unwrap();
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

fn user_guess_can_be_accepted(discovered_letters: String, user_guess: String) -> bool
{
	// hachiko.chars().nth(1);
	true
}

fn user_guessed_letter(secret_line: String, user_guess: String) -> bool
{
	// hachiko.chars().nth(1);
	true
}
