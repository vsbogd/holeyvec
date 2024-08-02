#[derive(Clone, Debug)]
enum Cell<T> {
    Value(T),
    Hole(usize),
}

#[derive(Clone, Debug)]
/// Vector with holes implementation.
pub struct HoleyVec<T> {
    first_hole: usize,
    vec: Vec<Cell<T>>,
}

impl<T> HoleyVec<T> {

    /// Initialize a new, empty vector.
    pub fn new() -> Self {
        Self{ first_hole: 0, vec: Vec::new() }
    }

    /// Return next index of the empty element. This index is used by [HoleyVec::push] method to
    /// put a passed value.
    ///
    /// ## Examples
    /// ```
    /// use holeyvec::HoleyVec;
    ///
    /// let mut v = HoleyVec::new();
    /// assert_eq!(v.next_index(), 0);
    ///
    /// v.push(42);
    /// assert_eq!(v.next_index(), 1);
    ///
    /// v.remove(0);
    /// assert_eq!(v.next_index(), 0);
    /// ```
    pub fn next_index(&self) -> usize {
        self.first_hole
    }

    /// Return number of indexes used. This number includes both empty and non-empty elements.
    /// 
    /// ## Examples
    /// ```
    /// use holeyvec::HoleyVec;
    ///
    /// let mut v = HoleyVec::new();
    /// assert_eq!(v.index_upper_bound(), 0);
    ///
    /// v.push(42);
    /// assert_eq!(v.index_upper_bound(), 1);
    ///
    /// v.remove(0);
    /// assert_eq!(v.index_upper_bound(), 1);
    /// ```
    pub fn index_upper_bound(&self) -> usize {
        self.vec.len()
    }

    /// Return underlying vector's capacity. Similar to [std::vec::Vec::capacity].
    ///
    /// ## Examples
    /// ```
    /// use holeyvec::HoleyVec;
    ///
    /// let mut v = HoleyVec::new();
    /// assert_eq!(v.capacity(), 0);
    ///
    /// v.push(42);
    /// assert!(v.capacity() >= 1);
    /// ```
    pub fn capacity(&self) -> usize {
        self.vec.capacity()
    }

    /// Check if element by index is empty.
    /// 
    /// ## Examples
    /// ```
    /// use holeyvec::HoleyVec;
    ///
    /// let mut v = HoleyVec::new();
    /// // No holes in empty vector
    /// assert!(!v.is_hole(0));
    ///
    /// v.push(42);
    /// assert!(!v.is_hole(0));
    ///
    /// v.remove(0);
    /// assert!(v.is_hole(0));
    /// ```
    pub fn is_hole(&self, index: usize) -> bool {
        match self.vec.get(index) {
            Some(Cell::Hole(_)) => true,
            _ => false,
        }
    }

