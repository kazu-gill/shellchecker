# Bash Script Checker

A standalone bash script linter written in Rust that works on both Windows and Linux.

## Features

This tool performs comprehensive checks on bash scripts in the following order:

1. **Syntax Check** - Validates basic bash syntax
   - Shebang line verification
   - Bracket/brace/parenthesis matching
   - Quote balance checking
   - Variable expansion syntax

2. **Best Practice Validation** - Checks for recommended practices
   - `set -e`, `set -u`, `set -o pipefail` usage
   - Variable quoting
   - `cd` command error handling
   - Command substitution style (prefer `$()` over backticks)

3. **Security Check** - Identifies potential security issues
   - `eval` usage
   - Piping curl/wget to shell
   - Dangerous `rm -rf` usage
   - Unvalidated user input in commands

4. **Style Check** - Enforces coding style conventions
   - Indentation consistency
   - Line length limits
   - Function naming conventions (snake_case)
   - Variable naming conventions

## Installation

### From Source
```bash
# Clone or download the source code
git clone 
cd shellchecker

# Build release binary
cargo build --release

# The binary will be located at:
# ./target/release/shellchecker (Linux/macOS)
# .\target\release\shellchecker.exe (Windows)
```

### Binary Usage

Copy the compiled binary to a location in your PATH or use it directly.

## Usage

### Basic Usage
```bash
# Check a single file (English output)
shellchecker script.sh

# Check a single file (Japanese output)
shellchecker -l ja script.sh

# Check all bash scripts in a directory
shellchecker /path/to/scripts

# Recursively check all bash scripts in a directory
shellchecker -r /path/to/scripts

# Show only errors (hide warnings and info)
shellchecker -e script.sh

# Japanese output with recursive scan
shellchecker -r -l ja /path/to/scripts
```

### Command Line Options
```
Usage: shellchecker [OPTIONS] <PATH>

Arguments:
  <PATH>  Path to bash script or directory

Options:
  -r, --recursive    Recursive directory scan
  -e, --errors-only  Show only errors (no warnings)
  -l, --language     Language for output (en, ja) [default: en]
  -h, --help         Print help
  -V, --version      Print version
```

### Language Support

The tool supports output in multiple languages:
- **English** (`-l en` or `--language en`) - Default
- **Japanese** (`-l ja` or `--language ja`)

## Output Format

The tool outputs issues in the following format:
```
L<line_number>: [SEVERITY] Category - Message
```

### English Output Example
```
Checking: example.sh
============================================================
L1: [ERROR] Syntax - Missing shebang line (#!/bin/bash or #!/bin/sh)
L5: [WARNING] Best Practice - Unquoted variable usage - consider using "$variable" to prevent word splitting
L12: [ERROR] Security - Usage of 'eval' is dangerous - avoid dynamic code execution
L20: [INFO] Style - Line too long (135 > 120 characters)

Summary: 2 error(s), 1 warning(s)
```

### Japanese Output Example
```
チェック中: example.sh
============================================================
L1: [エラー] 構文 - シバン行がありません (#!/bin/bash または #!/bin/sh)
L5: [警告] ベストプラクティス - クォートされていない変数 - 単語分割を防ぐため "$variable" の使用を検討してください
L12: [エラー] セキュリティ - 'eval' の使用は危険です - 動的なコード実行を避けてください
L20: [情報] スタイル - 行が長すぎます (135 > 120 文字)

サマリ: 2 個のエラー, 1 個の警告
```

### Severity Levels

- **ERROR** / **エラー**: Critical issues that should be fixed
- **WARNING** / **警告**: Potential problems or violations of best practices
- **INFO** / **情報**: Style suggestions and minor improvements

## Exit Codes

- `0`: No errors found (warnings/info may exist)
- `1`: Errors found or file/directory access failed

This makes the tool suitable for CI/CD pipelines.

## CI/CD Integration

### Example: GitHub Actions
```yaml
name: Bash Script Lint

on: [push, pull_request]

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Run shellchecker
        run: |
          wget /shellchecker
          chmod +x shellchecker
          ./shellchecker -r ./scripts
```

