pub trait Number: Copy {}
impl<T> Number for T where T: Copy {}

struct CollectionAnalyzer<'a, T> {
    numbers: &'a [T],
}
impl<'a, T> From<&'a [T]> for CollectionAnalyzer<'a, T> {
    fn from(value: &'a [T]) -> Self {
        CollectionAnalyzer { numbers: value }
    }
}

impl<'a, T> CollectionAnalyzer<'a, T> {}
