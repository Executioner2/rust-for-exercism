/// Yields each item of a and then each item of b
pub fn append<I, T>(a: I, b: I) -> impl Iterator<Item = T>
where
    I: Iterator<Item = T>,
{
    struct AppendIter<I: Iterator<Item = T>, T> {
        a: I,
        b: I,
    }

    impl<I, T> Iterator for AppendIter<I, T>
    where
        I: Iterator<Item = T>,
    {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.a.next().or_else(|| self.b.next())
        }
    }

    AppendIter { a, b }
}

/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I>(nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    struct ConcatIter<I: Iterator<Item = II>, II: Iterator<Item = T>, T> {
        iter: I,
        current: Option<II>,
    }

    impl<I, II, T> Iterator for ConcatIter<I, II, T>
    where
        I: Iterator<Item = II>,
        II: Iterator<Item = T>,
    {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(val) = self.current.as_mut().and_then(|cur| cur.next()) {
                return Some(val);
            }
            match self.iter.next() {
                Some(iter) => {
                    self.current = Some(iter);
                    self.next()
                }
                None => None,
            }
        }
    }

    ConcatIter {
        iter: nested_iter,
        current: None,
    }
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, F>(iter: I, predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    struct FilterIter<I: Iterator, F> {
        iter: I,
        predicate: F,
    }

    impl<I, F> Iterator for FilterIter<I, F>
    where
        I: Iterator,
        F: Fn(&I::Item) -> bool,
    {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            self.iter.find(|item| (self.predicate)(item))
        }
    }

    FilterIter { iter, predicate }
}

pub fn length<I: Iterator>(iter: I) -> usize {
    let mut count = 0;
    for _ in iter {
        count += 1;
    }
    count
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, U>(iter: I, function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    struct MapIter<I: Iterator, F: Fn(I::Item) -> U, U> {
        iter: I,
        function: F,
    }

    impl<I, F, U> Iterator for MapIter<I, F, U>
    where
        I: Iterator,
        F: Fn(I::Item) -> U,
    {
        type Item = U;

        fn next(&mut self) -> Option<Self::Item> {
            match self.iter.next() {
                Some(item) => Some((self.function)(item)),
                None => None,
            }
        }
    }

    MapIter { iter, function }
}

pub fn foldl<I, F, U>(mut iter: I, initial: U, function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    match iter.next() {
        Some(it) => foldl(iter, function(initial, it), function),
        None => initial,
    }
}

pub fn foldr<I, F, U>(mut iter: I, initial: U, function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    match iter.next_back() {
        Some(it) => foldr(iter, function(initial, it), function),
        None => initial,
    }
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator>(iter: I) -> impl Iterator<Item = I::Item> {
    struct ReverseIter<I: DoubleEndedIterator> {
        iter: I,
    }

    impl<I: DoubleEndedIterator> Iterator for ReverseIter<I> {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next_back()
        }
    }

    ReverseIter { iter }
}
