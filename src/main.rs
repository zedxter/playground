pub mod utils;
pub mod structures;

use crate::utils::is_substring;
use crate::utils::is_substring_by_char;

use crate::structures::LinkedList;

fn main() {
    let input = String::from("Very fancy text!");
    let term = String::from("fan");
    let result = is_substring(&input, &term);
    let result2 = is_substring_by_char(&input, &term);

    println!("The index of substring is {result}");
    println!("The char by char index of substring is {result2}");

    let mut ll = LinkedList::new();
    ll.push(123);
    ll.push(234);
    ll.push(345);
    let _elem = ll.pop();
    match _elem {
        Some(value) => println!("The popped value is {value}"),
        None => println!("The list is empty"),
    }
}
