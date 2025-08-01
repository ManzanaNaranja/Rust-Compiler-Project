use std::error::Error;
use std::vec;
use std::iter::Peekable;
use slice_deque::SliceDeque;
use std::collections::HashMap;

#[derive(PartialEq)]
enum Type {
    Var, 
    Arr,
    Fn,
}

struct Lex {
    it: Peekable<vec::IntoIter<u8>>,
    line: usize,
    problem: Option<Box<dyn Error>>,
}

#[derive(Debug)]
enum Tok {
    Func,
    Return,
    Int,
    Print,
    Read,
    While,
    If,
    Else,
    Break,
    Continue,
    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    Plus,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Assign,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equality,
    NotEqual,
    Identifier(Vec<u8>),
    Number(Vec<u8>),
    Empty,
}

impl Lex {
    fn make(file_path: &str) -> Result<Lex,Box<dyn Error>> {
        Ok(Lex{
            it:std::fs::read_to_string(file_path)?.into_bytes().into_iter().peekable(),
            line: 1,
            problem:None,
        })
    }
    
    fn next(&mut self) -> Tok {
        if let Some(tok) = self.lex() { tok }
        else { Tok::Empty }
    }

    fn token(&mut self, token: Tok) -> Option<Tok> {
        self.it.next();
        Some(token)
    }

    fn token2(&mut self, token: Tok, peek_check: u8, token2: Tok) -> Option<Tok> {
        self.it.next();
        if self.it.peek().is_none() { // check if at the end of input so don't throw error without consuming last char
            return Some(token)
        }
        let byte = self.it.peek()?;
        if *byte == peek_check {
            self.it.next();
            Some(token2)
        }

        else {
            Some(token)
        }
            
    }

    fn lex_number(&mut self) -> Option<Tok> {
        let mut num: Vec<u8> = vec![];
        while let Some(byte) = self.it.peek() {
            match byte {
                b'0'..=b'9' => {
                    num.push(*byte);
                    self.it.next();
                },
                _ => { break },
            }
        }
        return Some(Tok::Number(num))
    }

    fn ignore_line(&mut self) {
        while let Some(byte) = self.it.peek() {
            match byte {
                b'\n' | b'\r' => { 
                    break
                },
                _ => {
                    self.it.next();
                }
            }
        }
    }

    fn lex_id(&mut self) -> Option<Tok> {
        let mut id: Vec<u8> = vec![];
        while let Some(byte) = self.it.peek() {
            match byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'_' | b'0'..=b'9' => {
                    id.push(*byte);
                    self.it.next();
                },
                _ => { break },
            }
        }

        Some(match &id[..] {
            b"func" => Tok::Func,
            b"return" => Tok::Return,
            b"int" => Tok::Int,
            b"print" => Tok::Print,
            b"read" => Tok::Read,
            b"while" => Tok::While,
            b"if" => Tok::If,
            b"else" => Tok::Else,
            b"break" => Tok::Break,
            b"continue" => Tok::Continue,
            _ => Tok::Identifier(id)
        })
    }

    fn lex(&mut self) -> Option<Tok> {
        match self.it.peek()? {
            b'[' => { self.token(Tok::LeftBracket)},
            b']' => self.token(Tok::RightBracket),
            b'(' => self.token(Tok::LeftParen),
            b')' => self.token(Tok::RightParen),
            b'{' => self.token(Tok::LeftCurly),
            b'}' => self.token(Tok::RightCurly),
            b',' => self.token(Tok::Comma),
            b';' => self.token(Tok::Semicolon),
            b'+' => self.token(Tok::Plus),
            b'-' => self.token(Tok::Subtract),
            b'*' => self.token(Tok::Multiply),
            b'/' => self.token(Tok::Divide),
            b'%' => self.token(Tok::Modulus),
            b'=' => self.token2(Tok::Assign,b'=',Tok::Equality),
            b'<' => self.token2(Tok::Less, b'=',Tok::LessEqual),
            b'>' => self.token2(Tok::Greater, b'=',Tok::GreaterEqual),
            b' ' | b'\t' => {
                self.it.next();
                self.lex()
            },
            b'\r' => {
                self.it.next();
                let byte = self.it.peek()?;
                if *byte != b'\n' {
                    self.line+=1;
                } 
                self.lex()
            }
            b'\n' => {
                self.line+=1;
                self.it.next();
                self.lex()
            }
            b'!' => {
                self.it.next();
                if self.it.peek().is_none() { // no more chars left
                    self.problem = Some(format!("Lexer: found invalid char {}", "!").into()); 
                    return None;
                }
                let byte = self.it.peek()?;
                if *byte == b'=' {
                    self.it.next();
                    Some(Tok::NotEqual)
                } else {
                    self.problem = Some(format!("Lexer: found invalid char {}", "!").into()); 
                    return None;
                }
            },
            b'A'..=b'Z' | b'a'..=b'z' | b'_' => { self.lex_id() }
            b'0'..=b'9' => { self.lex_number() }
            b'#' => { self.it.next(); self.ignore_line(); self.lex() }
            ch => { self.problem = Some(format!("Lexer: found invalid char {}", *ch as char).into()); None }
        }
    }
}





