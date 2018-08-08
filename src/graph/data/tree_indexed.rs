//! This module defines an integer-indexed tree data structure.
//! It is similar to `GraphIndexed`, but has a dedicated root.

use ::GraphIndexed;
use ::DFS;

/// An integer-indexed tree data structure.
/// It is similar to `GraphIndexed`, but has a dedicated root.
///
/// # Examples
/// ```
/// use ralgo::{ TreeIndexed, GraphIndexed };
/// let graph = GraphIndexed::new(4, &[(0, 1), (1, 2), (1, 3)]);
/// let tree = TreeIndexed::new(&graph, 0).unwrap();
/// assert_eq!(tree.root(), 0);
/// assert_eq!(tree.n_vert(), 4);
/// assert_eq!(tree.n_edges(), 3);
/// assert_eq!(tree.children(0), &[1]);
/// assert_eq!(tree.parent(3).unwrap(), 1);
/// assert!(tree.parent(0).is_none());
/// ```
pub struct TreeIndexed {
    root: usize,
    offset: Vec<usize>,
    children: Vec<usize>,
    parent: Vec<usize>
}

impl TreeIndexed {

    /// If given `graph` is a tree, return a TreeIndexed instance
    /// created from the `graph` with given `root`. Otherwise
    /// return Err(()).
    ///
    /// # Arguments
    ///
    /// * `graph` - the graph to convert into tree;
    /// * `root` - the root vertex.
    ///
    /// # Panics
    ///
    /// If root >= graph.n_vert().
    ///
    pub fn new(graph: &GraphIndexed, root: usize)
            -> Result<TreeIndexed, ()>
    {
        let dfs = DFS::new(graph, root);
        if dfs.cycle_found() || dfs.n_vert_reached() < graph.n_vert() {
            return Err(());
        }

        let mut offset = vec![0 as usize; graph.n_vert()];
        for vert in 0 .. graph.n_vert() - 1 {
            for &neigh in graph.neighbors(vert) {
                if dfs.parent(vert) != Some(neigh) {
                    offset[vert+1] += 1;
                }
            }
        }
        for i in 2 .. graph.n_vert () {
            offset[i] += offset[i-1];
        }

        let mut pos = offset.clone();
        let mut children = vec![0; graph.n_edges()];
        for vert in 0 .. graph.n_vert() {
            for &neigh in graph.neighbors(vert) {
                if dfs.parent(vert) != Some(neigh) {
                    children[pos[vert]] = neigh;
                    pos[vert] += 1;
                }
            }
        }

        let parent = dfs.extract_parent();
        Ok(TreeIndexed{ root, offset, children, parent })
    }

    /// Return the root node of the tree.
    pub fn root(&self) -> usize {
        return self.root;
    }

    /// Return the number of vertices in the tree.
    pub fn n_vert(&self) -> usize {
        self.offset.len()
    }

    /// Return the number of edges in the tree.
    /// Notice that n_edges() = n_vert() - 1.
    pub fn n_edges(&self) -> usize {
        self.children.len()
    }

    /// Return the slice of child vertices to the given vertex.
    ///
    /// # Arguments
    ///
    /// * `node` - the vertex in question.
    ///
    /// # Panics
    ///
    /// If `node` >= `self.n_vert()`.
    ///
    pub fn children(&self, node: usize) -> &[usize] {
        if node < self.offset.len() - 1 {
            &self.children[self.offset[node] .. self.offset[node+1]]
        } else {
            &self.children[self.offset[node] ..]
        }
    }

    /// Return the parent of given `node`. For root node return None.
    ///
    /// # Arguments
    ///
    /// * `node` - the vertex in question.
    ///
    /// # Panics
    ///
    /// If `node` >= self.n_vert()`.
    pub fn parent(&self, node: usize) -> Option<usize> {
        if node == self.root {
            None
        } else {
            Some(self.parent[node])
        }
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::iter::FromIterator;

    fn vertices_equal(left: &[usize], right: &[usize]) -> bool {
        let left = HashSet::<usize>::from_iter(left.iter().cloned());
        let right = HashSet::<usize>::from_iter(right.iter().cloned());
        left == right
    }

    #[test]
    fn simple_tree_works() {
        let graph = ::GraphIndexed::new(5, &[
            (3, 2),
            (2, 1),
            (0, 2),
            (4, 3)
        ]);
        let tree = ::TreeIndexed::new(&graph, 3).unwrap();
        assert_eq!(tree.root(), 3);
        assert_eq!(tree.n_vert(), 5);
        assert_eq!(tree.n_edges(), 4);
        assert!(tree.parent(3).is_none());
        assert!(vertices_equal(tree.children(3), &[2, 4]));
        assert_eq!(tree.parent(2).unwrap(), 3);
        assert!(vertices_equal(tree.children(2), &[0, 1]));
        assert_eq!(tree.parent(0).unwrap(), 2);
        assert!(vertices_equal(tree.children(0), &[]));
        assert_eq!(tree.parent(1).unwrap(), 2);
        assert!(vertices_equal(tree.children(1), &[]));
        assert_eq!(tree.parent(4).unwrap(), 3);
        assert!(vertices_equal(tree.children(4), &[]));
    }

    #[test]
    fn cycle_detection_works() {
        let graph = ::GraphIndexed::new(4, &[
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0)
        ]);
        let tree = ::TreeIndexed::new(&graph, 1);
        assert!(tree.is_err());
    }

    #[test]
    fn disconnected_graph_detection_works() {
        let graph = ::GraphIndexed::new(4, &[
            (0, 1),
            (2, 3),
        ]);
        let tree = ::TreeIndexed::new(&graph, 1);
        assert!(tree.is_err());
    }
}
