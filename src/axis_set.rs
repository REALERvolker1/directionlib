//! A structure for bundling axis-specific information
// SPDX-License-Identifier: LGPL-3.0-only

#[cfg(feature = "arbitrary")]
use ::arbitrary::Arbitrary;

use {
    crate::{AxisIndex, SignedAxis, SignedAxisIndex},
    ::core::{
        mem::MaybeUninit,
        ops::{
            Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
            DivAssign, Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
            SubAssign,
        },
    },
};

/// A set of six `T` corresponding to axes
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck_derive::AnyBitPattern))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::component::Component))]
pub struct SignedAxisSet<T> {
    /// Corresponds to [`SignedAxis::XPos`] or [`Axis::X`](crate::Axis::X)
    pub pos_x: T,
    /// Corresponds to [`SignedAxis::XNeg`]
    pub neg_x: T,
    /// Corresponds to [`SignedAxis::YPos`] or [`Axis::Y`](crate::Axis::Y)
    pub pos_y: T,
    /// Corresponds to [`SignedAxis::YNeg`]
    pub neg_y: T,
    /// Corresponds to [`SignedAxis::ZPos`] or [`Axis::Z`](crate::Axis::Z)
    pub pos_z: T,
    /// Corresponds to [`SignedAxis::ZNeg`]
    pub neg_z: T,
}
impl<T> SignedAxisSet<T> {
    /// All the axes that each element corresponds with.
    pub const AXES: SignedAxisSet<SignedAxis> = SignedAxisSet {
        pos_x: SignedAxis::XPos,
        neg_x: SignedAxis::XNeg,
        pos_y: SignedAxis::YPos,
        neg_y: SignedAxis::YNeg,
        pos_z: SignedAxis::ZPos,
        neg_z: SignedAxis::ZNeg,
    };
    /// Fill in all the fields with a single value.
    pub const fn splat(value: T) -> Self
    where
        T: Copy,
    {
        Self {
            pos_x: value,
            neg_x: value,
            pos_y: value,
            neg_y: value,
            pos_z: value,
            neg_z: value,
        }
    }
    /// Fill in all the fields with the result of a function call.
    ///
    /// The order in which the function is called is undefined.
    pub fn splat_with(mut f: impl FnMut(SignedAxis) -> T) -> Self {
        Self {
            pos_x: f(SignedAxis::XPos),
            neg_x: f(SignedAxis::XNeg),
            pos_y: f(SignedAxis::YPos),
            neg_y: f(SignedAxis::YNeg),
            pos_z: f(SignedAxis::ZPos),
            neg_z: f(SignedAxis::ZNeg),
        }
    }
    /// Get a reference to each element
    pub const fn as_ref(&self) -> SignedAxisSet<&T> {
        SignedAxisSet {
            pos_x: &self.pos_x,
            neg_x: &self.neg_x,
            pos_y: &self.pos_y,
            neg_y: &self.neg_y,
            pos_z: &self.pos_z,
            neg_z: &self.neg_z,
        }
    }
    /// Get a mutable reference to each element
    pub const fn as_mut(&mut self) -> SignedAxisSet<&mut T> {
        SignedAxisSet {
            pos_x: &mut self.pos_x,
            neg_x: &mut self.neg_x,
            pos_y: &mut self.pos_y,
            neg_y: &mut self.neg_y,
            pos_z: &mut self.pos_z,
            neg_z: &mut self.neg_z,
        }
    }
    /// Bundle the contents of this set and another into a single set
    pub fn zip<U>(self, rhs: SignedAxisSet<U>) -> SignedAxisSet<(T, U)> {
        SignedAxisSet {
            pos_x: (self.pos_x, rhs.pos_x),
            neg_x: (self.neg_x, rhs.neg_x),
            pos_y: (self.pos_y, rhs.pos_y),
            neg_y: (self.neg_y, rhs.neg_y),
            pos_z: (self.pos_z, rhs.pos_z),
            neg_z: (self.neg_z, rhs.neg_z),
        }
    }
    /// Convert the members of this set into members of another set.
    ///
    /// The order in which the function is called is undefined. Rely on the index enum provided.
    pub fn map<U>(self, mut f: impl FnMut(SignedAxis, T) -> U) -> SignedAxisSet<U> {
        SignedAxisSet {
            pos_x: f(SignedAxis::XPos, self.pos_x),
            neg_x: f(SignedAxis::XNeg, self.neg_x),
            pos_y: f(SignedAxis::YPos, self.pos_y),
            neg_y: f(SignedAxis::YNeg, self.neg_y),
            pos_z: f(SignedAxis::ZPos, self.pos_z),
            neg_z: f(SignedAxis::ZNeg, self.neg_z),
        }
    }

