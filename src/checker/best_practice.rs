// src/checker/best_practice.rs
use crate::parser::ScriptParser;
use crate::report::{Report, Severity};
use crate::i18n::{
    Language, CAT_BEST_PRACTICE, MSG_USE_SET_E, MSG_USE_SET_U,
    MSG_USE_SET_PIPEFAIL, MSG_UNQUOTED_VARIABLE, MSG_CD_WITHOUT_CHECK,
    MSG_USE_DOLLAR_PAREN,
};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref SET_OPTIONS: Regex = Regex::new(r"^\s*set\s+-[euxo]").unwrap();
    static ref VARIABLE_USAGE: Regex = Regex::new(r"\$[A-Za-z_][A-Za-z0-9_]*").unwrap();
}

pub fn check(parser: &ScriptParser, report: &mut Report, language: &Language) {
    check_set_options(parser, report, language);
    check_variable_quoting(parser, report, language);
    check_cd_without_check(parser, report, language);
    check_command_substitution_style(parser, report, language);
}

fn check_set_options(parser: &ScriptParser, report: &mut Report, language: &Language) {
    let mut has_set_e = false;
    let mut has_set_u = false;
    let mut has_set_pipefail = false;

    for line in parser.lines() {
        if line.trimmed.starts_with('#') {
            continue;
        }

        if line.content.contains("set -e") || line.content.contains("set -o errexit") {
            has_set_e = true;
        }
        if line.content.contains("set -u") || line.content.contains("set -o nounset") {
            has_set_u = true;
        }
        if line.content.contains("set -o pipefail") {
            has_set_pipefail = true;
        }
    }

    if !has_set_e {
        report.add_issue(
            1,
            Severity::Warning,
            CAT_BEST_PRACTICE.get(language),
            MSG_USE_SET_E.get(language)
        );
    }
    if !has_set_u {
        report.add_issue(
            1,
            Severity::Warning,
            CAT_BEST_PRACTICE.get(language),
            MSG_USE_SET_U.get(language)
        );
    }
    if !has_set_pipefail {
        report.add_issue(
            1,
            Severity::Warning,
            CAT_BEST_PRACTICE.get(language),
            MSG_USE_SET_PIPEFAIL.get(language)
        );
    }
}

fn check_variable_quoting(parser: &ScriptParser, report: &mut Report, language: &Language) {
    for line in parser.lines() {
        if line.trimmed.starts_with('#') {
            continue;
        }

        // コマンド置換や条件式内は除外
        if line.content.contains("[[") || line.content.contains("$(") {
            continue;
        }

        // 変数使用箇所を検出
        for mat in VARIABLE_USAGE.find_iter(&line.content) {
            let var_usage = mat.as_str();
            let pos = mat.start();

            // 前後の文字をチェック
            let before_char = if pos > 0 {
                line.content.chars().nth(pos - 1)
            } else {
                None
            };

            let after_pos = mat.end();
            let after_char = line.content.chars().nth(after_pos);

            // クォートまたは波括弧で囲まれていない変数を検出
            let is_quoted = before_char == Some('"') || after_char == Some('"');
            let is_braced = var_usage.starts_with("${") || after_char == Some('}');

            if !is_quoted && !is_braced {
                report.add_issue(
                    line.number,
                    Severity::Warning,
                    CAT_BEST_PRACTICE.get(language),
                    MSG_UNQUOTED_VARIABLE.get(language)
                );
                break; // 1行につき1回だけ警告
            }
        }
    }
}

fn check_cd_without_check(parser: &ScriptParser, report: &mut Report, language: &Language) {
    for (idx, line) in parser.lines().iter().enumerate() {
        if line.trimmed.starts_with("cd ") {
            // 次の行が || や if [ $? で始まっているかチェック
            let next_line = parser.lines().get(idx + 1);
            let has_check = next_line.map_or(false, |nl| {
                nl.trimmed.starts_with("||") ||
                    nl.trimmed.starts_with("if") ||
                    line.content.contains("||") ||
                    line.content.contains("&&")
            });

            if !has_check {
                report.add_issue(
                    line.number,
                    Severity::Warning,
                    CAT_BEST_PRACTICE.get(language),
                    MSG_CD_WITHOUT_CHECK.get(language)
                );
            }
        }
    }
}

fn check_command_substitution_style(parser: &ScriptParser, report: &mut Report, language: &Language) {
    for line in parser.lines() {
        if line.trimmed.starts_with('#') {
            continue;
        }

        if line.content.contains("`") && !line.content.contains("\\`") {
            report.add_issue(
                line.number,
                Severity::Info,
                CAT_BEST_PRACTICE.get(language),
                MSG_USE_DOLLAR_PAREN.get(language)
            );
        }
    }
}