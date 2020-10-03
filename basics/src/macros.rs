macro_rules! a_macro {
    ()=> (
        println!("It's bullshit");
    )
}

macro_rules! x_and_y {
    (x=> $e:expr) => (println!("X: {}",$e));
    (y=> $e:expr) => (println!("Y: {}",$e));
}

macro_rules! build_fn {
    ($func_name:ident) => (
        fn $func_name(){
            println!("You called {:?}()", stringify!($func_name));
        }
    )
}

macro_rules! print_ex {
    ($e:expr) => {
        println!("{:?} = {:?}",stringify!($e),$e);
    };
}

macro_rules! exame {
    ($l:expr; and $r:expr ) => (
        println!("{:?} and {:?} is {:?}",
            stringify!($l),
            stringify!($r),
            $l && $r
        );
    );

    ($l:expr; or $r:expr ) => (
        println!("{:?} and {:?} is {:?}",
            stringify!($l),
            stringify!($r),
            $l || $r
        );
    );
}


// macro_rules! compr {
//     ($id1:ident | $id2:ident  <- [$start:expr;$end:expr], $cond : expr) => {

//         let mut vec = Vec::new(); //Sıçtı !!!
        
//         for num in $start..$end+1 {
//             if $cond(num) {
//                 vec.push(num);
//             }
//         }
//         vec
        
//     };
// }

// fn even(x:i32) -> bool{
//     x%2 == 0
// }
// fn odd(x:i32)->bool{
//     x%2 != 0
// }

pub fn test_it(){
    a_macro!();
    x_and_y!(x => 10);
    x_and_y!(y => 10 + 30);

    //let my_function = || println!("Shit");

    build_fn!(my_function);
    my_function();
    my_function();
    my_function();
    //entire expression block inside of '{ }' will be parameter
    print_ex!({
        let y = 20;
        let z = 20;
        z + y + 10 * 3 * 100
    });
    exame!(1==1; and 2==1 + 1);

    // let evens = compr![x | x <- [1;10], even];
    // println!("{:?}",evens);

    // let odds = compr![y | y <- [1;10], odd];
    // println!("{:?}",odds);
}