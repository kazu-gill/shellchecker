// src/report.rs
use std::fmt;
use crate::i18n::{Language, SEVERITY_ERROR, SEVERITY_WARNING, SEVERITY_INFO, NO_ISSUES, SUMMARY, ERRORS, WARNINGS};

#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

impl Severity {
    pub fn as_str(&self, lang: &Language) -> &str {
        match self {
            Severity::Error => SEVERITY_ERROR.get(lang),
            Severity::Warning => SEVERITY_WARNING.get(lang),
            Severity::Info => SEVERITY_INFO.get(lang),
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str(&Language::English))
    }
}

#[derive(Debug, Clone)]
pub struct Issue {
    pub line: usize,
    pub severity: Severity,
    pub category: String,
    pub message: String,
}

pub struct Report {
    issues: Vec<Issue>,
}

impl Report {
    pub fn new() -> Self {
        Report { issues: Vec::new() }
    }

    pub fn add_issue(&mut self, line: usize, severity: Severity, category: &str, message: &str) {
        self.issues.push(Issue {
            line,
            severity,
            category: category.to_string(),
            message: message.to_string(),
        });
    }

    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(|i| i.severity == Severity::Error)
    }

    pub fn print(&self, errors_only: bool, language: &Language) {
        if self.issues.is_empty() {
            println!("{}", NO_ISSUES.get(language));
            return;
        }

        for issue in &self.issues {
            if errors_only && issue.severity != Severity::Error {
                continue;
            }

            println!(
                "L{}: [{}] {} - {}",
                issue.line,
                issue.severity.as_str(language),
                issue.category,
                issue.message
            );
        }

        let error_count = self.issues.iter().filter(|i| i.severity == Severity::Error).count();
        let warning_count = self.issues.iter().filter(|i| i.severity == Severity::Warning).count();

        println!();
        println!(
            "{}: {} {}, {} {}",
            SUMMARY.get(language),
            error_count,
            ERRORS.get(language),
            warning_count,
            WARNINGS.get(language)
        );
    }
}