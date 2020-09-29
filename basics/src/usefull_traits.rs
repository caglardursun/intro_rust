use std::fmt::Display;
use std::fmt;

#[derive(Debug)]
pub struct TL(f32);

impl Display for TL
{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        let r = self.0 as f32;

        if r<0. {
            return write!(f,"-{:.2} TL",-r);
        }

        write!(f,"{:.2} TL",r)
    }
}

impl Clone for TL{
    fn clone(&self) -> TL{
        TL(self.0)
    }
}

impl Copy for TL {  //you don't have to write anything !
                    //If you remove the implimentation 
                    //assignmet converted into move, not copy !
                    //println!("{}",tl.to_string()); willl fucked up
}


pub fn test_it()
{
    let tl = TL(10.);
    // let b = tl.clone();
    let b = tl; // It won't work if you don't define the Copy 
    println!("{}",tl.to_string());
    println!("{}",b.to_string());

}