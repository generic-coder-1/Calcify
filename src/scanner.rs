macro_rules! either {
    ($test:expr => $true_expr:expr; $false_expr:expr) => {
        if $test {
            $true_expr
        } else {
            $false_expr
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    //single char
    Plus,
    Minus,
    Slash,
    Comma,
    SemiColon,
    Colon,
    Dot,
    Bang,
    At,
    Equal,
    Pipe,
    Star,
    Percent,
    Ampersand,
    //scope stuff
    LBrace,
    RBrace,
    LParen,
    RParen,
    LBrack,
    RBrack,
    LArrow,
    RArrow,
    //two char
    LessOrEqual,
    MoreOrEqual,
    SmallArrow,
    EqualEqual,
    BangEqual,
    PlusEqual,
    MinusEqual,
    DoubleColon,
    SHL,
    //idents
    Ident,
    String,
    Int,
    Float,
    //Keywords
    Fn,
    Struct,
    Enum,
    While,
    Let,
    Return,
    Self_,
    If,
    Impl,
    Else,
    True,
    False,
    Trait,
    And,
    Or,
    Mut,
    For,
    Continue,
    Break,
    Panic,
    //other stuff
    Error,
    EOF,
    Comment,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub line_offset:usize,
    pub lexeme: String,
}

pub struct Scanner {
    source_code: String,
    line: usize,
    start: usize,
    current: usize,
}

trait NewAlphanumric {
    fn is_new_alphanumeric(&self) -> bool;
    fn is_new_alpha(&self) -> bool;
}

impl NewAlphanumric for char {
    fn is_new_alphanumeric(&self) -> bool {
        self.is_alphanumeric() || *self == '_'
    }

    fn is_new_alpha(&self) -> bool {
        self.is_alphabetic() || *self == '_'
    }
}

impl Scanner {
    pub fn scan(source: String) -> Vec<Token> {
        let mut tokens = vec![];
        let mut scanner: Scanner = Self {
            start: 0,
            current: 0,
            source_code: source,
            line: 0,
        };
        while TokenType::EOF
            != tokens
                .last()
                .and_then(|token: &Token| Some(token.token_type))
                .unwrap_or(TokenType::SemiColon)
        {
            tokens.push(
                scanner
                    .scan_token(tokens.last().and_then(|token|Some(token.token_type == TokenType::Int || token.token_type == TokenType::Float)).unwrap_or(false))
                    .unwrap_or(scanner.gen_token(TokenType::EOF)),
            );
        }
        let mut i  = 0;
        while i<tokens.len(){
            if tokens[i].token_type == TokenType::Comment{
                tokens.remove(i);
                break;
            }
            i+=1;
        }
        tokens
    }
    fn scan_token(&mut self, is_last_numeric:bool) -> Option<Token> {
        loop {
            self.start = self.current;
            let p_whitespace = self.peek(0)?;
            if !p_whitespace.is_whitespace() {
                break;
            }
            let _ = self.advance()?;
        }
        let token_type = match self.advance()? {
            ';' => TokenType::SemiColon,
            '(' => TokenType::LParen,
            ')' => TokenType::RParen,
            '[' => TokenType::LBrack,
            ']' => TokenType::RBrack,
            '{' => TokenType::LBrace,
            '}' => TokenType::RBrace,
            '@' => TokenType::At,
            '%' => TokenType::Percent,
            '.' => TokenType::Dot,
            '*' => TokenType::Star,
            ',' => TokenType::Comma,
            '|' => TokenType::Pipe,
            '&' => TokenType::Ampersand,
            '>' => TokenType::RArrow,
            ':' => either!(self.check(':') => TokenType::DoubleColon; TokenType::Colon),
            '+' => either!(self.check('=') => TokenType::PlusEqual;   TokenType::Plus),
            '=' => either!(self.check('=') => TokenType::EqualEqual;  TokenType::Equal),
            '<' => either!(self.check('=') => TokenType::LessOrEqual; either!(self.check('<') =>TokenType::SHL; TokenType::LArrow)),
            '!' => either!(self.check('=') => TokenType::BangEqual;   TokenType::Bang),
            '-' => {
                if !is_last_numeric && self.peek(0).unwrap_or(' ').is_numeric(){
                    self.extract_numeric()
                } else {
                    either!(self.check('=') => TokenType::MinusEqual;  either!(self.check('>') => TokenType::SmallArrow; TokenType::Minus))
                }
            }
            'b' => self.check_keyword("reak", TokenType::Break)?,
            'c' => self.check_keyword("ontinue", TokenType::Continue)?,
            'l' => self.check_keyword("et", TokenType::Let)?,
            'r' => self.check_keyword("eturn", TokenType::Return)?,
            'w' => self.check_keyword("hile", TokenType::While)?,
            'i' => match self.advance()? {
                'f' => self.check_keyword("", TokenType::If)?,
                'm' => self.check_keyword("pl", TokenType::Impl)?,
                _ => self.extract_ident(),
            },
            'e' => match self.advance()? {
                'l' => self.check_keyword("se", TokenType::Else)?,
                'n' => self.check_keyword("um", TokenType::Enum)?,
                _ => self.extract_ident(),
            },
            'a' => self.check_keyword("nd", TokenType::And)?,
            'o' => self.check_keyword("r", TokenType::Or)?,
            'f' => match self.advance()? {
                'a' => self.check_keyword("lse", TokenType::False)?,
                'n' => self.check_keyword("", TokenType::Fn)?,
                'o' => self.check_keyword("r", TokenType::For)?,
                _ => self.extract_ident(),
            },
            'S' => self.check_keyword("elf", TokenType::Self_)?,
            's' => self.check_keyword("truct", TokenType::Struct)?,
            'T' => self.check_keyword("rait", TokenType::Trait)?,
            't' => self
                .advance()
                .and_then(|char| {
                    if char == 'r' {
                        Some(match self.advance()? {
                            'u' => self.check_keyword("e", TokenType::True)?,
                            'a' => self.check_keyword("it", TokenType::Trait)?,
                            _ => {None?;unreachable!()},
                        })
                    } else {
                        None
                    }
                })
                .unwrap_or(self.extract_ident()),
            'm' => self.check_keyword("ut", TokenType::Mut)?,
            'p' => self.check_keyword("anic", TokenType::Panic)?,
            a if a.is_numeric() => self.extract_numeric(),
            a if a.is_new_alpha() => self.extract_ident(),
            '"' => loop {
                if let Some(char) = self.advance() {
                    if char == '\\' {
                        continue;
                    }
                    if char != '"' {
                        continue;
                    } else {
                        break TokenType::String;
                    }
                }
                break TokenType::Error;
            },
            '/' => {
                either!(
                    self.check('/')=> {
                        let start_line = self.line.clone();
                        while start_line == self.line{self.advance();}
                        TokenType::Comment
                    };
                    TokenType::Slash
                )
            }
            _ => TokenType::Error,
        };
        Some(self.gen_token(token_type))
    }
    fn extract_ident(&mut self) -> TokenType {
        while self
            .peek(0)
            .and_then(|char| {
                if char.is_new_alphanumeric() {
                    Some(())
                } else {
                    None
                }
            })
            .is_some()
        {
            self.advance();
        }
        TokenType::Ident
    }
    fn check_keyword(&mut self, to_match: &str, token_type: TokenType) -> Option<TokenType> {
        let mut chars = to_match.chars();
        loop {
            let char = chars.next();
            match char {
                Some(char) => {
                    let actual_char = self.peek(0)?;
                    if actual_char != char {
                        break;
                    }
                    self.advance();
                }
                None => {
                    return either!(!self.peek(0).unwrap_or(' ').is_new_alphanumeric() => Some(token_type); Some(self.extract_ident()))
                }
            }
        }
        Some(self.extract_ident())
    }
    fn extract_numeric(&mut self) -> TokenType {
        loop {
            let p_char = self.peek(0);
            match p_char {
                Some(char) => {
                    if char.is_numeric() {
                        self.advance();
                        continue;
                    }
                    if char == '.' {
                        if self.peek(1).unwrap_or(' ').is_numeric(){
                            self.advance();
                            break;
                        }else{
                            return TokenType::Int;
                        }
                    }
                    return TokenType::Int;
                }
                None => {
                    return TokenType::Int;
                }
            }
        }
        while self
            .peek(0)
            .and_then(|char| if char.is_numeric() { Some(()) } else { None })
            .is_some()
        {
            self.advance();
        }
        TokenType::Float
    }
    fn check(&mut self, char: char) -> bool {
        match self.peek(0) {
            Some(actual_char) => {
                let res = actual_char == char;
                if res {
                    let _ = self.advance();
                }
                res
            }
            None => false,
        }
    }
    fn peek(&self, i: usize) -> Option<char> {
        self.source_code.chars().nth(self.current + i)
    }
    fn advance(&mut self) -> Option<char> {
        let res = self.peek(0)?;
        self.current += 1;
        if res == '\n' {
            self.line += 1;
        }
        Some(res)
    }
    fn gen_token(&mut self, token_type: TokenType) -> Token {
        Token {
            token_type,
            line: self.line,
            line_offset:self.start,
            lexeme: self.source_code[self.start..self.current].to_string(),
        }
    }
}
