fn main() {
    let mut counter = 0;
    let num = loop {
        if counter == 3 {
            break counter * 8;
        }
        counter += 1;
    };

    loop {
        println!("happy");
        break;
    }
    println!("{}", num);
    inside_loop();
    check_for();
}

fn check_for(){
    

    for num in (1..6){
       println!("{num}");
    }
}

fn inside_loop() {
    let mut count = 0;
   'count_loop: loop {
        count += 1;
        loop {
            if count == 1 {
                println!("{}", count);
                break;
            }
            if count == 2 {
                println!("{count}");
                break 'count_loop;
            }
            count += 1;
            println!("hhh");
        }
    }
}
