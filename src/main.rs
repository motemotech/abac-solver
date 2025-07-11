use z3::{Config, Context, Solver, Sort, FuncDecl, Symbol, DatatypeBuilder, ast::{Ast, Dynamic}};
use std::collections::HashMap;

fn main() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    // 1. 有限ドメインを列挙型（Datatype）として定義
    // ユーザー型: alice, bob, charlie のみ
    let user_sort = DatatypeBuilder::new(&ctx, "User")
        .variant("alice", vec![])
        .variant("bob", vec![])
        .variant("charlie", vec![])
        .finish();

    // リソース型: secret_data, dev_server のみ
    let resource_sort = DatatypeBuilder::new(&ctx, "Resource")
        .variant("secret_data", vec![])
        .variant("dev_server", vec![])
        .finish();

    // グループ型: admin, dev, guest のみ
    let group_sort = DatatypeBuilder::new(&ctx, "Group")
        .variant("admin", vec![])
        .variant("dev", vec![])
        .variant("guest", vec![])
        .finish();

    // 2. 定数を取得
    let alice = user_sort.variants[0].constructor.apply(&[]);
    let bob = user_sort.variants[1].constructor.apply(&[]);
    let charlie = user_sort.variants[2].constructor.apply(&[]);

    let secret_data = resource_sort.variants[0].constructor.apply(&[]);
    let dev_server = resource_sort.variants[1].constructor.apply(&[]);

    let admin_group = group_sort.variants[0].constructor.apply(&[]);
    let dev_group = group_sort.variants[1].constructor.apply(&[]);
    let guest_group = group_sort.variants[2].constructor.apply(&[]);

    // 3. 属性を関数として定義
    let user_in_group = FuncDecl::new(&ctx, "user_in_group", &[&user_sort.sort, &group_sort.sort], &Sort::bool(&ctx));
    let required_group_func = FuncDecl::new(&ctx, "required_group", &[&resource_sort.sort], &group_sort.sort);

    // 4. データセットの事実（Fact）をアサート
    // Alice の所属グループ: admin, dev
    solver.assert(&z3::ast::Bool::try_from(user_in_group.apply(&[&alice, &admin_group])).unwrap());
    solver.assert(&z3::ast::Bool::try_from(user_in_group.apply(&[&alice, &dev_group])).unwrap());
    solver.assert(&z3::ast::Bool::try_from(user_in_group.apply(&[&alice, &guest_group])).unwrap().not());

    // Bob の所属グループ: dev
    solver.assert(&z3::ast::Bool::try_from(user_in_group.apply(&[&bob, &admin_group])).unwrap().not());
    solver.assert(&z3::ast::Bool::try_from(user_in_group.apply(&[&bob, &dev_group])).unwrap());
    solver.assert(&z3::ast::Bool::try_from(user_in_group.apply(&[&bob, &guest_group])).unwrap().not());

    // Charlie の所属グループ: guest
    solver.assert(&z3::ast::Bool::try_from(user_in_group.apply(&[&charlie, &admin_group])).unwrap().not());
    solver.assert(&z3::ast::Bool::try_from(user_in_group.apply(&[&charlie, &dev_group])).unwrap().not());
    solver.assert(&z3::ast::Bool::try_from(user_in_group.apply(&[&charlie, &guest_group])).unwrap());

    // リソースが必要とするグループ
    solver.assert(&required_group_func.apply(&[&secret_data])._eq(&admin_group));
    solver.assert(&required_group_func.apply(&[&dev_server])._eq(&dev_group));

    // 5. 解を求めるための変数を準備
    let u_var = Dynamic::new_const(&ctx, "u", &user_sort.sort);
    let r_var = Dynamic::new_const(&ctx, "r", &resource_sort.sort);

    // 6. アクセス制御ルール：ユーザーがリソースの必要グループに所属している
    let required_group = required_group_func.apply(&[&r_var]);
    let has_access = user_in_group.apply(&[&u_var, &required_group]);
    solver.assert(&z3::ast::Bool::try_from(has_access).unwrap());

    // 7. ループですべての解を列挙
    println!("--- アクセス可能な (ユーザー, リソース) の組み合わせ ---");
    let mut solution_count = 0;
    loop {
        match solver.check() {
            z3::SatResult::Sat => {
                solution_count += 1;
                let model = solver.get_model().unwrap();

                // モデルから u_var と r_var の値を取得
                let found_user = model.eval(&u_var, true).unwrap();
                let found_resource = model.eval(&r_var, true).unwrap();

                // 人間が読める形式で表示
                println!("ペア {}: ({}, {})",
                    solution_count,
                    found_user.to_string(),
                    found_resource.to_string()
                );

                // 見つかった解を禁止する制約を追加して、次の解を探す
                let block_this_solution = u_var._eq(&found_user) & r_var._eq(&found_resource);
                solver.assert(&block_this_solution.not());
            }
            z3::SatResult::Unsat => {
                println!("\n--- これ以上見つかりません ---");
                break;
            }
            z3::SatResult::Unknown => {
                println!("\n--- ソルバーが不明な結果を返しました ---");
                break;
            }
        }
    }
}