    /// Call a function for each element.
    ///
    /// The order in which the function is called is undefined. Rely on the index enum provided.
    pub fn for_each(self, mut f: impl FnMut(SignedAxis, T)) {
        f(SignedAxis::XPos, self.pos_x);
        f(SignedAxis::XNeg, self.neg_x);
        f(SignedAxis::YPos, self.pos_y);
        f(SignedAxis::YNeg, self.neg_y);
        f(SignedAxis::ZPos, self.pos_z);
        f(SignedAxis::ZNeg, self.neg_z);
    }
    /// Call a function for each element ref.
    ///
    /// The order in which the function is called is undefined. Rely on the index enum provided.
    pub fn for_each_ref(&self, mut f: impl FnMut(SignedAxis, &T)) {
        f(SignedAxis::XPos, &self.pos_x);
        f(SignedAxis::XNeg, &self.neg_x);
        f(SignedAxis::YPos, &self.pos_y);
        f(SignedAxis::YNeg, &self.neg_y);
        f(SignedAxis::ZPos, &self.pos_z);
        f(SignedAxis::ZNeg, &self.neg_z);
    }
    /// Call a function for each element, potentially mutating it.
    ///
    /// The order in which the function is called is undefined. Rely on the index enum provided.
    pub fn for_each_mut(&mut self, mut f: impl FnMut(SignedAxis, &mut T)) {
        f(SignedAxis::XPos, &mut self.pos_x);
        f(SignedAxis::XNeg, &mut self.neg_x);
        f(SignedAxis::YPos, &mut self.pos_y);
        f(SignedAxis::YNeg, &mut self.neg_y);
        f(SignedAxis::ZPos, &mut self.pos_z);
        f(SignedAxis::ZNeg, &mut self.neg_z);
    }
    /// Call a fallible function for each element.
    ///
    /// The order in which the function is called is undefined. Rely on the index enum provided.
    pub fn try_for_each<E>(
        &self,
        mut f: impl FnMut(SignedAxis, &T) -> Result<(), E>,
    ) -> Result<(), E> {
        f(SignedAxis::XPos, &self.pos_x)?;
        f(SignedAxis::XNeg, &self.neg_x)?;
        f(SignedAxis::YPos, &self.pos_y)?;
        f(SignedAxis::YNeg, &self.neg_y)?;
        f(SignedAxis::ZPos, &self.pos_z)?;
        f(SignedAxis::ZNeg, &self.neg_z)?;
        Ok(())
    }

