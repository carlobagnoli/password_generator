use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::{self, Write};

use clap::Parser;

#[derive(Parser)]
#[clap(author, about, long_about = None)]
struct Args {
    /// The service in which your account is in.
    #[clap(value_parser)]
    service: String,

    /// Which characters to exclude for the final password
    #[clap(short, long, value_parser)]
    exclude: Option<String>,

    /// Which characters to use exclusively for the final password
    #[clap(short, long, value_parser)]
    include: Option<String>,
}

fn main()
{
    let args = Args::parse();

    let passphrase = rpassword::prompt_password("Enter your passphrase: ").unwrap();

    let permitted_chars: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&\'()*+,-./:;<=>?@[\\]^_`{|}~"
        .chars()
        .filter(|c| {
            if let Some(excluded) = &args.exclude {
                let excluded_chars: Vec<char> = excluded.chars().collect();
                return !excluded_chars.contains(c);
            }
            return true;
        })
        .filter(|c| {
            if let Some(included) = &args.include {
                let included_chars: Vec<char> = included.chars().collect();
                return included_chars.contains(c);
            }
            return true;
        })
        .collect();

    let key = passphrase + &args.service;

    println!("This is your password: {}", hash_to_string(permitted_chars, calculate_hash(&key)));
}

fn hash_to_string(chars: Vec<char>, hash: u64) -> String
{
    let mut acc = hash;
    let mut result: Vec<char> = vec![];
    let chars_len = chars.len() as u64;

    while acc > chars_len {
        acc = acc / chars_len;

        result.push(chars[(acc % chars_len) as usize]);
    }

    String::from_iter(result)
}

fn calculate_hash<T: Hash>(t: &T) -> u64
{
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
