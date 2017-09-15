
pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner { source: source }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        Vec::new()
    }
}

#[derive(Debug)]
pub struct Token {}
