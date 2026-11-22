//! One way to write a refusal: an enum whose every variant has a fixed
//! text. A refusal names the rule it broke and never repeats the input
//! (spec 0155, "Requests").

/// Only types of this crate may be a log outcome (see `log_chain`).
pub trait Sealed {}

/// Defines a refusal enum and its [`std::fmt::Display`], one fixed text
/// for each variant.
macro_rules! refusals {
    (
        $(#[$meta:meta])*
        $name:ident {
            $($(#[$vmeta:meta])* $variant:ident => $text:literal,)+
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name {
            $($(#[$vmeta])* $variant,)+
        }

        impl $crate::refusal::Sealed for $name {}

        impl $crate::log_chain::Outcome for $name {}

        impl ::std::fmt::Display for $name {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter<'_>,
            ) -> ::std::fmt::Result {
                f.write_str(match self {
                    $(Self::$variant => $text,)+
                })
            }
        }
    };
}