    /// Returns an array of references,
    /// where each index is directly mapped to its corresponding [`SignedAxis`].
    pub const fn as_ref_array(&self) -> [&T; 6] {
        let mut out = [&self.neg_x; _];

        // we want to keep this in order!

        out[SignedAxis::XNeg as usize] = &self.neg_x;
        out[SignedAxis::XPos as usize] = &self.pos_x;
        out[SignedAxis::YNeg as usize] = &self.neg_y;
        out[SignedAxis::YPos as usize] = &self.pos_y;
        out[SignedAxis::ZNeg as usize] = &self.neg_z;
        out[SignedAxis::ZPos as usize] = &self.pos_z;

        out
    }
    /// Returns an array of references,
    /// where each index is directly mapped to its corresponding [`SignedAxis`].
    pub const fn as_mut_array(&mut self) -> [&mut T; 6] {
        let mut out: [MaybeUninit<&mut T>; 6] = const { [const { MaybeUninit::uninit() }; _] };

        // we want to keep this in order!

        out[SignedAxis::XNeg as usize].write(&mut self.neg_x);
        out[SignedAxis::XPos as usize].write(&mut self.pos_x);
        out[SignedAxis::YNeg as usize].write(&mut self.neg_y);
        out[SignedAxis::YPos as usize].write(&mut self.pos_y);
        out[SignedAxis::ZNeg as usize].write(&mut self.neg_z);
        out[SignedAxis::ZPos as usize].write(&mut self.pos_z);

        // SAFETY: We initialized them all
        unsafe {
            [
                out[0].assume_init_read(),
                out[1].assume_init_read(),
                out[2].assume_init_read(),
                out[3].assume_init_read(),
                out[4].assume_init_read(),
                out[5].assume_init_read(),
            ]
        }
    }
    /// Consumes the set, returning an array
    /// where each index is directly mapped to its corresponding [`SignedAxis`].
    pub fn into_array(self) -> [T; 6] {
        let mut out: [MaybeUninit<T>; 6] = const { [const { MaybeUninit::uninit() }; _] };

        // we want to keep this in order!

        out[SignedAxis::XNeg as usize].write(self.neg_x);
        out[SignedAxis::XPos as usize].write(self.pos_x);
        out[SignedAxis::YNeg as usize].write(self.neg_y);
        out[SignedAxis::YPos as usize].write(self.pos_y);
        out[SignedAxis::ZNeg as usize].write(self.neg_z);
        out[SignedAxis::ZPos as usize].write(self.pos_z);

        // SAFETY: We initialized them all
        unsafe {
            [
                out[0].assume_init_read(),
                out[1].assume_init_read(),
                out[2].assume_init_read(),
                out[3].assume_init_read(),
                out[4].assume_init_read(),
                out[5].assume_init_read(),
            ]
        }
    }

    /// Create a set where each element is converted into [`Option::Some`]
    pub fn into_optional(self) -> SignedAxisSet<Option<T>> {
        self.map(|_, s| Some(s))
    }
}
impl<T> SignedAxisSet<MaybeUninit<T>> {
    /// 6x [`MaybeUninit::uninit`]
    pub const fn uninit() -> Self {
        Self {
            pos_x: MaybeUninit::uninit(),
            neg_x: MaybeUninit::uninit(),
            pos_y: MaybeUninit::uninit(),
            neg_y: MaybeUninit::uninit(),
            pos_z: MaybeUninit::uninit(),
            neg_z: MaybeUninit::uninit(),
        }
    }
    /// 6x [`MaybeUninit::new`]
    pub fn new(values: SignedAxisSet<T>) -> Self {
        values.map(|_, n| MaybeUninit::new(n))
    }
    /// Calls [`MaybeUninit::assume_init_ref`] on each element.
    /// # Safety
    /// Calling this function when even ONE of the elements is uninitialized causes Undefined Behavior.
    pub const unsafe fn assume_init_ref(&self) -> SignedAxisSet<&T> {
        // SAFETY: Caller asserts invariants are upheld
        unsafe {
            SignedAxisSet {
                pos_x: self.pos_x.assume_init_ref(),
                neg_x: self.neg_x.assume_init_ref(),
                pos_y: self.pos_y.assume_init_ref(),
                neg_y: self.neg_y.assume_init_ref(),
                pos_z: self.pos_z.assume_init_ref(),
                neg_z: self.neg_z.assume_init_ref(),
            }
        }
    }
    /// Calls [`MaybeUninit::assume_init_mut`] on each element.
    /// # Safety
    /// Calling this function when even ONE of the elements is uninitialized causes Undefined Behavior.
    pub const unsafe fn assume_init_mut(&mut self) -> SignedAxisSet<&mut T> {
        // SAFETY: Caller asserts invariants are upheld
        unsafe {
            SignedAxisSet {
                pos_x: self.pos_x.assume_init_mut(),
                neg_x: self.neg_x.assume_init_mut(),
                pos_y: self.pos_y.assume_init_mut(),
                neg_y: self.neg_y.assume_init_mut(),
                pos_z: self.pos_z.assume_init_mut(),
                neg_z: self.neg_z.assume_init_mut(),
            }
        }
    }
    /// Calls [`MaybeUninit::assume_init_read`] on each element.
    /// Whenever possible, it is preferable to use [`assume_init`](Self::assume_init) instead, which prevents duplicating the content of the `SignedAxisSet<MaybeUninit<T>>`.
    /// # Safety
    /// Calling this function when even ONE of the elements is uninitialized causes Undefined Behavior.
    pub const unsafe fn assume_init_read(&self) -> SignedAxisSet<T> {
        // SAFETY: Caller asserts invariants are upheld
        unsafe {
            SignedAxisSet {
                pos_x: self.pos_x.assume_init_read(),
                neg_x: self.neg_x.assume_init_read(),
                pos_y: self.pos_y.assume_init_read(),
                neg_y: self.neg_y.assume_init_read(),
                pos_z: self.pos_z.assume_init_read(),
                neg_z: self.neg_z.assume_init_read(),
            }
        }
    }
    /// Calls [`MaybeUninit::assume_init`] on each element.
    /// # Safety
    /// Calling this function when even ONE of the elements is uninitialized causes Undefined Behavior.
    pub unsafe fn assume_init(self) -> SignedAxisSet<T> {
        // SAFETY: Caller asserts invariants are upheld
        self.map(|_, n| unsafe { n.assume_init() })
    }
}

