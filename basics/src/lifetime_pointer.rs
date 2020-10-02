#[derive(Debug,PartialEq)]
pub struct TL(f32);

//error *tl == TL(3) &TL
//                      ^ expected named lifetime parameter
// fn money_pointer(i:f32) -> &TL
// {
//     tl is going to die as soon as function try to retun
//     let tl = TL(i);
//     &tl
// }

// fn money_pointer<'a>(i:f32) -> &'a TL
// {
//     let tl = TL(i);
//     &tl
// }

pub fn on_money(a:f32,b:f32) -> TL {
    let mut tl = TL(a);
    let r;
    {
        r = &tl; 
        tl.0 +=2.;
    }
    
    let res = TL(tl.0+b);
    res
}

pub fn test_it()
{
    // let tl = money_pointer(3.);
    // if *tl == TL(3.){
    //     println!("*tl == TL(3)");
    // }
    let tl = on_money(3., 4.);
    println!("{:?}",tl);
}