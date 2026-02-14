#![no_std]

macro_rules! enum_matcher_array {
    ($vis:vis const $arrayname:ident: $arrayty:ty = { @default $default:expr; $( $variant:ident => $rhs:expr, )+ }) => {
        $vis const $arrayname: [$arrayty; Self::COUNT] = {
            let mut out = [$default; _];
            let mut i = 0;

            while i != out.len() {
                out[i] = match Self::VARIANT_ARRAY[i] {
                    $( Self::$variant => $rhs, )+
                };
                i += 1;
            }

            out
        };
    };
}

macro_rules! enum_ordered_array {
    (
        $( #[$meta:meta] )*
        $vis:vis enum $enum:ident {
            $(
                $( #[$variant_meta:meta] )*
                $variant:ident = $repr:expr,
            )+
        }
    ) => {
        $( #[$meta] )*
        $vis enum $enum {
            $(
                $( #[$variant_meta] )*
                $variant = $repr,
            )+
        }
        impl $enum {
            /// The number of variants
            pub const COUNT: usize = *&[$( Self::$variant ),+].len();
            /// An ordered array of all variants
            pub const VARIANT_ARRAY: [Self; Self::COUNT] = {
                let output = [$( Self::$variant ),+];

                let mut i = 0;
                let mut last = -1;
                while i != Self::COUNT {
                    let current = output[i] as isize;
                    // Ensures the variants match 1:1 to the indices
                    assert!(current == last + 1);
                    last = current;
                    i += 1;
                }

                output
            };
            /// All enum variants in order
            pub const VARIANT_NAMES: [&'static str; Self::COUNT] = [$( ::core::stringify!($variant) ),+];

            /// Get the `Direction` corresponding to the input.
            /// # Panics
            /// This function will panic if `n` is out of range.
            #[inline(always)]
            pub const fn from_repr(n: u8) -> Self {
                Self::VARIANT_ARRAY[n as usize]
            }
            /// Get the `Direction` corresponding to the input, or `None` if the input is out of range.
            #[inline]
            pub const fn try_from_repr(n: u8) -> Option<Self> {
                const MAX_NUMERIC: u8 = $enum::VARIANT_ARRAY[$enum::VARIANT_ARRAY.len() - 1] as _;
                if n > MAX_NUMERIC {
                    None
                } else {
                    Some(Self::from_repr(n))
                }
            }

            /// Get the variant as a static string slice
            #[inline(always)]
            pub const fn to_str(self) -> &'static str {
                Self::VARIANT_NAMES[self as usize]
            }
        }
        impl ::core::fmt::Display for $enum {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(self.to_str())
            }
        }
    };
}

enum_ordered_array! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Direction {
        Up = 0,
        Right = 1,
        Front = 2,

        Down = 3,
        Left = 4,
        Back = 5,
    }
}

impl Direction {
    enum_matcher_array! {
        pub const VARIANTS_OPPOSITE: Self = {
            @default Self::Up;
            Back => Self::Front,
            Front => Self::Back,
            Left => Self::Right,
            Right => Self::Left,
            Up => Self::Down,
            Down => Self::Up,
        }
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
