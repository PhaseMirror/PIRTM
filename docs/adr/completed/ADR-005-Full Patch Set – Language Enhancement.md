## 📦 Full Patch Set – Language Enhancements for PiLang

Below is the complete code for adding **mutability**, **assignment**, **method calls**, **logical operators**, and **tuples**. The changes are organized by crate.

---

### 1. Lexer (`pirtm-lexer`)

**File:** `pirtm-lexer/src/lib.rs`

Add new token variants and regex patterns:

```rust
#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // ... existing ...

    // Keywords
    #[regex(r"mut\b", |_| Token::Mut)]
    Mut,

    // Logical operators
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("!")]
    Not,

    // ... existing (Ident, Integer, etc.)
}
```

---

### 2. AST (`pirtm-parser/src/ast.rs`)

Add new enum variants for `Stmt` and `Expr`:

```rust
// In Stmt
pub enum Stmt {
    // ... existing (Let, If, Loop, FnDef, etc.)
    LetMut {
        name: String,
        typ: Option<Type>,
        expr: Box<Expr>,
    },
    Assign {
        name: String,
        expr: Box<Expr>,
    },
}

// In Expr
pub enum Expr {
    // ... existing
    MethodCall {
        object: Box<Expr>,
        method: String,
        args: Vec<Expr>,
    },
    LogicalOp {
        op: LogicalOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Not {
        expr: Box<Expr>,
    },
    Tuple {
        elems: Vec<Expr>,
    },
}

pub enum LogicalOp {
    And,
    Or,
}
```

Also add `Type::Tuple(Vec<Type>)` if you support tuple types (optional, but useful).

---

### 3. Parser (`pirtm-parser/src/lib.rs`)

#### 3.1 `parse_statement` – handle `let mut` and assignment

```rust
fn parse_statement(&mut self) -> Result<Stmt, String> {
    match self.peek() {
        Some(Token::Keyword(ref s)) if s == "if" => self.parse_if(),
        Some(Token::Keyword(ref s)) if s == "while" => self.parse_while(),
        Some(Token::Keyword(ref s)) if s == "fn" => self.parse_fn(),
        Some(Token::Keyword(ref s)) if s == "let" => {
            self.next(); // consume 'let'
            let is_mut = self.peek_token(Token::Mut);
            if is_mut { self.next(); } // consume 'mut'
            let name = self.parse_identifier()?;
            let typ = if self.peek_token(Token::Colon) {
                self.next();
                Some(self.parse_type()?)
            } else { None };
            self.expect(Token::Equal)?;
            let expr = self.parse_expr()?;
            self.expect(Token::Semicolon)?;
            if is_mut {
                Ok(Stmt::LetMut { name, typ, expr: Box::new(expr) })
            } else {
                Ok(Stmt::Let { name, typ, expr: Box::new(expr) })
            }
        }
        Some(Token::Ident(name)) => {
            // Check if next token is '=' (assignment)
            if self.peek_next() == Some(Token::Equal) {
                self.next(); // consume ident
                self.next(); // consume '='
                let expr = self.parse_expr()?;
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Assign { name, expr: Box::new(expr) })
            } else {
                self.parse_expr_statement()
            }
        }
        // ... other cases (struct, enum, extern, etc.)
        _ => self.parse_expr_statement(),
    }
}
```

#### 3.2 `parse_expr` – add method calls, logical ops, tuples

We'll add precedence levels. We'll assume the existing `parse_expr` already handles binary ops. We'll add a new `parse_primary` that handles `tuple`, `not`, etc.

```rust
fn parse_expr(&mut self) -> Result<Expr, String> {
    // parse or (||) level
    self.parse_or()
}

fn parse_or(&mut self) -> Result<Expr, String> {
    let mut left = self.parse_and()?;
    while let Some(Token::Or) = self.peek() {
        self.next();
        let right = self.parse_and()?;
        left = Expr::LogicalOp { op: LogicalOp::Or, left: Box::new(left), right: Box::new(right) };
    }
    Ok(left)
}

fn parse_and(&mut self) -> Result<Expr, String> {
    let mut left = self.parse_comparison()?;
    while let Some(Token::And) = self.peek() {
        self.next();
        let right = self.parse_comparison()?;
        left = Expr::LogicalOp { op: LogicalOp::And, left: Box::new(left), right: Box::new(right) };
    }
    Ok(left)
}

// Existing comparison parsing remains; update to call parse_primary for atomic expressions.

fn parse_primary(&mut self) -> Result<Expr, String> {
    match self.peek() {
        Some(Token::Not) => {
            self.next();
            let expr = self.parse_primary()?;
            Ok(Expr::Not { expr: Box::new(expr) })
        }
        Some(Token::LParen) => {
            self.next();
            // Check if it's a tuple (contains comma)
            if self.peek() == Some(Token::RParen) {
                self.next();
                return Ok(Expr::Tuple { elems: Vec::new() });
            }
            let first = self.parse_expr()?;
            if self.peek_token(Token::Comma) {
                // It's a tuple
                let mut elems = vec![first];
                while self.peek_token(Token::Comma) {
                    self.next();
                    if self.peek_token(Token::RParen) { break; }
                    elems.push(self.parse_expr()?);
                }
                self.expect(Token::RParen)?;
                Ok(Expr::Tuple { elems })
            } else {
                self.expect(Token::RParen)?;
                Ok(first)  // parenthesized expression
            }
        }
        // ... existing for Literal, Ident, Atom, etc.
    }
}

// After parsing a primary, check for method call
fn parse_postfix(&mut self, left: Expr) -> Result<Expr, String> {
    if self.peek_token(Token::Dot) {
        // method call
        self.next();
        let method = self.parse_identifier()?;
        self.expect(Token::LParen)?;
        let mut args = Vec::new();
        while !self.peek_token(Token::RParen) {
            args.push(self.parse_expr()?);
            if self.peek_token(Token::Comma) { self.next(); }
        }
        self.expect(Token::RParen)?;
        Ok(Expr::MethodCall { object: Box::new(left), method, args })
    } else {
        Ok(left)
    }
}
```

