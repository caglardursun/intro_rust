///
/// Stack
///- Extremely Fast
///- Values must have Fixed Sizes
///- Always puts data in on Top
///- data pushed down as new data comes in
///
///
/// Heap
///- Less Organized and Slower
///- Accepts Dynamicly Sized Data or Data that can Grow
///- Returns a Pointer which goes on the stack
///- Pointer points to where the data is on the Heap
///
///
/// Onership Rules:
///  1) Each value has a variable which is its owner.
///  2) There can only be one owner at any given time.
///  3) When the owner goes out of scope, the value will be dropped out of memory.
///
///
/// Barrowing Rules:
///  1) Allowed infinite borrows for readonly access.
///  2) Readonly borrows make the original data immutable for their duration.
///  3) Only allowed to pass one borrow at a time for write access/mutability.
///
/// Rust Stack | Copy Types
///  bool
///  character
///  numbers
///  slices
///  fix sized arrays
///  tuples containing primitives
///  function pointers
///
/// 
/// 

///we return the ownership ... now v does not have a ownership
fn re(v:Vec<i32>) -> Vec<i32>{
    println!("{}",v[10]+v[11]);
    v//return the ownership
}

///Barrowing
fn barrow1(v:&Vec<i32>){
    println!("{}",(*v)[10]+(*v)[11]);
}

fn barrow2(v:&Vec<i32>){
    println!("{}",v[10]+v[11]);
}
//ownership still left the fnc caller
fn copy(a:i32, b:i32)
{
    println!("a is {} b is {} a+b = {}",a,b,a+b);
    //But If you were return something like 
    //a+b then ownership goes away 
}

pub fn barrowing()
{
    println!("Barrowing");
    let mut v = Vec::new();

    for elem in 5..100 {
        v.push(elem);
    }
    let y = re(v);
    //Now v belongs to the function now
    //println!("{}",v[0]);
    //println!("{}",v[0]);
    //|                   ^ value borrowed here after move
    //println!("{}",&v[5]);
    

    barrow1(&y);
    barrow2(&y);
    println!("{:?}",y);

    println!("\n");
}

pub fn ownership(){
    println!("Ownership");

    
    //x will fucked up coz ownership goes to y 
    //println!("{}",x);    
    
    let mut v = Vec::new();

    for elem in 5..100 {
        v.push(elem);
    }

    let t = re(v);
    //ownership goes to t not v so 
    //println!("{:?}",v); will not work
    println!("{:?}",t);

    let a =5;
    let b=10;
    //can run coz copying try to return a+b directlly panic error will occur
    copy(a, b);
    println!("{}",a);
    println!("{}",b);

    println!("\n");
}
///Because s is created inside dangle, when the code of dangle is finished, 
/// s will be deallocated. But we tried to return a reference to it. 
/// That means this reference would be pointing to an invalid String. 
/// That’s no good! Rust won’t let us do this.
// fn dangle()-> &String {
//     let s = String::from("Sh.t");
//     &s
// }

pub fn test_it()
{
    ownership();
    barrowing();
    

}