pub fn tuple_types()
{
    println!("\n");    

    let t = (1,'a',false);
    let f = (1,'a',(1,'a',false));
    println!("{} {} {}",t.0,t.1,t.2);
    //# means formatting & indentation here 
    println!("{:#?}",f);

    println!("\n");
}

pub fn array_types()
{
    let xs : [i32;5] = [1,2,3,4,0];
    println!("{:?}",xs);
    println!("First Element : {}",xs[0]);
    println!("Lenght : {}",xs.len());
    println!("Size of value : {}",std::mem::size_of_val(&xs));
    let slices = &xs[1..4];
    println!("{:?}",slices);
    println!("\n");
}

pub fn string_types()
{    
    let s = "Some &str";
    println!("{}",s);


    let s : String = String::from("Çağlar Dursun");
    println!("{}",s);
    //Another string object
    let ss = "Some &str converted into String".to_string();
    println!("{}",ss);

    //like in python x[:5] 
    let slice_of = &ss[0..5];
    println!("Slice of string : {}",slice_of);

    println!("Name contains Çağlar : {}",s.contains("Çağlar"));

    let age = String::from(" 38 years old");
    let result = s + &age;
    println!("{}",result);

    //raw string 
    let json = r#"        
        {
            name:"Çağlar",
            surname:"Dursun",
            age: 38
        }
    
    "#;

    println!("{}",json);

    println!("\n");
}

pub fn primitives()
{
    //Mut means can be changable
    //like a variable otherwise 
    //It's a read only variable
    let mut x:i8 = 1;
    let mut y:i16 = 2;
    let mut z:i32 = 3;
    x += 2;
    y += 2;
    z *=2;
    println!("{} {} {}",x,y,z);
    let a:f32 = 4.0;

    println!("i8 MAX {}",i8::MAX);
    println!("i16 MAX {}",i16::MAX);
    println!("i32 MAX {}",i32::MAX);
    println!("i64 MAX {}",i64::MAX);
    println!("f32 MAX {}",f32::MAX);    
    println!("usize MIN {} MAX {}" ,usize::MIN,usize::MAX);
    println!("a is {}",a);
}