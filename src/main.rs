use z3::ast::{Ast, Int};
use z3::{Config, Context, Solver, SatResult};

fn main() {
    // 1. Z3のConfigとContextを作成
    // ConfigはZ3のグローバルな設定を管理します。
    // Contextは、Z3のオブジェクト（変数、式、ソルバーなど）を管理する環境です。
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    // 2. Solverのインスタンスを作成
    // Solverは、制約を追加し、その充足可能性をチェックするためのオブジェクトです。
    let solver = Solver::new(&ctx);

    // 3. 変数の宣言
    // 'x' と 'y' という名前の整数の変数を宣言します。
    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");

    // 4. 制約の追加 (Assert)
    // 整数リテラルを作成します。
    let five = Int::from_i64(&ctx, 5);
    let one = Int::from_i64(&ctx, 1);

    // 制約1: x + y == 5
    // `_eq` メソッドは Ast トレイトで定義されており、等価性を表す式を生成します。
    solver.assert(&(&x + &y)._eq(&five));

    // 制約2: x > 1
    // `gt` (greater than) メソッドで不等式を表現します。
    solver.assert(&x.gt(&one));

    // 5. 充足可能性のチェック
    // solver.check() を呼び出すと、追加された制約がすべて満たされるかZ3が判定します。
    println!("制約:");
    println!("{}", solver);

    match solver.check() {
        // Sat (Satisfiable) の場合: 解が存在する
        SatResult::Sat => {
            println!("\n解が見つかりました (Sat)");

            // 6. モデルの取得と結果の表示
            // solver.get_model() で、制約を満たす解（モデル）を取得します。
            let model = solver.get_model().unwrap();

            // モデルを使って各変数の具体的な値を評価（eval）します。
            let x_val = model.eval(&x, true).unwrap();
            let y_val = model.eval(&y, true).unwrap();

            println!("x = {}", x_val);
            println!("y = {}", y_val);
        }
        // Unsat (Unsatisfiable) の場合: 解が存在しない
        SatResult::Unsat => {
            println!("\n解はありません (Unsat)");
        }
        // Unknown の場合: タイムアウトなどで解の有無が不明
        SatResult::Unknown => {
            println!("\n解の有無は不明です (Unknown)");
        }
    }
}