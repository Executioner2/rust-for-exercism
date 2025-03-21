#[derive(Debug)]
pub struct CustomSet<T> {
    data: Vec<T>,
    phantom: std::marker::PhantomData<T>,
}

impl<T: Ord + Clone> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data.iter().all(|x| other.contains(x))
            && other.data.iter().all(|x| self.contains(x))
    }
}

impl<T: Ord + Clone> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        Self {
            data: input.to_vec(),
            phantom: std::marker::PhantomData,
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.data.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.data.push(element)
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.data.iter().all(|x| other.contains(x))
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        !self.data.iter().any(|x| other.contains(x))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        CustomSet::new(&self.data.iter().filter(|&x| other.contains(x)).cloned().collect::<Vec<_>>())
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        CustomSet::new(&self.data.iter().filter(|&x| !other.contains(x)).cloned().collect::<Vec<_>>())
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        CustomSet::new(&self.data.iter().cloned().chain(other.data.iter().cloned()).collect::<Vec<_>>())
    }
}
