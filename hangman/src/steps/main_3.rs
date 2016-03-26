extern crate rand;

use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use rand::Rng;

fn main()
{
    let secret_line = read_input().expect("Failed to read input data!");
    println!("Secret line is: {}", secret_line);
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
