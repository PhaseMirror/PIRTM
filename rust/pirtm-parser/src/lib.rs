pub mod ast;
pub use ast::{LogicalOp, BinOp, EnsembleDecl, Expr, ImportStmt, Program, Stmt, Type};

use pirtm_lexer::{tokenize, Token};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let tokens = tokenize(input);
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.pos).cloned()
    }

    fn next(&mut self) -> Option<Token> {
        let token = self.peek();
        self.pos += 1;
        token
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if let Some(tok) = self.next() {
            if tok == expected {
                Ok(())
            } else {
                Err(format!("Expected {:?}, got {:?}", expected, tok))
            }
        } else {
            Err(format!("Expected {:?}, got EOF", expected))
        }
    }

    pub fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and()?;
        while self.peek() == Some(Token::OrOr) {
            self.next();
            let right = self.parse_and()?;
            left = Expr::LogicalOp { op: LogicalOp::Or, left: Box::new(left), right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_equality()?;
        while self.peek() == Some(Token::AndAnd) {
            self.next();
            let right = self.parse_equality()?;
            left = Expr::LogicalOp { op: LogicalOp::And, left: Box::new(left), right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_comparison()?;
        while let Some(tok) = self.peek() {
            match tok {
                Token::EqEq | Token::Neq => {
                    self.next();
                    let right = self.parse_comparison()?;
                    let op = if tok == Token::EqEq { BinOp::Eq } else { BinOp::Neq };
                    left = Expr::Binary { op, left: Box::new(left), right: Box::new(right) };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;
        while let Some(tok) = self.peek() {
            match tok {
                Token::Lt | Token::Gt | Token::Le | Token::Ge => {
                    self.next();
                    let right = self.parse_term()?;
                    let op = match tok {
                        Token::Lt => BinOp::Lt,
                        Token::Gt => BinOp::Gt,
                        Token::Le => BinOp::Le,
                        Token::Ge => BinOp::Ge,
                        _ => unreachable!(),
                    };
                    left = Expr::Binary { op, left: Box::new(left), right: Box::new(right) };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor()?;
        while let Some(tok) = self.peek() {
            match tok {
                Token::Plus | Token::Minus => {
                    self.next();
                    let right = self.parse_factor()?;
                    let op = if tok == Token::Plus { BinOp::Add } else { BinOp::Sub };
                    left = Expr::Binary { op, left: Box::new(left), right: Box::new(right) };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;
        while let Some(tok) = self.peek() {
            match tok {
                Token::Star | Token::Slash => {
                    self.next();
                    let right = self.parse_unary()?;
                    let op = if tok == Token::Star { BinOp::Mul } else { BinOp::Div };
                    left = Expr::Binary { op, left: Box::new(left), right: Box::new(right) };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if self.peek() == Some(Token::Bang) {
            self.next();
            let expr = self.parse_unary()?;
            return Ok(Expr::Not(Box::new(expr)));
        }
        self.parse_primary()
    }


    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Some(Token::Ident(name)) => {
                self.next();
                if name == "match" {
                    let expr = self.parse_expression()?;
                    self.expect(Token::LBrace)?;
                    let mut arms = Vec::new();
                    while let Some(tok) = self.peek() {
                        if tok == Token::RBrace { break; }
                        let pat = match self.next() {
                            Some(Token::Ident(p)) => p,
                            _ => return Err("Expected pattern in match".to_string()),
                        };
                        // expect => (for now assume Token::Ident("=>"))
                        match self.next() {
                            Some(Token::Ident(arr)) if arr == "=>" => {},
                            Some(Token::Equal) => { if let Some(Token::Gt) = self.peek() { self.next(); } },
                            _ => return Err("Expected => in match".to_string()),
                        }
                        let body = if self.peek() == Some(Token::LBrace) {
                            self.parse_block()?
                        } else {
                            let expr = self.parse_expression()?;
                            if self.peek() == Some(Token::Comma) { self.next(); }
                            vec![Stmt::Expr(expr)]
                        };
                        arms.push((pat, body));
                    }
                    self.expect(Token::RBrace)?;
                    Ok(Expr::Match { expr: Box::new(expr), arms })
                } else if name == "Ap" {
                    self.expect(Token::LPar)?;
                    let num = self.parse_integer()?;
                    self.expect(Token::RPar)?;
                    Ok(Expr::Atom { prime: num })
                } else if let Some(Token::LPar) = self.peek() {
                    // Function call
                    self.next(); // consume '('
                    let mut args = Vec::new();
                    if let Some(Token::RPar) = self.peek() {
                        self.next(); // consume ')'
                    } else {
                        loop {
                            let expr = self.parse_expression()?;
                            args.push(expr);
                            match self.peek() {
                                Some(Token::Comma) => { self.next(); }
                                Some(Token::RPar) => { self.next(); break; }
                                Some(tok) => return Err(format!("Unexpected token in call args: {:?}", tok)),
                                None => return Err("Unexpected EOF in call args".to_string()),
                            }
                        }
                    }
                    Ok(Expr::Call { name, args })
                } else if let Some(Token::LBrace) = self.peek() {
                    // Struct initialization
                    self.next(); // consume '{'
                    let mut fields = Vec::new();
                    while let Some(tok) = self.peek() {
                        if tok == Token::RBrace { break; }
                        let field_name = match self.next() {
                            Some(Token::Ident(p)) => p,
                            _ => return Err("Expected field name in struct init".to_string()),
                        };
                        let _ = self.expect(Token::Colon).or_else(|_| self.expect(Token::Ident(":".to_string()))); 
                        let val = self.parse_expression()?;
                        fields.push((field_name, Box::new(val)));
                        if self.peek() == Some(Token::Comma) {
                            self.next();
                        }
                    }
                    self.expect(Token::RBrace)?;
                    Ok(Expr::StructInit { name, fields })
                } else {
                    let mut base = Expr::Ident(name);
                    while self.peek() == Some(Token::Dot) {
                        self.next();
                        let field = match self.next() {
                            Some(Token::Ident(f)) => f,
                            _ => return Err("Expected field or method name after dot".to_string()),
                        };
                        if self.peek() == Some(Token::LPar) {
                            self.next();
                            let mut args = Vec::new();
                            if self.peek() != Some(Token::RPar) {
                                loop {
                                    args.push(self.parse_expression()?);
                                    if self.peek() == Some(Token::Comma) {
                                        self.next();
                                    } else {
                                        break;
                                    }
                                }
                            }
                            self.expect(Token::RPar)?;
                            base = Expr::MethodCall { obj: Box::new(base), method: field, args };
                        } else {
                            base = Expr::FieldAccess { obj: Box::new(base), field };
                        }
                    }
                    Ok(base)
                }
            }
            Some(Token::If) => {
                // if (cond) { then } else { else }
                self.next(); // consume 'if'
                self.expect(Token::LPar)?;
                let cond = self.parse_expression()?;
                self.expect(Token::RPar)?;
                self.expect(Token::LBrace)?;
                let then_branch = self.parse_block()?;
                let else_branch = if let Some(Token::Else) = self.peek() {
                    self.next(); // consume 'else'
                    self.expect(Token::LBrace)?;
                    Some(self.parse_block()?)
                } else {
                    None
                };
                Ok(Expr::If {
                    cond: Box::new(cond),
                    then_branch,
                    else_branch,
                })
            }
            Some(Token::Integer(val)) => {
                self.next();
                Ok(Expr::Literal(val))
            }
            Some(Token::Float(val)) => {
                self.next();
                Ok(Expr::FloatLit(val))
            }
            Some(Token::CharLit(val)) => {
                self.next();
                Ok(Expr::CharLit(val))
            }
            Some(Token::StringLit(val)) => {
                self.next();
                Ok(Expr::StringLit(val))
            }
            Some(Token::LPar) => {
                self.next();
                let mut elems = Vec::new();
                if self.peek() != Some(Token::RPar) {
                    loop {
                        elems.push(self.parse_expression()?);
                        if self.peek() == Some(Token::Comma) {
                            self.next();
                        } else {
                            break;
                        }
                    }
                }
                self.expect(Token::RPar)?;
                if elems.len() == 1 {
                    Ok(elems.pop().unwrap())
                } else {
                    Ok(Expr::Tuple(elems))
                }
            }
            Some(tok) => Err(format!("Unexpected token in expression: {:?}", tok)),
            None => Err("Unexpected EOF".to_string()),
        }
    }

    fn parse_integer(&mut self) -> Result<u64, String> {
        match self.next() {
            Some(Token::Integer(v)) => Ok(v),
            _ => Err("Expected integer".to_string()),
        }
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        // Expect opening brace already consumed
        let mut stmts = Vec::new();
        while let Some(tok) = self.peek() {
            match tok {
                Token::RBrace => {
                    self.next(); // consume closing brace
                    break;
                }
                _ => {
                    stmts.push(self.parse_statement()?);
                }
            }
        }
        Ok(stmts)
    }

    fn parse_ensemble_path(&mut self) -> Result<String, String> {
        let mut path = String::new();
        match self.next() {
            Some(Token::Ident(id)) => path.push_str(&id),
            other => {
                return Err(format!(
                    "Expected identifier in ensemble path, got {:?}",
                    other
                ))
            }
        }
        while let Some(Token::Minus) = self.peek() {
            self.next();
            path.push('-');
            match self.next() {
                Some(Token::Ident(id)) => path.push_str(&id),
                other => return Err(format!("Expected identifier after '-', got {:?}", other)),
            }
        }
        Ok(path)
    }

    fn parse_item_path(&mut self) -> Result<String, String> {
        let mut path = self.parse_ensemble_path()?;
        while let Some(Token::ColonColon) = self.peek() {
            self.next();
            path.push_str("::");
            match self.next() {
                Some(Token::Ident(id)) => path.push_str(&id),
                other => return Err(format!("Expected identifier after '::', got {:?}", other)),
            }
        }
        Ok(path)
    }

    fn parse_version(&mut self) -> Result<String, String> {
        let mut raw = String::new();
        while let Some(tok) = self.peek() {
            if tok == Token::Ident("prime".to_string()) || tok == Token::Semicolon {
                break;
            }
            match self.next() {
                Some(Token::Ident(s)) => raw.push_str(&s),
                Some(Token::Integer(i)) => raw.push_str(&i.to_string()),
                Some(Token::Dot) => raw.push('.'),
                _ => {}
            }
        }
        Ok(raw)
    }

    pub fn parse_statement(&mut self) -> Result<Stmt, String> {
        match self.peek() {
            Some(Token::Ensemble) => {
                self.next(); // consume 'ensemble'
                let name = self.parse_ensemble_path()?;
                let version = self.parse_version()?;
                let prime = match self.next() {
                    Some(Token::Ident(ref s)) if s == "prime" => {
                        self.expect(Token::Equal)?;
                        self.parse_integer()?
                    }
                    other => return Err(format!("Expected 'prime' identifier, got {:?}", other)),
                };
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Ensemble(EnsembleDecl {
                    name,
                    version,
                    prime,
                }))
            }
            Some(Token::Use) => {
                self.next(); // consume 'use'
                let path = self.parse_item_path()?;

                let mut alias = None;
                if let Some(Token::As) = self.peek() {
                    self.next();
                    if let Some(Token::Ident(id)) = self.next() {
                        alias = Some(id);
                    } else {
                        return Err("Expected identifier after 'as'".into());
                    }
                }

                let mut spectral_budget = None;
                if let Some(Token::With) = self.peek() {
                    self.next();
                    if let Some(Token::Ident(ref s)) = self.next() {
                        if s == "spectral_budget" {
                            self.expect(Token::Equal)?;
                            // Expect float
                            let int_part = self.parse_integer()?;
                            self.expect(Token::Dot)?;
                            let frac_part = self.parse_integer()?;
                            let float_str = format!("{}.{}", int_part, frac_part);
                            spectral_budget = float_str.parse::<f64>().ok();
                        } else {
                            return Err("Expected 'spectral_budget'".into());
                        }
                    } else {
                        return Err("Expected 'spectral_budget'".into());
                    }
                }

                self.expect(Token::Semicolon)?;
                Ok(Stmt::Import(ImportStmt {
                    path,
                    alias,
                    spectral_budget,
                }))
            }
            Some(Token::Let) => {
                self.next();
                let is_mut = if self.peek() == Some(Token::Mut) {
                    self.next();
                    true
                } else {
                    false
                };
                let name = match self.next() {
                    Some(Token::Ident(id)) => id,
                    other => return Err(format!("Expected identifier after let, got {:?}", other)),
                };
                self.expect(Token::Equal)?;
                let expr = self.parse_expression()?;
                self.expect(Token::Semicolon)?;
                if is_mut {
                    Ok(Stmt::LetMut { name, expr })
                } else {
                    Ok(Stmt::Let { name, expr })
                }
            }
            Some(Token::Return) => {
                self.next();
                let expr = if self.peek() == Some(Token::Semicolon) {
                    None
                } else {
                    Some(self.parse_expression()?)
                };
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Return(expr))
            }
            Some(Token::If) | Some(Token::Extern) | Some(Token::Fn) | Some(Token::Ident(_)) => {
                let peeked = self.peek();
                if peeked == Some(Token::If) || peeked == Some(Token::Ident("if".to_string())) {
                    self.next(); // consume 'if'
                    self.expect(Token::LPar)?;
                    let cond = self.parse_expression()?;
                    self.expect(Token::RPar)?;
                    self.expect(Token::LBrace)?;
                    let then_branch = self.parse_block()?;
                    let else_branch = if self.peek() == Some(Token::Else) || self.peek() == Some(Token::Ident("else".to_string())) {
                        self.next(); // consume 'else'
                        self.expect(Token::LBrace)?;
                        Some(self.parse_block()?)
                    } else {
                        None
                    };
                    return Ok(Stmt::If { cond, then_branch, else_branch });
                } else if peeked == Some(Token::Ident("while".to_string())) {
                    self.next(); // consume 'while'
                    self.expect(Token::LPar)?;
                    let cond = self.parse_expression()?;
                    self.expect(Token::RPar)?;
                    self.expect(Token::LBrace)?;
                    let body = self.parse_block()?;
                    return Ok(Stmt::Loop { cond: Some(cond), body });
                } else if peeked == Some(Token::Ident("loop".to_string())) {
                    self.next(); // consume 'loop'
                    self.expect(Token::LBrace)?;
                    let body = self.parse_block()?;
                    return Ok(Stmt::Loop { cond: None, body });
                } else if peeked == Some(Token::Ident("fn".to_string())) || peeked == Some(Token::Fn) {
                    self.next(); // consume 'fn'
                    let name = match self.next() {
                        Some(Token::Ident(id)) => id,
                        _ => return Err("Expected function name".to_string()),
                    };
                    let generics = self.parse_generics_decl()?;
                    self.expect(Token::LPar)?;
                    let mut params = Vec::new();
                    while let Some(tok) = self.peek() {
                        if tok == Token::RPar { break; }
                        if let Token::Ident(param_name) = self.next().unwrap() {
                            let _ = self.expect(Token::Colon).or_else(|_| self.expect(Token::Ident(":".to_string()))); 
                            let param_type = self.parse_type()?;
                            params.push((param_name, param_type));
                            if self.peek() == Some(Token::Comma) {
                                self.next(); // consume comma
                            }
                        } else {
                            return Err("Expected parameter name".into());
                        }
                    }
                    self.expect(Token::RPar)?;
                    let return_type = if self.peek() == Some(Token::Minus) {
                        self.next();
                        self.expect(Token::Gt)?;
                        Some(self.parse_type()?)
                    } else { None };
                    self.expect(Token::LBrace)?;
                    let body = self.parse_block()?;
                    return Ok(Stmt::FnDef { name, generics, params, return_type, body });
                } else if peeked == Some(Token::Ident("struct".to_string())) || peeked == Some(Token::Ident("struct".to_string())) {
                    self.next(); // consume 'struct'
                    let name = match self.next() {
                        Some(Token::Ident(id)) => id,
                        _ => return Err("Expected struct name".to_string()),
                    };
                    let generics = self.parse_generics_decl()?;
                    self.expect(Token::LBrace)?;
                    let mut fields = Vec::new();
                    while let Some(tok) = self.peek() {
                        if tok == Token::RBrace { break; }
                        if let Token::Ident(field_name) = self.next().unwrap() {
                            let _ = self.expect(Token::Colon).or_else(|_| self.expect(Token::Ident(":".to_string()))); 
                            let field_type = self.parse_type()?;
                            fields.push((field_name, field_type));
                            if self.peek() == Some(Token::Comma) {
                                self.next();
                            }
                        } else {
                            return Err("Expected field name".to_string());
                        }
                    }
                    self.expect(Token::RBrace)?;
                    return Ok(Stmt::StructDef { name, generics, fields });
                } else if peeked == Some(Token::Extern) || peeked == Some(Token::Ident("extern".to_string())) {
                    self.next(); // consume 'extern'
                    let abi = match self.next() {
                        Some(Token::StringLit(s)) => s,
                        _ => return Err("Expected ABI string literal after extern".to_string()),
                    };
                    if self.next() != Some(Token::Fn) {
                        return Err("Expected 'fn' after extern ABI".to_string());
                    }
                    let name = match self.next() {
                        Some(Token::Ident(id)) => id,
                        _ => return Err("Expected function name".to_string()),
                    };
                    self.expect(Token::LPar)?;
                    let mut params = Vec::new();
                    while let Some(tok) = self.peek() {
                        if tok == Token::RPar { break; }
                        if let Token::Ident(param_name) = self.next().unwrap() {
                            let _ = self.expect(Token::Colon).or_else(|_| self.expect(Token::Ident(":".to_string()))); 
                            let param_type = self.parse_type()?;
                            params.push((param_name, param_type));
                            if self.peek() == Some(Token::Comma) {
                                self.next(); // consume comma
                            }
                        } else {
                            return Err("Expected parameter name".into());
                        }
                    }
                    self.expect(Token::RPar)?;
                    let return_type = if self.peek() == Some(Token::Minus) {
                        self.next();
                        self.expect(Token::Gt)?;
                        Some(self.parse_type()?)
                    } else { None };
                    self.expect(Token::Semicolon)?;
                    return Ok(Stmt::ExternFn { name, params, return_type, abi });
                } else if peeked == Some(Token::Ident("enum".to_string())) {
                    self.next(); // consume 'enum'
                    let name = match self.next() {
                        Some(Token::Ident(id)) => id,
                        _ => return Err("Expected enum name".to_string()),
                    };
                    let generics = self.parse_generics_decl()?;
                    self.expect(Token::LBrace)?;
                    let mut variants = Vec::new();
                    while let Some(tok) = self.peek() {
                        if tok == Token::RBrace { break; }
                        if let Token::Ident(var_name) = self.next().unwrap() {
                            let mut var_type = None;
                            if self.peek() == Some(Token::LPar) {
                                self.next(); // consume '('
                                var_type = Some(self.parse_type()?);
                                self.expect(Token::RPar)?;
                            }
                            variants.push((var_name, var_type));
                            if self.peek() == Some(Token::Comma) {
                                self.next();
                            }
                        } else {
                            return Err("Expected variant name".to_string());
                        }
                    }
                    self.expect(Token::RBrace)?;
                    return Ok(Stmt::EnumDef { name, generics, variants });
                }
                let peeked = self.peek();
                if let Some(Token::Ident(name)) = peeked.clone() {
                    if self.tokens.get(self.pos + 1) == Some(&Token::Equal) {
                        self.next(); // consume ident
                        self.next(); // consume =
                        let expr = self.parse_expression()?;
                        if let Some(Token::Semicolon) = self.peek() {
                            self.next();
                        }
                        return Ok(Stmt::Assign { name, expr });
                    }
                }
                
                let expr = self.parse_expression()?;
                if let Some(Token::Semicolon) = self.peek() {
                    self.next();
                }
                Ok(Stmt::Expr(expr))
            }
            Some(Token::LBrace) => {
                self.next(); // consume '{'
                let inner = self.parse_block()?;
                Ok(Stmt::Block(inner))
            }
            _ => {
                let expr = self.parse_expression()?;
                if let Some(Token::Semicolon) = self.peek() {
                    self.next();
                }
                Ok(Stmt::Expr(expr))
            }
        }
    }

    pub fn parse_generics_decl(&mut self) -> Result<Vec<String>, String> {
        if self.peek() == Some(Token::Lt) {
            self.next(); // consume '<'
            let mut params = Vec::new();
            while let Some(tok) = self.peek() {
                if tok == Token::Gt { break; }
                if let Token::Ident(param_name) = self.next().unwrap() {
                    params.push(param_name);
                    if self.peek() == Some(Token::Comma) {
                        self.next();
                    }
                } else {
                    return Err("Expected generic parameter name".to_string());
                }
            }
            self.expect(Token::Gt)?;
            Ok(params)
        } else {
            Ok(Vec::new())
        }
    }

    pub fn parse_type(&mut self) -> Result<Type, String> {
        if self.peek() == Some(Token::LPar) {
            self.next();
            let mut elems = Vec::new();
            while let Some(tok) = self.peek() {
                if tok == Token::RPar { break; }
                elems.push(self.parse_type()?);
                if self.peek() == Some(Token::Comma) {
                    self.next();
                } else {
                    break;
                }
            }
            self.expect(Token::RPar)?;
            return Ok(Type::Tuple(elems));
        }

        if self.peek() == Some(Token::Ident("fn".to_string())) || self.peek() == Some(Token::Fn) {
            self.next(); // consume 'fn'
            self.expect(Token::LPar)?;
            let mut args = Vec::new();
            while let Some(tok) = self.peek() {
                if tok == Token::RPar { break; }
                args.push(self.parse_type()?);
                if self.peek() == Some(Token::Comma) {
                    self.next();
                }
            }
            self.expect(Token::RPar)?;
            
            let ret = if self.peek() == Some(Token::Minus) {
                self.next();
                self.expect(Token::Gt)?;
                Box::new(self.parse_type()?)
            } else {
                Box::new(Type::Simple("unit".to_string()))
            };
            return Ok(Type::Function(args, ret));
        }

        let name = match self.next() {
            Some(Token::Ident(id)) => id,
            _ => return Err("Expected type name".to_string()),
        };

        if self.peek() == Some(Token::Lt) {
            self.next(); // consume '<'
            let mut params = Vec::new();
            while let Some(tok) = self.peek() {
                if tok == Token::Gt { break; }
                params.push(self.parse_type()?);
                if self.peek() == Some(Token::Comma) {
                    self.next();
                }
            }
            self.expect(Token::Gt)?;
            Ok(Type::Generic(name, params))
        } else {
            Ok(Type::Simple(name))
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut stmts = Vec::new();
        while self.peek().is_some() {
            stmts.push(self.parse_statement()?);
        }
        Ok(Program { stmts })
    }
}

pub fn parse(input: &str) -> Result<Program, String> {
    let mut parser = Parser::new(input);
    parser.parse_program()
}

// ---------------------------------------------------------------------------
// PIRTM EBNF Decoder Parser (Mirroring pirtm/csc.py)
// ---------------------------------------------------------------------------

pub use ast::Statement;

#[derive(Debug, Clone, PartialEq)]
enum EBNFToken {
    Keyword(String),
    Lambda,
    Ident(String),
    Prime(String),
    Float(f64),
    Op(String),
}

pub struct PIRTMDecoderParser {
    tokens: Vec<EBNFToken>,
    pos: usize,
}

impl PIRTMDecoderParser {
    pub fn new(source: &str) -> Result<Self, String> {
        let tokens = Self::tokenize(source)?;
        Ok(Self { tokens, pos: 0 })
    }

    fn tokenize(source: &str) -> Result<Vec<EBNFToken>, String> {
        let mut tokens = Vec::new();
        let chars: Vec<char> = source.chars().collect();
        let mut i = 0;
        let n = chars.len();

        while i < n {
            let c = chars[i];
            if c.is_whitespace() {
                i += 1;
                continue;
            }
            if c == '#' {
                while i < n && chars[i] != '\n' {
                    i += 1;
                }
                continue;
            }
            if c == '\\' && i + 8 < n && &chars[i..i + 9] == &['\\', 'L', 'a', 'm', 'b', 'd', 'a', '_', 'm'] {
                tokens.push(EBNFToken::Lambda);
                i += 9;
                continue;
            }
            if c == 'Λ' && i + 2 < n && &chars[i..i + 3] == &['Λ', '_', 'm'] {
                tokens.push(EBNFToken::Lambda);
                i += 3;
                continue;
            }
            if c == '|' && i + 1 < n && chars[i + 1] == '>' {
                tokens.push(EBNFToken::Op("|>".to_string()));
                i += 2;
                continue;
            }
            if c == '[' || c == ']' || c == '(' || c == ')' || c == ',' || c == ';' || c == '*' || c == '<' || c == '>' {
                tokens.push(EBNFToken::Op(c.to_string()));
                i += 1;
                continue;
            }
            if c.is_ascii_digit() {
                let start = i;
                while i < n && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    i += 1;
                }
                let s: String = chars[start..i].iter().collect();
                if let Ok(f) = s.parse::<f64>() {
                    tokens.push(EBNFToken::Float(f));
                } else {
                    return Err(format!("Invalid numeric literal: {}", s));
                }
                continue;
            }
            if c.is_ascii_alphabetic() || c == '_' {
                let start = i;
                while i < n && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let s: String = chars[start..i].iter().collect();
                if s == "tensor" || s == "assert_contractive" {
                    tokens.push(EBNFToken::Keyword(s));
                } else if s.starts_with("p_") && s[2..].chars().all(|ch| ch.is_ascii_digit()) {
                    tokens.push(EBNFToken::Prime(s));
                } else {
                    tokens.push(EBNFToken::Ident(s));
                }
                continue;
            }
            return Err(format!("Unexpected character: {}", c));
        }
        Ok(tokens)
    }

    fn peek(&self) -> Option<&EBNFToken> {
        self.tokens.get(self.pos)
    }

    fn next_token(&mut self) -> Option<EBNFToken> {
        if self.pos < self.tokens.len() {
            let tok = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(tok)
        } else {
            None
        }
    }

    pub fn parse_source(&mut self) -> Result<Vec<Statement>, String> {
        let mut stmts = Vec::new();
        while self.peek().is_some() {
            stmts.push(self.parse_statement()?);
        }
        Ok(stmts)
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.peek() {
            Some(EBNFToken::Keyword(k)) if k == "tensor" => self.parse_tensor_declaration(),
            Some(EBNFToken::Keyword(k)) if k == "assert_contractive" => self.parse_contractivity_assertion(),
            Some(EBNFToken::Ident(_)) => self.parse_operator_application(),
            Some(other) => Err(format!("Unexpected token starting statement: {:?}", other)),
            None => Err("Unexpected EOF".to_string()),
        }
    }

    fn parse_tensor_declaration(&mut self) -> Result<Statement, String> {
        self.next_token(); // consume 'tensor'
        let ident = match self.next_token() {
            Some(EBNFToken::Ident(id)) => id,
            other => return Err(format!("Expected identifier after tensor, got {:?}", other)),
        };
        match self.next_token() {
            Some(EBNFToken::Op(ref op)) if op == "[" => {}
            other => return Err(format!("Expected '[' after tensor identifier, got {:?}", other)),
        }
        let mut primes = Vec::new();
        loop {
            match self.next_token() {
                Some(EBNFToken::Prime(p)) => primes.push(p),
                other => return Err(format!("Expected prime token p_N, got {:?}", other)),
            }
            match self.peek() {
                Some(EBNFToken::Op(ref op)) if op == "," => {
                    self.next_token(); // consume ','
                }
                _ => break,
            }
        }
        match self.next_token() {
            Some(EBNFToken::Op(ref op)) if op == "]" => {}
            other => return Err(format!("Expected ']' closing tensor primes, got {:?}", other)),
        }
        match self.next_token() {
            Some(EBNFToken::Op(ref op)) if op == ";" => {}
            other => return Err(format!("Expected ';' ending tensor declaration, got {:?}", other)),
        }
        Ok(Statement::TensorDeclaration { identifier: ident, primes })
    }

    fn parse_operator_application(&mut self) -> Result<Statement, String> {
        let ident = match self.next_token() {
            Some(EBNFToken::Ident(id)) => id,
            other => return Err(format!("Expected identifier, got {:?}", other)),
        };
        match self.next_token() {
            Some(EBNFToken::Op(ref op)) if op == "|>" => {}
            other => return Err(format!("Expected '|>' after identifier, got {:?}", other)),
        }
        let has_lambda = if let Some(EBNFToken::Lambda) = self.peek() {
            self.next_token(); // consume \Lambda_m
            match self.next_token() {
                Some(EBNFToken::Op(ref op)) if op == "*" => {}
                other => return Err(format!("Expected '*' after \\Lambda_m, got {:?}", other)),
            }
            true
        } else {
            false
        };
        let mut prime_chain = Vec::new();
        loop {
            match self.next_token() {
                Some(EBNFToken::Prime(p)) => prime_chain.push(p),
                other => return Err(format!("Expected prime token in chain, got {:?}", other)),
            }
            match self.peek() {
                Some(EBNFToken::Op(ref op)) if op == "*" => {
                    self.next_token(); // consume '*'
                }
                _ => break,
            }
        }
        match self.next_token() {
            Some(EBNFToken::Op(ref op)) if op == ";" => {}
            other => return Err(format!("Expected ';' ending operator application, got {:?}", other)),
        }
        Ok(Statement::OperatorApplication { identifier: ident, has_lambda, prime_chain })
    }

    fn parse_contractivity_assertion(&mut self) -> Result<Statement, String> {
        self.next_token(); // consume 'assert_contractive'
        match self.next_token() {
            Some(EBNFToken::Op(ref op)) if op == "(" => {}
            other => return Err(format!("Expected '(' after assert_contractive, got {:?}", other)),
        }
        let ident = match self.next_token() {
            Some(EBNFToken::Ident(id)) => id,
            other => return Err(format!("Expected identifier inside assert_contractive, got {:?}", other)),
        };
        match self.next_token() {
            Some(EBNFToken::Op(ref op)) if op == ")" => {}
            other => return Err(format!("Expected ')' closing assert_contractive, got {:?}", other)),
        }
        match self.next_token() {
            Some(EBNFToken::Op(ref op)) if op == "<" => {}
            other => return Err(format!("Expected '<' after assert_contractive(...), got {:?}", other)),
        }
        let bound = match self.next_token() {
            Some(EBNFToken::Float(f)) => f,
            other => return Err(format!("Expected float bound, got {:?}", other)),
        };
        match self.next_token() {
            Some(EBNFToken::Op(ref op)) if op == ";" => {}
            other => return Err(format!("Expected ';' ending contractivity assertion, got {:?}", other)),
        }
        Ok(Statement::ContractivityAssertion { identifier: ident, bound })
    }
}

pub fn parse_ebnf_statements(input: &str) -> Result<Vec<Statement>, String> {
    let mut parser = PIRTMDecoderParser::new(input)?;
    parser.parse_source()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_integer_expr() {
        let prog = parse("42").unwrap();
        assert_eq!(prog.stmts.len(), 1);
        match &prog.stmts[0] {
            Stmt::Expr(Expr::Literal(v)) => assert_eq!(*v, 42),
            _ => panic!("Expected literal expr"),
        }
    }

    #[test]
    fn parses_let_statement() {
        let prog = parse("let x = Ap(2); x + 3").unwrap();
        assert_eq!(prog.stmts.len(), 2);
        match &prog.stmts[0] {
            Stmt::Let { name, expr } => {
                assert_eq!(name, "x");
                assert!(matches!(expr, Expr::Atom { prime: 2 }));
            }
            _ => panic!("Expected let stmt"),
        }
    }

    #[test]
    fn test_ebnf_tensor_declaration() {
        let stmts = parse_ebnf_statements("tensor T_0 [p_2, p_3, p_5];").unwrap();
        assert_eq!(stmts.len(), 1);
        assert_eq!(
            stmts[0],
            Statement::TensorDeclaration {
                identifier: "T_0".to_string(),
                primes: vec!["p_2".to_string(), "p_3".to_string(), "p_5".to_string()],
            }
        );
    }

    #[test]
    fn test_ebnf_operator_application() {
        let stmts = parse_ebnf_statements("T_1 |> \\Lambda_m * p_11 * p_13;").unwrap();
        assert_eq!(stmts.len(), 1);
        assert_eq!(
            stmts[0],
            Statement::OperatorApplication {
                identifier: "T_1".to_string(),
                has_lambda: true,
                prime_chain: vec!["p_11".to_string(), "p_13".to_string()],
            }
        );
    }

    #[test]
    fn test_ebnf_contractivity_assertion() {
        let stmts = parse_ebnf_statements("assert_contractive(T_0) < 0.85;").unwrap();
        assert_eq!(stmts.len(), 1);
        assert_eq!(
            stmts[0],
            Statement::ContractivityAssertion {
                identifier: "T_0".to_string(),
                bound: 0.85,
            }
        );
    }

    #[test]
    fn test_parse_if() {
        let source = "if (x) { let y = 1; } else { let y = 2; }";
        let program = parse(source).unwrap();
        assert_eq!(program.stmts.len(), 1);
        assert!(matches!(program.stmts[0], Stmt::If { .. }));
    }

    #[test]
    fn test_parse_while() {
        let source = "while (x) { let y = 1; }";
        let program = parse(source).unwrap();
        assert_eq!(program.stmts.len(), 1);
        assert!(matches!(program.stmts[0], Stmt::Loop { cond: Some(_), .. }));
    }

    #[test]
    fn test_parse_loop() {
        let source = "loop { let y = 1; }";
        let program = parse(source).unwrap();
        assert_eq!(program.stmts.len(), 1);
        assert!(matches!(program.stmts[0], Stmt::Loop { cond: None, .. }));
    }

    #[test]
    fn test_parse_fn() {
        let source = "fn add(a: int, b: int) { let c = a; }";
        let mut parser = Parser::new(source);
        let program = parser.parse_program().unwrap();
        assert_eq!(program.stmts.len(), 1);
        if let Stmt::FnDef { name, params, generics, .. } = &program.stmts[0] {
            assert_eq!(name, "add");
            assert!(generics.is_empty());
            assert_eq!(params.len(), 2);
        } else {
            panic!("Expected FnDef");
        }
    }

    #[test]
    fn test_parse_struct_def() {
        let source = "struct Point { x: int, y: int }";
        let mut parser = Parser::new(source);
        let program = parser.parse_program().unwrap();
        match &program.stmts[0] {
            Stmt::StructDef { name, fields, .. } => {
                assert_eq!(name, "Point");
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0], ("x".to_string(), Type::Simple("int".to_string())));
                assert_eq!(fields[1], ("y".to_string(), Type::Simple("int".to_string())));
            }
            _ => panic!("Expected StructDef"),
        }
    }

    #[test]
    fn test_parse_struct_empty() {
        let source = "struct Empty {}";
        let mut parser = Parser::new(source);
        let program = parser.parse_program().unwrap();
        match &program.stmts[0] {
            Stmt::StructDef { name, fields, .. } => {
                assert_eq!(name, "Empty");
                assert!(fields.is_empty());
            }
            _ => panic!("Expected StructDef"),
        }
    }

    #[test]
    fn test_parse_enum_def() {
        let source = "enum Option { None, Some(int) }";
        let mut parser = Parser::new(source);
        let program = parser.parse_program().unwrap();
        match &program.stmts[0] {
            Stmt::EnumDef { name, variants, .. } => {
                assert_eq!(name, "Option");
                assert_eq!(variants.len(), 2);
                assert_eq!(variants[0], ("None".to_string(), None));
                assert_eq!(variants[1], ("Some".to_string(), Some(Type::Simple("int".to_string()))));
            }
            _ => panic!("Expected EnumDef"),
        }
    }

    #[test]
    fn test_parse_enum_empty() {
        let source = "enum Empty {}";
        let mut parser = Parser::new(source);
        let program = parser.parse_program().unwrap();
        match &program.stmts[0] {
            Stmt::EnumDef { name, variants, .. } => {
                assert_eq!(name, "Empty");
                assert!(variants.is_empty());
            }
            _ => panic!("Expected EnumDef"),
        }
    }
}
