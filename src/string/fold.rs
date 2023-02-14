pub trait Fold {
    fn fold(&self, width: usize) -> String;
}

impl Fold for str {
    fn fold(&self, width: usize) -> String {
        let capacity = (self.len() as f64 / width as f64).ceil() as usize;
        let mut lines = Vec::with_capacity(capacity);

        let mut curr = 0;
        for _ in 0..capacity {
            lines.push(&self[curr..(curr + width).min(self.len())]);
            curr += width;
        }

        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fold_long_string() {
        let sample = "Hello there my name is markiplier! Fake mrbeast Bruh this thing is amazing.";

        let actual = sample.fold(10);
        let expected = "Hello ther
e my name 
is markipl
ier! Fake 
mrbeast Br
uh this th
ing is ama
zing.";

        assert_eq!(actual, expected);
    }

    #[test]
    fn not_fold_short_string() {
        let sample = "Hi There";

        let actual = sample.fold(8);
        let expected = "Hi There";

        assert_eq!(actual, expected);
    }
}