    /// Get value by index.
    /// 
    /// ## Examples
    /// ```
    /// use holeyvec::HoleyVec;
    ///
    /// let mut v = HoleyVec::new();
    /// assert_eq!(v.get(0), None);
    ///
    /// v.push(42);
    /// assert_eq!(v.get(0), Some(&42));
    ///
    /// v.remove(0);
    /// assert_eq!(v.get(0), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<&T> {
        match self.vec.get(index) {
            Some(Cell::Value(value)) => Some(value),
            _ => None,
        }
    }

    /// Get mutable value by index.
    /// 
    /// ## Examples
    /// ```
    /// use holeyvec::HoleyVec;
    ///
    /// let mut v = HoleyVec::new();
    /// assert_eq!(v.get_mut(0), None);
    ///
    /// v.push(42);
    /// v.get_mut(0).map(|v| *v = 24);
    /// assert_eq!(v.get(0), Some(&24));
    ///
    /// v.remove(0);
    /// assert_eq!(v.get_mut(0), None);
    /// ```
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match self.vec.get_mut(index) {
            Some(Cell::Value(value)) => Some(value),
            _ => None,
        }
    }

    /// Push value to vector using the index of the first empty element (see
    /// [HoleyVec::next_index]).
    /// 
    /// ## Examples
    /// ```
    /// use holeyvec::HoleyVec;
    ///
    /// let mut v = HoleyVec::new();
    ///
    /// v.push(42);
    /// assert_eq!(v.get(0), Some(&42));
    ///
    /// v.push(43);
    /// assert_eq!(v.get(0), Some(&42));
    /// assert_eq!(v.get(1), Some(&43));
    ///
    /// v.remove(0);
    /// assert_eq!(v.get(0), None);
    /// assert_eq!(v.get(1), Some(&43));
    ///
    /// v.push(24);
    /// assert_eq!(v.get(0), Some(&24));
    /// assert_eq!(v.get(1), Some(&43));
    /// ```
    pub fn push(&mut self, value: T) -> usize {
        if self.first_hole >= self.vec.len() {
            let index = self.vec.len();
            self.vec.push(Cell::Value(value));
            self.first_hole = index + 1;
            index
        } else {
            let index = self.first_hole;
            match self.vec[index] {
                Cell::Hole(next_hole) => {
                    self.first_hole = next_hole;
                    self.vec[index] = Cell::Value(value);
                },
                _ => panic!("Unexpected state"),
            }
            index
        }
    }

    /// Remove value by index.
    /// 
    /// ## Examples
    /// ```
    /// use holeyvec::HoleyVec;
    ///
    /// let mut v = HoleyVec::new();
    ///
    /// v.push(42);
    /// v.push(24);
    /// assert_eq!(v.get(0), Some(&42));
    /// assert_eq!(v.get(1), Some(&24));
    ///
    /// v.remove(0);
    /// assert_eq!(v.get(0), None);
    /// assert_eq!(v.get(1), Some(&24));
    /// ```
    pub fn remove(&mut self, index: usize) -> T {
        let mut value = Cell::Hole(self.first_hole);
        std::mem::swap(&mut self.vec[index], &mut value);
        match value {
            Cell::Value(value) => {
                self.first_hole = index;
                value
            },
            Cell::Hole(_) => {
                panic!("Index doesn't exist");
            }
        }
    }

    /// Return iterator over non empty elements of the vector.
    /// 
    /// ## Examples
    /// ```
    /// use holeyvec::HoleyVec;
    ///
    /// let mut v = HoleyVec::new();
    /// let mut it = v.iter();
    /// assert_eq!(it.next(), None);
    ///
    /// v.push(1);
    /// v.push(2);
    /// v.push(3);
    /// v.remove(1);
    /// let mut it = v.iter();
    /// assert_eq!(it.next(), Some(&1));
    /// assert_eq!(it.next(), Some(&3));
    /// ```
    pub fn iter(&self) -> Iter<T> {
        Iter::new(self)
    }

    /// Return mutable iterator over non empty elements of the vector.
    /// 
    /// ## Examples
    /// ```
    /// use holeyvec::HoleyVec;
    ///
    /// let mut v = HoleyVec::new();
    /// let mut it = v.iter_mut();
    /// assert_eq!(it.next(), None);
    ///
    /// v.push(1);
    /// v.push(2);
    /// v.push(3);
    /// v.remove(1);
    /// let mut it = v.iter_mut();
    /// assert_eq!(it.next(), Some(&mut 1));
    /// assert_eq!(it.next(), Some(&mut 3));
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut::new(self)
    }
}

impl<T> std::ops::Index<usize> for HoleyVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("Index doesn't exist")
    }
}

impl<T> std::ops::IndexMut<usize> for HoleyVec<T> {

    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("Index doesn't exist")
    }
}

/// [HoleyVec] iterator to iterate through non-empty elements of the vector.
pub struct Iter<'a, T> {
    delegate: std::slice::Iter<'a, Cell<T>>
}

impl<'a, T> Iter<'a, T> {
    fn new(vec: &'a HoleyVec<T>) -> Self {
        Self{ delegate: vec.vec.iter() }
    }
}

impl<'a, T> std::iter::Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.delegate.next() {
                None => return None,
                Some(Cell::Hole(_)) => continue,
                Some(Cell::Value(value)) => return Some(value),
            }
        }
    }
}

impl<'a, T> IntoIterator for &'a HoleyVec<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// [HoleyVec] iterator to iterate through non-empty elements of the vector.
pub struct IterMut<'a, T> {
    delegate: std::slice::IterMut<'a, Cell<T>>
}

impl<'a, T> IterMut<'a, T> {
    fn new(vec: &'a mut HoleyVec<T>) -> Self {
        Self{ delegate: vec.vec.iter_mut() }
    }
}

impl<'a, T> std::iter::Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.delegate.next() {
                None => return None,
                Some(Cell::Hole(_)) => continue,
                Some(Cell::Value(value)) => return Some(value),
            }
        }
    }
}

impl<'a, T> IntoIterator for &'a mut HoleyVec<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
