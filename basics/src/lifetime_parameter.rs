//little bit hacking ...
//'a life time parameters
pub fn trim_left<'a>(s:&'a str) ->&'a str {
    
    for (i,c) in s.char_indices() 
    {
        if c == ' ' {
            continue;
        }
        return s.get(i..s.len()).unwrap();
    }

    ""
}


fn money_pointer(i32) 


pub fn test_it(){
    let r = trim_left("    Some of them want to use you");
    println!("{}",r);
}