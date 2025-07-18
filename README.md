# 🔍 ABAC Policy Analyzer with Z3

Attribute-Based Access Control (ABAC) ポリシーをZ3 SMTソルバーを使用して分析するツールです。

## 📋 概要

このツールは、ABACポリシーファイルを解析し、Z3制約ソルバーを使用してポリシーを満たす属性の組み合わせを列挙します。複数の複雑なABACファイルに対応しており、各ファイル固有のルールを実装しています。

### 🆕 新機能：ルール別分析
- **統合制約モード**：全ルールを統合した制約での分析（従来方式）
- **ルール別分析モード**：各ルールを個別に分析し、ルール固有の解を発見
- **選択的ルール分析**：特定のルールIDを指定して対象ルールのみ分析
- **詳細ルール情報**：各ルールの説明と内容を表示して理解を促進

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

プロジェクトには5つのABACポリシーファイルが含まれています：

| ファイル | 説明 | ユーザー数 | リソース数 | ルール数 | ファイルサイズ | 実行難易度 |
|---------|------|-----------|------------|----------|------------|------------|
| `data/university.abac` | 大学システム（gradebook、transcript等） | 22 | 34 | 10 | 7KB | ⭐ 初心者向け |
| `data/edocument.abac` | 文書管理システム（企業環境） | 500+ | 300+ | 25 | 193KB | ⭐⭐ 中級 |
| `data/workforce.abac` | 労働力管理システム（技術者、タスク等） | 200+ | 150+ | 28 | 145KB | ⭐⭐ 中級 |
| `data/edocument_extended.abac` | 拡張文書管理システム（多属性） | 1,000+ | 800+ | 30 | 1.3MB | ⭐⭐⭐ 上級 |
| `data/edocument_extended_large.abac` | 大規模文書管理システム（10K規模） | 10,000 | 10,000 | 30 | 14MB | ⭐⭐⭐⭐ エキスパート |

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
    -r, --rule-id <IDS>         特定のルールのみ分析 (例: --rule-id 1,3,5)
        --rule-by-rule          全ルールを個別に分析
    -h, --help                  ヘルプ情報を表示
```

### 実行例

#### 1. University ABAC（初心者向け）
```bash
# 統合制約での分析（従来方式）
cargo run -- --file data/university.abac --max-solutions 10

# 全ルールを個別に分析
cargo run -- --file data/university.abac --rule-by-rule --max-solutions 5

# 特定のルール（Rule 1のみ）を分析
cargo run -- --file data/university.abac --rule-id 1 --max-solutions 5
```
**期待される出力：**
- 学生のgradebookアクセス権の分析
- 教員の成績管理権限の確認
- transcriptアクセス制御の検証
- **実行時間**: 数秒

#### 2. Document Management System（中級）
```bash
# 統合制約での分析
cargo run -- --file data/edocument.abac --verbose

# ルール1（未登録顧客のアクセス制御）のみを分析
cargo run -- --file data/edocument.abac --rule-id 1 --max-solutions 10

# 複数のルール（Rule 1, 7, 10）を選択的に分析
cargo run -- --file data/edocument.abac --rule-id 1,7,10 --max-solutions 5
```
**期待される出力：**
- 文書の機密性に基づくアクセス制御
- 部署別のアクセス権限
- 顧客の文書閲覧権限
- **実行時間**: 10-30秒

#### 3. Workforce Management System（中級）
```bash
# 統合制約での分析
cargo run -- --file data/workforce.abac --max-solutions 8

# ルール7（技術者のタスクアクセス）のみを分析
cargo run -- --file data/workforce.abac --rule-id 7 --max-solutions 10

