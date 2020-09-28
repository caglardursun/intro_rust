//This is rediculus ... I'm gonna come back and write a code which explain properlly. This is a shit !
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

    println!("\n");
}
//we return the ownership ... now v does not have a ownership
fn re(v:Vec<i32>) -> Vec<i32>{
    println!("{}",v[10]+v[11]);
    v//return the ownership
}

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

}
