#[cfg(feature = "arbitrary")]
use ::arbitrary::Arbitrary;

use {
    crate::{AxisIndex, SignedAxis, SignedAxisIndex},
    ::core::{convert::Infallible, mem::MaybeUninit},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct SignedAxisSet<T> {
    pub pos_x: T,
    pub neg_x: T,
    pub pos_y: T,
    pub neg_y: T,
    pub pos_z: T,
    pub neg_z: T,
}
impl<T> SignedAxisSet<T> {
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
    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> SignedAxisSet<U> {
        SignedAxisSet {
            pos_x: f(self.pos_x),
            neg_x: f(self.neg_x),
            pos_y: f(self.pos_y),
            neg_y: f(self.neg_y),
            pos_z: f(self.pos_z),
            neg_z: f(self.neg_z),
        }
    }
    #[inline]
    pub fn fold<I, E>(self, init: I, mut f: impl FnMut(I, T) -> I) -> I {
        self.try_fold(init, |i, t| Ok::<I, Infallible>(f(i, t)))
            .unwrap()
    }
    pub fn try_fold<I, E>(
        self,
        mut init: I,
        mut f: impl FnMut(I, T) -> Result<I, E>,
    ) -> Result<I, E> {
        init = f(init, self.pos_x)?;
        init = f(init, self.neg_x)?;
        init = f(init, self.pos_y)?;
        init = f(init, self.neg_y)?;
        init = f(init, self.pos_z)?;
        init = f(init, self.neg_z)?;

        Ok(init)
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

    pub fn into_optional(self) -> SignedAxisSet<Option<T>> {
        SignedAxisSet {
            pos_x: Some(self.pos_x),
            neg_x: Some(self.neg_x),
            pos_y: Some(self.pos_y),
            neg_y: Some(self.neg_y),
            pos_z: Some(self.pos_z),
            neg_z: Some(self.neg_z),
        }
    }
}
impl<T> SignedAxisSet<MaybeUninit<T>> {
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
    pub fn new(values: SignedAxisSet<T>) -> Self {
        Self {
            pos_x: MaybeUninit::new(values.pos_x),
            neg_x: MaybeUninit::new(values.neg_x),
            pos_y: MaybeUninit::new(values.pos_y),
            neg_y: MaybeUninit::new(values.neg_y),
            pos_z: MaybeUninit::new(values.pos_z),
            neg_z: MaybeUninit::new(values.neg_z),
        }
    }
    /// Calls [`MaybeUninit::assume_init_ref`] on each element.
    /// # Safety
    /// Calling this function when even ONE of the elements is uninitialized causes Undefined Behavior.
    pub const unsafe fn assume_init_ref(&self) -> SignedAxisSet<&T> {
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
        unsafe {
            SignedAxisSet {
                pos_x: self.pos_x.assume_init(),
                neg_x: self.neg_x.assume_init(),
                pos_y: self.pos_y.assume_init(),
                neg_y: self.neg_y.assume_init(),
                pos_z: self.pos_z.assume_init(),
                neg_z: self.neg_z.assume_init(),
            }
        }
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
