//! This module defines a static graph data structure whose vertices
//! are indexed with integers 0, 1, ..., N-1. This graph can not
//! be modified once created.

/// The integer-indexed static graph data structure.
///
/// # Examples
/// ```
/// use ralgo::StaticGraph;
/// let graph = StaticGraph::new(5, &[(0, 1), (2, 3)]);
/// assert_eq!(graph.n_vert(), 5);
/// assert_eq!(graph.n_edges(), 2);
/// assert_eq!(graph.neighbors(0), &[1]);
/// assert_eq!(graph.neighbors(1), &[0]);
/// assert_eq!(graph.neighbors(2), &[3]);
/// assert_eq!(graph.neighbors(3), &[2]);
/// assert_eq!(graph.neighbors(4), &[]);
/// ```
pub struct StaticGraph {
    offset: Vec<usize>,
    neigh: Vec<usize>
}

impl StaticGraph {

    /// Return a new instance of StaticGraph.
    ///
    /// # Arguments
    ///
    /// * `n_vert` - number of vertices;
    /// * `edges` - pairs of adjacent vertices.
    ///
    /// # Panics
    ///
    /// If `edges` contains an element >= `n_vert`.
    ///
    pub fn new(n_vert: usize, edges: &[(usize, usize)]) -> StaticGraph {
        let mut offset = vec![0 as usize; n_vert];
        for &(u, v) in edges {
            if u < n_vert - 1 {
                offset[u+1] += 1;
            }
            if v < n_vert - 1 {
                offset[v+1] += 1;
            }
        }
        for i in 2 .. n_vert {
            offset[i] += offset[i-1];
        }

        let mut pos = offset.clone();
        let mut neigh = vec![0; 2*edges.len()];
        for (u, v) in edges.iter() {
            neigh[pos[*u]] = *v;
            pos[*u] += 1;
            neigh[pos[*v]] = *u;
            pos[*v] += 1;
        }

        StaticGraph{ offset, neigh }
    }

    /// Return the number of vertices in given Graph instance.
    pub fn n_vert(&self) -> usize {
        self.offset.len()
    }

    /// Return the number of edges in given Graph instance.
    pub fn n_edges(&self) -> usize {
        self.neigh.len() / 2
    }

    /// Return the slice of neighboring vertices to the given vertex.
    ///
    /// # Arguments
    ///
    /// * `vert` - the vertex in question.
    ///
    /// # Panics
    ///
    /// If `vert` >= `self.n_vert()`.
    ///
    pub fn neighbors(&self, vert: usize) -> &[usize] {
        if vert < self.offset.len() - 1 {
            &self.neigh[self.offset[vert] .. self.offset[vert+1]]
        } else {
            &self.neigh[self.offset[vert] ..]
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
    fn simple_graph_works() {
        let graph = ::StaticGraph::new(5, &[
            (0, 1),
            (2, 1),
            (3, 1),
        ]);
        assert_eq!(graph.n_vert(), 5);
        assert_eq!(graph.n_edges(), 3);
        assert!(vertices_equal(graph.neighbors(0), &[1]));
        assert!(vertices_equal(graph.neighbors(1), &[0, 2, 3]));
        assert!(vertices_equal(graph.neighbors(2), &[1]));
        assert!(vertices_equal(graph.neighbors(3), &[1]));
        assert!(vertices_equal(graph.neighbors(4), &[]));
    }

    #[test]
    fn empty_graph_works() {
        let graph = ::StaticGraph::new(5, &[]);
        assert_eq!(graph.n_vert(), 5);
        assert_eq!(graph.n_edges(), 0);
    }

    #[test]
    fn full_graph_works() {
        let graph = ::StaticGraph::new(4, &[
            (0, 1),
            (0, 2),
            (0, 3),
            (1, 2),
            (1, 3),
            (2, 3)
        ]);
        assert_eq!(graph.n_vert(), 4);
        assert_eq!(graph.n_edges(), 6);
        assert!(vertices_equal(graph.neighbors(0), &[1, 2, 3]));
        assert!(vertices_equal(graph.neighbors(1), &[0, 2, 3]));
        assert!(vertices_equal(graph.neighbors(2), &[0, 1, 3]));
        assert!(vertices_equal(graph.neighbors(3), &[0, 1, 2]));
    }
}
