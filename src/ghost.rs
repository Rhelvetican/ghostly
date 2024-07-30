use std::{marker::PhantomData, ops::Deref};

/// A ghost type that can be used to enforce invariants at compile time.
///
/// This type is a zero-sized type that can be used to enforce invariants at compile time.
/// It can be used to ensure that certain operations are only performed in certain contexts.
/// For example, you can use it to ensure that a certain operation is only performed in a certain
/// state of a state machine.
///
/// (That was written by ChatGPT and I don't know what it means.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Ghost<T> {
    inner: PhantomData<T>,
}

impl<T> Ghost<T> {
    /// Creates a new `Ghost` instance.
    ///
    /// ```rust
    /// use ghostly::Ghost;
    ///
    /// const GHOST: Ghost<()> = Ghost::new();
    /// ```
    pub const fn new() -> Self {
        Ghost { inner: PhantomData }
    }

    /// Transmutes the type of the `Ghost` instance.
    ///
    /// ```rust
    /// use ghostly::Ghost;
    ///
    /// let ghost: Ghost<()> = Ghost::new();
    /// ```
    pub const fn transmute<U>(self) -> Ghost<U> {
        Ghost::new()
    }

    /// Accesses the `Ghost` instance.
    pub fn access(&mut self, _: T) {
        self.inner = PhantomData;
    }

    /// Transfers the `Ghost` instance to another `Ghost` instance.
    pub fn transfer(self, other: &mut Ghost<T>) {
        other.inner = self.inner;
    }

    /// Wraps the `Ghost` instance in yet another `Ghost` instance.
    ///
    /// Why would you wanna do that? I don't know.
    pub fn wrap_self(self) -> Ghost<Ghost<T>> {
        Ghost::new()
    }
}

impl<T> Deref for Ghost<T> {
    type Target = PhantomData<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
