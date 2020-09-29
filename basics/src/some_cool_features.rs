fn greet(people: &[&str]) {
    match people {
        [] => println!("Yayk, there's no one here :("),
        [only_one] => println!("Selam, {}! Tek sap gelmişsin.", only_one),
        [first, second] => println!(
            "Selam, {} ve {}",
            first, second
        ),
        [first,second,third] => println!("Oha teker teker gelin aq. {}, {} ve {} bu ne la",first,second,third),
        _ => println!("Hey everyone, we seem to be {} here today.", people.len()),
    }
}

fn label_loop()
{
    let v = vec![2,4,6,8,0,12,14,16];
    'outer: for i in 0..10{
        for n in 0..10 {
            if n== 11 {
                break 'outer; //Similar to goto statement in c
            }
        }
    }
    let mut count = 1;
    loop{
        println!("Infinite loop {} ",count);
        //no count++ or ++count
        count +=1;
    }
}

pub fn test_it(){

    greet(&[]);
    // Yayk, there's no one here :(
    greet(&["Chaagalar"]);
    // Hey, there Chaagalar! You seem to be alone.
    greet(&["Çağlar", "Seda"]);
    // output: Hello, Joan and Hugh. Nice to see you are at least 2!
    greet(&["Çağlar", "Seda", "Ada"]);

    label_loop();

}