// fn main() -> Result<(), Box<dyn Error>> {
//     let args : Vec<String> = std::env::args().collect();
//     let mut lex = Lex::make(&args[1])?;

//     while let Some(ref tok) = lex.lex() {
//         match tok {
//             Tok::Identifier(vec) => { println!("Identifier(\"{}\")", String::from_utf8_lossy(&vec[..])); },
//             Tok::Number(vec) => {println!("Number(\"{}\")", String::from_utf8_lossy(&vec[..])); },
//             _ => { println!("{:?}", tok)}
//         }
//         // println!("{:?}", tok);
//     }
//     if let Some(err) = lex.problem {
//         println!("Problem, line {}: {}", lex.line,err);
//         return Err(err);
//     }
//     Ok(())
// }

fn main() -> Result<(), Box<dyn Error>> {
    let args : Vec<String> = std::env::args().collect(); 
    let mut par = Par::make(&args[1])?;

    while let Some(()) = par.parse() {  /* nop  */ }

    if let Some(err) = par.lex.problem {
        println!("Problem, (lexer line {}): {}", par.lex.line, err);
    }
    if let Some(err) = par.problem {
        println!("Problem, (lexer line {}): {}", par.lex.line, err);
        return Err(err);
    }

    Ok(())
}

struct Par {
    lex: Lex,
    toks: SliceDeque<Tok>,
    problem: Option<Box<dyn Error>>,
    t_count: usize,
    l_count: usize,
    
    types: Vec<HashMap<String, Type>>,
}

impl Par {
    fn make(file_path: &str) -> Result<Par, Box<dyn Error>> {
        let mut types = Vec::new();
        types.push(HashMap::new());
        
        Ok(Par{
            lex: Lex::make(file_path)?, toks: SliceDeque::new(), problem:None,
            t_count: 0, l_count: 0,
            types,
        })
    }

    fn tokens(&mut self, amt: usize) -> &mut [Tok] {
        while self.toks.len() < amt { self.toks.push_back(self.lex.next()); }
        &mut self.toks[0..amt]
    }

    fn consume(&mut self, amt: usize) { for _ in 0..amt { self.toks.pop_front(); } }

    fn temp_name(&mut self) -> Vec<u8> {
        let mut res = Vec::from(b"temp");
        res.extend_from_slice(&self.t_count.to_string().into_bytes());
        println!("%int {}", String::from_utf8_lossy(&res));
        self.t_count += 1;
        res
    }

    fn parse(&mut self) -> Option<()> {
        match self.tokens(1) {
            &mut [Tok::Func] => { self.function() },
            &mut [Tok::Empty] => { None },
            _ => { self.problem = Some(format!("invalid token").into()); None },
        }
    }


