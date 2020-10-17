use super::super::{BinTree, BinTreeCursor, BinTreeCursorMut, BinTreeMut};

pub struct LeftNode<T> {
    elem: T,
    npl: usize,
}

/// (大顶)左式堆.
#[derive(Debug, Default)]
pub struct LeftHeap<Tree> {
    tree: Tree,
}

impl<T, Tree> LeftHeap<Tree>
where
    Tree: BinTreeMut<Elem = LeftNode<T>>,
{
    // fn merge(&mut self, other: &mut Self) {
    //     if self.tree.is_empty() {
    //         let mut cursor = other.tree.cursor_mut();
    //         let (left, right) = (cursor.take_left(), cursor.take_right());

    //     }
    // }
}
