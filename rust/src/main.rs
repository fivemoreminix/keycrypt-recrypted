use keycrypt::*;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "keycrypt", about = "The keyboard-based encryption algorithm.")]
struct Opt {
    /// Whether to "encode" or "decode"
    #[structopt(short, long, default_value = "auto")]
    action: String,

    /// When "decode" is the action, a path to a wordlist is required
    #[structopt(short, long, required_if("action", "decrypt"))]
    wordlist: Option<PathBuf>,

    /// The message to perform the action on. Can be enclosed in quotes
    #[structopt()]
    message: String,

    /// Silence the auto message
    #[structopt(short, long)]
    silence: bool,
}

fn main() {
    let mut opt = Opt::from_args();

    if &opt.action == "auto" {
        if opt.message.contains(|c: char| c.is_ascii_digit()) {
            opt.action = "decode".to_string();
            if !opt.silence {
                println!("Chose to decode");
            }
        } else {
            opt.action = "encode".to_string();
            if !opt.silence {
                println!("Chose to encode");
            }
        }
    }

    if &opt.action == "encode" {
        println!("{}", encrypt_string(&opt.message));
    } else if &opt.action == "decode" {
        if opt.wordlist.is_none() {
            eprintln!("error: You forgot to provide a wordlist for decoding!");
            std::process::exit(1);
        }

        let words = match load_wordlist(&opt.wordlist.unwrap()) {
            Ok(words) => words,
            Err(e) => {
                eprintln!("error: {:?} was encountered trying to open the wordlist provided", e);
                std::process::exit(1);
            }
        };

        println!("{}", decrypt_ascii_string(&opt.message, &words));
    } else {
        eprintln!("error: {:?} is not a valid action", &opt.action);
        std::process::exit(1);
    }
}
