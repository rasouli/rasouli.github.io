use std::{cell::RefCell, rc::Rc};

fn main() {
    let mut root = BinaryTree::new(1);

    let mut nodeA = BinaryTree::new(7);
    nodeA.left = BinaryTree::wrap(BinaryTree::new(8));
    nodeA.right = BinaryTree::wrap(BinaryTree::new(9));

    let mut root_l = BinaryTree::new(2);
    root_l.right = BinaryTree::wrap(nodeA);
    root.left = BinaryTree::wrap(root_l);
    root.right = BinaryTree::wrap(BinaryTree::new(3));

    let tree = BinaryTree::wrap(root);
    let traversal = traversal::morisson_inorder_traversal(&tree);
    println!("traversal inorder resurt : {:?}", traversal);
    println!("salam")
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
    use std::borrow::BorrowMut;
    use std::cell::RefCell;
    use std::rc::Rc;

    pub fn morisson_inorder_traversal(root: &Option<Rc<RefCell<Box<BinaryTree>>>>) -> Vec<i32> {
        let mut inorder_traversal = Vec::<i32>::new();

        if root.is_none() {
            return inorder_traversal;
        }

        let mut current: Option<Rc<RefCell<Box<BinaryTree>>>> = root.clone();

        while let Some(current_rc_node) = current.clone() {
            let current_node = current_rc_node.borrow();

            if current_node.left.is_none() {
                inorder_traversal.push(current_node.value);
                current = current_node.right.clone();
                continue;
            }

            let mut predecessor: Option<Rc<RefCell<Box<BinaryTree>>>> = current_node.left.clone();
            loop {
                if let Some(pred_rc_node) = predecessor.clone() {
                    let pred_node = pred_rc_node.borrow();
                    if pred_node.right.is_none() || pred_node.right == current {
                        break;
                    }

                    predecessor = pred_node.right.clone();
                }
            }

            if let Some(pred_rc_node) = predecessor {
                let mut pred_node = pred_rc_node.as_ref().borrow_mut();

                if pred_node.right.is_none() {
                    pred_node.right = current.clone();
                    current = current_rc_node.as_ref().borrow_mut().left.clone();
                } else {
                    pred_node.right = None;
                    inorder_traversal.push(current_rc_node.borrow().value);
                    current = current_rc_node.as_ref().borrow_mut().right.clone();
                }
            }
        }

        inorder_traversal
    }
}
