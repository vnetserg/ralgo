pub struct DjsuIndex {
    root: Vec<usize>,
    height: Vec<usize>,
    count: usize
}

impl DjsuIndex {
    pub fn new(count: usize) -> DjsuIndex {
        let root = (0..count).collect();
        let height = vec![0; count];
        DjsuIndex{ root, height, count }
    }

    pub fn n_components(&self) -> usize {
        self.count
    }

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

    pub fn connected(&mut self, left: usize, right: usize) -> bool {
        self.find(left) == self.find(right)
    }

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
        }
        else {
            self.root[right] = left;
            self.height[right] += 1;
            return left;
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn init_works() {
        let mut djsu = ::DjsuIndex::new(25);
        assert_eq!(djsu.n_components(), 25);
        for i in 0..25 {
            assert_eq!(djsu.find(i), i);
        }
    }

    #[test]
    fn union_works() {
        let mut djsu = ::DjsuIndex::new(8);
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
        }
        for i in 4..7 {
            for k in 4..7 {
                assert!(djsu.connected(i, k));
            }
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
}
