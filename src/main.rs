// src/main.rs
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

mod checker;
mod parser;
mod report;
mod i18n;

use checker::Checker;
use i18n::Language;

#[derive(Parser, Debug)]
#[command(author, version, about = "Bash script checker", long_about = None)]
struct Args {
    /// Path to bash script or directory
    #[arg(value_name = "PATH")]
    path: PathBuf,

    /// Recursive directory scan
    #[arg(short, long)]
    recursive: bool,

    /// Show only errors (no warnings)
    #[arg(short, long)]
    errors_only: bool,

    /// Language for output (en, ja)
    #[arg(short, long, default_value = "en")]
    language: String,
}

fn main() {
    let args = Args::parse();

    let language = match args.language.as_str() {
        "ja" => Language::Japanese,
        "en" => Language::English,
        _ => {
            eprintln!("Error: Unsupported language '{}'. Use 'en' or 'ja'.", args.language);
            std::process::exit(1);
        }
    };

    let exit_code = if args.path.is_file() {
        check_file(&args.path, args.errors_only, &language)
    } else if args.path.is_dir() {
        check_directory(&args.path, args.recursive, args.errors_only, &language)
    } else {
        eprintln!("Error: Path does not exist: {:?}", args.path);
        std::process::exit(1);
    };

    std::process::exit(exit_code);
}

// src/main.rs の check_file 関数を修正
// src/main.rs の check_file 関数
fn check_file(path: &PathBuf, errors_only: bool, language: &Language) -> i32 {
    match fs::read_to_string(path) {
        Ok(content) => {
            match language {
                Language::English => println!("Checking: {}", path.display()),
                Language::Japanese => println!("チェック中: {}", path.display()),
            }
            println!("{}", "=".repeat(60));

            let checker = Checker::new(&content, language.clone());
            let report = checker.check();

            report.print(errors_only, language);

            if report.has_errors() {
                1
            } else {
                0
            }
        }
        Err(e) => {
            match language {
                Language::English => eprintln!("Error reading file {:?}: {}", path, e),
                Language::Japanese => eprintln!("ファイル読み込みエラー {:?}: {}", path, e),
            }
            1
        }
    }
}

fn check_directory(path: &PathBuf, recursive: bool, errors_only: bool, language: &Language) -> i32 {
    let mut has_errors = false;
    let max_depth = if recursive { usize::MAX } else { 1 };

    for entry in WalkDir::new(path).max_depth(max_depth) {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                match language {
                    Language::English => eprintln!("Error walking directory: {}", e),
                    Language::Japanese => eprintln!("ディレクトリ走査エラー: {}", e),
                }
                continue;
            }
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        if is_bash_script(path) {
            if check_file(&path.to_path_buf(), errors_only, language) != 0 {
                has_errors = true;
            }
            println!();
        }
    }

    if has_errors { 1 } else { 0 }
}

fn is_bash_script(path: &std::path::Path) -> bool {
    // .sh拡張子チェック
    if let Some(ext) = path.extension() {
        if ext == "sh" {
            return true;
        }
    }

    // シバン行チェック
    if let Ok(content) = fs::read_to_string(path) {
        if let Some(first_line) = content.lines().next() {
            return first_line.starts_with("#!/bin/bash")
                || first_line.starts_with("#!/bin/sh")
                || first_line.starts_with("#!/usr/bin/env bash")
                || first_line.starts_with("#!/usr/bin/env sh");
        }
    }

    false
}