// #[derive(Debug, Clone, PartialEq)]
#[derive(Debug)]
pub enum Token {
    // keywords
    HeyChat,
    Yo,
    Is,
    Dogwater,
    Yap,
    Sus,
    Then,
    Lockin,
    Times,
    PutFries,
    Bet,
    TypeShi,
    Cap,
    Nocap,

    // operators
    Plus,
    GreaterThan,  // >>
    LessThan,     // <<

    // literals
    Identifier(String),
    Number(i32),
    StringLiteral(String),

    // misc
    Newline,
    Eof,
}
