/*
 * Box<> を使った再帰的データ構造。
 * こちらはコンパイルすることができる。
 */
pub enum BinaryTree<T> {
    Node {
        value: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    },
    Leaf {
        value: T,
    },
    Empty
}

/*
 * ある意味素朴に定義してみた再帰的データ構造。
 * こちらはコンパイルすることができない。
 * また、木の末端ノード（葉）のような、子を持たないノードを表現することができていない。
 */
pub struct InvalidBinaryTree<T> {
    value: T,
    left: InvalidBinaryTree<T>,
    right: InvalidBinaryTree<T>,
}

/*
 * Box<> を使わず、ある意味素朴に再帰的データ構造を定義してみた例。
 * こちらはコンパイルすることができない。
 */
pub enum InvalidBinaryTree2<T> {
    Node {
        value: T,
        left: InvalidBinaryTree2<T>,
        right: InvalidBinaryTree2<T>,
    },
    Leaf {
        value: T,
    },
    Empty
}
