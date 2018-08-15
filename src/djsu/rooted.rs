//! This module defines a rooted disjoint set union data structure
//! that is indexed with natural numbers 0, 1, ..., N. Every
//! connected component has a dedicated root node. After a union
//! the newly formed connected component is rooted at the same node
//! that the 'major' subcomponent had been rooted at.
use ::DjsuIndexed;

/// The rooted disjoint set union data structure that is indexed
/// with natural numbers.
///
/// # Examples
///
/// ```
/// use ralgo::DjsuRooted;
/// let mut djsu = DjsuRooted::new(5);
/// djsu.union(1, 0);
/// djsu.union(1, 2);
/// assert_eq!(djsu.n_components(), 3);
/// assert_eq!(djsu.find(2), 1);
/// ```
pub struct DjsuRooted {
    djsu: DjsuIndexed,
    root: Vec<usize>
}

impl DjsuRooted {

    /// Return a DjsoRooted structure with given capacity.
    ///
    /// # Arguments
    ///
    /// * `count` - the number of components to start with.
    ///
    pub fn new(count: usize) -> DjsuRooted {
        let djsu = DjsuIndexed::new(count);
        let root = (0..count).collect();
        DjsuRooted{ djsu, root }
    }

    /// Return the current number of connected components.
    pub fn n_components(&self) -> usize {
        self.djsu.n_components()
    }

    /// Return the root of the connected component that given
    /// element belongs to.
    ///
    /// # Arguments
    ///
    /// * `ind` - the element in question.
    ///
    pub fn find(&mut self, ind: usize) -> usize {
        let repr = self.djsu.find(ind);
        self.root[repr]
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
        self.djsu.connected(left, right)
    }

    /// Connect two components that two given elements belong to.
    /// The resulting component inherits the root of the 'major'
    /// subcomponent. Return the root of the merged component.
    ///
    /// # Arguments
    ///
    /// * `major` - the first element. Its' root is inherited
    ///             by the resulting component;
    /// * `minor` - the second element. Its' root is forsaken.
    ///
    pub fn union(&mut self, major: usize, minor: usize) -> usize {
        let repr = self.djsu.union(major, minor);
        self.root[repr] = self.root[major];
        self.root[repr]
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn init_works() {
        let mut djsu = ::DjsuRooted::new(25);
        assert_eq!(djsu.n_components(), 25);
        for i in 0..25 {
            assert_eq!(djsu.find(i), i);
        }
    }

    #[test]
    fn union_works() {
        let mut djsu = ::DjsuIndexed::new(8);
        djsu.union(0, 1);
        djsu.union(1, 2);
        djsu.union(2, 3);
        djsu.union(4, 5);
        djsu.union(5, 6);

        assert_eq!(djsu.n_components(), 3);
        for i in 0..4 {
            for k in 0..4 {
                assert!(djsu.connected(i, k));
            }
            assert_eq!(djsu.find(i), 0);
        }
        for i in 4..7 {
            for k in 4..7 {
                assert!(djsu.connected(i, k));
            }
            assert_eq!(djsu.find(i), 4);
        }
        for i in 0..4 {
            for k in 4..7 {
                assert!(!djsu.connected(i, k));
            }
        }
        for i in 0..7 {
            assert!(!djsu.connected(i, 7));
        }
    }

    #[test]
    fn simple_test() {
        let mut djsu = ::DjsuIndexed::new(5);
        djsu.union(3, 4);
        djsu.union(3, 2);
        djsu.union(2, 0);
        assert_eq!(djsu.find(3), 3);
    }
}