# 全ルールを個別に分析（詳細モード）
cargo run -- --file data/workforce.abac --rule-by-rule --verbose
```
**期待される出力：**
- 技術者のタスクアクセス権
- 管理者の部下タスク管理権限
- プロバイダー別のアクセス制御
- **実行時間**: 10-30秒

#### 4. 拡張文書管理システム（上級）
```bash
cargo run --release -- --file data/edocument_extended.abac --max-solutions 5
```
**期待される出力：**
- 地理的属性による制御
- セキュリティクリアランス制御
- プロジェクト管理属性
- **実行時間**: 1-3分

#### 5. 大規模文書管理システム（エキスパート）
```bash
cargo run --release -- --file data/edocument_extended_large.abac --max-solutions 3
```
**期待される出力：**
- 10,000ユーザー・10,000リソースの大規模分析
- 実行時間計測付き
- **実行時間**: 5-15分
- **注意**: 大量のメモリを使用します

### 詳細分析の実行例

#### 段階的なデータセット分析
```bash
# 1. 小規模データで動作確認
cargo run -- --file data/university.abac --verbose

# 2. 中規模データで基本性能測定
cargo run --release -- --file data/edocument.abac --max-solutions 10

# 3. 中規模データで詳細分析
cargo run --release -- --file data/workforce.abac --max-solutions 15 --verbose

# 4. 大規模データで性能測定
cargo run --release -- --file data/edocument_extended.abac --max-solutions 5

# 5. 超大規模データでスケーラビリティテスト
cargo run --release -- --file data/edocument_extended_large.abac --max-solutions 3
```

#### ルール別分析の活用例
```bash
# 1. 特定のルールのトラブルシューティング（University）
cargo run -- --file data/university.abac --rule-id 1 --verbose

# 2. 複数ルールの比較分析（Document）
cargo run -- --file data/edocument.abac --rule-id 1,7,15 --max-solutions 8

# 3. 全ルールの包括的分析（Workforce）
cargo run -- --file data/workforce.abac --rule-by-rule --max-solutions 3

# 4. 高速なルール検証（Extended Document）
cargo run --release -- --file data/edocument_extended.abac --rule-id 7,11,17 --max-solutions 2

# 5. 大規模データでの個別ルール性能測定
cargo run --release -- --file data/edocument_extended_large.abac --rule-id 1 --max-solutions 5
```

#### パフォーマンス比較分析
```bash
# 全ABACファイルでの実行時間比較
echo "=== University ABAC ===" && cargo run --release -- --file data/university.abac --max-solutions 5
echo "=== Document ABAC ===" && cargo run --release -- --file data/edocument.abac --max-solutions 5  
echo "=== Workforce ABAC ===" && cargo run --release -- --file data/workforce.abac --max-solutions 5
echo "=== Extended Document ABAC ===" && cargo run --release -- --file data/edocument_extended.abac --max-solutions 5
echo "=== Large Document ABAC ===" && cargo run --release -- --file data/edocument_extended_large.abac --max-solutions 5
```

#### 大規模データ専用オプション
```bash
# 高速実行（解を1つだけ）
cargo run --release -- --file data/edocument_extended_large.abac --max-solutions 1

# 詳細分析（時間に余裕がある場合）
cargo run --release -- --file data/edocument_extended_large.abac --max-solutions 10 --verbose

# メモリ効率を重視した実行
RUST_MIN_STACK=8388608 cargo run --release -- --file data/edocument_extended_large.abac --max-solutions 5
```

## 📊 出力形式

### 基本情報（実行時間計測付き）
```
🔍 ABAC Policy Analyzer with Z3
Analyzing file: data/university.abac
⏱️  Start time: 2024-01-15T10:30:00.123456Z

📖 Parsing ABAC file...
✅ Successfully parsed ABAC file
⏱️  Parse time: 12.34ms
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

#### 統合制約モード（従来）
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

