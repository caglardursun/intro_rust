#[derive(Debug)]
enum List
{
    Cons(i32,Box<List>),
    End,
}
use List::Cons;
use List::End;

fn print_list(){
    let l = Cons(1,Box::new(Cons(2,Box::new(Cons(3,Box::new(End)) ))));

    println!("{:#?}",l);
    print!("\n");
}

fn print_box_val()
{
    let y = 4;
    let x = &y;
    let z = Box::new(y);
    if *x == *z {
        println!("True");
    }

    print!("\n");
}

fn run<F>(f:F) 
where F: Fn() {
    f();
}

fn add3<F>(f:F) -> i32
where F: Fn(i32) -> i32{
    f(3)
}

fn print_closure2()
{
    let p = ||  println!("Hello from run fnc");
    run(p);

    let x = |i| i*10;
    println!("3 * 10 = {}",add3(x));    
}





fn print_closure(){
    //f (i:32) -> i32 { i+1 }
    //written as ...
    let f = |i| i+1;
    let g = |i:i32| -> i32 {i+1};
    //parameterless fnc.
    let p = || println!("This is the closure called");

    println!("f(4) : {}",f(4));
    println!("g(4) : {}",g(4));
    p();
}


fn create() -> Box<Fn()> {
    //move all of the values inside of the closure will die
    Box::new(move || println!("This is a closure box"))
}

fn create_box(){
    let x = create();
    x();
}


/*
    v.iter() -> next()

    trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }
*/

fn vector_stuff(){
     let v = vec![0,2,4,6,3];
    //let v : Vec<i32> = Vec::new();
    println!("v  {}",v.iter().any(|&x| x!=2));


    let is_even = |n:u32| -> bool {n%2 == 0};
    let mut c = 0;

    for n in 0..{
        let x = n * n;
        if x>=1000 
        {
            break;
        }else if is_even(x) {
            c +=x;
        }

    }
    print!("c : {}",c);

    

    print!("\n");
}

fn map_fnc(){

    let is_even = |i| -> bool{ i%2==0 };

    let s:u32 = 
    (0 ..).map(|n| n*n)
        .take_while(|&n| n<10000)
        .filter(|&n| is_even(n))
        .fold(0, |s,i| {s+i as u32});

    println!("{}",s);

}

pub fn test_it() 
{
   print_list();
   print_box_val();
   print_closure();
   print_closure2();
   create_box();
   vector_stuff();
   map_fnc();
}