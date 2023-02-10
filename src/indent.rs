pub trait Indent {
    fn indent(&self, by: usize) -> String;
}

impl Indent for String {
    fn indent(&self, by: usize) -> String {
        let indentation = " ".repeat(by);
        indentation.clone() + &self.replace("\n", &format!("\n{}", indentation))
    }
}
