pub mod control_word;
pub mod font_definition;
pub mod footnote;
pub mod logger;
pub mod processor;
pub mod rtf_cell;
pub mod sunderer;
pub mod symbol;
pub mod template;
pub mod utils;
pub mod worker;

// #[test]
// fn hasher() {
//     use std::collections::hash_map::DefaultHasher;
//     use std::hash::{Hash, Hasher};
//     let input_str1 = r"D:\projects\rusty\playground\rtf\.sample_data\samples";
//     let input_str2 = r"D:\projects\rusty\playground\rtf\.sample_data\samples";
//     let mut hasher1 = DefaultHasher::new();
//     let mut hasher2 = DefaultHasher::new();

//     let bytes1 = input_str1.as_bytes();
//     let bytes2 = input_str2.as_bytes();

//     bytes1.hash(&mut hasher1);
//     bytes2.hash(&mut hasher2);

//     let hash_code1 = hasher1.finish();
//     let hash_code2 = hasher2.finish();
//     println!("The hash code of the string is: {}", hash_code1);
//     println!("The hash code of the string is: {}", hash_code2);
//     assert_eq!(hash_code1, hash_code2);
// }
