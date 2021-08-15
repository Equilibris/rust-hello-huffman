#[derive(Hash, Eq, Debug)]
pub enum BinaryTree<T> {
    Leaf(T),
    Node(Box<BinaryTree<T>>, Box<BinaryTree<T>>),
}

impl<T> BinaryTree<T> {
    pub fn unwrap_left(&self) -> &Box<Self> {
        use BinaryTree::Node;

        match self {
            Node(left, _) => left,
            _ => panic!("Attempted a node unwrap on a leaf value"),
        }
    }
    pub fn unwrap_right(&self) -> &Box<Self> {
        use BinaryTree::Node;

        match self {
            Node(_, right) => right,
            _ => panic!("Attempted a node unwrap on a leaf value"),
        }
    }

    pub fn left(&self) -> Option<&Box<Self>> {
        use BinaryTree::Node;

        match self {
            Node(left, _) => Option::Some(left),
            _ => Option::None,
        }
    }
    pub fn right(&self) -> Option<&Box<Self>> {
        use BinaryTree::Node;

        match self {
            Node(_, right) => Option::Some(right),
            _ => Option::None,
        }
    }

    pub fn is_localized_root(&self) -> bool {
        use BinaryTree::Node;

        match self {
            Node(left, right) => left.is_leaf() && right.is_leaf(),
            _ => false,
        }
    }
    pub fn is_leaf(&self) -> bool {
        use BinaryTree::Leaf;
        match self {
            Leaf(_) => true,
            _ => false,
        }
    }

    pub fn leaf_value(&self) -> Option<&T> {
        use BinaryTree::Leaf;

        match self {
            Leaf(val) => Option::Some(val),
            _ => Option::None,
        }
    }

    pub fn unwrap(&self) -> &T {
        use BinaryTree::Leaf;

        match self {
            Leaf(val) => val,
            _ => panic!("Attempted unwrap of node"),
        }
    }
}

impl<T: Eq> PartialEq for BinaryTree<T> {
    fn eq(&self, other: &Self) -> bool {
        use BinaryTree::{Leaf, Node};

        match (self, other) {
            (&Leaf(ref v), &Leaf(ref o)) => v == o,
            (&Node(ref v_l, ref v_r), &Node(ref o_l, ref o_r)) => v_l == o_l && v_r == o_r,

            _ => false,
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_tree::BinaryTree::{Leaf, Node};

    #[test]
    fn it_compares_truthy() {
        assert_eq!(Leaf(0) == Leaf(0), true);
        assert_eq!(
            Node(Box::new(Leaf(0)), Box::new(Leaf(1)))
                == Node(Box::new(Leaf(0)), Box::new(Leaf(1))),
            true
        );
    }

    #[test]
    fn it_compares_falsely() {
        assert_eq!(Leaf(1) == Leaf(0), false);
        assert_eq!(
            Node(Box::new(Leaf(1)), Box::new(Leaf(0)))
                == Node(Box::new(Leaf(0)), Box::new(Leaf(1))),
            false
        );
        assert_eq!(Node(Box::new(Leaf(1)), Box::new(Leaf(0))) == Leaf(0), false)
    }

    #[test]
    fn it_compares_deeply() {
        assert_eq!(
            Node(
                Box::new(Node(
                    Box::new(Leaf(0)),
                    Box::new(Node(
                        Box::new(Node(Box::new(Leaf(0)), Box::new(Leaf(1)))),
                        Box::new(Node(
                            Box::new(Leaf(0)),
                            Box::new(Node(Box::new(Leaf(0)), Box::new(Leaf(1))))
                        ))
                    ))
                )),
                Box::new(Leaf(1))
            ) == Node(
                Box::new(Node(
                    Box::new(Leaf(0)),
                    Box::new(Node(
                        Box::new(Node(Box::new(Leaf(0)), Box::new(Leaf(1)))),
                        Box::new(Node(
                            Box::new(Leaf(0)),
                            Box::new(Node(Box::new(Leaf(0)), Box::new(Leaf(1))))
                        ))
                    ))
                )),
                Box::new(Leaf(1))
            ),
            true
        );

        assert_eq!(
            Node(
                Box::new(Node(
                    Box::new(Leaf(0)),
                    Box::new(Node(
                        Box::new(Node(Box::new(Leaf(0)), Box::new(Leaf(1)))),
                        Box::new(Node(
                            Box::new(Leaf(0)),
                            Box::new(Node(Box::new(Leaf(0)), Box::new(Leaf(1))))
                        ))
                    ))
                )),
                Box::new(Leaf(1))
            ) == Node(
                Box::new(Node(
                    Box::new(Leaf(0)),
                    Box::new(Node(
                        Box::new(Node(Box::new(Leaf(1)), Box::new(Leaf(1)))),
                        Box::new(Node(
                            Box::new(Leaf(0)),
                            Box::new(Node(Box::new(Leaf(0)), Box::new(Leaf(1))))
                        ))
                    ))
                )),
                Box::new(Leaf(1))
            ),
            false
        );
    }
}
