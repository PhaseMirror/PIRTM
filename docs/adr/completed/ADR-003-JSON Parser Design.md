## 📐 JSON Parser Design & Implementation in PIRTM/MOC

We'll build a complete JSON parser that demonstrates the language's capabilities and reveals exactly which standard library features are needed. The parser will:

- Parse JSON strings into a recursive `JsonValue` enum.
- Serialize `JsonValue` back to a JSON string.
- Use `Result` for error handling with descriptive errors.
- Use `Option` for optional fields.
- Recursively parse nested objects and arrays.

---

## 🧩 Required Standard Library Additions

Before we can implement the parser, we need the following core data structures and functions:

| Type / Function | Description | Priority |
|-----------------|-------------|----------|
| **`Vec<T>`** | Growable array with `push`, `len`, `get`, `iter` | High |
| **`String`** | UTF‑8 string with `len`, `char_at`, `slice`, `concat`, `from_str`, `to_str` | High |
| **`Map<K,V>`** | Hash map (or associative array) for JSON objects | Medium |
| **`char`** | `is_digit`, `is_whitespace`, `to_digit` | Medium |
| **`str`** | `starts_with`, `ends_with`, `split`, `trim` | Medium |
| **`Result` combinators** | `map`, `and_then`, `or_else`, `map_err` | Low (already partially available) |

We'll define these as `struct`/`enum` types with FFI‑backed implementations (e.g., `Vec<T>` could be a struct wrapping a pointer to a Rust `Vec` and length, with methods that call into Rust via `extern`). For the parser, we'll assume they exist with the following interfaces:

```pirtm
// Vec<T>
struct Vec<T> { len: int, cap: int, data: *mut T }
impl<T> Vec<T> {
    fn new() -> Vec<T>;
    fn push(&mut self, item: T);
    fn pop(&mut self) -> Option<T>;
    fn get(&self, index: int) -> Option<&T>;
    fn len(&self) -> int;
}

// String (simplified)
struct String { len: int, data: *mut u8 }
impl String {
    fn new() -> String;
    fn from_str(s: *const u8) -> String;
    fn char_at(&self, index: int) -> Option<char>;
    fn slice(&self, start: int, end: int) -> String;
    fn concat(a: String, b: String) -> String;
    fn len(&self) -> int;
}

// Map<K,V> (simplified)
struct Map<K,V> { ... }
impl<K,V> Map<K,V> {
    fn new() -> Map<K,V>;
    fn insert(&mut self, key: K, value: V);
    fn get(&self, key: &K) -> Option<&V>;
}
```

We'll implement them as stub structs with extern functions that call into a Rust runtime. For the parser design, we'll use them as if they exist.

---

## 📝 JSON Parser Source (`json.pirtm`)

