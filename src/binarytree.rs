#[derive(Debug)]
pub struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
    pub occurrence: i32,
}

impl<T> TreeNode<T> {
    pub fn new(value: T, occurrence: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
            occurrence,
        }
    }

    pub fn insert(&mut self, leftroot: TreeNode<T>, rightroot: TreeNode<T>)
    where
        T: PartialOrd,
    {
        if leftroot.occurrence < rightroot.occurrence {
            self.right = Some(Box::new(leftroot));
            self.left = Some(Box::new(rightroot));
        } else {
            self.right = Some(Box::new(rightroot));
            self.left = Some(Box::new(leftroot));
        }
    }
}

pub fn mergetrees(left: TreeNode<String>, right: TreeNode<String>) -> TreeNode<String> {
    let value = format!("{}{}", right.value, left.value);
    let occurrence = right.occurrence + left.occurrence;
    let mut root = TreeNode::new(value, occurrence);
    root.insert(left, right);
    root
}
