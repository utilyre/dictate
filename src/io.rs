use std::env;

pub struct Args {
    pub word: String,
}

impl Args {
    pub fn parse() -> Result<Self, &'static str> {
        let mut args = env::args();
        args.next();
        let Some(word) = args.next() else {
            return Err("expected `word` as first argument");
        };


        Ok(Self { word })
    }
}
