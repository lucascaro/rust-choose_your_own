extern crate term_size;
use colored::*;

#[macro_export]
macro_rules! p {
    () => (println!());
    ($($arg:tt)*) => (println!("{}", $($arg)*));
}

pub fn separator() -> String {
    let width = match term_size::dimensions() {
        Some((w, _)) => w,
        _ => 20,
    };
    format!("{:-^1$}", "", width)
}

pub fn page_title(page_no: usize) -> String {
    format!(
        "{}\n{}\n{}",
        separator(),
        format!("Current page: {}", page_no + 1).white().dimmed(),
        separator(),
    )
}

pub fn page_footer() -> String {
    format!("{}\n{}", "x) quit".cyan(), "Your choice:".cyan().bold())
}
