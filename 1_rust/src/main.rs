//Bai 1

// fn main() {
//     let org_arr = [2, 3, 5, 1, 2, 3, 5, 6, 8, 10, 11, 6, 8, 10, 2, 3, 5, 6, 8, 10];
//     let sub_arr = [6, 8, 10];
//     let mut index = 0;
//     while index <= org_arr.len() - sub_arr.len() {
//         if &org_arr[index..index+sub_arr.len()] == sub_arr {
//             println!("A match start from index {}", index);
//         }
//         index += 1;
//     }
// }


//Bai 2

use std::io;
fn main() {
    println!("Input your character:");
    let origin = "adbcdaDd";
    let bytes = origin.as_bytes();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let mut count = 0;
    let mut s = String::from("");
    for (_i, &item) in bytes.iter().enumerate() {
        if item.to_ascii_lowercase()== input.to_lowercase().as_bytes()[0] {
            count += 1;
        } else {
            let c = item as char;
            s.push(c);
        }
    }
    println!("{}", count);
    println!("{}", s);
}