//! This module defines depth-first search (DFS) procedure and
//! data structure.

use ::GraphIndexed;

/// The depth-first search (DFS) data structure and algorithm
/// implementation.
///
/// # Examples
/// ```
/// use ralgo::{ DFS, GraphIndexed };
/// let graph = GraphIndexed::new(5, &[(0, 1), (2, 3)]);
/// let dfs = DFS::new(&graph, 1);
/// assert_eq!(dfs.source(), 1);
/// assert_eq!(dfs.n_vert_reached(), 2);
/// assert!(dfs.is_reached(0));
/// assert!(!dfs.is_reached(2));
/// assert!(!dfs.cycle_found());
/// ```
pub struct DFS {
    source: usize,
    parent: Vec<usize>,
    cycle_found: bool,
    n_vert_reached: usize
}

impl DFS {

    /// Launch DFS on given graph. Return DFS structure holding
    /// the results.
    ///
    /// # Arguments
    ///
    /// * `graph` - the graph to launch DFS on;
    /// * `source` - the source vertex.
    ///
    /// # Panics
    ///
    /// If source >= graph.n_vert().
    ///
    pub fn new(graph: &GraphIndexed, source: usize) -> DFS {
        let parent: Vec<usize> = (0..graph.n_vert()).collect();
        let cycle_found = false;
        let n_vert_reached = 0;
        let mut dfs = DFS{ source, parent, cycle_found, n_vert_reached };

        let mut visited: Vec<bool> = vec![false; graph.n_vert()];
        dfs.run(graph, source, &mut visited);

        dfs
    }

    /// Internally used method that is called recursively
    /// when running DFS.
    fn run(&mut self, graph: &GraphIndexed, source: usize,
           visited: &mut Vec<bool>) {
        visited[source] = true;
        self.n_vert_reached += 1;
        for &neigh in graph.neighbors(source) {
            if !visited[neigh] {
                self.parent[neigh] = source;
                self.run(graph, neigh, visited);
            } else if neigh != self.parent[source] {
                self.cycle_found = true;
            }
        }
    }

    /// Return the source vertex.
    pub fn source(&self) -> usize {
        self.source
    }

    /// If given `node` has been reached during DFS and is not source,
    /// return the parent node (the node that led to given `node`).
    /// Otherwise return None.
    ///
    /// # Arguments
    ///
    /// * `node` - the vertex in question.
    ///
    /// # Panics
    ///
    /// If `node` >= the nubmer of vertices in the graph.
    ///
    pub fn parent(&self, node: usize) -> Option<usize> {
        if self.parent[node] == node {
            None
        } else {
            Some(self.parent[node])
        }
    }

    /// Return `true` if given node has been reached during DFS.
    /// Always returns `true` for source vertex.
    ///
    /// # Arguments
    ///
    /// * `node` - the vertex in question.
    ///
    /// # Panics
    ///
    /// If `node` >= the nubmer of vertices in the graph.
    ///
    pub fn is_reached(&self, node: usize) -> bool {
        (node == self.source) || (self.parent[node] != node)
    }

    /// Return `true` if cycle has been found. If there exists
    /// a cycle reachable from the source vertex, DFS is guaranteed
    /// to find it.
    pub fn cycle_found(&self) -> bool {
        self.cycle_found
    }

    /// Number of vertices reached during DFS.
    pub fn n_vert_reached(&self) -> usize {
        self.n_vert_reached
    }
}


#[cfg(test)]
mod test {

    #[test]
    fn empty_graph_works() {
        let graph = ::GraphIndexed::new(5, &[]);
        let dfs = ::DFS::new(&graph, 0);

        assert_eq!(dfs.source(), 0);
        assert_eq!(dfs.n_vert_reached(), 1);
        assert!(!dfs.cycle_found());

        for v in 0..5 {
            assert_eq!(dfs.is_reached(v), v == 0);
            assert!(dfs.parent(v).is_none());
        }
    }

    #[test]
    fn cycle_works() {
        let graph = ::GraphIndexed::new(10, &[
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 0)
        ]);
        let dfs = ::DFS::new(&graph, 0);

        assert_eq!(dfs.source(), 0);
        assert_eq!(dfs.n_vert_reached(), 5);
        assert!(dfs.cycle_found());

        for v in 0..5 {
            assert!(dfs.is_reached(v));
            assert_eq!(dfs.parent(v).is_none(), v == 0);
        }

        for v in 5..10 {
            assert!(!dfs.is_reached(v));
            assert!(dfs.parent(v).is_none());
        }
    }

    #[test]
    fn full_graph_works() {
        let graph = ::GraphIndexed::new(4, &[
            (0, 1),
            (0, 2),
            (0, 3),
            (1, 2),
            (1, 3),
            (2, 3)
        ]);
        let dfs = ::DFS::new(&graph, 3);
        assert_eq!(dfs.source(), 3);
        assert_eq!(dfs.n_vert_reached(), 4);
        assert!(dfs.cycle_found());
        for v in 0..4 {
            assert!(dfs.is_reached(v));
            assert_eq!(dfs.parent(v).is_none(), v == 3);
        }
    }
}
