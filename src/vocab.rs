pub struct Vocab {
    pub words: Vec<String>,
}

impl Vocab {
    pub fn new() -> Self {
        Vocab {
            words: vec![]
        }
    }

    pub fn add(&mut self, bs: &[u8]) -> Option<usize> {
        std::str::from_utf8(bs)
            .map(|sl| {
                match self.words.iter().position(|c| sl == c) {
                    Some(idx) => idx,
                    None => {
                        self.words.push(sl.to_owned());
                        self.words.len() - 1
                    }}})
            .ok()
    }

    pub fn map(&self, idx: usize) -> &str {
        &self.words[idx]
    }
}
