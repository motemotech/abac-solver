# 🔍 ABAC Policy Analyzer with Z3

Attribute-Based Access Control (ABAC) ポリシーをZ3 SMTソルバーを使用して分析するツールです。

## 📋 概要

このツールは、ABACポリシーファイルを解析し、Z3制約ソルバーを使用してポリシーを満たす属性の組み合わせを列挙します。複数の複雑なABACファイルに対応しており、各ファイル固有のルールを実装しています。

## 🚀 セットアップ

### 前提条件
- Rust (1.70以上)
- Z3ライブラリ

### インストール
```bash
# プロジェクトをクローン/ダウンロード
cd abac-solver

# 依存関係をインストール
cargo build --release
```

## 📁 対応ABACファイル

プロジェクトには3つのABACポリシーファイルが含まれています：

| ファイル | 説明 | ユーザー数 | リソース数 | ルール数 |
|---------|------|-----------|------------|----------|
| `data/university.abac` | 大学システム（gradebook、transcript等） | 22 | 34 | 10 |
| `data/edocument.abac` | 文書管理システム（複雑な企業環境） | 500+ | 300+ | 25 |
| `data/workforce.abac` | 労働力管理システム（技術者、タスク等） | 200+ | 150+ | 28 |

## 🔧 使用方法

### 基本実行

```bash
# デフォルト（university.abac）で実行
cargo run

# 特定のファイルを指定
cargo run -- --file data/edocument.abac

# workforce.abacを分析
cargo run -- --file data/workforce.abac
```

### コマンドライン引数

```bash
cargo run -- [OPTIONS]

OPTIONS:
    -f, --file <FILE>           ABACファイルのパス [default: data/university.abac]
    -m, --max-solutions <NUM>   各ルールで見つける解の最大数 [default: 5]
    -v, --verbose               詳細な出力を表示
    -h, --help                  ヘルプ情報を表示
```

### 実行例

#### 1. University ABACの分析
```bash
cargo run -- --file data/university.abac --max-solutions 10
```
**期待される出力：**
- 学生のgradebookアクセス権の分析
- 教員の成績管理権限の確認
- transcriptアクセス制御の検証

#### 2. Document Management Systemの分析
```bash
cargo run -- --file data/edocument.abac --verbose
```
**期待される出力：**
- 文書の機密性に基づくアクセス制御
- 部署別のアクセス権限
- 顧客の文書閲覧権限

#### 3. Workforce Management Systemの分析
```bash
cargo run -- --file data/workforce.abac --max-solutions 8
```
**期待される出力：**
- 技術者のタスクアクセス権
- 管理者の部下タスク管理権限
- プロバイダー別のアクセス制御

### 詳細分析の実行例

```bash
# 最大解数を増やして詳細分析
cargo run -- --file data/edocument.abac --max-solutions 15 --verbose

# 全ABACファイルを順番に分析
cargo run -- --file data/university.abac
cargo run -- --file data/edocument.abac  
cargo run -- --file data/workforce.abac
```

## 📊 出力形式

### 基本情報
```
🔍 ABAC Policy Analyzer with Z3
Analyzing file: data/university.abac

📖 Parsing ABAC file...
✅ Successfully parsed ABAC file
📊 Parsed 22 users, 34 resources, 10 rules
```

### ポリシー構造分析
```
📋 Policy Structure Analysis
User Roles/Positions:
  student: 10
  faculty: 4
  staff: 4
  applicant: 2

Resource Types:
  gradebook: 6
  transcript: 10
  application: 12
  roster: 6
```

### Z3制約解決結果
```
🎯 Finding accessible combinations (max 5):
  ✅ 1: csStu1 can access cs101gradebook
    👤 User: csStu1
      position: student
      department: cs
      crsTaken: {"cs101"}
    📄 Resource: cs101gradebook
      type: gradebook
      crs: cs101
      departments: {"cs"}
```

## 🧠 実装されたルール

### University ABAC
- **Rule 1**: 学生が受講科目のgradebookにアクセス可能
- **Rule 2**: 教員・TAが担当科目のgradebook管理可能
- その他transcript、roster、applicationのアクセス制御

### Document ABAC  
- **Rule 1**: 未登録顧客は送付された文書のみ閲覧可能
- **Rule 7**: Sales部門のみinvoiceの送受信が可能
- その他部署別・機密性レベル別のアクセス制御

### Workforce ABAC
- **Rule 7**: 技術者は割り当てられたタスクのみアクセス可能
- **Rule 8**: 管理者は部下のタスクを管理可能
- その他プロバイダー・地域別のアクセス制御

## 🛠️ 開発・カスタマイズ

### 新しいABACファイルの追加

1. `data/`ディレクトリに新しい`.abac`ファイルを配置
2. `src/main.rs`の`add_generic_constraints`関数に新しいファイル用の制約を追加：

```rust
} else if policy.filename.contains("your_new_file") {
    add_your_new_file_constraints(ctx, solver, policy, user_id, resource_id, can_access);
}
```

3. 対応する制約関数を実装

### デバッグ・トラブルシューティング

```bash
# コンパイルエラーの確認
cargo check

# 詳細なエラー情報
RUST_BACKTRACE=1 cargo run -- --file data/your_file.abac

# テスト実行
cargo test
```

## 📝 ファイル構造

```
abac-solver/
├── src/
│   └── main.rs          # メインプログラム
├── data/
│   ├── university.abac  # 大学システムポリシー
│   ├── edocument.abac   # 文書管理システムポリシー
│   └── workforce.abac   # 労働力管理システムポリシー
├── Cargo.toml          # 依存関係設定
└── README.md           # このファイル
```

## ⚡ クイックスタート

```bash
# 1. 基本分析
cargo run

# 2. 複雑なファイルの分析
cargo run -- --file data/edocument.abac --max-solutions 10

# 3. 詳細情報付きで分析
cargo run -- --file data/workforce.abac --verbose

# 4. ヘルプ表示
cargo run -- --help
```

## 🔧 技術仕様

- **言語**: Rust
- **SMTソルバー**: Z3
- **対応ファイル形式**: ABAC policy files (.abac)
- **パーサー**: 正規表現ベース
- **制約解決**: Z3制約ソルバー

## 📚 参考文献

- [ABAC (Attribute-Based Access Control)](https://en.wikipedia.org/wiki/Attribute-based_access_control)
- [Z3 SMT Solver](https://github.com/Z3Prover/z3)
- [Rust Z3 Bindings](https://docs.rs/z3/latest/z3/)

---

**問題が発生した場合は、`--verbose`フラグを使用して詳細な出力を確認してください。** 