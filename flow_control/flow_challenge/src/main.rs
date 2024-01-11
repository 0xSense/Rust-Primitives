use std::ops::Index;

fn main() {
    let number = [1, 9 ,-2, 0, 23,20, -7 , 13, 37,20,56, -18, 20, 3];
    let mut max: i32;
    let mut min: i32;
    let mut mean: f64;

    // challenge code goes here 

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests Passed!");   

    for item in number {
        println!("This is the current number {}", item);
    }
}
