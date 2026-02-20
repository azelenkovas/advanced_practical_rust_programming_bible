#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    /// Create an empty list
    pub fn new() -> Self {
        List { head: None }
    }

    /// Push a new element at the front
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(), // take moves ownership of the old head
        });
        self.head = Some(new_node);
    }

    /// Pop the front element
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    /// Peek at the front element
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    /// Peek mutably at the front element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

// It's not really a linked list, it's a linked stack
fn main() {
    let mut list = List::new();
    list.push(10);
    list.push(20);
    list.push(30);

    println!("Top element: {:?}", list.peek());

    while let Some(val) = list.pop() {
        println!("Popped {:?}", val);
    }
}
