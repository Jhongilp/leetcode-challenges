fn main() {
    println!("Hello, world!");
    println!("result: {}", arrange_coins(1000000000));
}

fn arrange_coins(n: i32) -> i32 {
    let mut coins_left = n;
    let mut r = 0;
    let mut i = 0;
    loop {
        if i > coins_left {
            break;
        }
        coins_left -= i;
        r = i;
        i = i + 1;
    }
    r
}
