use std::thread;
use std::sync::mpsc; //multiple producer single consumer  
use std::time::Duration;

fn basic_usage(){

    let mut c = vec![];

    for i in 0..10{
        c.push(
            //move closure Capture a closure's environment by value.

            //rust thread doesn't guarantee that jop order
            thread::spawn(move || {
                println!("Thread number {}",i);
            })
        );
    }

    for j in 0..10 {
        
         println!("Main thread {}",j);        
         
        
    }

}

pub fn joining(){
    let mut c = vec![];

    for i in 0..10{
        c.push(
            //move closure Capture a closure's environment by value.

            //rust thread doesn't guarantee that jop order
            thread::spawn(move || {
                println!("Thread number {}",i);
            })
        );
    }

    for j in c {
         
         j.join();
        
    }

}

pub fn handle_join(){
    
    let mut v = vec![2,4,6,8];

    //move closure is often used when threads are involved.
    let handle = thread::spawn(move || {
        println!("Vector {:?}",v);
    });

    handle.join().unwrap();
    // v is no longer awaible
    //println!("{:?}",v);
}


pub fn channels()
{
    let (tx,rx) = mpsc::channel();
    thread::spawn(move || {tx.send(42).unwrap();});
    //recv -> Blocking tryrecv -> Nonblockig
    println!("got {}",rx.recv().unwrap());
}


const NUM_TIMERS: usize = 24;

fn timer(d:usize, tx:mpsc::Sender<usize>){
    thread::spawn(move || 
    {
        println!("{} setting timer ...",d);        
        thread::sleep(Duration::from_secs(d as u64));
        println!("{} : sent !",d);
        tx.send(d).unwrap();
    });
}

fn timer_caller(){
    let (tx,rx) = mpsc::channel();
    for i in 0..NUM_TIMERS {
        timer(i, tx.clone());
    }

    for v in rx.iter().take(NUM_TIMERS) {
        println!("{} : recived",v);
    }
}
pub fn test_it(){

    basic_usage();
    joining();
    handle_join();
    channels();
    timer_caller();
}