In `parse_expr`, after getting the primary, call `parse_postfix` to chain methods.

We'll also adjust the precedence so that `parse_comparison` uses `parse_primary` and `parse_postfix`.

---

### 4. Type System (`pirtm-parser/src/typecheck.rs`)

We'll assume you have a type checking module; add cases for the new nodes. For brevity, I'll outline the key checks:

- **`LetMut`**: similar to `Let`, but marks variable as mutable in the environment.
- **`Assign`**: check that variable exists and is mutable, and the expression type matches the variable type.
- **`MethodCall`**: resolve the method on the object's type. For built‑in types, map to FFI functions (e.g., `String::len`, `Vec::push`). For now, you can hard‑code a small set.
- **`LogicalOp`**: both sides must be `bool`.
- **`Not`**: operand must be `bool`.
- **`Tuple`**: type is `Tuple` of the element types.

---

### 5. MLIR Lowering (`pirtm-mlir/src/pirtm/transpiler/visitor.rs`)

#### 5.1 Environment tracking

We need a symbol table that tracks both the SSA value (for immutable variables) and the pointer (for mutable variables). We'll store an enum:

```rust
enum VarInfo {
    SSA(inkwell::values::BasicValueEnum),
    Ptr(inkwell::values::PointerValue),
}
```

- For `Let` (immutable), we store the SSA value.
- For `LetMut`, we allocate memory with `builder.build_alloca`, store the initial value, and store the pointer.
- For `Assign`, we load the pointer, store the new value.

#### 5.2 Visit methods for new statements/expressions

Add `visit_let_mut`, `visit_assign`, `visit_method_call`, `visit_logical_op`, `visit_not`, `visit_tuple`.

Example for assignment:

```rust
fn visit_assign(&mut self, stmt: &Stmt::Assign) -> Result<PirtmOp, String> {
    // Look up the variable in the environment
    let var_info = self.env.get(&stmt.name).ok_or("Undefined variable")?;
    let ptr = match var_info {
        VarInfo::Ptr(p) => p,
        _ => return Err("Cannot assign to immutable variable".to_string()),
    };
    let val = self.visit_expr(&stmt.expr)?;
    // Emit store
    // We need to generate the appropriate MLIR operation; we'll add to ops.
    // For now, we'll generate a PirtmOp::Store { ptr, val }
    Ok(PirtmOp::Store {
        ptr: Box::new(PirtmOp::from_ssa(ptr)),
        val: Box::new(val),
    })
}
```

We'll need to extend `PirtmOp` with variants for `Alloca`, `Store`, `Load`, `MethodCall`, `LogicalOp`, `Not`, `Tuple`, etc.

#### 5.3 Emit MLIR for new ops

In `ops.rs`, add new variants and their `emit_mlir` implementation. For instance:

```rust
PirtmOp::Alloca { typ } => Ok(format!("%alloca = llvm.alloca {} x i1", typ)),
PirtmOp::Store { ptr, val } => {
    let ptr_s = ptr.emit_mlir()?;
    let val_s = val.emit_mlir()?;
    Ok(format!("llvm.store {}, {}", val_s, ptr_s))
}
// ...
```

---

### 6. Updated `json_parser.pirtm` with Native Syntax

Replace the FFI state hacks with native mutability and method calls. Example snippet:

```pirtm
fn parse_value(p: &mut Parser) -> Result<JsonValue, str> {
    p.skip_whitespace();
    let c = p.peek();
    match c {
        Some('n') => { if !p.consume_literal("null") { return Err("expected null"); } JsonValue::Null }
        Some('t') => { if !p.consume_literal("true") { return Err("expected true"); } JsonValue::Bool(true) }
        // ...
    }
}

// where Parser is a struct with methods like peek, consume_literal, etc.
```

Now we can write `p.pos = p.pos + 1;` and `p.peek()` as method calls, using native syntax.

We'll provide the full updated `json_parser.pirtm` file separately.

---

### 7. Integration and Testing

After applying these patches:

1. **Update `pirtm-parser`** with the new AST and parser.
2. **Update `pirtm-mlir`** with new ops and visitor logic.
3. **Run `cargo test`** to verify all existing tests pass.
4. **Write new tests** for each feature (assignment, method calls, logical ops, tuples).
5. **Compile the updated `json_parser.pirtm`** and run it (either in simulation or with real tools).

---

### 📦 Full Patch Files

I’ll provide the complete modified files in the next response. Since the answer length is limited, I'll give you the core changes and the full `json_parser.pirtm` code. If you need the entire files, I can produce them incrementally.

Let me know if you want the complete code for each file, or if the above summaries are sufficient for you to implement. I'm ready to produce the detailed code.
