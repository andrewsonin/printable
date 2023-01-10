use std::fmt::{Display, Formatter};

/// Printable wrapper.
#[derive(Clone, Copy)]
pub struct Printable<'a, T> {
    data: T,
    sep: &'a str,
    left_bound: &'a str,
    right_bound: &'a str,
}

impl<'a, T> Printable<'a, T>
{
    /// Customizes separator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use printable::AsPrintable;
    ///
    /// let v = vec![1, 2, 3];
    /// assert_eq!(format!("{}", v.iter().printable()), "[1, 2, 3]");
    /// assert_eq!(format!("{}", v.iter().printable().with_separator(".")), "[1.2.3]")
    /// ```
    pub fn with_separator<'b>(self, sep: &'b str) -> Printable<'b, T>
        where 'a: 'b
    {
        let Self {
            data,
            left_bound,
            right_bound,
            ..
        } = self;
        Printable {
            data,
            sep,
            left_bound,
            right_bound,
        }
    }

    /// Customizes left bound.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use printable::AsPrintable;
    ///
    /// let v = vec![1, 2, 3];
    /// assert_eq!(format!("{}", v.iter().printable()), "[1, 2, 3]");
    /// assert_eq!(format!("{}", v.iter().printable().with_left_bound("{")), "{1, 2, 3]")
    /// ```
    pub fn with_left_bound<'b>(self, left_bound: &'b str) -> Printable<'b, T>
        where 'a: 'b
    {
        let Self {
            data,
            right_bound,
            sep,
            ..
        } = self;
        Printable {
            data,
            sep,
            left_bound,
            right_bound,
        }
    }

    /// Customizes right bound.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use printable::AsPrintable;
    ///
    /// let v = vec![1, 2, 3];
    /// assert_eq!(format!("{}", v.iter().printable()), "[1, 2, 3]");
    /// assert_eq!(format!("{}", v.iter().printable().with_right_bound("}")), "[1, 2, 3}")
    /// ```
    pub fn with_right_bound<'b>(self, right_bound: &'b str) -> Printable<'b, T>
        where 'a: 'b
    {
        let Self {
            data,
            left_bound,
            sep,
            ..
        } = self;
        Printable {
            data,
            sep,
            left_bound,
            right_bound,
        }
    }
}

/// Wrap iterators into [`Printable`].
///
/// # Warning
///
/// Avoid creating [`Printable`] from memory-owning iterators such as [`std::vec::IntoIter`],
/// since it clones the owned data every time [`Display::fmt`] is called.
///
/// # Examples
///
/// ```rust
/// use printable::AsPrintable;
///
/// let v = vec![1, 2, 3];
/// assert_eq!(format!("{}", v.iter().printable()), "[1, 2, 3]");
///
/// let v: Vec<usize> = vec![1];
/// assert_eq!(format!("{}", v.iter().printable()), "[1]");
///
/// let v: Vec<usize> = vec![];
/// assert_eq!(format!("{}", v.iter().printable()), "[]")
/// ```
pub trait AsPrintable: Iterator + Clone
    where
        Self::Item: Display,
{
    /// Wrap custom struct that can produce printable iterator into [`Printable`].
    fn printable(self) -> Printable<'static, Self> {
        Printable {
            data: self,
            sep: ", ",
            left_bound: "[",
            right_bound: "]",
        }
    }
}

impl<T> AsPrintable for T
    where
        T: Iterator + Clone,
        T::Item: Display,
{}

impl<'a, T> From<T> for Printable<'a, T::IntoIter>
    where
        T: IntoIterator,
        T::Item: Display,
        T::IntoIter: Clone
{
    fn from(value: T) -> Self {
        value.into_iter().printable()
    }
}

impl<'a, T> Display for Printable<'a, T>
    where
        T: Clone + Iterator,
        T::Item: Display
{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result
    {
        let Self {
            data,
            sep,
            left_bound,
            right_bound
        } = self;
        f.write_str(left_bound)?;
        let mut iterator = data.clone();
        if let Some(v) = iterator.next()
        {
            v.fmt(f)?;
            for v in iterator
            {
                f.write_str(sep)?;
                v.fmt(f)?
            }
        }
        f.write_str(right_bound)
    }
}