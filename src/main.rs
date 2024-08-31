mod binarytree;

use binarytree::TreeNode;
use std::collections::HashMap;
use std::env::args;
use std::process::exit;
use std::{fs, io};

#[derive(Debug)]
pub struct Token {
    value: String,
    occurrence: i32,
}

impl Token {
    pub fn new(value: String, occurrence: i32) -> Self {
        Token { value, occurrence }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        eprintln!("Error less argument ");
        exit(1);
    }
    let file_contents = fs::read_to_string(&args[1])?;
    let mut map: HashMap<char, i32> = HashMap::new();
    file_contents.chars().for_each(|c| {
        if let Some(x) = map.get_mut(&c) {
            *x += 1;
        } else {
            map.insert(c, 1);
        }
    });

    let mut mapped_vector: Vec<Token> = Vec::new();
    let mut treenode_vector: Vec<TreeNode<String>> = Vec::new();

    for ele in map {
        mapped_vector.push(Token::new(ele.0.to_string(), ele.1));
    }
    mapped_vector.sort_by(|a, b| a.occurrence.partial_cmp(&b.occurrence).unwrap());

    for token in mapped_vector {
        treenode_vector.push(TreeNode::new(token.value));
    }
    dbg!(treenode_vector);
    Ok(())
}
