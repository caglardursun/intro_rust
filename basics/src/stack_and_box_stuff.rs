
#[derive(Debug,PartialEq)]
pub enum LinkedList<T>{
    Tail,
    Head(T, Box<LinkedList<T>>),
}

use self::LinkedList::*;

impl<T> LinkedList<T>{
    pub fn empty() ->Self{
        Tail,
    }
    pub fn new(t:T)->Self {
        Head(t,Box::new(Tail))
    }
    pub fn push(self,t:T)->Self {
        Head(t,Box::new(self))
    }
}

pub fn test_it()
{

    let l = LinkedList::new(3);
    l = l.push(4);
    println!("{}", l == Head(4,Box::new(Head(3,Box::new(Tail)))));

}
