use crate::Vectorable;

#[doc(hidden)]
#[macro_export]
#[allow(unused_macros)]
macro_rules! impl_vec {
    ($from: ty, $to: ty, $transform: expr) => {
        impl Vectorable<$to> for $from {
            #[inline]
            fn into(&self) -> Vec<$to> {
                $transform(self)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
#[allow(unused_macros)]
macro_rules! impl_vec_k {
    ($from: ty, $transform: expr) => {
        impl<K: Copy> Vectorable<K> for $from {
            #[inline]
            fn into(&self) -> Vec<K> {
                $transform(self)
            }
        }
    };
}

impl_vec!(&'static str, u8, |x: &'static str| x.as_bytes().to_owned());
impl_vec!(&'static str, char, |x: &'static str| x.chars().collect());
impl_vec!(String, u8, |x: &String| x.as_bytes().to_owned());
impl_vec!(String, char, |x: &String| x.chars().collect());
impl_vec_k!(Vec<K>, |x: &Vec<K>| x.to_owned());
impl_vec_k!(&[K], |x: &[K]| x.to_owned());
