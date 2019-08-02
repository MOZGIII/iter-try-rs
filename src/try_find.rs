use std::ops::Try;

pub trait TryFindExt<Tryable: Try<Ok = Option<Self::Item>>>: Iterator {
    /// Applies function to the elements of iterator and returns
    /// the first non-none result or the first error.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use iter_try::TryFindExt;
    ///
    /// let a = ["1", "2", "lol", "NaN", "5"];
    ///
    /// let is_my_num = |s: &str, search: i32| -> Result<bool, std::num::ParseIntError> {
    ///     Ok(s.parse::<i32>()?  == search)
    /// };
    ///
    /// let result = a.iter().try_find(|&&s| is_my_num(s, 2));
    /// assert_eq!(result, Ok(Some(&"2")));
    ///
    /// let result = a.iter().try_find(|&&s| is_my_num(s, 5));
    /// assert!(result.is_err());
    /// ```
    fn try_find<F, R>(&mut self, mut f: F) -> Tryable
    where
        Self: Sized,
        R: Try<Ok = bool, Error = Tryable::Error>,
        F: FnMut(&Self::Item) -> R,
    {
        let done = self.try_for_each(move |x| match f(&x).into_result() {
            Ok(false) => Ok(()),
            Ok(true) => Err(Ok(x)),
            Err(x) => Err(Err(x)),
        });
        let result = match done {
            Ok(..) => None,
            Err(x) => Some(x),
        }
        .transpose();
        match result {
            Ok(x) => Tryable::from_ok(x),
            Err(x) => Tryable::from_error(x),
        }
    }
}

/// We only provide gneeric implementation for Result type to make the API
/// usable without requiring use to provide the type at every use to
/// disambiguate the inferrence and allow for elegant use with `?` operator.
impl<I: Iterator<Item = Item>, Item, E> TryFindExt<Result<Option<Item>, E>> for I {}

#[test]
fn test_try_find() {
    let xs: &[isize] = &[];
    assert_eq!(xs.iter().try_find(testfn), Ok(None));
    let xs: &[isize] = &[1, 2, 3, 4];
    assert_eq!(xs.iter().try_find(testfn), Ok(Some(&2)));
    let xs: &[isize] = &[1, 3, 4];
    assert_eq!(xs.iter().try_find(testfn), Err(()));

    let xs: &[isize] = &[1, 2, 3, 4, 5, 6, 7];
    let mut iter = xs.iter();
    assert_eq!(iter.try_find(testfn), Ok(Some(&2)));
    assert_eq!(iter.try_find(testfn), Err(()));
    assert_eq!(iter.next(), Some(&5));

    fn testfn(x: &&isize) -> Result<bool, ()> {
        if **x == 2 {
            return Ok(true);
        }
        if **x == 4 {
            return Err(());
        }
        Ok(false)
    }
}
