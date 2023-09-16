pub use frunk;
use frunk::hlist::Sculptor;
pub use plugs_derive::*;
use std::{ops::Deref, sync::Arc};

use crate as plugs;

pub struct Plug<T> {
    inner: Arc<T>,
}

impl<T> Plug<T> {
    pub fn new(plug: T) -> Self {
        Self {
            inner: Arc::new(plug),
        }
    }
}

impl<T> Clone for Plug<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Deref for Plug<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.inner
    }
}

pub trait Bundle: Sized + 'static {
    type PlugList<'a>: frunk::hlist::HList;
    fn from_refs(hlist: Self::PlugList<'_>) -> Self;
    fn refs(&self) -> Self::PlugList<'_>;

    fn query<Sub, Magic>(&self) -> Sub
    where
        Sub: Bundle,
        for<'a> Self::PlugList<'a>: frunk::hlist::Sculptor<Sub::PlugList<'a>, Magic>,
    {
        Sub::from_refs(self.refs().sculpt().0)
    }

    fn insert<T>(self, plug: T) -> PlugList<T, Self> {
        PlugList {
            plug: Plug::new(plug),
            other: self,
        }
    }
}

#[derive(Bundle, Copy, Clone)]
pub struct EmptyBundle;

#[derive(Bundle)]
pub struct PlugList<T: 'static, Other: Bundle> {
    plug: Plug<T>,
    #[bundle(other)]
    other: Other,
}
