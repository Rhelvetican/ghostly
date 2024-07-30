use std::{marker::PhantomData, ops::Deref};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GhostRef<'a, T> {
    inner: PhantomData<&'a T>,
}

impl<'a, T> GhostRef<'a, T> {
    pub const fn new() -> Self {
        GhostRef { inner: PhantomData }
    }

    pub const fn transmute<U>(self) -> GhostRef<'a, U> {
        GhostRef::new()
    }

    pub fn access(&mut self, _: &'a T) {
        self.inner = PhantomData;
    }

    pub fn wrap_self(self) -> GhostRef<'a, GhostRef<'a, T>> {
        GhostRef::new()
    }
}

impl<'a, T> Deref for GhostRef<'a, T> {
    type Target = PhantomData<&'a T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T> From<PhantomData<&'a T>> for GhostRef<'a, T> {
    fn from(_: PhantomData<&'a T>) -> Self {
        GhostRef::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GhostMutRef<'a, T> {
    inner: PhantomData<&'a T>,
}

impl<'a, T> GhostMutRef<'a, T> {
    pub const fn new() -> Self {
        GhostMutRef { inner: PhantomData }
    }

    pub const fn transmute<U>(self) -> GhostMutRef<'a, U> {
        GhostMutRef::new()
    }

    pub fn access(&mut self, _: &'a T) {
        self.inner = PhantomData;
    }

    pub fn transfer(self, other: &mut GhostMutRef<'a, T>) {
        other.inner = self.inner;
    }

    pub fn wrap_self(self) -> GhostMutRef<'a, GhostMutRef<'a, T>> {
        GhostMutRef::new()
    }
}

impl<'a, T> Deref for GhostMutRef<'a, T> {
    type Target = PhantomData<&'a T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T> From<PhantomData<&'a T>> for GhostMutRef<'a, T> {
    fn from(_: PhantomData<&'a T>) -> Self {
        GhostMutRef::new()
    }
}
