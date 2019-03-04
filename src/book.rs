// Book
#[derive(Debug, Clone, Deserialize)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub pages: Vec<Page>,
}

impl Book {
    // pub fn new(title: &str, author: &str, pages: Vec<Page>) -> Book {
    //     return Book {
    //         title: String::from(title),
    //         author: String::from(author),
    //         pages: pages,
    //     };
    // }
}

// Page
#[derive(Debug, Clone, Deserialize)]
pub struct Page {
    pub page_no: usize,
    pub text: String,
    pub choices: Option<Vec<Choice>>,
}

impl Page {
    // pub fn new(page_no: usize, text: &str, choices: Option<Vec<Choice>>) -> Page {
    //     return Page {
    //         page_no: page_no,
    //         text: String::from(text),
    //         choices: choices,
    //     };
    // }
}

// Choice
#[derive(Debug, Clone, Deserialize)]
pub struct Choice {
    pub text: String,
    pub page: usize,
}

impl Choice {
    // pub fn new(page: usize, text: &str) -> Choice {
    //     return Choice {
    //         page: page,
    //         text: String::from(text),
    //     };
    // }
}
