extern crate term_size;

#[macro_export]
macro_rules! p {
    () => (println!());
    ($($arg:tt)*) => (println!("{}", $($arg)*));
}

pub fn separator() {
    if let Some((w, _)) = term_size::dimensions() {
        println!("{:-^1$}", "", w);
    } else {
        println!("{:-^1$}", "", 20);
    }
}
