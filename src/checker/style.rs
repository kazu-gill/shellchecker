// src/checker/style.rs
use crate::parser::ScriptParser;
use crate::report::{Report, Severity};
use crate::i18n::{
    Language, CAT_STYLE, MSG_USE_SPACES, MSG_INCONSISTENT_INDENT,
    msg_line_too_long, msg_function_naming, msg_variable_naming,
};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref FUNCTION_DEF: Regex = Regex::new(r"^function\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(").unwrap();
    static ref SIMPLE_FUNCTION_DEF: Regex = Regex::new(r"^([a-zA-Z_][a-zA-Z0-9_]*)\s*\(\s*\)").unwrap();
}

pub fn check(parser: &ScriptParser, report: &mut Report, language: &Language) {
    check_indentation(parser, report, language);
    check_line_length(parser, report, language);
    check_function_naming(parser, report, language);
    check_variable_naming(parser, report, language);
}

fn check_indentation(parser: &ScriptParser, report: &mut Report, language: &Language) {
    for line in parser.lines() {
        if line.trimmed.is_empty() || line.trimmed.starts_with('#') {
            continue;
        }

        let leading_spaces = line.content.len() - line.content.trim_start().len();

        // タブ文字チェック
        if line.content.starts_with('\t') {
            report.add_issue(
                line.number,
                Severity::Info,
                CAT_STYLE.get(language),
                MSG_USE_SPACES.get(language)
            );
        }

        // インデントが2または4の倍数かチェック
        if leading_spaces > 0 && leading_spaces % 2 != 0 {
            report.add_issue(
                line.number,
                Severity::Info,
                CAT_STYLE.get(language),
                MSG_INCONSISTENT_INDENT.get(language)
            );
        }
    }
}

fn check_line_length(parser: &ScriptParser, report: &mut Report, language: &Language) {
    const MAX_LINE_LENGTH: usize = 120;

    for line in parser.lines() {
        if line.content.len() > MAX_LINE_LENGTH {
            report.add_issue(
                line.number,
                Severity::Info,
                CAT_STYLE.get(language),
                &msg_line_too_long(line.content.len(), MAX_LINE_LENGTH, language)
            );
        }
    }
}

fn check_function_naming(parser: &ScriptParser, report: &mut Report, language: &Language) {
    for line in parser.lines() {
        if let Some(caps) = FUNCTION_DEF.captures(&line.content) {
            let func_name = &caps[1];
            if func_name.chars().any(|c| c.is_uppercase()) {
                report.add_issue(
                    line.number,
                    Severity::Info,
                    CAT_STYLE.get(language),
                    &msg_function_naming(func_name, language)
                );
            }
        } else if let Some(caps) = SIMPLE_FUNCTION_DEF.captures(&line.content) {
            let func_name = &caps[1];
            if func_name.chars().any(|c| c.is_uppercase()) {
                report.add_issue(
                    line.number,
                    Severity::Info,
                    CAT_STYLE.get(language),
                    &msg_function_naming(func_name, language)
                );
            }
        }
    }
}

fn check_variable_naming(parser: &ScriptParser, report: &mut Report, language: &Language) {
    let var_assign = Regex::new(r"^\s*([A-Z][A-Z0-9_]*)\s*=").unwrap();

    for line in parser.lines() {
        if line.trimmed.starts_with('#') {
            continue;
        }

        if let Some(caps) = var_assign.captures(&line.content) {
            let var_name = &caps[1];
            // 環境変数風の大文字変数名は許容
            if var_name.len() < 3 || !var_name.chars().all(|c| c.is_uppercase() || c == '_') {
                report.add_issue(
                    line.number,
                    Severity::Info,
                    CAT_STYLE.get(language),
                    &msg_variable_naming(var_name, language)
                );
            }
        }
    }
}