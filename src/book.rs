use colored::*;
use std::fmt;

// Book
#[derive(Debug, Clone, Deserialize)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub pages: Vec<Page>,
}

impl Book {}

// Page
#[derive(Debug, Clone, Deserialize)]
pub struct Page {
    pub page_no: usize,
    pub text: String,
    pub choices: Option<Vec<Choice>>,
}

impl Page {
    pub fn choices_str(&self) -> String {
        match self.choices {
            Some(ref choices) => choices
                .iter()
                .enumerate()
                .map(Page::choice_str)
                .collect::<Vec<String>>()
                .join("\n"),
            None => String::from(""),
        }
    }

    fn choice_str((c, choice): (usize, &Choice)) -> String {
        format!("{}) {}", c + 1, choice).cyan().to_string()
    }
}

// Choice
#[derive(Debug, Clone, Deserialize)]
pub struct Choice {
    pub text: String,
    pub page: usize,
}

impl Choice {}

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (go to page {})", self.text, self.page)
    }
}