#### ルール別分析モード（新機能）
```
🔍 === Rule 1 Analysis ===
📝 Description: 学生が受講した科目のgradebookにアクセス可能
📄 Content: permit(student, gradebook, read) :- userAttrib(student, crsTaken, courses), resourceAttrib(gradebook, crs, course), member(course, courses)

🎯 Finding solutions for Rule 1 (max 5):
  ✅ 1: csStu1 can access cs101gradebook
    👤 User: csStu1
      position: student
      department: cs
      crsTaken: {"cs101"}
    📄 Resource: cs101gradebook
      type: gradebook
      crs: cs101
      departments: {"cs"}

  📊 Rule 1 summary: 8 accessible combinations found
==================================================

🔍 === Rule 2 Analysis ===
📝 Description: 教員・TAが担当科目のgradebook管理可能
📄 Content: permit(faculty|ta, gradebook, add|read) :- userAttrib(faculty|ta, crsTaught, courses), resourceAttrib(gradebook, crs, course), member(course, courses)

🎯 Finding solutions for Rule 2 (max 5):
  ✅ 1: csFac1 can access cs101gradebook
  ...

✨ Rule-by-rule analysis complete!
```

### パフォーマンス要約（大規模データ用）
```
✨ Analysis Complete!
⏱️  Total execution time: 2m 34.56s
📊 Performance Summary:
   - Parsing: 1.23s (0.8%)
   - Structure analysis: 234.56ms (0.2%)
   - Z3 setup: 45.67ms (0.0%)
   - Z3 analysis: 2m 32.87s (99.0%)
```

## 🧠 実装されたルール

### University ABAC（10ルール）
- **Rule 1**: 学生が受講科目のgradebookにアクセス可能
- **Rule 2**: 教員・TAが担当科目のgradebook管理可能
- その他transcript、roster、applicationのアクセス制御

### Document ABAC（25ルール）
- **Rule 1**: 未登録顧客は送付された文書のみ閲覧可能
- **Rule 7**: Sales部門のみinvoiceの送受信が可能
- その他部署別・機密性レベル別のアクセス制御

### Workforce ABAC（28ルール）
- **Rule 7**: 技術者は割り当てられたタスクのみアクセス可能
- **Rule 8**: 管理者は部下のタスクを管理可能
- その他プロバイダー・地域別のアクセス制御

### Extended Document ABAC（30ルール）
- **Rule 7**: セキュリティクリアランスによる階層制御
- **Rule 11**: 地域ベースのアクセス制御
- **Rule 17**: 予算権限による承認制御
- **Rule 21**: 優先度による制限制御
- 地理的属性、時間ベース、プロジェクト管理等の拡張制御

### Large Document ABAC（30ルール + 大規模データ）
- Extended Document ABACと同じルールセット
- **10,000ユーザー × 10,000リソース**の大規模データセット
- 地理的分散、複雑な組織構造、多層セキュリティ制御
- **実行時間計測機能**による性能分析対応

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
│   └── main.rs                      # メインプログラム（実行時間計測機能付き）
├── data/
│   ├── university.abac              # 大学システムポリシー（22ユーザー、34リソース）
│   ├── edocument.abac               # 文書管理システムポリシー（500ユーザー、300リソース）
│   ├── workforce.abac               # 労働力管理システムポリシー（200ユーザー、150リソース）
│   ├── edocument_extended.abac      # 拡張文書管理システム（1000ユーザー、800リソース）
│   ├── edocument_extended_large.abac # 大規模文書管理システム（10Kユーザー、10Kリソース）
│   ├── abac_edocument.py            # edocument.abac生成スクリプト
│   └── abac_edocument_extended.py   # 大規模ABACファイル生成スクリプト
├── Cargo.toml                       # 依存関係設定
└── README.md                        # このファイル
```

## ⚡ クイックスタート

### 初心者向け（推奨開始地点）
```bash
# 1. 基本分析（デフォルト）
cargo run

# 2. 詳細情報付きで分析
cargo run -- --file data/university.abac --verbose

# 3. 特定のルールのみ分析（Rule 1）
cargo run -- --file data/university.abac --rule-id 1

