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
