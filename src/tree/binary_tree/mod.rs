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

    fn find(&self, v: &T) -> Option<&Node<T>> {
        let mut node = Some(&self.root);
        while node.is_some() && &node.unwrap().v != v {
            if v < &node.unwrap().v { // 选择左边节点
                node = Some(node.unwrap().left.as_ref().unwrap().as_ref());
            } else {
                node = Some(node.unwrap().right.as_ref().unwrap().as_ref());
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

    fn inorder_traversal(&self, node: Option<&Node<T>>) {
        if let Some(n) = node {
            self.inorder_traversal(n.left.as_ref().map(|v| v.as_ref()));
            // todo 访问节点
            self.inorder_traversal(n.right.as_ref().map(|v| v.as_ref()));
        }
    }

    fn preorder_traversal(&self, node: Option<&Node<T>>) {
        if let Some(n) = node {
            // todo 访问节点
            self.preorder_traversal(n.left.as_ref().map(|v| v.as_ref()));
            self.preorder_traversal(n.right.as_ref().map(|v| v.as_ref()));
        }
    }

    fn postorder_traversal(&self, node: Option<&Node<T>>) {
        if let Some(n) = node {
            self.postorder_traversal(n.left.as_ref().map(|v| v.as_ref()));
            self.postorder_traversal(n.right.as_ref().map(|v| v.as_ref()));
            // todo 访问节点
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