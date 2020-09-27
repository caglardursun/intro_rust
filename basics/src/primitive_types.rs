pub fn tuple_types(){

    let t = (1,'a',false);

    let f = (1,'a',(1,'a',false));

    //# means indentation here 
    println!("{} {} {}",t.0,t.1,t.2);
    println!("{:#?}",f);

}

pub fn string_types(){

    let s = "Some &str";
    println!("{}",s);
    let s : String = String::from("Çağlar Dursun");
    println!("{}",s);
}