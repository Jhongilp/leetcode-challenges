fn main() {
    println!("Hello, world!");
    let candies = vec![2, 3, 5, 1, 3];
    let extra = 3;
    let results = kids_with_candies(candies, extra);
    println!("RESULTS: {:?}", results);
}

// fn get_biggest(candies: &Vec<i32>) -> i32 {
//     let mut max = 0;
//     for c in candies {
//         if c > &max {
//             max = c.clone();
//         }
//     }
//     max
// }

fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    // let biggest = get_biggest(&candies);
    let biggest = match candies.iter().max() {
        Some(n) => n,
        None => &candies[0],
    };

    return candies
        .iter()
        .map(|c| c + extra_candies >= *biggest)
        .collect();
}
