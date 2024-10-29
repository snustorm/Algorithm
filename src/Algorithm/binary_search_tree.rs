
// 二叉树
#[derive(Debug)]
pub enum BinarySearchTree<T> {
    Empty,
    Node {
        value: T,
        left: Box<BinarySearchTree<T>>,
        right: Box<BinarySearchTree<T>>,
    }
}

impl<T> BinarySearchTree<T> {
    // Creates a new empty tree
    pub fn new() -> Self {
        BinarySearchTree::Empty
    }

    // Inserts a value into the tree
    //
    // `where T: Ord` is a constraint (trait bound) that ensures the type T
    // must implement the `Ord` trait. This allows us to compare values of type T
    // using ordering operators like `<`, `>`, and `==`.
    //
    // `Ord` is required for the binary tree to know where to place new values.
    // For example, if the new value is less than the current node's value,
    // it will be placed in the left subtree; otherwise, in the right subtree.
    pub fn insert(self, value: T) -> Self
    where
        T: Ord,
    {
        match self {
            // If the tree is empty, insert the new value as a root node
            BinarySearchTree::Empty => BinarySearchTree::Node {
                value,
                left: Box::new(BinarySearchTree::Empty),
                right: Box::new(BinarySearchTree::Empty),
            },
            // If the tree is not empty, compare the new value with the current node's value
            BinarySearchTree::Node { value: v, left, right } => {
                if value < v {
                    // Insert the new value into the left subtree if it's less than the current value
                    BinarySearchTree::Node {
                        value: v,
                        left: Box::new(left.insert(value)),
                        right,
                    }
                } else {
                    // Insert the new value into the right subtree if it's greater or equal
                    BinarySearchTree::Node {
                        value: v,
                        left,
                        right: Box::new(right.insert(value)),
                    }
                }
            }
        }
    }

    //In-order traversal the prints all value in ascending order
    pub fn in_order_traversal(&self)
    where
        T: std::fmt::Display,
    {
        match self {
            BinarySearchTree::Empty => {}
            BinarySearchTree::Node { value, left, right} => {
                //Traverse the left subtree first
                left.in_order_traversal();
                //Pint the current node value
                println!("Value: {}", value);
                //Traverse the right subtree then
                right.in_order_traversal();

            }
        }
    }

    pub fn pre_order_traversal(&self)
    where
        T: std::fmt::Display,
    {
        match self {
            BinarySearchTree::Empty => {}
            BinarySearchTree::Node { value, left, right} => {
                //Pint the current node value
                println!("Value: {}", value);
                //Traverse the left subtree then
                left.in_order_traversal();
                //Traverse the right subtree then
                right.in_order_traversal();

            }
        }
    }

    pub fn post_order_traversal(&self)
    where
        T: std::fmt::Display,
    {
        match self {
            BinarySearchTree::Empty => {}
            BinarySearchTree::Node { value, left, right} => {
                //Traverse the left subtree first
                left.in_order_traversal();
                //Traverse the right node value
                left.in_order_traversal();
                //Pint the current node value
                println!("Value: {}", value);
            }
        }
    }

}


