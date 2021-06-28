type NodeBox<T> = Option<Box<Node<T>>>;

// Template Node
#[derive(Debug)]
struct Node<T> {
    payload: T,
    left: NodeBox<T>,
    right: NodeBox<T>
}

// Implement for Node<T> if T implements partial order
impl <T: PartialOrd> Node<T> {
    fn new(s: T) -> Node<T> {
        Node{payload: s, left: None, right: None}
    }

    // Inserts into a box
    fn boxer(node: Node<T>) -> NodeBox<T> {
        Some(Box::new(node))
    }

    // Boxes a node and inserts it on the left
    fn set_left(&mut self, node: Node<T>) {
        self.left = Self::boxer(node);
    }

    // Boxes a node and inserts it on the right
    fn set_right(&mut self, node: Node<T>) {
        self.right = Self::boxer(node);
    }

    // Recursively inserts nodes into the the binary tree
    // Nodes less than current node go to the left,
    // nodes greater than the current node go to the right.
    // This is done recursively until the insert location
    // is None, then add the node
    fn insert(&mut self, data: T) {
        if data < self.payload {
            match self.left {
                Some(ref mut n) => n.insert(data),
                None => self.set_left(Self::new(data)),
            }
        } else {
            match self.right {
                Some(ref mut n) => n.insert(data),
                None => self.set_right(Self::new(data)),
            }
        }
    }
}

fn main() {

    // String binary tree
    let mut root = Node::new("root".to_string());
    root.insert("one".to_string());
    root.insert("two".to_string());
    root.insert("four".to_string());

    println!("root: {:#?}", root);

    // Int binary tree
    let mut int_root = Node::new(0);
    int_root.insert(2);
    int_root.insert(3);
    int_root.insert(10);
    int_root.insert(-25);

    println!("int root: {:#?}", int_root);

}
