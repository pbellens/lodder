#[derive(PartialEq, Clone, Copy)]
pub enum Level {
    OVERVIEW = 0,
    MESSAGE = 1,
    DETAILS = 2,
}

#[derive(PartialEq, Clone, Copy)]
pub struct Entry {
    pub level: Level,
    pub index: usize,
}

pub struct Vocabulary {
    pub words: Vec<Vec<String>>,
}

impl Vocabulary {
    pub fn new(levels: usize) -> Self {
        Vocabulary { words: std::iter::repeat(0).take(levels).map(|_| vec![]).collect() }
    }

    pub fn add(&mut self, level: Level, bs: &[u8]) -> Option<Entry> {
        std::str::from_utf8(bs)
            .map(|sl| {
                let w = &mut self.words[level as usize];
                match w.iter().position(|c| sl == c) {
                    Some(index) => Entry{level, index},
                    None => {
                        w.push(sl.to_owned());
                        Entry{level, index: w.len() - 1}
                    }
                }})
            .ok()
    }

    pub fn map(&self, entry: Entry) -> &str {
        &self.words[entry.level as usize][entry.index]
    }

    pub fn snapshot(&self) -> Vec<usize> {
        self.words.iter().map(|ws| ws.len()).collect()
    }

    pub fn snip(&self, snapshot: &[usize]) -> Self {
        Vocabulary {
            words: self.words.iter()
                .zip(snapshot.iter())
                .map(|(ws, s)| {
                    ws.iter().skip(*s).map(|w| w.clone()).collect()
                })
                .collect()
        }
    }

    pub fn join(&mut self, snapshot: Vocabulary) {
        for (ws, e) in self.words.iter_mut().zip(snapshot.words.into_iter()) {
            ws.extend(e);
        }
    }

}
