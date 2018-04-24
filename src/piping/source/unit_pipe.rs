use std::ops::BitOr;

use source::Source;
use piping::source::Pipe;

/// A `UnitPipe` is a simple container wrapping a `Source`
///
/// ```plain
/// ╠════════════
/// ║ ╭────────╮
/// ║ │ Source │
/// ║ ╰────────╯
/// ╠════════════
/// └─┬────────┘
///   └ UnitPipe
/// ```
#[derive(Clone, Debug)]
pub struct UnitPipe<T> {
    source: T,
}

impl<T> UnitPipe<T>
{
    #[inline]
    pub fn new(source: T) -> Self {
        Self { source }
    }
}

impl<T> From<T> for UnitPipe<T>
where
    T: Source,
{
    #[inline]
    fn from(source: T) -> Self {
        Self::new(source)
    }
}

impl<T, Rhs> BitOr<Rhs> for UnitPipe<T> {
    type Output = Pipe<Self, Rhs>;

    #[inline]
    fn bitor(self, rhs: Rhs) -> Self::Output {
        Pipe::new(self, rhs)
    }
}

impl<T> Source for UnitPipe<T>
where
    T: Source,
{
    type Output = T::Output;

    #[inline]
    fn source(&mut self) -> Option<Self::Output> {
        self.source.source()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALUE: usize = 42;

    struct DummySource;

    impl Source for DummySource {
        type Output = usize;

        #[inline]
        fn source(&mut self) -> Option<Self::Output> {
            Some(VALUE)
        }
    }

    #[test]
    fn source() {
        const COUNT: usize = 3;
        let pipe = UnitPipe::new(DummySource);
        let subject: Vec<_> = (0..COUNT).scan(pipe, |pipe, _| {
            pipe.source()
        }).collect();
        let expected = vec![VALUE; COUNT];
        assert_eq!(subject, expected);
    }
}