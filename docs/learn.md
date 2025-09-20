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

## Path Operations & Directory Management

**Getting current working directory:**
```rust
use std::env;

// Returns Result<PathBuf, io::Error>
let current_dir = env::current_dir()?;
```

**Changing directory:**
```rust
use std::env;

// Basic usage - returns Result<(), io::Error>
env::set_current_dir("/path/to/directory")?;

// With error handling
match env::set_current_dir("/home/user") {
    Ok(()) => println!("Directory changed successfully"),
    Err(e) => println!("Failed to change directory: {}", e),
}
```

**Important limitations:**
- `set_current_dir()` does NOT handle shell expansions like `~` or `$HOME`
- You must expand these manually before calling the function
- It only accepts literal paths, not shell patterns

**PathBuf vs Path (Ownership patterns):**
```rust
// PathBuf - owned (like String)
let mut path_buf = PathBuf::from("/home/user");
path_buf.push("documents");  // Can modify

// Path - borrowed (like &str)  
let path: &Path = Path::new("/home/user/file.txt");
// path.push("something"); // Won't compile - can't modify

// PathBuf derefs to &Path automatically
let path_buf = PathBuf::from("/home");
let path_ref: &Path = &path_buf;  // Automatic conversion
```

**Key relationship:** PathBuf:Path :: String:&str

## Variable Declaration & Scope

**const vs let vs static:**
```rust
// const - compile-time constants, can be anywhere
const MAX_SIZE: usize = 100;  // Known at compile time

// let - local variables, function scope only
fn main() {
    let x = 5;              // Immutable
    let mut y = 10;         // Mutable
}

// static - global variables, program lifetime
static GLOBAL_VAR: usize = 42;  // Immutable global
```

**For mutable global state:**
```rust
use std::sync::Mutex;

// Prefer thread-safe wrappers over static mut
static GLOBAL_COUNTER: Mutex<i32> = Mutex::new(0);

// Usage
let mut counter = GLOBAL_COUNTER.lock().unwrap();
*counter += 1;
```

**Scope rules:**
- `const`: Can be declared anywhere, known at compile time
- `let`: Local scope only (inside functions/blocks)
- `static`: Global scope only (module level)

## Advanced Error Handling & Closures

**unwrap_or vs unwrap_or_else:**
```rust
// unwrap_or - always evaluates fallback
let home = env::var("HOME").unwrap_or("/".to_string());

// unwrap_or_else - only calls closure if needed (lazy)
let home = env::var("HOME").unwrap_or_else(|_| "/".to_string());
```

**When to use each:**
- `unwrap_or`: Simple default values
- `unwrap_or_else`: Expensive operations you only want to run if needed

**Closure syntax breakdown:**
```rust
|_| "/".to_string()
// |_|  = function that ignores its parameter
//      "/".to_string() = what it returns

// Pattern: |parameters| return_value
|| 42                    // No parameters
|x| x + 1                // One parameter (used)
|_| "default"            // One parameter (ignored)
|x, y| x + y             // Two parameters
```

**Python comparison:**
```rust
// Rust closure
|_| "/".to_string()

// Python lambda  
lambda _: "/"
```

## String Manipulation & Safety

**Controlled replacements:**
```rust
// replacen - replace only N occurrences (safer)
path.replacen("~", &home, 1)  // Replace only first occurrence

// replace - replace ALL occurrences
path.replace("~", &home)      // Replace every ~
```

**Why this matters:**
- Path: `~/my~folder~with~tildes/file`
- With `replace`: `/home/user/my/home/userfolder/home/userwith/home/usertildes/file` (wrong)
- With `replacen`: `/home/user/my~folder~with~tildes/file` (correct)

**Borrowing in method calls:**
```rust
let home = String::from("/home/user");

path.replacen("~", &home, 1)  // Borrow (let method read it)
path.replacen("~", home, 1)   // Move (give ownership away)
```

**Simple rule:** Use `&` when you want to "lend" data, not "give it away"

## Shell vs OS Distinctions

**Shell features that Rust doesn't handle automatically:**
- Tilde expansion (`~` → `/home/user`)
- Environment variable expansion (`$HOME` → `/home/user`)
- Glob patterns (`*.txt` → list of files)
- Command substitution (`` `date` `` → actual date)

**Key insight:** When building a shell, YOU implement these features before making OS calls.

```rust
// Manual tilde expansion example
fn expand_home(path: &str) -> String {
    if path == "~" || path.starts_with("~/") {
        let home = env::var("HOME").unwrap_or("/".to_string());
        path.replacen("~", &home, 1)
    } else {
        path.to_string()
    }
}
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
16. **PathBuf vs Path:** PathBuf is owned (like String), Path is borrowed (like &str)
17. **Variable Scope:** `const` = anywhere, `let` = local only, `static` = global only
18. **Directory Operations:** `env::current_dir()` gets CWD, `env::set_current_dir()` changes it
19. **Shell vs OS:** Rust doesn't handle shell expansions (~, $HOME) - you must implement them
20. **Closure Syntax:** `|parameters| return_value` - anonymous functions, `|_|` ignores parameter
21. **Lazy Evaluation:** `unwrap_or_else` only calls closure when needed, `unwrap_or` always evaluates
22. **Controlled Replacement:** `replacen()` limits replacements, `replace()` changes all occurrences
23. **Borrowing vs Moving:** Use `&variable` to lend data, `variable` to transfer ownership

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
- Use `PathBuf` when you need to build/modify paths, `&Path` for function parameters
- Prefer `let mut` for local mutable variables over global `static mut`
- Use `unwrap_or()` for simple defaults, `unwrap_or_else()` for expensive operations
- Implement shell expansions manually before calling OS functions like `set_current_dir()`
- Use `replacen()` instead of `replace()` when you only want to change specific occurrences
- Always use `&` when passing owned data to methods that only need to read it
- Understand the difference between shell features (expansions) and OS features (system calls)