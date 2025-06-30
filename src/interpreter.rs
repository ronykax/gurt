use crate::parser::{Stmt, Expr};
use std::collections::HashMap;

pub fn interpret(stmts: Vec<Stmt>) {
    let mut vars = HashMap::new();

    for stmt in stmts {
        match stmt {
            Stmt::VarDecl(name, expr) => {
                let value = eval_expr(&expr, &vars);
                vars.insert(name, value);
            }

            Stmt::Print(expr) => {
                let value = eval_expr(&expr, &vars);
                println!("{}", value);
            }
        }
    }
}

fn eval_expr(expr: &Expr, vars: &HashMap<String, String>) -> String {
    match expr {
        Expr::Number(n) => n.to_string(),
        Expr::String(s) => s.clone(),
        Expr::Variable(name) => vars.get(name).cloned().unwrap_or("undefined".to_string()),
        _ => "unsupported".to_string(),
    }
}
