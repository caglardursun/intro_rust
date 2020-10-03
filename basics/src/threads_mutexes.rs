use std::thread;

pub fn test_it(){
    let mut c = vec![];

    for i in 0..10{
        c.push(
            //move closure 
            thread::spawn(move || {
                println!("Thread number {}",i);
            })
        );
    }

    for j in 0..10 {
        println!("Main thread {}",j)
    }

}