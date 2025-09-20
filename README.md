# Rust Implementation of the shell

## Set up and run
### Install Rust

Install Rust

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Add it to your path

    PATH=$PATH:$HOME/.cargo/bin

### Compile and run the shell

clone this repository,

```shell
git clone https://github.com/truonghm/shell-rust.git
```

and run

```shell
cd shell-rust && cargo run
```

## Roadmap
- [x] 
- [x] REPL  
- [x] Builtin: echo, type, exit  
- [x] Running external programs (using PATH)  
- [x] pwd, cd  
- [ ] Quoting: single quotes, double quotes, backslash  
- [ ] Redirection: stdout, stderr  
- [ ] Autocompletion: builtin, with arguments, etc.  
- [ ] Pipelines  
- [ ] History: listing history, up/down navigation, execution  
- [ ] Persisting history: read/write from file, append to file, modify history on exit  
- [ ] Parse tree  