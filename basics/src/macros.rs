macro_rules! a_macro {
    ()=> (
        println!("It's bullshit");
    )
}

macro_rules! x_and_y {
    (x=> $e:expr) => (println!("X: {}",$e));
    (y=> $e:expr) => (println!("Y: {}",$e));
}


pub fn test_it(){
    a_macro!();
    x_and_y!(x => 10);
    x_and_y!(y => 10 + 30);
}