    fn function(&mut self) -> Option<()> {
        let name = match self.tokens(3) {   
            &mut [Tok::Func, Tok::Identifier(ref mut id), Tok::LeftParen] => {
                let name = std::mem::take(id);
                self.consume(3);
                name
            },
            _ => {
                self.problem = Some(format!("Not Funct").into());
                return None;
            }
        };
        // print!("function header: {}", String::from_utf8_lossy(&name));
        print!("%func {}(", String::from_utf8_lossy(&name));
        let mut params: Vec<String> = Vec::new();

        loop  {
            match self.tokens(3) {
                &mut [Tok::Int, Tok::Identifier(ref mut id), Tok::Comma] => {
                    let arg = std::mem::take(id);
                    self.consume(3);
                    print!("%int {}, ", String::from_utf8_lossy(&arg));
                    if let Ok(string) = String::from_utf8(arg) {
                        params.push(string);
                    }
                },
                &mut [Tok::Int, Tok::Identifier(ref mut id),Tok::RightParen] => {
                    let arg = std::mem::take(id);
                    self.consume(3);
                    print!("%int {})\n", String::from_utf8_lossy(&arg));
                    if let Ok(string) = String::from_utf8(arg) {
                        params.push(string);
                    }
                    break
                }
                &mut [Tok::RightParen, _,_] => {
                    self.consume(1);
                    print!(")\n");
                    break;
                }
                _=> {
                    self.problem = Some(format!("problem").into());
                    return None
                }
            }
        }
        let opt = self.statements(params);
        println!("%endfunc");
        return opt;


        
    }

    fn statements(&mut self, params: Vec<String>) -> Option<()> {
        match self.tokens(1) {
            &mut [Tok::LeftCurly] => {
                self.types.push(HashMap::new());
                self.consume(1);

                for param in params {
                    if let Some(_already_present) = self.types.last_mut().unwrap().insert((*param).to_string(), Type::Var) {
                        self.problem = Some(format!("duplicate parameter name").into());

                        return None;
                    }
                }
                // println!("{{");
            },
            _ => {self.problem= Some(format!("missing {{").into()); return None; }
        }
        loop {
            if let Tok::RightCurly = self.tokens(1)[0] {
                self.consume(1);
                // println!("}}\n");
                self.types.pop();
                break Some(());
            } 
            
            self.statement()?;

            
        }

    }

