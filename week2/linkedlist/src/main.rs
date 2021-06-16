// We want to build a LinkedList that stores unsigned integers (u32s).
// Later in the course we’ll discuss how to deal with generics
// so we can have a single linked list implementation that can store any type,
// but for the purposes of this lecture, u32s are all that matter.
// We want to support constant time insertion and deletion (LIFO i.e. a stack).
// Furthermore, we want our list to keep track of its size and be able to display itself.

struct LinkedList {
    head: Option<Box<Node>>,
    size: usize,
}

struct Node {
    value: u32,
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: u32, next: Option<Box<Node>>) -> Node {
        Node {
            value: value,
            next: next,
        }
    }
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList {
            head: None,
            size: 0,
        }
    }
    pub fn get_size(&self) -> usize {
        self.size
    }
    pub fn is_empty(&self) -> bool {
        self.get_size() == 0
    }
    pub fn push(&mut self, value: u32) {
        let new_node: Box<Node> = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }
    pub fn pop(&mut self) -> Option<u32> {
        let node = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }

    pub fn display(&self) {
        let mut current: &Option<Box<Node>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                }
                None => break,
            }
        }
        println!("{}", result);
    }
}

fn main() {
    let mut list: LinkedList = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    let mut x: Option<u32> = Some(5);
    let x_ref: &mut Option<u32> = &mut x;
    // println!("result of take: {:?}", x_ref.take());
    // println!("left at x:{:?}", x);
    for i in 1..10 {
        list.push(i);
    }
    println!("list size:{}", list.get_size());

    list.display();
    list.pop();
    println!("list size:{}", list.get_size());
    list.display();
}