# 4. 全ルールを個別に分析
cargo run -- --file data/university.abac --rule-by-rule
```

### 中級者向け
```bash
# 5. 文書管理システムの分析
cargo run --release -- --file data/edocument.abac --max-solutions 10

# 6. 労働力管理システムの分析
cargo run --release -- --file data/workforce.abac --verbose

# 7. 特定ルールの詳細分析（Document System - Rule 1, 7）
cargo run --release -- --file data/edocument.abac --rule-id 1,7 --verbose

# 8. 全ルール個別分析（Workforce System）
cargo run --release -- --file data/workforce.abac --rule-by-rule --max-solutions 3
```

### 上級者向け
```bash
# 9. 拡張文書管理システム（1000ユーザー）
cargo run --release -- --file data/edocument_extended.abac --max-solutions 5

# 10. 大規模システム（10,000ユーザー）- 実行時間計測付き
cargo run --release -- --file data/edocument_extended_large.abac --max-solutions 3

# 11. 拡張システムでの特定ルール分析（Rule 7, 11, 17）
cargo run --release -- --file data/edocument_extended.abac --rule-id 7,11,17 --max-solutions 3

# 12. 大規模データでの個別ルール性能測定
cargo run --release -- --file data/edocument_extended_large.abac --rule-id 1 --max-solutions 2
```

### その他
```bash
# ヘルプ表示
cargo run -- --help

# 全ファイルの実行時間比較
for file in data/*.abac; do echo "=== $file ==="; cargo run --release -- --file "$file" --max-solutions 1; done
```

## 🔧 技術仕様

- **言語**: Rust
- **SMTソルバー**: Z3
- **対応ファイル形式**: ABAC policy files (.abac)
- **パーサー**: 正規表現ベース
- **制約解決**: Z3制約ソルバー
- **実行時間計測**: 高精度タイマー（`std::time::Instant`）
- **スケール**: 最大10,000ユーザー × 10,000リソース
- **メモリ使用量**: 大規模データで最大数GB
- **パフォーマンス**: リリースモード推奨（`--release`フラグ）

## 📚 参考文献

- [ABAC (Attribute-Based Access Control)](https://en.wikipedia.org/wiki/Attribute-based_access_control)
- [Z3 SMT Solver](https://github.com/Z3Prover/z3)
- [Rust Z3 Bindings](https://docs.rs/z3/latest/z3/)

## ⚠️  重要な注意事項

### 大規模データセット使用時
- **`edocument_extended_large.abac`** (14MB) は大量のメモリ（数GB）を使用します
- 実行には **`--release`** フラグの使用を強く推奨します
- 初回実行時は5-15分程度の時間がかかる場合があります
- システムの**メモリとCPU使用量**を監視してください

### 実行時間計測機能
- 全てのフェーズ（パース、構造分析、Z3セットアップ、Z3分析）で実行時間を計測
- パフォーマンス比較により、どの処理が時間を要するかを特定可能
- 大規模データでのスケーラビリティ測定に活用できます

### トラブルシューティング
- **メモリ不足**: `--max-solutions` の値を小さくしてください
- **実行時間が長い**: `--max-solutions 1` で高速実行を試してください
- **詳細なエラー情報**: `--verbose` フラグを使用してください

---

**問題が発生した場合は、`--verbose`フラグを使用して詳細な出力を確認してください。** 


ポリシーのどれかにマッチしておけばよい。
ポリシーのどれかにマッチしている組み合わせを列挙する
ルールの数を増やすのには興味があるが、機械的にできるのか不明

ネットワークのポリシーに応用したい
送信元と送信先にそれぞれ属性のセットがあって、結局今と同じようなデータのフォーマットになる

ハッシュテーブルと論ゲストプレフィックス

ネットワークにおける応用を考えたときのエンコードの方法も考える

abacの標準的なフォーマットは？
小谷さんが、調べて標準的なフォーマットを出す。
ネットワークに適応する場合は、