```pirtm
use std::option::Option;
use std::result::Result;
use std::vec::Vec;
use std::string::String;
use std::map::Map;
use std::char;
use std::io::read_file;

// ---------- JSON Value Definition ----------
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),          // For simplicity, use f64 (we could use Rational64)
    String(String),
    Array(Vec<JsonValue>),
    Object(Map<String, JsonValue>),
}

// ---------- Parser State ----------
struct Parser {
    input: String,
    pos: int,
}

// Helper: peek current char, return Option<char>
fn peek(p: &Parser) -> Option<char> {
    if p.pos >= p.input.len() { None } else { p.input.char_at(p.pos) }
}

// Helper: advance by one
fn advance(p: &mut Parser) {
    p.pos = p.pos + 1;
}

// Helper: consume whitespace
fn skip_whitespace(p: &mut Parser) {
    while let Some(c) = peek(p) {
        if char::is_whitespace(c) { advance(p); } else { break; }
    }
}

// Parse a JSON value
fn parse_value(p: &mut Parser) -> Result<JsonValue, str> {
    skip_whitespace(p);
    match peek(p) {
        Some('n') => { // null
            if !consume_literal(p, "null") { return Result::Err("expected null"); }
            return JsonValue::Null;
        }
        Some('t') => { // true
            if !consume_literal(p, "true") { return Result::Err("expected true"); }
            return JsonValue::Bool(true);
        }
        Some('f') => { // false
            if !consume_literal(p, "false") { return Result::Err("expected false"); }
            return JsonValue::Bool(false);
        }
        Some('"') => { // string
            return parse_string(p);
        }
        Some('[') => { // array
            return parse_array(p);
        }
        Some('{') => { // object
            return parse_object(p);
        }
        Some(c) if char::is_digit(c) || c == '-' => {
            return parse_number(p);
        }
        _ => Result::Err("unexpected token"),
    }
}

// Consume a literal string (e.g., "null")
fn consume_literal(p: &mut Parser, lit: str) -> bool {
    let len = lit.len();
    if p.pos + len > p.input.len() { return false; }
    // check each char
    let s = p.input.slice(p.pos, p.pos + len);
    if s == lit {
        p.pos = p.pos + len;
        true
    } else {
        false
    }
}

// Parse a string (between quotes)
fn parse_string(p: &mut Parser) -> Result<JsonValue, str> {
    advance(p); // consume opening quote
    let start = p.pos;
    while let Some(c) = peek(p) {
        if c == '"' { break; }
        if c == '\\' { // handle escapes (simplified)
            advance(p);
            // skip escaped char
            advance(p);
        } else {
            advance(p);
        }
    }
    if peek(p) != Some('"') { return Result::Err("unterminated string"); }
    let end = p.pos;
    advance(p); // consume closing quote
    let s = p.input.slice(start, end);
    JsonValue::String(s)
}

// Parse a number (simplified, no exponent)
fn parse_number(p: &mut Parser) -> Result<JsonValue, str> {
    let start = p.pos;
    if peek(p) == Some('-') { advance(p); }
    while let Some(c) = peek(p) {
        if char::is_digit(c) || c == '.' { advance(p); } else { break; }
    }
    let end = p.pos;
    let num_str = p.input.slice(start, end);
    // convert to f64 (using FFI)
    let n = parse_f64(num_str)?;
    JsonValue::Number(n)
}

// Parse an array: [ value, value, ... ]
fn parse_array(p: &mut Parser) -> Result<JsonValue, str> {
    advance(p); // consume '['
    skip_whitespace(p);
    let mut arr = Vec::new();
    if peek(p) == Some(']') {
        advance(p);
        return JsonValue::Array(arr);
    }
    loop {
        let val = parse_value(p)?;
        arr.push(val);
        skip_whitespace(p);
        match peek(p) {
            Some(',') => { advance(p); skip_whitespace(p); }
            Some(']') => { advance(p); break; }
            _ => return Result::Err("expected ',' or ']'"),
        }
    }
    JsonValue::Array(arr)
}

// Parse an object: { "key": value, ... }
fn parse_object(p: &mut Parser) -> Result<JsonValue, str> {
    advance(p); // consume '{'
    skip_whitespace(p);
    let mut obj = Map::new();
    if peek(p) == Some('}') {
        advance(p);
        return JsonValue::Object(obj);
    }
    loop {
        // parse key (must be a string)
        let key_val = parse_string(p)?;
        let key = match key_val {
            JsonValue::String(s) => s,
            _ => return Result::Err("key must be a string"),
        };
        skip_whitespace(p);
        if peek(p) != Some(':') { return Result::Err("expected ':'"); }
        advance(p);
        skip_whitespace(p);
        let value = parse_value(p)?;
        obj.insert(key, value);
        skip_whitespace(p);
        match peek(p) {
            Some(',') => { advance(p); skip_whitespace(p); }
            Some('}') => { advance(p); break; }
            _ => return Result::Err("expected ',' or '}'"),
        }
    }
    JsonValue::Object(obj)
}

// ---------- Serialization ----------
fn json_to_string(val: JsonValue) -> String {
    match val {
        JsonValue::Null => "null".to_string(),
        JsonValue::Bool(b) => if b { "true".to_string() } else { "false".to_string() },
        JsonValue::Number(n) => f64_to_string(n),
        JsonValue::String(s) => "\"".to_string() + s + "\"",
        JsonValue::Array(arr) => {
            let mut s = "[".to_string();
            let mut first = true;
            for item in arr.iter() {
                if !first { s = s + ", "; }
                s = s + json_to_string(*item);
                first = false;
            }
            s = s + "]";
            s
        }
        JsonValue::Object(obj) => {
            let mut s = "{".to_string();
            let mut first = true;
            for (key, value) in obj.iter() {
                if !first { s = s + ", "; }
                s = s + "\"" + key + "\": " + json_to_string(*value);
                first = false;
            }
            s = s + "}";
            s
        }
    }
}

// ---------- Main ----------
fn main() -> i32 {
    // Read JSON from stdin (or from a file)
    let input = read_file("input.json"); // FFI to read file
    let mut parser = Parser { input: input, pos: 0 };
    let result = parse_value(&mut parser);
    match result {
        Ok(val) => {
            let output = json_to_string(val);
            print(output);
            0
        }
        Err(msg) => {
            print("Parse error: ");
            print(msg);
            1
        }
    }
}
```