    fn statement(&mut self) -> Option<()> {
        match self.tokens(8) {
            &mut[Tok::Int, Tok::LeftBracket,Tok::Number(ref mut num),Tok::RightBracket,Tok::Identifier(ref mut id), Tok::Semicolon,_,_] => {
                let num = std::mem::take(num);
                let id = std::mem::take(id);
                self.consume(6);

                // println!("declare array: {}, {}", String::from_utf8_lossy(&id), String::from_utf8_lossy(&num));
                println!("%int[] {}, {}", String::from_utf8_lossy(&id), String::from_utf8_lossy(&num));
                Some(())
            },
            &mut[Tok::Int, Tok::Identifier(ref mut id),Tok::Semicolon,_,_,_,_,_] => {
                let id = std::mem::take(id);
                self.consume(3);
                // println!("declare var:  {}", String::from_utf8_lossy(&id));
                println!("%int {}", String::from_utf8_lossy(&id));
                Some(())
            },
            &mut[Tok::Int, Tok::Identifier(ref mut id), Tok::Assign,_,_,_,_,_] => {
                let id = std::mem::take(id);
                self.consume(3);
                if let Some(rhs) = self.expr() {
                    // println!("assign var: {} = {}", String::from_utf8_lossy(&id), String::from_utf8_lossy(&rhs));
                    println!("%int {}", String::from_utf8_lossy(&id));
                    println!("%mov {}, {}", String::from_utf8_lossy(&id), String::from_utf8_lossy(&rhs));
                    if let Tok::Semicolon = self.tokens(1)[0] {
                        self.consume(1);
                        Some(())
                    } else {
                        self.problem = Some(format!("missing ;").into()); return None; 
                    }
                } else {
                    None
                }
            },

            &mut[Tok::Identifier(ref mut id), Tok::Assign,_,_,_,_,_,_] => {
                let id = std::mem::take(id);
                // print!("{}",String::from_utf8_lossy(&id));
                self.consume(2);
                if let Some(rhs) = self.expr() {
                    // println!("assign var: {} = {}", String::from_utf8_lossy(&id), String::from_utf8_lossy(&rhs));
                    println!("%mov {}, {}", String::from_utf8_lossy(&id), String::from_utf8_lossy(&rhs));
                    if let Tok::Semicolon = self.tokens(1)[0] {
                        self.consume(1);
                        Some(())
                    } else {
                        self.problem = Some(format!("missing ;").into()); return None; 
                    }

                } else {
                    None
                }
            },

            &mut[Tok::Identifier(ref mut id), Tok::LeftBracket, _,_,_,_,_,_] => {
                let id = std::mem::take(id); // assign arr: array[0] = 2
                self.consume(2);
                if let Some(index) = self.expr() {
                    match self.tokens(2) {
                        &mut[Tok::RightBracket, Tok::Assign] => {
                            self.consume(2);
                            if let Some(rhs) = self.expr() {
                                if let Tok::Semicolon = self.tokens(1)[0] {
                                    self.consume(1);
                                    // println!("assign arr:  {}[{}] = {}", String::from_utf8_lossy(&id),String::from_utf8_lossy(&index),String::from_utf8_lossy(&rhs));
                                    println!("%mov [{}+{}], {}", String::from_utf8_lossy(&id),String::from_utf8_lossy(&index),String::from_utf8_lossy(&rhs));
                                    Some(())
                                } else {
                                    self.problem = Some(format!("missing ;").into()); return None; 
                                }
                            } else {
                                None
                            }
                        },
                        _ => {self.problem = Some(format!("invalid syntax").into()); return None; }
                    }
                } else {
                    None
                }
            },
            &mut[Tok::While, _,_,_,_,_,_,_] => {
                self.consume(1);
                print!("while(");
                if let Some(cond) = self.expr() {
                    println!("cond({}))", String::from_utf8_lossy(&cond));
                    self.statements(Vec::new())
                } else {
                    None
                }
            }, 
            &mut[Tok::If, _,_,_,_,_,_,_] => {
                self.consume(1);
                print!("if(");
                if let Some(cond) = self.expr() {
                    println!("cond({}))", String::from_utf8_lossy(&cond));
                    self.statements(Vec::new())?;
                    match self.tokens(1) {
                        &mut [Tok::Else] => {
                            self.consume(1);
                            print!("else ");
                            self.statements(Vec::new())?
                        },
                        _ => { /* nop */},
                    }
                    Some(())
                } else {
                    None
                }
            }, 
            &mut[Tok::Print, Tok::LeftParen,_,_,_,_,_,_] => {
                self.consume(2);
                if let Some(cond) = self.expr() {
                    // println!("print: {}", String::from_utf8_lossy(&cond));
                    println!("%out {}", String::from_utf8_lossy(&cond));
                    match self.tokens(2) {
                        &mut[Tok::RightParen, Tok::Semicolon] => {
                            self.consume(2);
                            Some(())
                        },
                        _ => {self.problem = Some(format!("invalid syntax").into()); return None; }
                    }
                } else {
                    None
                }
            },
            &mut[Tok::Read, Tok::LeftParen, Tok::Identifier(ref mut id), Tok::RightParen, Tok::Semicolon, _,_,_] => {
                let id = std::mem::take(id);
                self.consume(5);
                // println!("read: {}", String::from_utf8_lossy(&id));
                println!("%input {}", String::from_utf8_lossy(&id));
                Some(())
            },
            &mut[Tok::Read, Tok::LeftParen, Tok::Identifier(ref mut id), Tok::LeftBracket,_,_,_,_] => {
                let id = std::mem::take(id);
                self.consume(4);
                if let Some(cond) = self.expr() {
                    // println!("read: {}[{}]", String::from_utf8_lossy(&id),String::from_utf8_lossy(&cond));
                    let temp = self.temp_name();
                    println!("%input {}", String::from_utf8_lossy(&temp));
                    println!("%mov [{}+{}], {}", String::from_utf8_lossy(&id),String::from_utf8_lossy(&cond), String::from_utf8_lossy(&temp));
                    // %int temp28
                    // %input temp28
                    // %mov [arr + 1], temp28
                    match self.tokens(3) {
                        &mut[Tok::RightBracket, Tok::RightParen, Tok::Semicolon] => {
                            self.consume(3);
                            Some(())
                        },
                        _ => {self.problem = Some(format!("invalid syntax").into()); return None; }
                    }
                } else {
                    None
                }
            },
            &mut[Tok::Return,_,_,_,_,_,_,_] => {
                self.consume(1);
                if let Some(cond) = self.expr() {
                    // println!("return: {}", String::from_utf8_lossy(&cond));
                    println!("%ret {}", String::from_utf8_lossy(&cond));
                    if let Tok::Semicolon = self.tokens(1)[0] {
                        self.consume(1);
                        Some(())
                    } else {
                        self.problem = Some(format!("missing ;").into()); return None; 
                    }
                } else {
                    None
                }
            },
            &mut[Tok::Break,Tok::Semicolon,_,_,_,_,_,_] => {
                self.consume(2);
                println!("break");
                Some(())
            },
            &mut[Tok::Continue,Tok::Semicolon,_,_,_,_,_,_] => {
                self.consume(2);
                println!("continue");
                Some(())
            }
            // &mut[Tok::Identifier(ref mut id), Tok::Assign]
            _ => {self.problem= Some(format!("invalid syntax").into()); return None; }
        }
    }

