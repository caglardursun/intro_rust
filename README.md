# intro_rust Introduction to Rust Language

My personal experience about Rust Language. If you don't want to debug, just go to the directory of the apps. Open up the terminal and run `cargo run` command. If you wanna debug the application. Make sure vscode code launch configuration is like that 

```
    version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug Basics",
            "program": "${workspaceFolder}/basics/target/debug/basics",
            "args": [],
            "cwd": "${workspaceFolder}",
            "preLaunchTask":"Cargo build"
        }
    ]
```

### primitive_types : 
Some of the tryouts about basic data types 
like `string tuple, i8,i32, usize` etc.

### ownership_barrowing: 
The main difference of Rust from the other languages. The performance comes from the idea of ownership & barrowing. Behind the `Safe Language` idea.

### defining_traits:
Descripbes how to use traits on stuct and enums. 

### generic_stuff: 
Describes how to use templates and introduction to metadata programming on Rust

### execute_commands : 
How to execute process and pipe them their outputs

### generic_iterators : 
Iterator templates 

### result_and_option: 
Build in Structs which uses for many place and build in std library. `Option` and `Result` 

### some_cool_features : 
These features comes with Rust 2018. 
