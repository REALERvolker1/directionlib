#![allow(unused)]

use crate::index::{AxisIndex, SignedAxisIndex};

macro_rules! impl_xyz {
    () => {
        impl_xyz!(self);
    };
    ($inp:ident) => {
        impl_xyz!($inp -> ($inp.x, $inp.y, $inp.z));
    };
    ($inp:ident -> ($x:expr, $y:expr, $z:expr)) => {
        fn axis_x(&$inp) -> Self::Item {
            $x
        }
        fn axis_y(&$inp) -> Self::Item {
            $y
        }
        fn axis_z(&$inp) -> Self::Item {
            $z
        }
    };
    (@neg) => {
        impl_xyz!(@neg self -> (
            -self.axis_pos_x(),
            -self.axis_pos_y(),
            -self.axis_pos_z()
        ));
    };
    (@neg $inp:ident -> ($x:expr, $y:expr, $z:expr)) => {
        fn axis_neg_x(&$inp) -> Self::Item {
            $x
        }
        fn axis_neg_y(&$inp) -> Self::Item {
            $y
        }
        fn axis_neg_z(&$inp) -> Self::Item {
            $z
        }
    };
}

#[cfg(feature = "mint")]
pub mod impl_mint {
    use {
        super::*,
        ::mint::{
            ColumnMatrix3, ColumnMatrix3x4, RowMatrix3, RowMatrix3x2, RowMatrix3x4, Vector2,
            Vector3, Vector4,
        },
    };

    macro_rules! impl_generic {
        ($ty:ident) => {
            impl_generic!($ty @unsigned);
            impl<T> SignedAxisIndex for $ty<T>
            where
                T: core::ops::Neg<Output = T> + Copy,
            {
                impl_xyz!(@neg);
            }
        };
        ($ty:ident @unsigned) => {
            impl<'a, T> AxisIndex for &'a $ty<T> {
                type Item = &'a T;
                impl_xyz!(self -> (&self.x, &self.y, &self.z));
            }
            impl<T: Copy> AxisIndex for $ty<T> {
                type Item = T;
                impl_xyz!();
            }
        };
        ($ty:ident @with $retty:ty) => {
            impl_generic!($ty @unsigned @with $retty);
            impl<T> SignedAxisIndex for $ty<T>
            where
                T: core::ops::Neg<Output = T> + Copy,
            {
                impl_xyz!(@neg);
            }
        };
        ($ty:ident @unsigned @with $retty:ty) => {
            impl<'a, T> AxisIndex for &'a $ty<T> {
                type Item = &'a $retty;
                impl_xyz!(self -> (&self.x, &self.y, &self.z));
            }
            impl<T: Copy> AxisIndex for $ty<T> {
                type Item = $retty;
                impl_xyz!();
            }
        };
    }

    impl_generic!(Vector3);
    impl_generic!(Vector4);

    impl_generic!(RowMatrix3x2 @unsigned @with Vector2<T>);
    impl_generic!(RowMatrix3 @unsigned @with Vector3<T>);
    impl_generic!(RowMatrix3x4 @unsigned @with Vector4<T>);

    impl_generic!(ColumnMatrix3 @unsigned @with Vector3<T>);
    impl_generic!(ColumnMatrix3x4 @unsigned @with Vector3<T>);
}

#[cfg(feature = "nalgebra")]
pub mod impl_nalgebra {
    use {super::*, ::core::ops::Neg, ::nalgebra::Vector3};

    impl<'a, T> AxisIndex for &'a Vector3<T> {
        type Item = &'a T;
        impl_xyz!(self -> (&self.data.0[0][0], &self.data.0[0][1], &self.data.0[0][2]));
    }

    impl<T: Copy> AxisIndex for Vector3<T> {
        type Item = T;
        impl_xyz!(self -> (*AxisIndex::axis_x(&self), *AxisIndex::axis_y(&self), *AxisIndex::axis_z(&self)));
    }
    impl<T> SignedAxisIndex for Vector3<T>
    where
        T: Neg<Output = T> + Copy,
    {
        impl_xyz!(@neg);
    }
}

#[cfg(feature = "glam")]
pub mod impl_glam {
    use {
        super::*,
        ::glam::{
            Affine3, Affine3A, DAffine3, DMat3, DMat4, DVec3, DVec4, Mat3, Mat3A, Mat4, Vec3,
            Vec3A, Vec4,
        },
    };

    macro_rules! impl_glam {
        ($this:ty, $item:ty) => {
            impl AxisIndex for $this {
                type Item = $item;
                impl_xyz!();
            }
            impl SignedAxisIndex for $this {
                impl_xyz!(@neg);
            }
        };
        (@mat $this:ty, $item:ty) => {
            impl AxisIndex for $this {
                type Item = $item;
                impl_xyz!(self -> (self.x_axis, self.y_axis, self.z_axis));
            }
            impl SignedAxisIndex for $this {
                impl_xyz!(@neg);
            }
        };
        (@affine $this:ty, $item:ty) => {
            impl AxisIndex for $this {
                type Item = $item;
                impl_xyz!(self -> (
                    (self.matrix3.axis_x(), self.translation.axis_x()),
                    (self.matrix3.axis_y(), self.translation.axis_y()),
                    (self.matrix3.axis_z(), self.translation.axis_z())
                ));
            }
            impl SignedAxisIndex for $this {
                impl_xyz!(@neg self -> (
                    (self.matrix3.axis_neg_x(), self.translation.axis_neg_x()),
                    (self.matrix3.axis_neg_y(), self.translation.axis_neg_y()),
                    (self.matrix3.axis_neg_z(), self.translation.axis_neg_z())
                ));
            }
        };
    }
    impl_glam!(Vec3, f32);
    impl_glam!(Vec3A, f32);
    impl_glam!(Vec4, f32);
    impl_glam!(DVec3, f64);
    impl_glam!(DVec4, f64);

    impl_glam!(@mat Mat3, Vec3);
    impl_glam!(@mat Mat3A, Vec3A);
    impl_glam!(@mat Mat4, Vec4);
    impl_glam!(@mat DMat3, DVec3);
    impl_glam!(@mat DMat4, DVec4);

    impl_glam!(@affine Affine3, (Vec3, f32));
    impl_glam!(@affine Affine3A, (Vec3A, f32));
    impl_glam!(@affine DAffine3, (DVec3, f64));
}
