use std::{cell::RefCell, rc::Rc};

fn main() {
    println!("hello world");
}

#[derive(Debug, PartialEq, Eq)]
pub struct BinaryTree {
    pub value: i32,
    pub left: Option<Rc<RefCell<Box<BinaryTree>>>>,
    pub right: Option<Rc<RefCell<Box<BinaryTree>>>>,
}

impl BinaryTree {
    pub fn new(value: i32) -> Self {
        BinaryTree {
            value,
            left: None,
            right: None,
        }
    }

    pub fn wrap(node: BinaryTree) -> Option<Rc<RefCell<Box<BinaryTree>>>> {
        Some(Rc::new(RefCell::new(Box::new(node))))
    }
}

mod traversal {
    use crate::BinaryTree;
    use std::{
        borrow::{Borrow, BorrowMut},
        cell::RefCell,
        rc::Rc,
    };

    pub fn morisson_inorder_traversal(root: &Option<Rc<RefCell<Box<BinaryTree>>>>) -> Vec<i32> {
        let mut inorder_traversal = Vec::<i32>::new();

        if root.is_none() {
            return inorder_traversal;
        }

        let mut current: Option<Rc<RefCell<Box<BinaryTree>>>> = root.clone();
        let mut predecessor: Option<Rc<RefCell<Box<BinaryTree>>>> = None;
        while let Some(rc_current_node) = current.clone() {
            {
                let current_node = rc_current_node.as_ref().borrow();

                if current_node.left.is_none() {
                    inorder_traversal.push(current_node.value);
                    current = current_node.right.clone();
                    continue;
                }

                predecessor = current_node.left.clone();
                while let Some(rc_pred_node) = predecessor.clone() {
                    let pred_node = rc_pred_node.as_ref().borrow();
                    if pred_node.right.is_none() || pred_node.right == current {
                        break;
                    }

                    predecessor = pred_node.right.clone();
                }
            }

            if let Some(ref rc_pred_node) = predecessor.clone() {
                let mut pred_node = rc_pred_node.as_ref().borrow_mut();
                if let Some(ref rc_current_node) = current.clone() {
                    if pred_node.right.is_none() {
                        pred_node.right = current.clone();
                        current = rc_current_node.as_ref().borrow().left.clone();
                    } else {
                        pred_node.right = None;
                        inorder_traversal.push(rc_current_node.as_ref().borrow().value);
                        current = rc_current_node.as_ref().borrow().right.clone();
                    }
                }
            }
        }

        inorder_traversal
    }

    #[cfg(test)]
    mod tests {
        use crate::{traversal, BinaryTree};

        #[test]
        fn inorder_morisson_traversal_tests() {
            let mut root = BinaryTree::new(1);
            let mut root_left = BinaryTree::new(2);

            let mut root_left_right = BinaryTree::new(7);
            root_left_right.left = BinaryTree::wrap(BinaryTree::new(8));
            root_left_right.right = BinaryTree::wrap(BinaryTree::new(9));

            root_left.right = BinaryTree::wrap(root_left_right);
            root.left = BinaryTree::wrap(root_left);
            root.right = BinaryTree::wrap(BinaryTree::new(3));

            let tree = BinaryTree::wrap(root);
            let traversal = traversal::morisson_inorder_traversal(&tree);
            assert_eq!(traversal, vec![2, 8, 7, 9, 1, 3]);
        }
    }
}
