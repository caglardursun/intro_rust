use std::collections::HashMap;
use std::env::args;


/*
    SOmething like that ...

    Result<T,E> {
        Ok(T)
        Error(E)
    }

    Option<T>{
        Some(T),
        None
    }

*/

pub fn test_it()
{
    let mut hm = HashMap::new();
    hm.insert(1, "One");
    hm.insert(2, "Two");
    hm.insert(3, "Three");
    hm.insert(5, "Five");

    let result = hm.get(&3);
    println!("{:?}",result);//returns Some("Three")

    match hm.get(&4) {
        Some(value) => println!("{}",value),
        None => println!("Nothing found")
    };

    match hm.get(&3) {
        Some(value) => println!("{}",value),
        None => println!("Nothing found")
    };

    //same idea 
    let result_correct = hm.get(&4).unwrap_or(&"If 4 does not exists (which is btw.) this will return");
    println!("{}",result_correct);

    let result_sure = hm.get(&3).unwrap();
    println!("We already know that 3 exists {}",result_sure);

    //Here we are Options return ... 

    match "3t".parse::<f32>(){
        Ok(value)=> println!("Value is {}",value),
        Err(e)=>println!("Error occured {}",e)
    }


    match "3".parse::<f32>(){
        Ok(value)=> println!("Value is {}",value),
        Err(e)=>println!("Error occured {}",e)
    }

    //cargo run -- h i p d
    match getArg(3){
        Ok(value)=>println!("Ok guyes : {}",value),
        Err(e)=>println!("Now fuck off {}",e)
    }
    
}

pub fn getArg(n:usize) ->Result<String,String> 
{
    for (index,a) in args().enumerate() 
    {
        if index == n 
        {
                return Ok(a);
        }   
    }
    Err("Not Enough Args".to_string())
}