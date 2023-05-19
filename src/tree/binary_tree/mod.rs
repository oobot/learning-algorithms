
struct Tree<T: Ord> {
    root: Option<Box<Node<T>>>,
}

struct Node<T> {
    v: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> Tree<T> {

    fn new () -> Self {
        Self { root: None }
    }

    fn find(&self, v: &T) -> Option<&Node<T>> {
        let mut node = self.root.as_ref();
        while node.is_some() && &node.unwrap().v != v {
            if v < &node.unwrap().v { // 选择左边节点
                node = node.unwrap().left.as_ref();
            } else {
                node = node.unwrap().right.as_ref();
            }
        }
        node.map(|v| v.as_ref())
    }

    fn insert(&mut self, v: T) {
        if self.root.is_none() {
            self.root.insert(Box::new(Node::from(v)));
            return;
        }

        let mut current = self.root.as_mut().unwrap();
        loop {
            if &v < &current.v { // 走左边
                if current.left.is_none() {
                    current.left.replace(Box::new(Node::from(v)));
                    return;
                }
                current = current.left.as_mut().unwrap();
            } else { // 走右边
                if current.right.is_none() {
                    current.right.replace(Box::new(Node::from(v)));
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

    fn min_node(&self) -> Option<&Node<T>> {
        let mut node = self.root.as_ref()?;
        while let Some(v) = &node.left {
            node = v;
        }
        Some(node)
    }

    fn max_node(&self) -> Option<&Node<T>> {
        let mut node = self.root.as_ref()?;
        while let Some(v) = &node.right {
            node = v;
        }
        Some(node)
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