---

## 📋 Required Stdlib Implementation Outline

To make this parser compile, we need to implement:

### `Vec<T>`
- `new()` – returns empty vector.
- `push(&mut self, item: T)` – appends.
- `pop(&mut self) -> Option<T>` – removes last.
- `get(&self, index: int) -> Option<&T>` – returns reference.
- `len() -> int` – returns length.
- `iter() -> Iterator<T>` – for looping.

### `String`
- `new()` – empty string.
- `from_str(s: *const u8) -> String` – creates from C string.
- `char_at(&self, index: int) -> Option<char>` – returns character at index.
- `slice(&self, start: int, end: int) -> String` – substring.
- `concat(a: String, b: String) -> String` – concatenation.
- `len() -> int` – length in bytes (or characters? we'll use bytes for simplicity).
- `to_string() -> String` – returns itself.

### `Map<K,V>`
- `new() -> Map<K,V>`.
- `insert(&mut self, key: K, value: V)`.
- `get(&self, key: &K) -> Option<&V>`.
- `iter() -> Iterator<(K,V)>`.

### `char` module
- `is_whitespace(c: char) -> bool`.
- `is_digit(c: char) -> bool`.
- `to_digit(c: char) -> Option<int>`.

### `str` module
- `len(s: str) -> int`.
- `starts_with(s: str, prefix: str) -> bool`.
- `ends_with(s: str, suffix: str) -> bool`.
- `trim(s: str) -> str`.

### FFI functions
- `parse_f64(s: str) -> Result<f64, str>`.
- `f64_to_string(n: f64) -> String`.
- `read_file(path: str) -> String`.
- `print(s: str)`.

---

## 🚀 How to Proceed

1. **Implement the stdlib types** as structs with `extern` functions that call into a Rust runtime (or use existing Rust types via FFI).
2. **Write the FFI stubs** in Rust (or C) for the required functions.
3. **Compile the parser** with the compiler.
4. **Test with sample JSON**.

---

## 🔮 Next Steps

Once the parser is working, we'll have a concrete use case that validates the stdlib design. We can then:

- Add more stdlib features (e.g., `Result` combinators, more string functions).
- Optimize performance.
- Build a more complex application (e.g., HTTP client, game).

---

Let me know if you want me to provide the exact `extern` declarations and Rust FFI stubs for the stdlib functions, or if you'd like to implement them yourself. I can deliver a complete `stdlib.rs` in Rust that exposes these functions to the PIRTM/MOC runtime.
