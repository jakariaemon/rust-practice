## Notes
Compiling and Running Are Separate Steps 
Before running a Rust program, you must compile it using the Rust compiler by entering the rustc command. 
```
rustc main.rs
```
After compiling successfully, Rust outputs a binary executable.main.exe on Windows, but main on all other platforms. 
```
./main # or .\main.exe on Windows
```
Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed. If you give someone a .rb, .py, or .js file, they need to have a Ruby, Python, or JavaScript implementation installed (respectively). But in those languages, you only need one command to compile and run your program. Everything is a trade-off in language design. 

## Cargo is Rust’s build system and package manager.  
## Structure 
- Cargo.toml is the manifest file for Rust. It’s where you keep metadata for your project, as well as dependencies. (Tom’s Obvious, Minimal Language)
- src/main.rs is where we’ll write our application code.

## Building and Running a Cargo Project 
```
cargo build
./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
``` 
## we can also use cargo run to compile the code and then run the resultant executable all in one command 
```
cargo run
```
### Cargo also provides a command called cargo check. This command quickly checks your code to make sure it compiles but doesn’t produce an executable

### When your project is finally ready for release, you can use cargo build --release to compile it
