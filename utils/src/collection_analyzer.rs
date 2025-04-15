use std::ops::AddAssign;

pub trait Number: Copy + AddAssign + Into<f64> {}
impl<T> Number for T where T: Copy + AddAssign + Into<f64> {}

pub struct CollectionAnalyzer<'a, T: Number> {
    data: &'a [T],
}
impl<'a, T: Number> From<&'a [T]> for CollectionAnalyzer<'a, T> {
    fn from(data: &'a [T]) -> Self {
        CollectionAnalyzer { data }
    }
}
impl<'a, T: Number> CollectionAnalyzer<'a, T> {
    /// Calculates the average (mean) of the values in the collection.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::CollectionAnalyzer;
    ///
    /// let data = vec![1.0, 2.0, 3.0];
    /// let collection = CollectionAnalyzer::from(&data[..]);
    /// assert_eq!(collection.average(), 2.0);
    ///
    /// let data: Vec<i32> = vec![];
    /// let collection = CollectionAnalyzer::from(&data[..]);
    /// assert!(collection.average().is_nan());
    /// ```
    pub fn average(&self) -> f64 {
        if self.data.is_empty() {
            return std::f64::NAN;
        }
        let sum: f64 = self.data.iter().map(|&num| num.into()).sum();
        sum / self.data.len() as f64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculates_average_correctly() {
        let data = vec![-3, 0, 3];
        let collection = CollectionAnalyzer::from(&data[..]);
        assert_eq!(collection.average(), 0.0);

        let data = vec![1_u32, 2, 3, 4];
        let collection = CollectionAnalyzer::from(&data[..]);
        assert_eq!(collection.average(), 2.5);

        let data = vec![1.5, 3.5];
        let collection = CollectionAnalyzer::from(&data[..]);
        assert_eq!(collection.average(), 2.5);

        let data = vec![1.0f32, 1.0, 2.0, 2.0];
        let collection = CollectionAnalyzer::from(&data[..]);
        assert_eq!(collection.average(), 1.5);

        let data: Vec<i32> = vec![];
        let collection = CollectionAnalyzer::from(&data[..]);
        assert!(collection.average().is_nan());
    }
}
