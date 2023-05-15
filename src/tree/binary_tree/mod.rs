use std::collections::VecDeque;

struct Node<T> {
    v: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

struct Tree<T: Ord> {
    root: Node<T>,
}

impl<T: Ord> Tree<T> {

    fn from(v: T) -> Self {
        Self {
            root: Node::from(v)
        }
    }

    fn find(&self, v: &T) -> Option<Box<&Node<T>>> {
        let mut node = Some(Box::new(&self.root));
        while node.is_some() && &node.as_ref().unwrap().v != v {
            if v < &node.as_ref().unwrap().v { // 选择左边节点
                node = Some(Box::new(node.as_ref().unwrap().left.as_ref().unwrap().as_ref()));
            } else { // 选择右边节点
                node = Some(Box::new(node.as_ref().unwrap().right.as_ref().unwrap().as_ref()));
            }
        }
        node
    }

    fn insert(&mut self, v: T) {
        let mut current = &mut self.root;
        loop {
            if &v < &current.v { // 走左边
                if current.left.is_none() {
                    current.left = Some(Box::new(Node::from(v)));
                    return;
                }
                current = current.left.as_mut().unwrap();
            } else { // 走右边
                if current.right.is_none() {
                    current.right = Some(Box::new(Node::from(v)));
                    return;
                }
                current = current.right.as_mut().unwrap();
            }
        }
    }

    fn delete(&mut self, v: &T) -> Option<Node<T>> {
        todo!()
    }

}

impl<T> Node<T> {
    fn from(v: T) -> Self {
        Self {
            v,
            left: None,
            right: None,
        }
    }
}