#[cfg(feature = "arbitrary")]
impl<'a, T: Arbitrary<'a>> Arbitrary<'a> for SignedAxisSet<T> {
    #[inline]
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Self {
            pos_x: u.arbitrary()?,
            neg_x: u.arbitrary()?,
            pos_y: u.arbitrary()?,
            neg_y: u.arbitrary()?,
            pos_z: u.arbitrary()?,
            neg_z: u.arbitrary()?,
        })
    }
}
impl<T> SignedAxisSet<Option<T>> {
    /// Take the value out of the selected option, leaving behind `None`
    pub const fn take(&mut self, axis: SignedAxis) -> Option<T> {
        match axis {
            SignedAxis::XNeg => self.neg_x.take(),
            SignedAxis::XPos => self.pos_x.take(),
            SignedAxis::YNeg => self.neg_y.take(),
            SignedAxis::YPos => self.pos_y.take(),
            SignedAxis::ZNeg => self.neg_z.take(),
            SignedAxis::ZPos => self.pos_z.take(),
        }
    }
}
impl<'a, T> AxisIndex for &'a SignedAxisSet<T> {
    type Item = &'a T;
    fn axis_x(&self) -> Self::Item {
        &self.pos_x
    }
    fn axis_y(&self) -> Self::Item {
        &self.pos_y
    }
    fn axis_z(&self) -> Self::Item {
        &self.pos_z
    }
}
impl<T: Copy> AxisIndex for SignedAxisSet<T> {
    type Item = T;
    fn axis_x(&self) -> Self::Item {
        self.pos_x
    }
    fn axis_y(&self) -> Self::Item {
        self.pos_y
    }
    fn axis_z(&self) -> Self::Item {
        self.pos_z
    }
}
impl<T> SignedAxisIndex for &'_ SignedAxisSet<T> {
    fn axis_neg_x(&self) -> Self::Item {
        &self.neg_x
    }
    fn axis_neg_y(&self) -> Self::Item {
        &self.neg_y
    }
    fn axis_neg_z(&self) -> Self::Item {
        &self.neg_z
    }
}
impl<T: Copy> SignedAxisIndex for SignedAxisSet<T> {
    fn axis_neg_x(&self) -> Self::Item {
        self.neg_x
    }
    fn axis_neg_y(&self) -> Self::Item {
        self.neg_y
    }
    fn axis_neg_z(&self) -> Self::Item {
        self.neg_z
    }
}
#[cfg(feature = "num-traits")]
pub mod impl_num_traits {
    use {
        super::*,
        ::num_traits::{ConstOne, ConstZero, MulAdd, MulAddAssign, One, Zero},
    };

    impl<T: Zero> Zero for SignedAxisSet<T> {
        fn zero() -> Self {
            Self::splat_with(|_| Zero::zero())
        }
        fn is_zero(&self) -> bool {
            let mut res = true;
            self.for_each_ref(|_, n| {
                if !n.is_zero() {
                    res = false;
                }
            });

            res
        }
    }
    impl<T: ConstZero> ConstZero for SignedAxisSet<T> {
        const ZERO: Self = Self {
            neg_x: ConstZero::ZERO,
            neg_y: ConstZero::ZERO,
            neg_z: ConstZero::ZERO,
            pos_x: ConstZero::ZERO,
            pos_y: ConstZero::ZERO,
            pos_z: ConstZero::ZERO,
        };
    }
    impl<T: One + PartialEq> One for SignedAxisSet<T> {
        fn one() -> Self {
            Self::splat_with(|_| One::one())
        }
        fn is_one(&self) -> bool
        where
            Self: PartialEq,
        {
            let mut res = true;
            self.for_each_ref(|_, n| {
                if !n.is_one() {
                    res = false;
                }
            });

            res
        }
    }
    impl<T: ConstOne + PartialEq> ConstOne for SignedAxisSet<T> {
        const ONE: Self = Self {
            neg_x: ConstOne::ONE,
            neg_y: ConstOne::ONE,
            neg_z: ConstOne::ONE,
            pos_x: ConstOne::ONE,
            pos_y: ConstOne::ONE,
            pos_z: ConstOne::ONE,
        };
    }