    fn mul_expr(&mut self) -> Option<Vec<u8>> {
        let mut lhs = self.base_expr()?;
        loop {
            match self.tokens(1) {
                &mut [Tok::Multiply] | &mut [Tok::Divide] | &mut [Tok::Modulus] => {
                    let op = match self.tokens(1) {
                        &mut [Tok::Multiply] => "mult",
                        &mut [Tok::Divide] => "div",
                        &mut [Tok::Modulus] => "mod",
                        _ => unreachable!(),
                    };
                    self.consume(1);
                    let rhs = self.base_expr()?;
                    let temp = self.temp_name();
                    println!("%{} {}, {}, {}", op, String::from_utf8_lossy(&temp), String::from_utf8_lossy(&lhs), String::from_utf8_lossy(&rhs));
                    lhs = temp;
                }
                _ => break,
            }
        }
        Some(lhs)
    }

    fn add_expr(&mut self) -> Option<Vec<u8>> {
        let mut lhs = self.mul_expr()?;
        loop {
            match self.tokens(1) {
                &mut [Tok::Plus] | &mut [Tok::Subtract] => {
                    let op = match self.tokens(1) {
                        &mut [Tok::Plus] => "add",
                        &mut [Tok::Subtract] => "sub",
                        _ => unreachable!(),
                    };
                    self.consume(1);
                    let rhs = self.mul_expr()?;
                    let temp = self.temp_name();
                    println!("%{} {}, {}, {}", op, String::from_utf8_lossy(&temp), String::from_utf8_lossy(&lhs), String::from_utf8_lossy(&rhs));
                    lhs = temp;
                }
                _ => break,
            }
        }
        Some(lhs)
    }

    fn bool_expr(&mut self) -> Option<Vec<u8>> {
        let mut lhs = self.add_expr()?;
        loop {
            match self.tokens(1) {
                &mut [Tok::Less] => {
                    self.consume(1);
                    let rhs = self.add_expr()?;
                    let temp = self.temp_name();
                    println!("%lt {}, {}, {}", String::from_utf8_lossy(&temp), String::from_utf8_lossy(&lhs), String::from_utf8_lossy(&rhs));
                    lhs = temp;
                }
                &mut [Tok::Greater] => {
                    self.consume(1);
                    let rhs = self.add_expr()?;
                    let temp = self.temp_name();
                    println!("%gt {}, {}, {}", String::from_utf8_lossy(&temp), String::from_utf8_lossy(&lhs), String::from_utf8_lossy(&rhs));
                    lhs = temp;
                }
                &mut [Tok::GreaterEqual] => {
                    self.consume(1);
                    let rhs = self.add_expr()?;
                    let temp = self.temp_name();
                    println!("%ge {}, {}, {}", String::from_utf8_lossy(&temp), String::from_utf8_lossy(&lhs), String::from_utf8_lossy(&rhs));
                    lhs = temp;
                }
                &mut [Tok::LessEqual] => {
                    self.consume(1);
                    let rhs = self.add_expr()?;
                    let temp = self.temp_name();
                    println!("%le {}, {}, {}", String::from_utf8_lossy(&temp), String::from_utf8_lossy(&lhs), String::from_utf8_lossy(&rhs));
                    lhs = temp;
                }
                _ => break,
            }
        }
        Some(lhs)
    }

    fn expr(&mut self) -> Option<Vec<u8>> {
        let mut lhs = self.bool_expr()?;
        loop {
            match self.tokens(1) {
                &mut [Tok::Equality] | &mut [Tok::NotEqual] => {
                    let op = match self.tokens(1) {
                        &mut [Tok::Equality] => "eq",
                        &mut [Tok::NotEqual] => "neq",
                        _ => unreachable!(),
                    };
                    self.consume(1);
                    let rhs = self.bool_expr()?;
                    let temp = self.temp_name();
                    println!("%{} {}, {}, {}", op, String::from_utf8_lossy(&temp), String::from_utf8_lossy(&lhs), String::from_utf8_lossy(&rhs));
                    lhs = temp;
                }
                _ => break,
            }
        }
        Some(lhs)
    }

