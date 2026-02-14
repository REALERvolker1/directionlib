#![no_std]

macro_rules! enum_matcher_array {
    (
        $( #[$meta:meta] )*
        $vis:vis const $arrayname:ident: $arrayty:ty =
        {
            @default $default:expr;
            $( $variant:ident => $rhs:expr, )+
        }
    ) => {
        $( #[$meta] )*
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
    (@inner_to_primitive $enum:ident) => {
        enum_ordered_array!(@inner_from $enum, u8);
        enum_ordered_array!(@inner_from $enum, i8);
        enum_ordered_array!(@inner_from $enum, u16);
        enum_ordered_array!(@inner_from $enum, i16);
        enum_ordered_array!(@inner_from $enum, u32);
        enum_ordered_array!(@inner_from $enum, i32);
        enum_ordered_array!(@inner_from $enum, u64);
        enum_ordered_array!(@inner_from $enum, i64);
        enum_ordered_array!(@inner_from $enum, u128);
        enum_ordered_array!(@inner_from $enum, i128);
        enum_ordered_array!(@inner_from $enum, usize);
        enum_ordered_array!(@inner_from $enum, isize);

        #[cfg(feature = "num-traits")]
        impl ::num_traits::ToPrimitive for $enum {
            fn to_u8(&self) -> Option<u8> {
                Some((*self).into())
            }
            fn to_i8(&self) -> Option<i8> {
                Some((*self).into())
            }
            fn to_u16(&self) -> Option<u16> {
                Some((*self).into())
            }
            fn to_i16(&self) -> Option<i16> {
                Some((*self).into())
            }
            fn to_u32(&self) -> Option<u32> {
                Some((*self).into())
            }
            fn to_i32(&self) -> Option<i32> {
                Some((*self).into())
            }
            fn to_u64(&self) -> Option<u64> {
                Some((*self).into())
            }
            fn to_i64(&self) -> Option<i64> {
                Some((*self).into())
            }
            fn to_u128(&self) -> Option<u128> {
                Some((*self).into())
            }
            fn to_i128(&self) -> Option<i128> {
                Some((*self).into())
            }
            fn to_usize(&self) -> Option<usize> {
                Some((*self).into())
            }
            fn to_isize(&self) -> Option<isize> {
                Some((*self).into())
            }
            fn to_f32(&self) -> Option<f32> {
                let n: i32 = (*self).into();
                n.to_f32()
            }
            fn to_f64(&self) -> Option<f64> {
                let n: i64 = (*self).into();
                n.to_f64()
            }
        }
    };
    (@inner_from $enum:ident, $ty:ty) => {
        impl From<$enum> for $ty {
            fn from(value: $enum) -> Self {
                value as _
            }
        }
        #[cfg(feature = "num-traits")]
        impl ::num_traits::AsPrimitive<$ty> for $enum {
            fn as_(self) -> $ty {
                self as _
            }
        }
    };
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
        enum_ordered_array!(@inner_to_primitive $enum);
    };
}

enum_ordered_array! {
    /// A simple direction selection enum, designed for matching, selecting, indexing, etc.
    /// For more complex 2D or 3D scenes, consider using a vector instead.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[cfg_attr(feature = "serde", derive(serde_derive::Serialize, serde_derive::Deserialize))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
    #[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::component::Component))]
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
        /// An array of variants, each corresponding to its own "opposite", or "reverse".
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

    /// Get the opposite of the provided `Direction`
    /// ```
    /// use directionlib::Direction;
    ///
    /// let current = Direction::Front;
    ///
    /// assert_eq!(current.reverse(), Direction::Back);
    ///
    /// let vertical = Direction::Down;
    ///
    /// assert_eq!(vertical.reverse(), Direction::Up);
    /// ```
    pub const fn reverse(self) -> Self {
        Self::VARIANTS_OPPOSITE[self as usize]
    }
}
