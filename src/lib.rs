#![cfg_attr(not(test), no_std)]
#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![warn(clippy::missing_safety_doc, clippy::undocumented_unsafe_blocks)]
#![allow(
    clippy::too_many_lines,
    reason = "In logo/{a-z}.rs there are functions longer than 100 lines"
)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "There's no point in them, it will make the code cleaner"
)]
#![allow(clippy::cargo_common_metadata)]

#[doc(hidden)]
#[macro_export]
macro_rules! __noalias_impl {
    ($name:ident = $typ:ty) => {
        impl $name {
            #[must_use]
            pub fn new(value: $typ) -> Self {
                Self(value)
            }

            #[must_use]
            pub const fn into_inner(self) -> $typ {
                self.0
            }
        }

        impl AsRef<$typ> for $name {
            fn as_ref(&self) -> &$typ {
                &self.0
            }
        }

        impl From<$typ> for $name {
            fn from(value: $typ) -> Self {
                Self::new(value)
            }
        }

        impl From<$name> for $typ {
            fn from(value: $name) -> Self {
                value.into_inner()
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __noalias_impl_trait {
    (Display for $name:ident = $typ:ty) => {
        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(&self.0, f)
            }
        }
    };

    (Debug for $name:ident = $typ:ty) => {
        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Debug::fmt(&self.0, f)
            }
        }
    };

    (PartialEq for $name:ident = $typ:ty) => {
        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl PartialEq<$typ> for $name {
            fn eq(&self, other: &$typ) -> bool {
                self.0 == *other
            }
        }

        impl PartialEq<$name> for $typ {
            fn eq(&self, other: &$name) -> bool {
                *self == other.0
            }
        }
    };

    (Eq for $name:ident = $typ:ty) => {
        impl Eq for $name {}
    };

    ($other:ident for $name:ident = $typ:ty) => {
        compile_error!(concat!(
            "noalias!: unknown trait `",
            stringify!($other),
            "`, supports only Display, Debug, PartialEq & Eq"
        ));
    };
}

#[macro_export]
macro_rules! noalias {
    // --- WITH TRAITS ---

    // Public
    ($(
        $(#[$attr:meta])*
        pub type $name:ident = $typ:ty [$($traits:ident),+ $(,)?]$(;)?
    )*) => {
        $(
            $(#[$attr])*
            #[repr(transparent)]
            pub struct $name($typ);
            $crate::__noalias_impl!($name = $typ);

            $(
                $crate::__noalias_impl_trait!($traits for $name = $typ);
            )+
        )*
    };

    // Private
    ($(
        $(#[$attr:meta])*
        type $name:ident = $typ:ty [$($traits:ident),+ $(,)?]$(;)?
    )*) => {
        $(
            $(#[$attr])*
            #[repr(transparent)]
            pub struct $name($typ);
            $crate::__noalias_impl!($name = $typ);

            $(
                $crate::__noalias_impl_trait!($traits for $name = $typ);
            )+
        )*
    };

    // --- WITHOUT TRAITS ---

    // Public
    ($(
        $(#[$attr:meta])*
        pub type $name:ident = $typ:ty$(;)?
    )*) => {
        $(
            $(#[$attr])*
            #[repr(transparent)]
            pub struct $name($typ);
            $crate::__noalias_impl!($name = $typ);
        )*
    };

    // Private
    ($(
        $(#[$attr:meta])*
        type $name:ident = $typ:ty$(;)?
    )*) => {
        $(
            $(#[$attr])*
            #[repr(transparent)]
            pub struct $name($typ);
            $crate::__noalias_impl!($name = $typ);
        )*
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn cchar_test() {
        noalias!(pub type CChar = i8 [Display];);
        let cchar = CChar::new(10);

        let struc = format!("{cchar}");
        assert_eq!(struc, 10.to_string());
    }

    #[test]
    fn multi_init_test() {
        use core::any::TypeId;
        fn same_type<T: 'static, U: 'static>() -> bool {
            TypeId::of::<T>() == TypeId::of::<U>()
        }

        noalias!(
            pub type CChar = i8;
            pub type CInt = i32;
            pub type CShort = i16;
        );

        assert!(!same_type::<CChar, i8>());
        assert!(!same_type::<CInt, i32>());
        assert!(!same_type::<CShort, i16>());
    }

    #[test]
    fn into_inner_test() {
        noalias!(pub type CShort = i16);

        let struc = CShort::new(42);
        let inner = struc.into_inner();

        assert_eq!(inner, 42);
    }

    #[test]
    fn partial_eq_test() {
        noalias!(pub type CShort = i16 [PartialEq, Debug]);

        let struc = CShort::new(42);
        assert_eq!(struc, 42);
    }
}