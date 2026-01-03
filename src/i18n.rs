// src/i18n.rs
#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    English,
    Japanese,
}

pub struct Message {
    pub en: &'static str,
    pub ja: &'static str,
}

impl Message {
    pub fn get(&self, lang: &Language) -> &str {
        match lang {
            Language::English => self.en,
            Language::Japanese => self.ja,
        }
    }
}

// Severity levels
pub const SEVERITY_ERROR: Message = Message {
    en: "ERROR",
    ja: "エラー",
};

pub const SEVERITY_WARNING: Message = Message {
    en: "WARNING",
    ja: "警告",
};

pub const SEVERITY_INFO: Message = Message {
    en: "INFO",
    ja: "情報",
};

// Summary messages
pub const NO_ISSUES: Message = Message {
    en: "✓ No issues found",
    ja: "✓ 問題は見つかりませんでした",
};

pub const SUMMARY: Message = Message {
    en: "Summary",
    ja: "サマリ",
};

pub const ERRORS: Message = Message {
    en: "error(s)",
    ja: "個のエラー",
};

pub const WARNINGS: Message = Message {
    en: "warning(s)",
    ja: "個の警告",
};

// Categories
pub const CAT_SYNTAX: Message = Message {
    en: "Syntax",
    ja: "構文",
};

pub const CAT_BEST_PRACTICE: Message = Message {
    en: "Best Practice",
    ja: "ベストプラクティス",
};

pub const CAT_SECURITY: Message = Message {
    en: "Security",
    ja: "セキュリティ",
};

pub const CAT_STYLE: Message = Message {
    en: "Style",
    ja: "スタイル",
};

// Error messages - Syntax
pub const MSG_MISSING_SHEBANG: Message = Message {
    en: "Missing shebang line (#!/bin/bash or #!/bin/sh)",
    ja: "シバン行がありません (#!/bin/bash または #!/bin/sh)",
};

pub const MSG_INVALID_SHEBANG: Message = Message {
    en: "Shebang does not specify bash or sh",
    ja: "シバン行でbashまたはshが指定されていません",
};

pub const MSG_UNMATCHED_BRACKET: Message = Message {
    en: "Unmatched closing bracket ']'",
    ja: "対応しない閉じ括弧 ']'",
};

pub const MSG_UNCLOSED_BRACKET: Message = Message {
    en: "Unclosed bracket '['",
    ja: "閉じられていない括弧 '['",
};

pub const MSG_UNCLOSED_BRACE: Message = Message {
    en: "Unclosed brace '{'",
    ja: "閉じられていない波括弧 '{'",
};

pub const MSG_UNCLOSED_PAREN: Message = Message {
    en: "Unclosed parenthesis '('",
    ja: "閉じられていない丸括弧 '('",
};

pub const MSG_UNCLOSED_SINGLE_QUOTE: Message = Message {
    en: "Unclosed single quote",
    ja: "閉じられていないシングルクォート",
};

pub const MSG_UNCLOSED_DOUBLE_QUOTE: Message = Message {
    en: "Unclosed double quote",
    ja: "閉じられていないダブルクォート",
};

pub const MSG_UNCLOSED_VAR_EXPANSION: Message = Message {
    en: "Unclosed variable expansion ${...}",
    ja: "閉じられていない変数展開 ${...}",
};

pub const MSG_UNCLOSED_CMD_SUBST: Message = Message {
    en: "Unclosed command substitution $(...)",
    ja: "閉じられていないコマンド置換 $(...)",
};

// Best Practice messages
pub const MSG_USE_SET_E: Message = Message {
    en: "Consider using 'set -e' to exit on errors",
    ja: "'set -e' の使用を検討してください（エラー時に終了）",
};

pub const MSG_USE_SET_U: Message = Message {
    en: "Consider using 'set -u' to treat unset variables as errors",
    ja: "'set -u' の使用を検討してください（未定義変数をエラーとして扱う）",
};

pub const MSG_USE_SET_PIPEFAIL: Message = Message {
    en: "Consider using 'set -o pipefail' to catch errors in pipelines",
    ja: "'set -o pipefail' の使用を検討してください（パイプライン内のエラーを検出）",
};

pub const MSG_UNQUOTED_VARIABLE: Message = Message {
    en: "Unquoted variable usage - consider using \"$variable\" to prevent word splitting",
    ja: "クォートされていない変数 - 単語分割を防ぐため \"$variable\" の使用を検討してください",
};

pub const MSG_CD_WITHOUT_CHECK: Message = Message {
    en: "cd command without error checking - consider using 'cd dir || exit 1'",
    ja: "cdコマンドにエラーチェックがありません - 'cd dir || exit 1' の使用を検討してください",
};

pub const MSG_USE_DOLLAR_PAREN: Message = Message {
    en: "Use $(...) instead of backticks for command substitution",
    ja: "コマンド置換にはバッククォートではなく $(...) を使用してください",
};

// Security messages
pub const MSG_EVAL_DANGEROUS: Message = Message {
    en: "Usage of 'eval' is dangerous - avoid dynamic code execution",
    ja: "'eval' の使用は危険です - 動的なコード実行を避けてください",
};

pub const MSG_CURL_PIPE_SH: Message = Message {
    en: "Piping curl/wget directly to shell is dangerous - download and inspect first",
    ja: "curl/wgetを直接シェルにパイプするのは危険です - まずダウンロードして検査してください",
};

pub const MSG_DANGEROUS_RM: Message = Message {
    en: "Dangerous rm -rf usage with variable or root path - add proper validation",
    ja: "変数またはルートパスでの危険な rm -rf の使用 - 適切な検証を追加してください",
};

pub const MSG_USER_INPUT_IN_CMD: Message = Message {
    en: "User input used in command execution - validate and sanitize input",
    ja: "ユーザー入力がコマンド実行で使用されています - 入力の検証とサニタイズを行ってください",
};

// Style messages
pub const MSG_USE_SPACES: Message = Message {
    en: "Use spaces instead of tabs for indentation",
    ja: "インデントにはタブではなくスペースを使用してください",
};

pub const MSG_INCONSISTENT_INDENT: Message = Message {
    en: "Inconsistent indentation - use 2 or 4 spaces",
    ja: "インデントが不統一です - 2または4スペースを使用してください",
};

pub fn msg_line_too_long(length: usize, max: usize, lang: &Language) -> String {
    match lang {
        Language::English => format!("Line too long ({} > {} characters)", length, max),
        Language::Japanese => format!("行が長すぎます ({} > {} 文字)", length, max),
    }
}

pub fn msg_function_naming(func_name: &str, lang: &Language) -> String {
    match lang {
        Language::English => format!(
            "Function name '{}' should use snake_case (lowercase with underscores)",
            func_name
        ),
        Language::Japanese => format!(
            "関数名 '{}' はスネークケース（小文字とアンダースコア）を使用してください",
            func_name
        ),
    }
}

pub fn msg_variable_naming(var_name: &str, lang: &Language) -> String {
    match lang {
        Language::English => format!(
            "Local variable '{}' should use lowercase with underscores",
            var_name
        ),
        Language::Japanese => format!(
            "ローカル変数 '{}' は小文字とアンダースコアを使用してください",
            var_name
        ),
    }
}