// Copyright 2023 Natalie Baker // AGPLv3 //

use std::marker::PhantomData;
use std::hash::Hash;

use crate::util::SmolStr;

#[derive(Debug)]
#[repr(transparent)]
pub struct StrId<T>(SmolStr, PhantomData<T>);

impl<T> StrId<T> {
    pub const EMPTY: Self = Self(SmolStr::EMPTY, PhantomData);

    pub const fn from_name(id: &str) -> Self {
        Self(SmolStr::new(id), PhantomData)
    }

    pub const fn from_raw(id: u128) -> Self {
        Self(SmolStr::from_raw(id), PhantomData)
    }

    pub fn to_str(self) -> String {
        self.0.to_str()
    }

    pub fn to_raw(self) -> u128 {
        self.0.to_raw()
    }
}

unsafe impl<T> Send for StrId<T> { }
unsafe impl<T> Sync for StrId<T> { }

impl<T> Default for StrId<T> {
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}

impl<T> Copy for StrId<T> { }

impl<T> Clone for StrId<T> {
    fn clone(&self) -> Self {
        *self
    }
}


impl<T> Eq for StrId<T> { }

impl<T> PartialEq for StrId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}


impl<T> Hash for StrId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}


#[macro_export]
macro_rules! newtype_str_id {
    ($vis:vis $name:ident) => {
        
        #[derive(Debug)]
        #[repr(transparent)]
        $vis struct $name<T>($crate::util::StrId<T>);

        impl<T> $name<T> {

            pub const EMPTY: Self = Self($crate::util::StrId::<T>::EMPTY);

            $vis const fn from_name(id: &str) -> Self {
                Self($crate::util::StrId::from_name(id))
            }

            $vis const fn from_raw(id: u128) -> Self {
                Self($crate::util::StrId::from_raw(id))
            }

            $vis fn to_str(&self) -> String {
                self.0.to_str()
            }

            $vis fn to_raw(&self) -> u128 {
                self.0.to_raw()
            }
        }

        impl<T> core::default::Default for $name<T> {
            fn default() -> Self {
                Self(Default::default())
            }
        }

        unsafe impl<T> core::marker::Send for $name<T> { }
        unsafe impl<T> core::marker::Sync for $name<T> { }

        impl<T> core::marker::Copy for $name<T> {
            
        }

        impl<T> core::clone::Clone for $name<T> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<T> core::cmp::Eq for $name<T> {

        }

        impl<T> core::cmp::PartialEq for $name<T> {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }

        impl<T> core::hash::Hash for $name<T> {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }

    };
}

