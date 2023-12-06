// Copyright 2023 Natalie Baker // AGPLv3 //

use std::marker::PhantomData;
use std::hash::Hash;

#[derive(Debug)]
#[repr(transparent)]
pub struct StrId<T>(&'static str, PhantomData<T>);


impl<T> StrId<T> {
    pub const fn new(id: &'static str) -> Self {
        Self(id, PhantomData)
    }

    pub fn name(&self) -> &'static str {
        self.0
    }
}

impl<T> Copy for StrId<T> {
    
}

impl<T> Clone for StrId<T> {
    fn clone(&self) -> Self {
        Self(self.0, PhantomData)
    }
}

impl<T> Eq for StrId<T> {

}

impl<T> PartialEq for StrId<T> {
    fn eq(&self, other: &Self) -> bool {
        core::ptr::eq::<str>(self.0, other.0)
    }
}

impl<T> Hash for StrId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.as_ptr().hash(state);
    }
}

#[macro_export]
macro_rules! newtype_str_id {
    ($vis:vis $name:ident) => {
        
        #[derive(Debug)]
        #[repr(transparent)]
        $vis struct $name<T>($crate::StrId<T>);

        impl<T> $name<T> {
            $vis const fn new(id: &'static str) -> Self {
                Self(StrId::new(id))
            }

            $vis fn name(&self) -> &'static str {
                self.0.name()
            }
        }

        impl<T> core::marker::Copy for $name<T> {
            
        }

        impl<T> core::clone::Clone for $name<T> {
            fn clone(&self) -> Self {
                Self(self.0)
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

