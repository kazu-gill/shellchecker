// src/checker/security.rs
use crate::parser::ScriptParser;
use crate::report::{Report, Severity};
use crate::i18n::{
    Language, CAT_SECURITY, MSG_EVAL_DANGEROUS, MSG_CURL_PIPE_SH,
    MSG_DANGEROUS_RM, MSG_USER_INPUT_IN_CMD,
};

pub fn check(parser: &ScriptParser, report: &mut Report, language: &Language) {
    check_eval_usage(parser, report, language);
    check_curl_pipe_sh(parser, report, language);
    check_rm_rf_usage(parser, report, language);
    check_user_input_in_commands(parser, report, language);
}

fn check_eval_usage(parser: &ScriptParser, report: &mut Report, language: &Language) {
    for line in parser.lines() {
        if line.trimmed.starts_with('#') {
            continue;
        }

        if line.content.contains("eval ") {
            report.add_issue(
                line.number,
                Severity::Error,
                CAT_SECURITY.get(language),
                MSG_EVAL_DANGEROUS.get(language)
            );
        }
    }
}

fn check_curl_pipe_sh(parser: &ScriptParser, report: &mut Report, language: &Language) {
    for line in parser.lines() {
        if (line.content.contains("curl") || line.content.contains("wget"))
            && line.content.contains("|")
            && (line.content.contains("sh") || line.content.contains("bash")) {
            report.add_issue(
                line.number,
                Severity::Error,
                CAT_SECURITY.get(language),
                MSG_CURL_PIPE_SH.get(language)
            );
        }
    }
}

fn check_rm_rf_usage(parser: &ScriptParser, report: &mut Report, language: &Language) {
    for line in parser.lines() {
        if line.trimmed.starts_with('#') {
            continue;
        }

        if line.content.contains("rm -rf /") || line.content.contains("rm -rf $") {
            report.add_issue(
                line.number,
                Severity::Error,
                CAT_SECURITY.get(language),
                MSG_DANGEROUS_RM.get(language)
            );
        }
    }
}

fn check_user_input_in_commands(parser: &ScriptParser, report: &mut Report, language: &Language) {
    for line in parser.lines() {
        if line.trimmed.starts_with('#') {
            continue;
        }

        // read コマンドで取得した変数が直接使われているかチェック
        if (line.content.contains("read ") || line.content.contains("$1") || line.content.contains("$@"))
            && (line.content.contains("eval") || line.content.contains("$(") || line.content.contains("`")) {
            report.add_issue(
                line.number,
                Severity::Warning,
                CAT_SECURITY.get(language),
                MSG_USER_INPUT_IN_CMD.get(language)
            );
        }
    }
}