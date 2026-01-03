// src/parser.rs
#[derive(Debug, Clone)]
pub struct ScriptLine {
    pub number: usize,
    pub content: String,
    pub trimmed: String,
}

pub struct ScriptParser {
    lines: Vec<ScriptLine>,
}

impl ScriptParser {
    pub fn new(content: &str) -> Self {
        let lines = content
            .lines()
            .enumerate()
            .map(|(idx, line)| ScriptLine {
                number: idx + 1,
                content: line.to_string(),
                trimmed: line.trim().to_string(),
            })
            .collect();

        ScriptParser { lines }
    }

    pub fn lines(&self) -> &[ScriptLine] {
        &self.lines
    }
}