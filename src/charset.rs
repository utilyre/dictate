use atty::Stream;

static CHARSET_UTF8: Charset = Charset {
    list: '•',
    section_left: '',
    section_right: '',
};
static CHARSET_ASCII: Charset = Charset {
    list: '*',
    section_left: '<',
    section_right: '>',
};

static OVERRIDE: std::sync::Mutex<Option<Charset>> = std::sync::Mutex::new(None);

#[derive(Clone, Copy)]
pub struct Charset {
    pub list: char,
    pub section_left: char,
    pub section_right: char,
}

pub fn get() -> Charset {
    let ch = OVERRIDE
        .lock()
        .expect("another thread panicked while holding the lock");

    match &*ch {
        Some(ch) => *ch,
        None if atty::is(Stream::Stdout) => CHARSET_UTF8,
        None => CHARSET_ASCII,
    }
}

pub fn set_override(utf8: bool) {
    let mut ch = OVERRIDE
        .lock()
        .expect("another thread panicked while holding the lock");

    if utf8 {
        *ch = Some(CHARSET_UTF8)
    } else {
        *ch = Some(CHARSET_ASCII)
    }
}
