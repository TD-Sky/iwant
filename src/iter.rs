use std::iter::Peekable;

pub trait PeekExt: Iterator {
    fn on_some<F, E>(self, f: F) -> Result<(), E>
    where
        F: FnOnce(Peekable<Self>) -> Result<(), E>,
        Self: Sized;
}

impl<I, T> PeekExt for I
where
    I: Iterator<Item = T>,
{
    fn on_some<F, E>(self, f: F) -> Result<(), E>
    where
        F: FnOnce(Peekable<Self>) -> Result<(), E>,
        Self: Sized,
    {
        let mut iter = self.peekable();
        if iter.peek().is_some() {
            f(iter)
        } else {
            Ok(())
        }
    }
}
