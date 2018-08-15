//! This module defines the offline LCA (Lowest Common Ancestor)
//! algorithm. Tarjan algorithm is implemented. It requires that
//! all the pairs of interest are given beforehand.
use std::collections::HashMap;
use ::{ TreeIndexed, DjsuRooted };

/// The structure that holds LCA results.
///
/// # Examples
/// ```
/// use ralgo::{ GraphIndexed, TreeIndexed, LcaOffline };
/// let graph = GraphIndexed::new(4, &[(0, 1), (2, 1), (1, 3)]);
/// let tree = TreeIndexed::new(&graph, 3).unwrap();
/// let lca = LcaOffline::new(&tree, &[(0, 2), (1, 3)]);
/// assert_eq!(lca.ancestor(2, 0).unwrap(), 1);
/// assert_eq!(lca.ancestor(1, 3).unwrap(), 3);
/// assert!(lca.ancestor(2, 3).is_err());
/// ```
pub struct LcaOffline {
    result: HashMap<(usize, usize), usize>
}

impl LcaOffline {

    /// Preform the LCA algorithm. Return a struct
    /// holding the results.
    ///
    /// # Arguments
    ///
    /// * `tree` -- the tree to perform queries on;
    /// * `queries` -- list of pairs of vertices to compute LCA for.
    ///
    /// # Panics
    ///
    /// If `queries` contains vertices >= `tree.n_vert()`.
    ///
    pub fn new(tree: &TreeIndexed, queries: &[(usize, usize)])
            -> LcaOffline {
        let mut pairs = vec![Vec::new(); tree.n_vert()];
        for &(v, u) in queries {
            pairs[v].push(u);
            pairs[u].push(v);
        }

        let mut visited = vec![false; tree.n_vert()];
        let mut djsu = DjsuRooted::new(tree.n_vert());
        let mut result = HashMap::new();
        for vert in tree.postorder() {
            visited[vert] = true;
            for &other in &pairs[vert] {
                if visited[other] {
                    let ordered = LcaOffline::order(vert, other);
                    result.insert(ordered, djsu.find(other));
                }
            }
            if vert != tree.root() {
                djsu.union(tree.parent(vert).unwrap(), vert);
            }
        }
        LcaOffline{ result }
    }

    /// Given two integers `x` and `y`, swap them if `x` >= `y`.
    fn order(x: usize, y: usize) -> (usize, usize) {
        if x < y {
            (x, y)
        } else {
            (y, x)
        }
    }

    /// Return the LCA of two given vertices. If given pair
    /// was not passed to `LcaOffline::new`, return `Err(())`.
    ///
    /// # Arguments
    ///
    /// * `v`, `u` -- vertices in question.
    ///
    /// # Panics
    ///
    /// If either `v` or `u` is invalid vertex for a given tree,
    /// it is undefined whether this method panics or returns error.
    ///
    pub fn ancestor(&self, v: usize, u: usize) -> Result<usize, ()> {
        match self.result.get(&LcaOffline::order(v, u)) {
            Some(&x) => Ok(x),
            None => Err(())
        }
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_basic() {
        let graph = ::GraphIndexed::new(5, &[
            (3, 2),
            (2, 1),
            (0, 2),
            (4, 3)
        ]);
        let tree = ::TreeIndexed::new(&graph, 3).unwrap();
        let lca = ::LcaOffline::new(&tree, &[
            (1, 0), (1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4),
            (2, 0), (3, 0), (4, 0), (2, 2)
        ]);

        assert_eq!(lca.ancestor(1, 0).unwrap(), 2);
        assert_eq!(lca.ancestor(2, 1).unwrap(), 2);
        assert_eq!(lca.ancestor(1, 3).unwrap(), 3);
        assert_eq!(lca.ancestor(4, 1).unwrap(), 3);
        assert_eq!(lca.ancestor(2, 3).unwrap(), 3);
        assert_eq!(lca.ancestor(4, 2).unwrap(), 3);
        assert_eq!(lca.ancestor(4, 3).unwrap(), 3);
        assert_eq!(lca.ancestor(2, 0).unwrap(), 2);
        assert_eq!(lca.ancestor(0, 3).unwrap(), 3);
        assert_eq!(lca.ancestor(4, 0).unwrap(), 3);
        assert_eq!(lca.ancestor(2, 2).unwrap(), 2);
        
        assert!(lca.ancestor(0, 0).is_err());
        assert!(lca.ancestor(1, 1).is_err());
        assert!(lca.ancestor(3, 3).is_err());
        assert!(lca.ancestor(4, 4).is_err());
    }
}
