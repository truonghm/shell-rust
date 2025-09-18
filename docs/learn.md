# Rust Learning Summary

## String Operations

**String Slicing:**
```rust
let slice = &text[0..5];   // "hello" 
let slice = &text[6..];    // from index 6 to end
let slice = &text[..5];    // from start to index 5
let slice = &text[..];     // entire string
```

**Removing Prefixes (most elegant way):**
```rust
if let Some(after) = text.strip_prefix("test") {
    // after contains everything after "test"
}
```

**String Splitting:**
```rust
let second = text.split(" ").nth(1);  // Returns Option<&str>
let count = text.split(" ").count();  // Get number of parts

// Collect to Vec if you need both parts and length
let parts: Vec<&str> = text.split(" ").collect();
let length = parts.len();
```

## Option Type & Pattern Matching

**Option safely handles "maybe no value" situations:**
```rust
// Safe extraction with if let
if let Some(value) = text.split(" ").nth(1) {
    println!("{}", value);  // value is &str, not Option<&str>
}

// Match expression for handling both cases
match text.split(" ").nth(1) {
    Some(value) => println!("Found: {}", value),
    None => println!("No second element"),
}

// Provide default value
let value = text.split(" ").nth(1).unwrap_or("default");

// vs unsafe (can panic!)
let value = text.split(" ").nth(1).unwrap();
```

**Key concept:** `Some(value)` extracts the value from the Option, while `let x = option` keeps it wrapped.

## Constants & Arrays

**String constants:**
```rust
const COMMANDS: [&str; 2] = ["echo", "exit"];
const COMMANDS: [String; 2] = ["echo", "exit"];

// Type can be inferred
const COMMANDS = ["echo", "exit"];  // Rust infers [&str; 2]
```

**Checking membership:**
```rust
if COMMANDS.contains(&command) {
    // found it
}
```

**Why &str vs String:**
- `&str`: Reference to string data (compile-time friendly)
- `String`: Owned, heap-allocated string (runtime allocation needed)

## Control Flow & Syntax

**While loops:**
```rust
while condition {
    // code
}

// Or infinite loop
loop {
    if some_condition {
        break;
    }
}
```

**Semicolons:**
- **Need after:** statements like `let x = 5;`, `println!("hello");`, `return;`
- **Don't need after:** control flow blocks (`if {}`, `while {}`, `match {}`)

**Printing:**
```rust
println!("{}", variable);           // Need format string with {}
println!("{:?}", variable);         // Debug formatting
println!("Value: {}", variable);    // With text
```

## Error Handling

**Different approaches:**
```rust
// Panic (crash the program)
if condition {
    panic!("Something went wrong!");
}

// Assert (panic if condition is false)
assert!(condition, "Custom message");

// Return error (more idiomatic)
if condition {
    return Err("Error message");  // if function returns Result
}
```

## Key Differences from Python

| Operation           | Python                             | Rust                              |
| ------------------- | ---------------------------------- | --------------------------------- |
| String prefix check | `text.startswith("test")`          | `text.starts_with("test")`        |
| Remove prefix       | `text[4:]` or `text[len("test"):]` | `text.strip_prefix("test")`       |
| String slicing      | `text[4:]`                         | `&text[4..]`                      |
| Get split element   | `text.split(" ")[1]`               | `text.split(" ").nth(1).unwrap()` |
| Length after split  | `len(text.split(" "))`             | `text.split(" ").count()`         |
| Membership test     | `item in list`                     | `list.contains(&item)`            |
| Raise error         | `raise ValueError()`               | `panic!()` or `return Err()`      |
| Print variable      | `print(variable)`                  | `println!("{}", variable)`        |

## Important Concepts

1. **Ownership & Borrowing:** `&` creates a reference/borrow
2. **Iterator vs Collection:** `split()` returns iterator, need `.collect()` or `.count()` for length
3. **Option Type:** Rust's way of handling nullable values safely
4. **Pattern Matching:** `if let Some(x) = option` extracts values safely
5. **Compile-time Safety:** Many errors caught at compile time vs runtime
6. **Memory Management:** No garbage collector, but automatic memory management through ownership

## Best Practices

- Use `Option` and pattern matching instead of unwrap when possible
- Prefer `&str` for constants and function parameters
- Use `String` when you need to own/modify the string
- Use `strip_prefix()` instead of manual slicing
- Handle errors with `Result` type rather than panicking
- Let Rust infer types when obvious, specify when clarity helps