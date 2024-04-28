macro_rules! either {
    ($test:expr => $true_expr:expr; $false_expr:expr) => {
        if $test {
            $true_expr
        }
        else {
            $false_expr
        }
    }
}

#[derive(Debug,Clone, Copy,PartialEq)]
pub enum TokenType {
    //single char
    Plus,
    Minus,
    Slash,
    Comma,
    SemiColon,
    Colon,
    Star,
    Dot,
    Bang,
    At,
    Ampersand,
    Equal,
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
    //idents
    Ident,
    String,
    Num,
    //Keywords
    Fn,
    Struct,
    Enum,
    While,
    Let,
    Return,
    SelfCal,
    If,
    Impl,
    Else,
    True,
    False,
    Trait,
    And,
    Or,
    Gen,
    //other stuff
    Error,
    EOF,
    Comment,
}

#[derive(Debug,Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub lexeme: String,
}

pub struct Scanner {
    source_code: String,
    line: usize,
    start: usize,
    current: usize
}

trait NewAlphanumric{
    fn is_new_alphanumeric(&self)->bool;
    fn is_new_alpha(&self)->bool;
}

impl NewAlphanumric for char{
    fn is_new_alphanumeric(&self)->bool {
        self.is_alphanumeric() || *self == '_'
    }
    
    fn is_new_alpha(&self)->bool {
        self.is_alphabetic() || *self == '_'
    }
}

impl Scanner {
    pub fn scan(source:String)->Vec<Token>{
        let mut tokens = vec![];
        let mut scanner: Scanner = Self{
            start: 0,
            current: 0,
            source_code: source,
            line: 0,
        };
        while TokenType::EOF != tokens.last().and_then(|token: &Token|Some(token.token_type)).unwrap_or(TokenType::SemiColon){
            tokens.push(scanner.scan_token().unwrap_or(scanner.gen_token(TokenType::EOF)));
        }
        tokens
    }
    fn scan_token(&mut self)->Option<Token>{
        loop{   
            self.start = self.current;
            let p_whitespace = self.peek(0)?;
            if !p_whitespace.is_whitespace(){
                break;
            }
            let _ = self.advance()?;
        }
        let token_type = match self.advance()? {
            ';'=>TokenType::SemiColon,
            '('=>TokenType::LParen,
            ')'=>TokenType::RParen,
            '['=>TokenType::LBrack,
            ']'=>TokenType::RBrack,
            '{'=>TokenType::LBrace,
            '}'=>TokenType::RBrace,
            '*'=>TokenType::Star,
            '&'=>TokenType::Ampersand,
            '@'=>TokenType::At,
            '.'=>TokenType::Dot,
            ':'=>TokenType::Colon,
            ','=>TokenType::Comma,
            '+'=>either!(self.check('=') => TokenType::PlusEqual;   TokenType::Plus),
            '='=>either!(self.check('=') => TokenType::EqualEqual;  TokenType::Equal),
            '<'=>either!(self.check('=') => TokenType::LessOrEqual; TokenType::LArrow),
            '>'=>either!(self.check('=') => TokenType::MoreOrEqual; TokenType::RArrow),
            '!'=>either!(self.check('=') => TokenType::BangEqual;   TokenType::Bang),
            '-'=>{
                if self.peek(0).unwrap_or(' ').is_numeric(){
                    self.extract_numeric()
                }else{
                    either!(self.check('=') => TokenType::MinusEqual;  either!(self.check('>') => TokenType::SmallArrow; TokenType::Minus))
                }
            },
            'l'=>self.check_keyword("et", TokenType::Let)?,
            'r'=>self.check_keyword("eturn", TokenType::Return)?,
            'w'=>self.check_keyword("hile", TokenType::While)?,
            'i'=>self.check_keyword("f", TokenType::If)?,
            'I'=>self.check_keyword("dent", TokenType::Impl)?,
            'e'=>self.check_keyword("lse", TokenType::Else)?,
            'E'=>self.check_keyword("num", TokenType::Enum)?,
            'a'=>self.check_keyword("nd", TokenType::And)?,
            'o'=>self.check_keyword("r", TokenType::Or)?,
            'f'=>{
                match self.advance()?{
                    'a'=>self.check_keyword("lse", TokenType::False)?,
                    'n'=>self.check_keyword("", TokenType::Fn)?,
                    _=>self.extract_ident()
                }
            }
            'S'=>self.check_keyword("truct", TokenType::Struct)?,
            's'=>self.check_keyword("elf", TokenType::SelfCal)?,
            'T'=>self.check_keyword("rait", TokenType::Trait)?,
            't'=>self.check_keyword("rue", TokenType::True)?,
            'g'=>self.check_keyword("en", TokenType::Gen)?,
            a if a.is_numeric() => {
                self.extract_numeric()
            },
            a if a.is_new_alpha() =>{
                self.extract_ident()
            }
            '"'=>{
                loop{
                    if let Some(char) = self.advance(){
                        if char!='"'{
                            continue;
                        }else{
                            break TokenType::String;
                        }
                    }
                    break TokenType::Error;
                }
            }
            '/'=>{
                either!(
                    self.check('/')=> {
                        let start_line = self.line.clone();
                        while start_line == self.line{self.advance();}
                        TokenType::Comment
                    };
                    TokenType::Slash
                )
            }
            _=>TokenType::Error,
        };
        Some(self.gen_token(token_type))
    }
    fn extract_ident(&mut self)->TokenType{
        while self.peek(0).and_then(|char|if char.is_new_alphanumeric(){Some(())}else{None}).is_some(){self.advance();};
        TokenType::Ident
    }
    fn check_keyword(&mut self,to_match:&str,token_type:TokenType)->Option<TokenType>{
        let mut chars = to_match.chars();
        loop {
            let char = chars.next();
            match char {
                Some(char) => {
                    let actual_char = self.peek(0)?;
                    if actual_char != char{
                        break;
                    }
                    self.advance();
                },
                None => {
                    return either!(!self.peek(0).unwrap_or(' ').is_new_alphanumeric() => Some(token_type); Some(self.extract_ident()))
                },
            }
        }
        Some(self.extract_ident())
    }
    fn extract_numeric(&mut self)->TokenType{
        loop{
            let p_char = self.peek(0);
            match p_char{
                Some(char) =>{
                    if char.is_numeric(){self.advance();continue;}
                    if char == '.'{self.advance();break;}
                    return TokenType::Num;
                },
                None=>{return TokenType::Num;},
            } 
        }
        while self.peek(0).and_then(|char|if char.is_numeric(){Some(())}else{None}).is_some(){self.advance();};
        TokenType::Num
    }
    fn check(&mut self, char:char)->bool{
        match self.peek(0) {
            Some(actual_char) => {
                let res = actual_char == char;
                if res{
                    let _ = self.advance();
                }
                res
            },
            None => false,
        }
    }
    fn peek(&self,i:usize)->Option<char>{
        self.source_code.chars().nth(self.current+i)
    }
    fn advance(&mut self)->Option<char>{
        let res = self.peek(0)?;
        self.current+=1;
        if res == '\n'{
            self.line+=1;
        }
        Some(res)
    }
    fn gen_token(&mut self,token_type:TokenType)->Token{
        Token { token_type, line:self.line, lexeme: self.source_code[self.start..self.current].to_string() }
    }
}
