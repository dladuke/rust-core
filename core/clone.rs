// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub trait Clone {
    fn clone(&self) -> Self;

    #[inline(always)]
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

impl<T: Clone> Clone for ~T {
    fn clone(&self) -> ~T { ~(**self).clone() }

    fn clone_from(&mut self, source: &~T) {
        **self = (**source).clone()
    }
}

impl<'a, T> Clone for &'a T {
    #[inline(always)]
    fn clone(&self) -> &'a T { *self }
}

impl<'a, T> Clone for &'a [T] {
    #[inline(always)]
    fn clone(&self) -> &'a [T] { *self }
}

impl<'a> Clone for &'a str {
    #[inline(always)]
    fn clone(&self) -> &'a str { *self }
}

impl<T> Clone for *T {
    #[inline(always)]
    fn clone(&self) -> *T { *self }
}

impl<T> Clone for *mut T {
    #[inline(always)]
    fn clone(&self) -> *mut T { *self }
}

macro_rules! clone_impl(
    ($t:ty) => {
        impl Clone for $t {
            #[inline(always)]
            fn clone(&self) -> $t { *self }
        }
    }
)

clone_impl!(int)
clone_impl!(i8)
clone_impl!(i16)
clone_impl!(i32)
clone_impl!(i64)

clone_impl!(uint)
clone_impl!(u8)
clone_impl!(u16)
clone_impl!(u32)
clone_impl!(u64)

clone_impl!(f32)
clone_impl!(f64)

clone_impl!(())
clone_impl!(bool)
clone_impl!(char)

pub trait DeepClone: Clone {
    fn deep_clone(&self) -> Self;

    #[inline(always)]
    fn deep_clone_from(&mut self, source: &Self) {
        *self = source.deep_clone()
    }
}

impl<T: DeepClone> DeepClone for ~T {
    fn deep_clone(&self) -> ~T { ~(**self).deep_clone() }

    fn deep_clone_from(&mut self, source: &~T) {
        **self = (**source).deep_clone()
    }
}

macro_rules! deep_clone_impl(
    ($t:ty) => {
        impl DeepClone for $t {
            #[inline(always)]
            fn deep_clone(&self) -> $t { *self }
        }
    }
)

deep_clone_impl!(int)
deep_clone_impl!(i8)
deep_clone_impl!(i16)
deep_clone_impl!(i32)
deep_clone_impl!(i64)

deep_clone_impl!(uint)
deep_clone_impl!(u8)
deep_clone_impl!(u16)
deep_clone_impl!(u32)
deep_clone_impl!(u64)

deep_clone_impl!(f32)
deep_clone_impl!(f64)

deep_clone_impl!(())
deep_clone_impl!(bool)
deep_clone_impl!(char)
