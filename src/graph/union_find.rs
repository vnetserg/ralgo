//! This module defines a union-find data structure
//! (aka disjoint set union). The elements in the set
//! are indexed with integers 0, 1, ..., N-1.

/// The integer-indexed union-find data structure
/// (aka disjoint set union).
///
/// # Examples
///
/// ```
/// use ralgo::UnionFind;
/// let mut uf = UnionFind::new(5);
/// uf.union(0, 1);
/// uf.union(1, 2);
/// assert_eq!(uf.n_components(), 3);
/// assert!(uf.connected(0, 2));
/// assert!(!uf.connected(1, 3));
/// ```
pub struct UnionFind {
    root: Vec<usize>,
    height: Vec<usize>,
    count: usize,
}

impl UnionFind {
    /// Return a UnionFind structure with given capacity.
    ///
    /// # Arguments
    ///
    /// * `count` - the number of components to start with.
    ///
    pub fn new(count: usize) -> UnionFind {
        let root = (0..count).collect();
        let height = vec![0; count];
        UnionFind {
            root,
            height,
            count,
        }
    }

    /// Return the current number of connected components.
    pub fn n_components(&self) -> usize {
        self.count
    }

    /// Return the representative of the connected component
    /// that given element belongs to.
    ///
    /// # Arguments
    ///
    /// * `ind` - the element in question.
    ///
    pub fn find(&mut self, mut ind: usize) -> usize {
        let mut root = ind;
        while self.root[root] != root {
            root = self.root[root];
        }
        while self.root[ind] != ind {
            let next = self.root[ind];
            self.root[ind] = root;
            ind = next;
        }
        root
    }

    /// Return `true` if two given elements belong to the same
    /// connected component, `false` otherwise.
    ///
    /// # Arguments
    ///
    /// * `left` - the fist element in question;
    /// * `right` - the second element.
    ///
    pub fn connected(&mut self, left: usize, right: usize) -> bool {
        self.find(left) == self.find(right)
    }

    /// Connect two components that two given elements belong to.
    /// Return the representative of the merged component.
    ///
    /// # Arguments
    ///
    /// * `left` - the first element;
    /// * `right` - the second element.
    ///
    pub fn union(&mut self, left: usize, right: usize) -> usize {
        let left = self.find(left);
        let right = self.find(right);
        if left == right {
            return left;
        }

        self.count -= 1;
        if self.height[left] < self.height[right] {
            self.root[left] = right;
            return right;
        } else if self.height[left] > self.height[right] {
            self.root[right] = left;
            return left;
        } else {
            self.root[right] = left;
            self.height[right] += 1;
            return left;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UnionFind;

    #[test]
    fn init_works() {
        let mut uf = UnionFind::new(25);
        assert_eq!(uf.n_components(), 25);
        for i in 0..25 {
            assert_eq!(uf.find(i), i);
        }
    }

    #[test]
    fn union_works() {
        let mut uf = UnionFind::new(8);
        uf.union(0, 1);
        uf.union(1, 2);
        uf.union(2, 3);
        uf.union(4, 5);
        uf.union(5, 6);

        assert_eq!(uf.n_components(), 3);
        for i in 0..4 {
            for k in 0..4 {
                assert!(uf.connected(i, k));
            }
        }
        for i in 4..7 {
            for k in 4..7 {
                assert!(uf.connected(i, k));
            }
        }
        for i in 0..4 {
            for k in 4..7 {
                assert!(!uf.connected(i, k));
            }
        }
        for i in 0..7 {
            assert!(!uf.connected(i, 7));
        }
    }

    #[test]
    fn big_case_works() {
        let mut uf = UnionFind::new(99999);
        for i in (5..99999).step_by(3) {
            uf.union(i, i - 3);
            uf.union(i - 1, i - 4);
        }
        for i in (2..99999).step_by(3) {
            assert_eq!(uf.find(i), uf.find(2));
            assert_eq!(uf.find(i - 1), uf.find(1));
            assert_eq!(uf.find(i - 2), i - 2);
        }
    }
}
