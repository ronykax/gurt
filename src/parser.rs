use crate::token::Token;

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    String(String),
    Variable(String),
    BinaryOp(Box<Expr>, String, Box<Expr>),
    Call(String),
}

#[derive(Debug)]
pub enum Stmt {
    VarDecl(String, Expr),
    Print(Expr),
}

pub fn parse(tokens: Vec<Token>) -> Vec<Stmt> {
    let mut stmts = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Yo => {
                if let Token::Identifier(name) = &tokens[i + 1] {
                    if let Token::Is = tokens[i + 2] {
                        let expr = match &tokens[i + 3] {
                            Token::Number(n) => Expr::Number(*n),
                            Token::StringLiteral(s) => Expr::String(s.clone()),
                            Token::Identifier(id) => Expr::Variable(id.clone()),
                            _ => panic!("gng invalid value after 'is' 💔"),
                        };
                        stmts.push(Stmt::VarDecl(name.clone(), expr));
                        i += 5;
                        continue;
                    }
                }
            }

            Token::Yap => {
                let expr = match &tokens[i + 1] {
                    Token::StringLiteral(s) => Expr::String(s.clone()),
                    Token::Identifier(id) => Expr::Variable(id.clone()),
                    _ => panic!("gng invalid yap 💔"),
                };
                stmts.push(Stmt::Print(expr));
                i += 3;
                continue;
            }

            _ => {
                i += 1;
            }
        }
    }

    stmts
}
