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

#[macro_export]
macro_rules! noalias {
    ($(#[$attr:meta])* pub type $name:ident = $typ:ty) => {
        $(#[$attr])*
        pub struct $name($typ);

        impl $name {
            pub fn new(value: $typ) -> Self {
                Self(value)
            }
        }

        impl From<$typ> for $name {
            fn from(value: $typ) -> Self {
                Self::new(value)
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Display::fmt(&self.0, f)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn cchar_test() {
        noalias!(pub type CChar = i8);
        let cchar = CChar(10);

        let struc = format!("{cchar}");
        assert_eq!(struc, 10.to_string());
    }
}