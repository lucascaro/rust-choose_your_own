#[macro_use]
extern crate serde_derive;
extern crate colored;
extern crate toml;

use colored::*;
use std::io;

mod book;
mod config;
#[macro_use]
mod print;
use print::*;

fn main() {
    let book = config::load("examples/test1.toml").unwrap();
    // println!("VALUE: {:#?}", cfg);

    p!(format!("Welcome to {}.", book.title).yellow().bold());
    p!(format!("Written by {}.", book.author).yellow());

    let mut current_page: usize = 0;
    while let Some(_) = book.pages[current_page].choices {
        current_page = choose(current_page, &book.pages[current_page]);
    }
    choose(current_page, &book.pages[current_page]);
    println!("You have finished the book!")
}

fn choose(page_no: usize, page: &book::Page) -> usize {
    p!(page_title(page_no + 1));
    println!("{}", page.text);
    match page.choices {
        Some(ref choices) => {
            p!(page.choices_str());
            p!(page_footer());
            get_choice(&choices).unwrap_or(page_no)
        }
        None => page_no,
    }
}

fn get_choice(choices: &[book::Choice]) -> Option<usize> {
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read choice.");

    let choice = choice.trim();
    if choice == "x" {
        println!("Bye!");
        std::process::exit(0);
    }
    let choice: usize = choice.parse().unwrap_or(0);

    let num_choices = choices.len();
    if choice > 0 && choice <= num_choices {
        let dst_page = choices[choice - 1].page;
        return Some(dst_page - 1);
    }
    None
}
