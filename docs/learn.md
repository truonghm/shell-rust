# Rust Learning Summary

## String Memory Management

**Reusing String allocations in loops:**
```rust
// Inefficient - new allocation every iteration
loop {
    let mut input = String::new();  // allocates new buffer
    // ... use input
}

// Efficient - reuse allocation
let mut input = String::new();
loop {
    input.clear();  // keeps capacity, clears content
    // ... use input
}
```

**Key benefits:**
- Avoids repeated malloc/free cycles
- Retains String capacity for better performance with large inputs
- Reduces memory allocator pressure
- Especially important for shells/REPLs that run many iterations

## Process Execution & External Commands

**Running external programs with std::process::Command:**
```rust
use std::process::Command;

// Basic execution - equivalent to subprocess.run()
let output = Command::new("ls")
    .arg("-l")
    .output()
    .expect("Failed to execute command");

// Just run and get exit status
Command::new("program").args(&["arg1", "arg2"]).status().ok();

// With argument parsing from user input
let mut parts = command.split_whitespace();
let program = parts.next().unwrap();
Command::new(program).args(parts).status().ok();
```

**Why use std::process::Command vs manual PATH search:**
- `Command::new()` automatically handles PATH lookup and permission checking
- Only manually search PATH when you need to report locations (like `type` command)
- More robust than manual parsing (handles edge cases, OS differences)
- Let the standard library handle execution complexity

## Vec Operations & Indexing

**Vec indexing and slicing:**
```rust
let vec: Vec<&str> = vec!["hello", "world", "rust"];

// Direct indexing (panics if out of bounds)
let first = vec[0];        // "hello"
let second = vec[1];       // "world"

// Safe indexing (returns Option)
let first = vec.get(0);    // Some("hello")
let invalid = vec.get(10); // None

// Slicing (returns &[T])
let slice = &vec[1..3];    // ["world", "rust"]
let slice = &vec[1..];     // ["world", "rust"] (from index 1 to end)
let slice = &vec[..2];     // ["hello", "world"] (from start to index 2)

// Safe slicing
let slice = vec.get(1..3); // Some(["world", "rust"]) or None
```

**Python vs Rust comparison:**
```python
# Python - forgiving with bounds, negative indices
vec = ["hello", "world", "rust"]
vec[1:3]    # ['world', 'rust']
vec[1:10]   # ['world', 'rust'] (doesn't error)
vec[-1]     # 'rust' (negative indexing works)
```

```rust
// Rust - strict bounds checking, no negative indices
let vec = vec!["hello", "world", "rust"];
let slice = &vec[1..3];    // ["world", "rust"]
// vec[1..10] would PANIC!
// vec[-1] doesn't exist - use vec[vec.len()-1]
```

## Error Handling - Rust's Alternative to Try/Catch

**The unwrap() method:**
```rust
// unwrap() extracts value but panics on error
let result: Result<i32, &str> = Ok(42);
let value = result.unwrap(); // Gets 42

let result: Result<i32, &str> = Err("failed");
let value = result.unwrap(); // PANIC! Program crashes
```

**When to use unwrap():**
- Prototyping and examples
- When you're absolutely certain it won't fail
- Tests (where failures should be obvious)

