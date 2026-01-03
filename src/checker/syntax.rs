// src/checker/syntax.rs
use crate::parser::ScriptParser;
use crate::report::{Report, Severity};
use crate::i18n::{
    Language, CAT_SYNTAX, MSG_MISSING_SHEBANG, MSG_INVALID_SHEBANG,
    MSG_UNMATCHED_BRACKET, MSG_UNCLOSED_BRACKET, MSG_UNCLOSED_BRACE,
    MSG_UNCLOSED_PAREN, MSG_UNCLOSED_SINGLE_QUOTE, MSG_UNCLOSED_DOUBLE_QUOTE,
    MSG_UNCLOSED_VAR_EXPANSION, MSG_UNCLOSED_CMD_SUBST,
};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref UNCLOSED_BRACE: Regex = Regex::new(r"\$\{[^}]*$").unwrap();
    static ref UNCLOSED_PAREN: Regex = Regex::new(r"\$\([^)]*$").unwrap();
}

pub fn check(parser: &ScriptParser, report: &mut Report, language: &Language) {
    check_shebang(parser, report, language);
    check_brackets(parser, report, language);
    check_quotes(parser, report, language);
    check_variable_expansion(parser, report, language);
}

fn check_shebang(parser: &ScriptParser, report: &mut Report, language: &Language) {
    if let Some(first_line) = parser.lines().first() {
        let content = &first_line.content;
        if !content.starts_with("#!") {
            report.add_issue(
                1,
                Severity::Error,
                CAT_SYNTAX.get(language),
                MSG_MISSING_SHEBANG.get(language)
            );
        } else if !content.contains("bash") && !content.contains("sh") {
            report.add_issue(
                1,
                Severity::Warning,
                CAT_SYNTAX.get(language),
                MSG_INVALID_SHEBANG.get(language)
            );
        }
    }
}

fn check_brackets(parser: &ScriptParser, report: &mut Report, language: &Language) {
    let mut bracket_stack = Vec::new();
    let mut brace_stack = Vec::new();
    let mut paren_stack = Vec::new();

    for line in parser.lines() {
        if line.trimmed.starts_with('#') {
            continue;
        }

        for (idx, ch) in line.content.chars().enumerate() {
            match ch {
                '[' => bracket_stack.push((line.number, idx)),
                ']' => {
                    if bracket_stack.is_empty() {
                        report.add_issue(
                            line.number,
                            Severity::Error,
                            CAT_SYNTAX.get(language),
                            MSG_UNMATCHED_BRACKET.get(language)
                        );
                    } else {
                        bracket_stack.pop();
                    }
                }
                '{' if idx == 0 || line.content.chars().nth(idx - 1) != Some('$') => {
                    brace_stack.push((line.number, idx));
                }
                '}' => {
                    if !brace_stack.is_empty() {
                        brace_stack.pop();
                    }
                }
                '(' if idx == 0 || line.content.chars().nth(idx - 1) != Some('$') => {
                    paren_stack.push((line.number, idx));
                }
                ')' => {
                    if !paren_stack.is_empty() {
                        paren_stack.pop();
                    }
                }
                _ => {}
            }
        }
    }

    for (line_num, _) in bracket_stack {
        report.add_issue(line_num, Severity::Error, CAT_SYNTAX.get(language), MSG_UNCLOSED_BRACKET.get(language));
    }
    for (line_num, _) in brace_stack {
        report.add_issue(line_num, Severity::Error, CAT_SYNTAX.get(language), MSG_UNCLOSED_BRACE.get(language));
    }
    for (line_num, _) in paren_stack {
        report.add_issue(line_num, Severity::Error, CAT_SYNTAX.get(language), MSG_UNCLOSED_PAREN.get(language));
    }
}

fn check_quotes(parser: &ScriptParser, report: &mut Report, language: &Language) {
    for line in parser.lines() {
        if line.trimmed.starts_with('#') {
            continue;
        }

        let mut in_single_quote = false;
        let mut in_double_quote = false;
        let mut escape_next = false;

        for ch in line.content.chars() {
            if escape_next {
                escape_next = false;
                continue;
            }

            match ch {
                '\\' => escape_next = true,
                '\'' if !in_double_quote => in_single_quote = !in_single_quote,
                '"' if !in_single_quote => in_double_quote = !in_double_quote,
                _ => {}
            }
        }

        if in_single_quote {
            report.add_issue(
                line.number,
                Severity::Error,
                CAT_SYNTAX.get(language),
                MSG_UNCLOSED_SINGLE_QUOTE.get(language)
            );
        }
        if in_double_quote {
            report.add_issue(
                line.number,
                Severity::Error,
                CAT_SYNTAX.get(language),
                MSG_UNCLOSED_DOUBLE_QUOTE.get(language)
            );
        }
    }
}

fn check_variable_expansion(parser: &ScriptParser, report: &mut Report, language: &Language) {
    for line in parser.lines() {
        if line.trimmed.starts_with('#') {
            continue;
        }

        if UNCLOSED_BRACE.is_match(&line.content) {
            report.add_issue(
                line.number,
                Severity::Error,
                CAT_SYNTAX.get(language),
                MSG_UNCLOSED_VAR_EXPANSION.get(language)
            );
        }

        if UNCLOSED_PAREN.is_match(&line.content) {
            report.add_issue(
                line.number,
                Severity::Error,
                CAT_SYNTAX.get(language),
                MSG_UNCLOSED_CMD_SUBST.get(language)
            );
        }
    }
}