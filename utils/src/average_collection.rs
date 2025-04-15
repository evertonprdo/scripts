use crate::Number;

pub struct AverageCollection<T: Number> {
    values: Vec<T>,
    average: Option<f64>,
}
impl<T: Number> From<Vec<T>> for AverageCollection<T> {
    fn from(value: Vec<T>) -> Self {
        AverageCollection {
            values: value,
            average: None,
        }
    }
}

impl<T> AverageCollection<T>
where
    T: Number,
{
    pub fn add(&mut self, value: T) {
        self.average = None;
        self.values.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.average = None;
        self.values.pop()
    }

    /// Calculates the average of the collection.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::AverageCollection;
    ///
    /// let mut data = AverageCollection::from(vec![1.0, 2.0, 3.0]);
    /// assert_eq!(data.average(), Some(2.0));
    /// ```
    ///
    /// If the collection has no elements, the average returns `None`.
    ///
    /// ```
    /// use utils::AverageCollection;
    ///
    /// let mut data: AverageCollection<f64> = AverageCollection::from(vec![]);
    /// assert_eq!(data.average(), None);
    /// ```
    pub fn average(&mut self) -> Option<f64> {
        if self.average == None && !self.values.is_empty() {
            self.average = Some(self.calc_average());
        }

        self.average
    }

    fn calc_average(&self) -> f64 {
        if self.values.is_empty() {
            panic!("Cannot calculate average of empty array");
        }

        let mut iter = self.values.iter();
        let mut sum = *iter.next().unwrap();

        while let Some(value) = iter.next() {
            sum += *value
        }

        let sum: f64 = sum.into();
        sum / self.values.len() as f64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn average_cache() {
        let mut collection = AverageCollection::from(vec![5.0]);

        assert_eq!(collection.average, None);
        assert_eq!(collection.average(), Some(5.0));
        assert_eq!(collection.average, Some(5.0));

        collection.add(10.0);
        assert_eq!(collection.average, None);
        assert_eq!(collection.average(), Some(7.5));
        assert_eq!(collection.average, Some(7.5));

        assert_eq!(collection.pop(), Some(10.0));
        assert_eq!(collection.average, None);

        assert_eq!(collection.pop(), Some(5.0));
        assert_eq!(collection.pop(), None);

        assert_eq!(collection.average(), None);
    }

    #[test]
    #[should_panic(expected = "Cannot calculate")]
    fn average_empty_array() {
        let data: AverageCollection<f64> = AverageCollection::from(vec![]);
        data.calc_average();
    }
}
