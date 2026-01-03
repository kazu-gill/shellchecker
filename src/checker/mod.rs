// src/checker/mod.rs
mod syntax;
mod best_practice;
mod security;
mod style;

use crate::parser::ScriptParser;
use crate::report::Report;
use crate::i18n::Language;

pub struct Checker {
    parser: ScriptParser,
    language: Language,
}

impl Checker {
    pub fn new(content: &str, language: Language) -> Self {
        Checker {
            parser: ScriptParser::new(content),
            language,
        }
    }

    pub fn check(&self) -> Report {
        let mut report = Report::new();

        // 1. 構文チェック
        syntax::check(&self.parser, &mut report, &self.language);

        // 2. ベストプラクティス検証
        best_practice::check(&self.parser, &mut report, &self.language);

        // 3. セキュリティチェック
        security::check(&self.parser, &mut report, &self.language);

        // 4. スタイルチェック
        style::check(&self.parser, &mut report, &self.language);

        report
    }
}