    fn base_expr(&mut self) -> Option<Vec<u8>> {
        match self.tokens(1) {
            &mut [Tok::Identifier(ref id)] => {
                let id = id.clone();
                self.consume(1);

                // Check for undeclared variable
                let id_str = String::from_utf8_lossy(&id).to_string();
                let mut declared = false;

                // Iterate through all scopes (starting from the innermost) to check if the variable is declared
                // for scope in self.types.iter().rev() {
                //     if scope.contains_key(&id_str) {
                //         declared = true;
                //         break;
                //     }
                // }

                // if !declared {
                //     self.problem = Some(format!("undeclared variable: {}", id_str).into());
                //     return None;
                // }
                // Handle array indexing: ID [ expr ]
                if let &mut [Tok::LeftBracket] = self.tokens(1) {
                    self.consume(1);
                    if let Some(index) = self.expr() {
                        if let &mut [Tok::RightBracket] = self.tokens(1) {
                            self.consume(1);
                            let temp = self.temp_name();
                            // println!("assign: {} = {}[{}]", String::from_utf8_lossy(&temp), String::from_utf8_lossy(&id), String::from_utf8_lossy(&index));
                            println!("%mov {}, [{}+{}]", String::from_utf8_lossy(&temp), String::from_utf8_lossy(&id), String::from_utf8_lossy(&index));
                            Some(temp)
                        } else {
                            self.problem = Some("Expected ']' after array index.".into());
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
                // Handle function calls: ID ( args )
                else if let &mut [Tok::LeftParen] = self.tokens(1) {
                    self.consume(1);
                    let stuff: Option<Vec<Vec<u8>>> = self.args();
                    let temp = self.temp_name();
                    if let Some(arguments) = stuff {
                        // print!("call: {} = {}(",String::from_utf8_lossy(&temp), String::from_utf8_lossy(&id));
                        print!("%call {}, {}(",String::from_utf8_lossy(&temp), String::from_utf8_lossy(&id));
                        // println!("Calling function {} with arguments:", String::from_utf8_lossy(&id));
                        for (i, arg) in arguments.iter().enumerate() {
                            print!("{}, ", String::from_utf8_lossy(arg));
                        }
                    }
                    if let &mut [Tok::RightParen] = self.tokens(1) {
                        println!(")");
                        self.consume(1);
                    } else {
                        self.problem = Some("Expected ')' after function arguments.".into());
                        return None;
                    }
                    Some(temp)
                } else {
                    // Standalone identifier
                    Some(id)
                }
            }
            &mut [Tok::Number(ref num)] => {
                let num = num.clone();
                self.consume(1);
                Some(num)
            }
            &mut [Tok::LeftParen] => {
                self.consume(1);
                let expr = self.expr();
                if let &mut [Tok::RightParen] = self.tokens(1) {
                    self.consume(1);
                } else {
                    self.problem = Some("Expected ')'".into());
                    return None;
                }
                expr
            }
            &mut [Tok::Empty] => {
                None
            }
            _ => {
                self.problem = Some("Unexpected token in base expression.".into());
                None
            }
        }
    }

    fn args(&mut self) -> Option<Vec<Vec<u8>>> {
        let mut arguments = Vec::new();
        if let Some(arg) = self.expr() {
            arguments.push(arg);
            while let &mut [Tok::Comma] = self.tokens(1) {
                self.consume(1);
                if let Some(arg) = self.expr() {
                    arguments.push(arg);
                } else {
                    self.problem = Some("Expected expression after ',' in arguments".into());
                    return None;
                }
            }
        }
        Some(arguments)
    }

    fn type_check(&mut self, i:usize, name:&String, check_type: Type) -> Option<()> {
        if 0 == i {
            return None;
        }
        let i = i-1;
        if let Some(symbol) = self.types[i].get(name) {
            if *symbol == check_type { Some(()) }
            else { None }
        }
        else { self.type_check(i,name,check_type) }
    }
}

