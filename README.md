# Introduction to Rust Language

My personal experience about Rust Language. If you don't want to debug, just go to the directory of the apps. Open up the terminal and run `cargo run` command. If you wanna debug the application, make sure vscode code launch configuration is like that 

```json
{

    "version": "0.2.0",
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
}
```

I just use it for my linux distro btw. Windows C++ debugging just works fine. But don't expect to much. Coz no debug helper, no natvis file. So If you wanna debug the complex datatypes (I'm not talking about the vector data type btw.), good luck !

If you wanna check the library, just go to the directory run `cargo test` command.


### primitive_types : 
Some of the definitions on basic data types like `string tuple, i8,i32, usize` etc.

```rust
    println!("i8 MAX {}",i8::MAX);
    println!("i16 MAX {}",i16::MAX);
    println!("i32 MAX {}",i32::MAX);
    println!("i64 MAX {}",i64::MAX);
    println!("f32 MAX {}",f32::MAX);    
    println!("usize MIN {} MAX {}" ,usize::MIN,usize::MAX);

    let s : String = String::from("Çağlar Dursun");
    println!("{}",s);
    //Another string object
    let ss = "Some &str converted into String".to_string();
    println!("{}",ss);


    let xs : [i32;5] = [1,2,3,4,0];
    println!("{:?}",xs);
    println!("First Element : {}",xs[0]);
    println!("Lenght : {}",xs.len());
    println!("Size of value : {}",std::mem::size_of_val(&xs));
    let slices = &xs[1..4];

    //tuples
    let t = (1,'a',false);
    let f = (1,'a',(1,'a',false));
    println!("{} {} {}",t.0,t.1,t.2);

```

### ownership_barrowing: 
The main difference of Rust from the other languages. The performance comes from the idea of ownership & barrowing. Behind the `Safe Language` idea.

### defining_traits:
Describes how to use traits on `stuct` and `enums`. 

### generic_stuff: 
Describes how to use templates and introduction to metadata programming on Rust

### execute_commands : 
How to execute process and pipe them their outputs

```rust
//just run on linux/unix type OS's which has bash

let _c = Command::new("ls")
    .arg("-l")
    .arg(path)
    .output()
    .expect("ls is not usable");

```

### generic_iterators : 
Describes how to define iterator templates 

```rust
use std::ops::AddAssign;
use std::cmp::PartialOrd;

pub struct Stepper<T>
{
    curr:T,
    step:T,
    stop:T,
}
```

### result_and_option : 
Build in Structs which uses for many place and build in std library. `Option` and `Result` 

```rust
    Result<T> {
        Ok(T),
        Error(E),
    }

    Option<T>{
        Some(T),
        None,
    }
```

### box_and_closure : 
Defines how to use `Box` smart pointer and closure functions 

```rust
    enum List
    {
        Cons(i32,Box<List>),
        End,
    }
    let l = Cons(1,Box::new(Cons(2,Box::new(Cons(3,Box::new(End)) ))));


    let f = |i| i+1;
    let g = |i:i32| -> i32 {i+1};

```

### macros : 
How to define your own macros for the rust


```rust 
macro_rules! x_and_y {
    (x=> $e:expr) => (println!("X: {}",$e));
    (y=> $e:expr) => (println!("Y: {}",$e));
}

macro_rules! build_fn {
    ($func_name:ident) => (
        fn $func_name(){
            println!("You called {:?}()", stringify!($func_name));
        }
    )
}

```
### threads_mutexes :
Defines how to create a thread and mutex.

```rust 
use std::thread;
use std::sync::mpsc;
//Channels,thread move


let mut v = vec![2,4,6,8];

//move closure is often used when threads are involved.
let handle = thread::spawn(move || {
        println!("Vector {:?}",v);
});

handle.join().unwrap();


let (tx,rx) = mpsc::channel();
thread::spawn(move || {tx.send(42).unwrap();});
//recv -> Blocking tryrecv -> Nonblockig
println!("got {}",rx.recv().unwrap());



```
### some_cool_features : 
These features comes with Rust 2018. 

## test library project :
This library project is used for to check how to give a library referance to external lib project. And document generating documentation using `cargo doc` command. It's kind a cool actually. 