    impl<T: MulAdd<M, A>, M, A> MulAdd<SignedAxisSet<M>, SignedAxisSet<A>> for SignedAxisSet<T> {
        type Output = SignedAxisSet<T::Output>;
        fn mul_add(self, a: SignedAxisSet<M>, b: SignedAxisSet<A>) -> Self::Output {
            self.zip(a)
                .zip(b)
                .map(|_, ((i, mul), add)| i.mul_add(mul, add))
        }
    }
    impl<T: MulAddAssign<M, A>, M, A> MulAddAssign<SignedAxisSet<M>, SignedAxisSet<A>>
        for SignedAxisSet<T>
    {
        fn mul_add_assign(&mut self, a: SignedAxisSet<M>, b: SignedAxisSet<A>) {
            self.as_mut()
                .zip(a)
                .zip(b)
                .for_each(|_, ((i, mul), add)| i.mul_add_assign(mul, add));
        }
    }
}
macro_rules! overloaders {
    ($trait:ident, $traitmethod:ident) => {
        impl<T: $trait<U>, U> $trait<SignedAxisSet<U>> for SignedAxisSet<T> {
            type Output = SignedAxisSet<T::Output>;
            fn $traitmethod(self, rhs: SignedAxisSet<U>) -> Self::Output {
                self.zip(rhs).map(|_, (a, b)| a.$traitmethod(b))
            }
        }
    };
    (@ass $trait:ident, $traitmethod:ident) => {
        impl<T: $trait<U>, U> $trait<SignedAxisSet<U>> for SignedAxisSet<T> {
            fn $traitmethod(&mut self, rhs: SignedAxisSet<U>) {
                self.as_mut()
                    .zip(rhs.into_optional())
                    .for_each_mut(|_, (s, r)| s.$traitmethod(r.take().unwrap()))
            }
        }
    };
}
overloaders!(Add, add);
overloaders!(Sub, sub);
overloaders!(Mul, mul);
overloaders!(Div, div);
overloaders!(Rem, rem);
overloaders!(BitAnd, bitand);
overloaders!(BitOr, bitor);
overloaders!(BitXor, bitxor);
overloaders!(Shl, shl);
overloaders!(Shr, shr);

overloaders!(@ass AddAssign, add_assign);
overloaders!(@ass SubAssign, sub_assign);
overloaders!(@ass MulAssign, mul_assign);
overloaders!(@ass DivAssign, div_assign);
overloaders!(@ass RemAssign, rem_assign);
overloaders!(@ass BitAndAssign, bitand_assign);
overloaders!(@ass BitOrAssign, bitor_assign);
overloaders!(@ass BitXorAssign, bitxor_assign);
overloaders!(@ass ShlAssign, shl_assign);
overloaders!(@ass ShrAssign, shr_assign);

impl<T> IntoIterator for SignedAxisSet<T> {
    type IntoIter = SignedAxisSetIter<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        SignedAxisSetIter {
            idx: const { SignedAxis::VARIANT_ARRAY[0] },
            inner: self.into_optional(),
        }
    }
}

const MAX_AXIS_IDX: SignedAxis = *SignedAxis::VARIANT_ARRAY.last().unwrap();

/// An iterator over the axes in an axis set, in order.
pub struct SignedAxisSetIter<T> {
    inner: SignedAxisSet<Option<T>>,
    idx: SignedAxis,
}
impl<T> Iterator for SignedAxisSetIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;

        if self.idx != MAX_AXIS_IDX {
            self.idx = SignedAxis::VARIANT_ARRAY[self.idx as usize + 1];
        }

        self.inner.take(idx)
    }
}