### Example: GitLab CI
```yaml
bash-lint:
  stage: test
  script:
    - wget /shellchecker
    - chmod +x shellchecker
    - ./shellchecker -r ./scripts
```

## Supported Platforms

- Linux (x86_64, ARM64)
- Windows (x86_64)
- macOS (x86_64, ARM64)

## Project Structure
```
shellchecker/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs           # CLI entry point
│   ├── parser.rs         # Script parsing
│   ├── report.rs         # Report generation
│   ├── i18n.rs           # Internationalization
│   └── checker/
│       ├── mod.rs        # Checker orchestration
│       ├── syntax.rs     # Syntax validation
│       ├── best_practice.rs  # Best practice checks
│       ├── security.rs   # Security checks
│       └── style.rs      # Style checks
```

## License

This project is licensed under the MIT License - see the [LICENSE](#license-text) section below for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### License Text
```
MIT License

Copyright (c) 2026 Kazuki Fujimura

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

# Bash Script Checker

Windows と Linux の両方で動作する、Rust で書かれたスタンドアロンの bash スクリプトリンターです。

## 機能

このツールは、以下の順序で bash スクリプトの包括的なチェックを実行します：

1. **構文チェック** - 基本的な bash 構文の検証
   - シバン行の確認
   - 括弧/ブレース/丸括弧のマッチング
   - クォートのバランスチェック
   - 変数展開の構文

2. **ベストプラクティス検証** - 推奨される記述方法のチェック
   - `set -e`、`set -u`、`set -o pipefail` の使用
   - 変数のクォート
   - `cd` コマンドのエラーハンドリング
   - コマンド置換のスタイル（バッククォートより `$()` を推奨）

3. **セキュリティチェック** - 潜在的なセキュリティ問題の特定
   - `eval` の使用
   - curl/wget のパイプからシェルへの実行
   - 危険な `rm -rf` の使用
   - 検証されていないユーザー入力のコマンドでの使用

4. **スタイルチェック** - コーディングスタイル規約の強制
   - インデントの一貫性
   - 行の長さ制限
   - 関数の命名規則（スネークケース）
   - 変数の命名規則

## インストール

### ソースからビルド
```bash
# ソースコードのクローンまたはダウンロード
git clone 
cd shellchecker

# リリースバイナリのビルド
cargo build --release

# バイナリの場所：
# ./target/release/shellchecker (Linux/macOS)
# .\target\release\shellchecker.exe (Windows)
```

### バイナリの使用

コンパイルされたバイナリを PATH の通った場所にコピーするか、直接使用してください。

## 使い方

### 基本的な使用方法
```bash
# 単一ファイルのチェック（英語出力）
shellchecker script.sh

# 単一ファイルのチェック（日本語出力）
shellchecker -l ja script.sh

# ディレクトリ内のすべての bash スクリプトをチェック
shellchecker /path/to/scripts

# ディレクトリ内のすべての bash スクリプトを再帰的にチェック
shellchecker -r /path/to/scripts

# エラーのみ表示（警告と情報を非表示）
shellchecker -e script.sh

# 日本語出力で再帰的スキャン
shellchecker -r -l ja /path/to/scripts
```

### コマンドラインオプション
```
使用方法: shellchecker [オプション] <パス>

引数:
  <パス>  bash スクリプトまたはディレクトリへのパス

オプション:
  -r, --recursive    ディレクトリを再帰的にスキャン
  -e, --errors-only  エラーのみ表示（警告を非表示）
  -l, --language     出力言語 (en, ja) [デフォルト: en]
  -h, --help         ヘルプを表示
  -V, --version      バージョンを表示
```

### 言語サポート

ツールは複数の言語での出力をサポートしています：
- **英語** (`-l en` または `--language en`) - デフォルト
- **日本語** (`-l ja` または `--language ja`)

## 出力形式

ツールは以下の形式で問題を出力します：
```
L<行番号>: [重大度] カテゴリ - メッセージ
```

### 英語出力の例
```
Checking: example.sh
============================================================
L1: [ERROR] Syntax - Missing shebang line (#!/bin/bash or #!/bin/sh)
L5: [WARNING] Best Practice - Unquoted variable usage - consider using "$variable" to prevent word splitting
L12: [ERROR] Security - Usage of 'eval' is dangerous - avoid dynamic code execution
L20: [INFO] Style - Line too long (135 > 120 characters)

Summary: 2 error(s), 1 warning(s)
```

### 日本語出力の例
```
チェック中: example.sh
============================================================
L1: [エラー] 構文 - シバン行がありません (#!/bin/bash または #!/bin/sh)
L5: [警告] ベストプラクティス - クォートされていない変数 - 単語分割を防ぐため "$variable" の使用を検討してください
L12: [エラー] セキュリティ - 'eval' の使用は危険です - 動的なコード実行を避けてください
L20: [情報] スタイル - 行が長すぎます (135 > 120 文字)

サマリ: 2 個のエラー, 1 個の警告
```

### 重大度レベル

- **ERROR** / **エラー**: 修正すべき重大な問題
- **WARNING** / **警告**: 潜在的な問題またはベストプラクティス違反
- **INFO** / **情報**: スタイル提案と軽微な改善点

## 終了コード

- `0`: エラーなし（警告/情報は存在する可能性あり）
- `1`: エラーが見つかった、またはファイル/ディレクトリへのアクセスに失敗

これにより、CI/CD パイプラインでの使用に適しています。

## CI/CD との統合

### 例: GitHub Actions
```yaml
name: Bash Script Lint

on: [push, pull_request]

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: shellchecker の実行
        run: |
          wget /shellchecker
          chmod +x shellchecker
          ./shellchecker -r ./scripts
```

### 例: GitLab CI
```yaml
bash-lint:
  stage: test
  script:
    - wget /shellchecker
    - chmod +x shellchecker
    - ./shellchecker -r ./scripts
```

## サポートプラットフォーム

- Linux (x86_64, ARM64)
- Windows (x86_64)
- macOS (x86_64, ARM64)

## プロジェクト構成
```
shellchecker/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs           # CLI エントリーポイント
│   ├── parser.rs         # スクリプト解析
│   ├── report.rs         # レポート生成
│   ├── i18n.rs           # 国際化対応
│   └── checker/
│       ├── mod.rs        # チェッカーの制御
│       ├── syntax.rs     # 構文検証
│       ├── best_practice.rs  # ベストプラクティスチェック
│       ├── security.rs   # セキュリティチェック
│       └── style.rs      # スタイルチェック
```

## ライセンス

このプロジェクトは MIT ライセンスの下でライセンスされています。詳細については、以下の[ライセンス条文](#ライセンス条文)セクションを参照してください。

## 貢献

貢献を歓迎します！プルリクエストをお気軽に送信してください。

### ライセンス条文
```
MIT License

Copyright (c) 2026 Kazuki Fujimura

以下に定める条件に従い、本ソフトウェアおよび関連文書のファイル（以下「ソフトウェア」）の複製を取得する
すべての人に対し、ソフトウェアを無制限に扱うことを無償で許可します。これには、ソフトウェアの複製を使用、
複写、変更、結合、掲載、頒布、サブライセンス、および/または販売する権利、およびソフトウェアを提供する
相手に同じことを許可する権利も無制限に含まれます。

上記の著作権表示および本許諾表示を、ソフトウェアのすべての複製または重要な部分に記載するものとします。

ソフトウェアは「現状のまま」で、明示であるか暗黙であるかを問わず、何らの保証もなく提供されます。
ここでいう保証とは、商品性、特定の目的への適合性、および権利非侵害についての保証も含みますが、
それに限定されるものではありません。作者または著作権者は、契約行為、不法行為、またはそれ以外であろうと、
ソフトウェアに起因または関連し、あるいはソフトウェアの使用またはその他の扱いによって生じる一切の請求、
損害、その他の義務について何らの責任も負わないものとします。
```