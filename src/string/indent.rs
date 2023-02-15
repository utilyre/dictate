pub trait Indent {
    fn indent(&self, by: usize) -> String;
}

impl Indent for str {
    fn indent(&self, by: usize) -> String {
        self.lines()
            .map(|line| " ".repeat(by) + line)
            .collect::<Vec<String>>()
            .join("\n")
    }
}
