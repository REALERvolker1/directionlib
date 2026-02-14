macro_rules! enum_matcher_array {
    (
        $( #[$meta:meta] )*
        $vis:vis const $arrayname:ident: $arrayty:ty = {
            $( $variant:ident => $rhs:expr, )+
        }
    ) => {
        enum_matcher_array!(
            $( #[$meta] )*
            $vis const $arrayname: [$arrayty; Self::COUNT] = {
                @selections Self::VARIANT_ARRAY;
                $( Self::$variant => $rhs, )+
            }
        );
    };
    (
        $( #[$meta:meta] )*
        $vis:vis const $arrayname:ident: [$arrayty:ty; $len:expr] = {
            @selections $selections:expr;
            $( $matches:pat => $rhs:expr, )+
        }
    ) => {
        $( #[$meta] )*
        $vis const $arrayname: [$arrayty; $len] = {
            let mut out = [$selections[0]; _];
            let mut i = 0;

            while i != out.len() {
                out[i] = match $selections[i] {
                    $( $matches => $rhs, )+
                };
                i += 1;
            }

            out
        };
    };
}
pub(crate) use enum_matcher_array;

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
            #[inline(always)]
            fn to_u8(&self) -> Option<u8> {
                Some((*self).into())
            }
            #[inline(always)]
            fn to_i8(&self) -> Option<i8> {
                Some((*self).into())
            }
            #[inline(always)]
            fn to_u16(&self) -> Option<u16> {
                Some((*self).into())
            }
            #[inline]
            fn to_i16(&self) -> Option<i16> {
                Some((*self).into())
            }
            #[inline]
            fn to_u32(&self) -> Option<u32> {
                Some((*self).into())
            }
            #[inline]
            fn to_i32(&self) -> Option<i32> {
                Some((*self).into())
            }
            #[inline]
            fn to_u64(&self) -> Option<u64> {
                Some((*self).into())
            }
            #[inline]
            fn to_i64(&self) -> Option<i64> {
                Some((*self).into())
            }
            #[inline]
            fn to_u128(&self) -> Option<u128> {
                Some((*self).into())
            }
            #[inline]
            fn to_i128(&self) -> Option<i128> {
                Some((*self).into())
            }
            #[inline]
            fn to_usize(&self) -> Option<usize> {
                Some((*self).into())
            }
            #[inline]
            fn to_isize(&self) -> Option<isize> {
                Some((*self).into())
            }
            #[inline]
            fn to_f32(&self) -> Option<f32> {
                let n: i32 = (*self).into();
                n.to_f32()
            }
            #[inline]
            fn to_f64(&self) -> Option<f64> {
                let n: i64 = (*self).into();
                n.to_f64()
            }
        }
    };
    (@inner_from $enum:ident, $ty:ty) => {
        impl From<$enum> for $ty {
            #[inline(always)]
            fn from(value: $enum) -> Self {
                value as _
            }
        }
        #[cfg(feature = "num-traits")]
        impl ::num_traits::AsPrimitive<$ty> for $enum {
            #[inline(always)]
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
        #[cfg(feature = "arbitrary")]
        impl<'a> arbitrary::Arbitrary<'a> for $enum {
            #[inline]
            fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
                let n = u.choose(&Self::VARIANT_ARRAY)?;
                Ok(*n)
            }
        }

        enum_ordered_array!(@inner_to_primitive $enum);
    };
}
pub(crate) use enum_ordered_array;
