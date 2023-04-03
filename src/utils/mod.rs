mod csv_parser;
mod hash;
mod build_tree;

pub use csv_parser::{parse_csv_to_entries};
pub use hash::{poseidon};
pub use build_tree::{build_merkle_tree_from_entries};

// Add the big_intify_username function
pub fn big_intify_username(username: &str) -> u64 {
    let utf8_bytes = username.as_bytes();
    let hex_string = hex::encode(utf8_bytes);
    u64::from_str_radix(&hex_string, 16).unwrap_or(0)
}

