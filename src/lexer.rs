use crate::token::{self, Token};

// take .gurt and return the list of all the tokens
pub fn lex(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for line in source.lines() {
        let line = line.trim(); // remove whitespace

        // skip line if empty
        if line.is_empty() {
            continue;
        }

        let parts = line.split_whitespace().collect::<Vec<_>>();
        let mut i = 0;

        while i < parts.len() {
            // identify token for each word
            match parts[i] {
                "heychat" => tokens.push(Token::HeyChat),
                "yo" => tokens.push(Token::Yo),
                "yap" => tokens.push(Token::Yap),
                "sus" => tokens.push(Token::Sus),
                "then" => tokens.push(Token::Then),
                "lockin" => tokens.push(Token::Lockin),
                "times" => tokens.push(Token::Times),
                "put_the_fries_in_the_bag" => tokens.push(Token::PutFries),
                "bet" => tokens.push(Token::Bet),
                "typeshi" => tokens.push(Token::TypeShi),
                "dogwater" => tokens.push(Token::Dogwater),

                "is" => tokens.push(Token::Is),
                "plus" => tokens.push(Token::Plus),
                ">>" => tokens.push(Token::GreaterThan),
                "<<" => tokens.push(Token::LessThan),

                // if a part starts with '"' then loop until it ended with '"' again and combine it into one string literal ah
                _ if parts[i].starts_with("\"") => {
                    let mut string = parts[i].trim_matches('"').to_string();
                    while !parts[i].ends_with("\"") && i + 1 < parts.len() {
                        i += 1;
                        string.push(' ');
                        string.push_str(parts[i].trim_matches('"'));
                    }
                    tokens.push(Token::StringLiteral(string));
                }

                // if smth isn't keyword or number, identify it as an identifier (name)
                _ => {
                    if let Ok(num) = parts[i].parse::<i32>() {
                        tokens.push(Token::Number(num));
                    } else {
                        tokens.push(Token::Identifier(parts[i].to_string()));
                    }
                }
            }

            i += 1; // move to the next word in the line
        }

        tokens.push(Token::Newline); // next line
    }

    tokens.push(Token::Eof); // end of file
    tokens
}
