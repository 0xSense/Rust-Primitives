fn main() {
//    let mut count = 0;

//     let result = loop {
//         if count == 10 {
//             break count * 10;
//         }

//         count += 1;
//         println!("count is {}", count);
//     };

//      println!("After the loop!");
//      println!("result is {}", result);

    // while loop
    // let mut count = 0;
    // let letters = ['a', 'b', 'c'];

    // while count < letters.len() {
    //     println!("letter is {}", letters);
    //     count += 1;
    // }

    // for loop
    // let message = ['h', 'e','l','l', 'o'];

    // for (index, item) in message.iter().enumerate() {
    //     println!("item {} is {}", index, item);
    //     if item == 'e' {
    //         break;
    //     }
    // }

    // for number in 0..5 {
    //     println!("number is {}", number);
    // }

    // nested loops

    let mut matrix  = [[1,2,3], [4,5,6], [7,8,9]];

    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
            *num += 10;
            print!("{}\t" , num);
        }
        println!();
    }
}

// fn conditional_assignment() {
//     let max_x_odd = true;
//     let x = if make_x_odd {1} else {2.0};
// }
