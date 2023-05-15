
struct Node<T> {
    v: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

struct Tree<T: Ord> {
    root: Node<T>,
}

impl<T: Ord> Tree<T> {

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
        todo!()
    }

    fn delete(&mut self, v: &T) -> Option<Node<T>> {
        todo!()
    }

}