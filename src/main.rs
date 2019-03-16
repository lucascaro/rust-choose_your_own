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
    separator();
    p!(format!("Current page: {}", page_no + 1).white().dimmed());
    separator();
    println!("{}", page.text);
    if page.choices.is_some() {
        let choices = page.choices.as_ref().unwrap();
        let num_choices = choices.len();
        for c in 0..num_choices {
            p!(format!(
                "{}) {} (go to page {})",
                c + 1,
                choices[c].text,
                choices[c].page
            )
            .cyan());
        }
        p!("x) quit".cyan());
        p!("Your choice:".cyan().bold());

        return match get_choice(&choices) {
            None => page_no,
            Some(p) => p,
        };
    }
    return page_no;
}

fn get_choice(choices: &Vec<book::Choice>) -> Option<usize> {
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
    return None;
}
