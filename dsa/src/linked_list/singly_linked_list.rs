#[derive(Clone)]
struct SSLNode<T> {
    value: T,
    next: Option<Box<SSLNode<T>>>,
}

impl<T> SSLNode<T>
where
    T: Clone + Copy + ToString,
{
    pub fn new(value: T, next: Option<Box<SSLNode<T>>>) -> Self {
        Self {
            value: value,
            next: next,
        }
    }
}

pub struct SinglyLinkedList<T> {
    head: Option<Box<SSLNode<T>>>,
    length: i32,
}

impl<T> SinglyLinkedList<T>
where
    T: Clone + Copy + ToString,
{
    pub fn init() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    pub fn push_front(&mut self, value: T) {
        self.head = Some(Box::new(SSLNode::new(value, self.head.take())));
        self.length += 1;
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = SSLNode::new(value, None);

        match self.head.as_mut() {
            Some(head) => {
                let mut cur = head;

                while cur.next.is_some() {
                    cur = cur.next.as_mut().unwrap();
                }
                cur.next = Some(Box::new(new_node));
            }
            None => {
                self.head = Some(Box::new(new_node));
            }
        };
        self.length += 1;
    }

    pub fn print(&self) {
        if let Some(head) = &self.head {
            println!("length: {}", self.length);
            Self::print_from_node(&mut head.clone());
        }
    }

    fn print_from_node(node: &mut Box<SSLNode<T>>) {
        print!("{}", node.value.to_string());

        let mut cur = node;
        while let Some(next) = cur.next.as_mut() {
            print!("->{}", next.value.to_string());
            cur = next;
        }
    }
}