**Better error handling patterns (Rust's try/catch equivalent):**
```rust
// match for full control (like try/except)
match Command::new("program").status() {
    Ok(status) => {
        if status.success() {
            println!("Command succeeded");
        } else {
            eprintln!("Command failed with code: {:?}", status.code());
        }
    }
    Err(e) => eprintln!("Failed to execute: {}", e),
}

// if let for simple cases
if let Err(e) = Command::new("program").status() {
    eprintln!("Error: {}", e);
}

// ? operator for propagating errors (like re-raising)
fn run_command() -> Result<(), std::io::Error> {
    let status = Command::new("program").status()?; // Returns error if fails
    Ok(())
}

// expect() for better panic messages
let value = result.expect("This should never fail because...");

// unwrap_or() for default values
let value = result.unwrap_or("default");
```

**Key differences from Python:**
- No exceptions - use `Result<T, E>` types instead
- Explicit error handling at every call site
- `match` is like comprehensive try/except blocks
- `?` operator is like re-raising exceptions up the call stack

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

## File Operations & Path Handling

**File existence checking:**
```rust
use std::path::Path;

let file_path = Path::new("directory/filename.txt");
if file_path.exists() {
    // file exists
}
```

**Path to string conversion:**
```rust
let file_path = Path::new("directory/filename.txt");

// Best for printing to users
println!("{}", file_path.display());

// Always works, handles invalid UTF-8
let path_str = file_path.to_string_lossy();

// Returns Option<&str>, None if invalid UTF-8
if let Some(path_str) = file_path.to_str() {
    println!("{}", path_str);
}
```

**File permissions checking:**
```rust
use std::fs;
use std::os::unix::fs::PermissionsExt;

if let Ok(metadata) = fs::metadata(file_path) {
    // Check if any execute bit is set (user/group/other)
    if metadata.permissions().mode() & 0o111 != 0 {
        // File is executable
    }
}
```

## Environment Variables

**Reading environment variables:**
```rust
use std::env;

// Returns Result<String, VarError>
match env::var("PATH") {
    Ok(val) => println!("PATH = {}", val),
    Err(_) => println!("PATH not set"),
}

// With default value
let path = env::var("PATH").unwrap_or("default_path".to_string());

// Panic if not found (use carefully)
let path = env::var("PATH").expect("PATH must be set");
```

## Function Signatures & Lifetimes

**Lifetime elision in function signatures:**
```rust
// This is legal due to lifetime elision
pub fn check_type(command: &str) -> &str {
    "builtin"  // can return string literals
}

// Compiler automatically infers this as:
// pub fn check_type<'a>(command: &'a str) -> &'a str

// The returned &str must live at least as long as input &str
```

## Collections & Iterators

**Working with split() and collect():**
```rust
let path = env::var("PATH").expect("PATH must be set");

// split() returns iterator, collect() creates Vec
let paths: Vec<&str> = path.split(":").collect();

// Or convert Vec to slice reference
let paths_vec: Vec<&str> = path.split(":").collect();
let paths: &[&str] = &paths_vec;

// Often better to iterate directly without collecting
for path_item in path.split(":") {
    // use path_item directly
}
```

**Array type annotations:**
```rust
// For function parameters expecting arrays of paths
pub fn check_type(command: &str, paths: &[&str]) {
    for path in paths {
        let path_obj = Path::new(path);
        // use path_obj
    }
}

// Most flexible with generics
pub fn check_type<P: AsRef<Path>>(command: &str, paths: &[P]) {
    for path in paths {
        let path_obj = path.as_ref();  // converts to &Path
    }
}
```

## Debugging Techniques

**Debug output without interfering with tests:**
```rust
// Use eprintln! for debug output (goes to stderr)
eprintln!("DEBUG: Full PATH = {}", path);

// println! goes to stdout and interferes with test expectations
println!("{} is {}", cmd, file_path_str);  // This is for actual output
```

## Important Concepts

1. **Ownership & Borrowing:** `&` creates a reference/borrow
2. **Iterator vs Collection:** `split()` returns iterator, need `.collect()` or `.count()` for length
3. **Option Type:** Rust's way of handling nullable values safely
4. **Pattern Matching:** `if let Some(x) = option` extracts values safely
5. **Compile-time Safety:** Many errors caught at compile time vs runtime
6. **Memory Management:** No garbage collector, but automatic memory management through ownership
7. **Lifetime Elision:** Compiler automatically infers lifetimes in simple cases like single input reference
8. **Dead Code Analysis:** Rust warns about unused functions/constants even if they contain used code
9. **File Permissions:** PATH lookup requires execute permissions (`0o111`), not just file existence
10. **Split Ordering:** `string.split()` preserves left-to-right order from original string
11. **Result Type:** Rust's way of handling errors without exceptions - must be explicitly handled
12. **String Buffer Reuse:** Keep String allocations outside loops and use `.clear()` for efficiency
13. **Vec Bounds Checking:** Rust panics on out-of-bounds access, use `.get()` for safe access
14. **Process Execution:** `std::process::Command` handles PATH lookup automatically
15. **Error Propagation:** `?` operator passes errors up the call stack like re-raising exceptions

## Best Practices

- Use `Option` and pattern matching instead of unwrap when possible
- Prefer `&str` for constants and function parameters
- Use `String` when you need to own/modify the string
- Use `strip_prefix()` instead of manual slicing
- Handle errors with `Result` type rather than panicking
- Let Rust infer types when obvious, specify when clarity helps
- Use `Path::new()` for file operations instead of string concatenation
- Check execute permissions with `mode() & 0o111 != 0` for PATH lookup, not just existence
- Use `eprintln!` for debug output to avoid interfering with stdout in tests
- Use `display()` method for user-friendly path printing
- Prefer `Vec<&str>` over `&[&str]` when collecting from `split()`
- Use `expect()` with descriptive messages instead of `unwrap()` for required environment variables
- Avoid duplicate constants across modules - keep them where they're used
- Keep String allocations outside loops and use `.clear()` for performance
- Use `std::process::Command` instead of manual PATH searching for execution
- Prefer `.get()` over direct indexing for Vec when bounds might be invalid
- Use `match` for comprehensive error handling instead of ignoring with `.ok()`
- Use `?` operator for clean error propagation in functions returning Result
- Use `unwrap()` only for prototyping, tests, or when absolutely certain of success
- Prefer `if let Err(e) = result` for simple error handling patterns