mod binarytree;

use binarytree::TreeNode;

fn main() {
    let mut root = TreeNode::new("M");
    root.insert("A");
    root.insert("H");
    root.insert("X");
    root.insert("W");
    root.insert("R");

    println!("{:?}", root);
}
