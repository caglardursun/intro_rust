use std::collections::HashMap;
/*
    SOmething like that ...

    Result<T> {
        Ok(T)
        Error(T)
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

}