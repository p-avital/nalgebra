//! Vectors with dimensions known at compile-time.

#[allow(missing_doc)]; // we allow missing to avoid having to document the vector components.

use std::cast;
use std::cmp;
use std::num::{Zero, One, Float, Bounded};
use std::vec::{Items, MutItems};
use std::iter::{Iterator, FromIterator};
use traits::operations::{ApproxEq, PartialOrd, PartialOrdering, Less, Equal, Greater, NotComparable};

use traits::geometry::{Transform, Rotate, FromHomogeneous, ToHomogeneous, Dot, Norm,
                       Translation, Translate};
use traits::structure::{Basis, Cast, Dim, Indexable, Iterable, IterableMut};

mod metal;
mod vec_macros;

/// Vector of dimension 0.
#[deriving(Eq, Encodable, Decodable, Clone, DeepClone, Rand, Zero, Show, ToStr)]
pub struct Vec0<N>;

/// Vector of dimension 1.
#[deriving(Eq, Encodable, Decodable, Clone, DeepClone, Hash, Rand, Zero, Show, ToStr)]
pub struct Vec1<N> {
    /// First component of the vector.
    x: N
}

double_dispatch_binop_decl_trait!(Vec1, Vec1MulRhs)
double_dispatch_binop_decl_trait!(Vec1, Vec1DivRhs)
double_dispatch_binop_decl_trait!(Vec1, Vec1AddRhs)
double_dispatch_binop_decl_trait!(Vec1, Vec1SubRhs)
double_dispatch_cast_decl_trait!(Vec1, Vec1Cast)
mul_redispatch_impl!(Vec1, Vec1MulRhs)
div_redispatch_impl!(Vec1, Vec1DivRhs)
add_redispatch_impl!(Vec1, Vec1AddRhs)
sub_redispatch_impl!(Vec1, Vec1SubRhs)
cast_redispatch_impl!(Vec1, Vec1Cast)
new_impl!(Vec1, x)
ord_impl!(Vec1, x)
vec_axis_impl!(Vec1, x)
vec_cast_impl!(Vec1, Vec1Cast, x)
indexable_impl!(Vec1, 1)
at_fast_impl!(Vec1, 1)
new_repeat_impl!(Vec1, val, x)
dim_impl!(Vec1, 1)
container_impl!(Vec1)
// (specialized) basis_impl!(Vec1, 1)
add_impl!(Vec1, Vec1AddRhs, x)
sub_impl!(Vec1, Vec1SubRhs, x)
mul_impl!(Vec1, Vec1MulRhs, x)
div_impl!(Vec1, Vec1DivRhs, x)
neg_impl!(Vec1, x)
dot_impl!(Vec1, x)
scalar_mul_impl!(Vec1, f64, Vec1MulRhs, x)
scalar_mul_impl!(Vec1, f32, Vec1MulRhs, x)
scalar_mul_impl!(Vec1, u64, Vec1MulRhs, x)
scalar_mul_impl!(Vec1, u32, Vec1MulRhs, x)
scalar_mul_impl!(Vec1, u16, Vec1MulRhs, x)
scalar_mul_impl!(Vec1, u8, Vec1MulRhs, x)
scalar_mul_impl!(Vec1, i64, Vec1MulRhs, x)
scalar_mul_impl!(Vec1, i32, Vec1MulRhs, x)
scalar_mul_impl!(Vec1, i16, Vec1MulRhs, x)
scalar_mul_impl!(Vec1, i8, Vec1MulRhs, x)
scalar_mul_impl!(Vec1, uint, Vec1MulRhs, x)
scalar_mul_impl!(Vec1, int, Vec1MulRhs, x)
scalar_div_impl!(Vec1, f64, Vec1DivRhs, x)
scalar_div_impl!(Vec1, f32, Vec1DivRhs, x)
scalar_div_impl!(Vec1, u64, Vec1DivRhs, x)
scalar_div_impl!(Vec1, u32, Vec1DivRhs, x)
scalar_div_impl!(Vec1, u16, Vec1DivRhs, x)
scalar_div_impl!(Vec1, u8, Vec1DivRhs, x)
scalar_div_impl!(Vec1, i64, Vec1DivRhs, x)
scalar_div_impl!(Vec1, i32, Vec1DivRhs, x)
scalar_div_impl!(Vec1, i16, Vec1DivRhs, x)
scalar_div_impl!(Vec1, i8, Vec1DivRhs, x)
scalar_div_impl!(Vec1, uint, Vec1DivRhs, x)
scalar_div_impl!(Vec1, int, Vec1DivRhs, x)
scalar_add_impl!(Vec1, f64, Vec1AddRhs, x)
scalar_add_impl!(Vec1, f32, Vec1AddRhs, x)
scalar_add_impl!(Vec1, u64, Vec1AddRhs, x)
scalar_add_impl!(Vec1, u32, Vec1AddRhs, x)
scalar_add_impl!(Vec1, u16, Vec1AddRhs, x)
scalar_add_impl!(Vec1, u8, Vec1AddRhs, x)
scalar_add_impl!(Vec1, i64, Vec1AddRhs, x)
scalar_add_impl!(Vec1, i32, Vec1AddRhs, x)
scalar_add_impl!(Vec1, i16, Vec1AddRhs, x)
scalar_add_impl!(Vec1, i8, Vec1AddRhs, x)
scalar_add_impl!(Vec1, uint, Vec1AddRhs, x)
scalar_add_impl!(Vec1, int, Vec1AddRhs, x)
scalar_sub_impl!(Vec1, f64, Vec1SubRhs, x)
scalar_sub_impl!(Vec1, f32, Vec1SubRhs, x)
scalar_sub_impl!(Vec1, u64, Vec1SubRhs, x)
scalar_sub_impl!(Vec1, u32, Vec1SubRhs, x)
scalar_sub_impl!(Vec1, u16, Vec1SubRhs, x)
scalar_sub_impl!(Vec1, u8, Vec1SubRhs, x)
scalar_sub_impl!(Vec1, i64, Vec1SubRhs, x)
scalar_sub_impl!(Vec1, i32, Vec1SubRhs, x)
scalar_sub_impl!(Vec1, i16, Vec1SubRhs, x)
scalar_sub_impl!(Vec1, i8, Vec1SubRhs, x)
scalar_sub_impl!(Vec1, uint, Vec1SubRhs, x)
scalar_sub_impl!(Vec1, int, Vec1SubRhs, x)
translation_impl!(Vec1)
norm_impl!(Vec1, x)
approx_eq_impl!(Vec1, x)
round_impl!(Vec1, x)
one_impl!(Vec1, x)
from_iterator_impl!(Vec1, iterator)
bounded_impl!(Vec1, x)
iterable_impl!(Vec1, 1)
iterable_mut_impl!(Vec1, 1)
to_homogeneous_impl!(Vec1, Vec2, y, x)
from_homogeneous_impl!(Vec1, Vec2, y, x)
translate_impl!(Vec1)
rotate_impl!(Vec1)
transform_impl!(Vec1)

/// Vector of dimension 2.
#[deriving(Eq, Encodable, Decodable, Clone, DeepClone, Hash, Rand, Zero, Show, ToStr)]
pub struct Vec2<N> {
    /// First component of the vector.
    x: N,
    /// Second component of the vector.
    y: N
}

double_dispatch_binop_decl_trait!(Vec2, Vec2MulRhs)
double_dispatch_binop_decl_trait!(Vec2, Vec2DivRhs)
double_dispatch_binop_decl_trait!(Vec2, Vec2AddRhs)
double_dispatch_binop_decl_trait!(Vec2, Vec2SubRhs)
double_dispatch_cast_decl_trait!(Vec2, Vec2Cast)
mul_redispatch_impl!(Vec2, Vec2MulRhs)
div_redispatch_impl!(Vec2, Vec2DivRhs)
add_redispatch_impl!(Vec2, Vec2AddRhs)
sub_redispatch_impl!(Vec2, Vec2SubRhs)
cast_redispatch_impl!(Vec2, Vec2Cast)
new_impl!(Vec2, x, y)
ord_impl!(Vec2, x, y)
vec_axis_impl!(Vec2, x, y)
vec_cast_impl!(Vec2, Vec2Cast, x, y)
indexable_impl!(Vec2, 2)
at_fast_impl!(Vec2, 2)
new_repeat_impl!(Vec2, val, x, y)
dim_impl!(Vec2, 2)
container_impl!(Vec2)
// (specialized) basis_impl!(Vec2, 1)
add_impl!(Vec2, Vec2AddRhs, x, y)
sub_impl!(Vec2, Vec2SubRhs, x, y)
mul_impl!(Vec2, Vec2MulRhs, x, y)
div_impl!(Vec2, Vec2DivRhs, x, y)
neg_impl!(Vec2, x, y)
dot_impl!(Vec2, x, y)
scalar_mul_impl!(Vec2, f64, Vec2MulRhs, x, y)
scalar_mul_impl!(Vec2, f32, Vec2MulRhs, x, y)
scalar_mul_impl!(Vec2, u64, Vec2MulRhs, x, y)
scalar_mul_impl!(Vec2, u32, Vec2MulRhs, x, y)
scalar_mul_impl!(Vec2, u16, Vec2MulRhs, x, y)
scalar_mul_impl!(Vec2, u8, Vec2MulRhs, x, y)
scalar_mul_impl!(Vec2, i64, Vec2MulRhs, x, y)
scalar_mul_impl!(Vec2, i32, Vec2MulRhs, x, y)
scalar_mul_impl!(Vec2, i16, Vec2MulRhs, x, y)
scalar_mul_impl!(Vec2, i8, Vec2MulRhs, x, y)
scalar_mul_impl!(Vec2, uint, Vec2MulRhs, x, y)
scalar_mul_impl!(Vec2, int, Vec2MulRhs, x, y)
scalar_div_impl!(Vec2, f64, Vec2DivRhs, x, y)
scalar_div_impl!(Vec2, f32, Vec2DivRhs, x, y)
scalar_div_impl!(Vec2, u64, Vec2DivRhs, x, y)
scalar_div_impl!(Vec2, u32, Vec2DivRhs, x, y)
scalar_div_impl!(Vec2, u16, Vec2DivRhs, x, y)
scalar_div_impl!(Vec2, u8, Vec2DivRhs, x, y)
scalar_div_impl!(Vec2, i64, Vec2DivRhs, x, y)
scalar_div_impl!(Vec2, i32, Vec2DivRhs, x, y)
scalar_div_impl!(Vec2, i16, Vec2DivRhs, x, y)
scalar_div_impl!(Vec2, i8, Vec2DivRhs, x, y)
scalar_div_impl!(Vec2, uint, Vec2DivRhs, x, y)
scalar_div_impl!(Vec2, int, Vec2DivRhs, x, y)
scalar_add_impl!(Vec2, f64, Vec2AddRhs, x, y)
scalar_add_impl!(Vec2, f32, Vec2AddRhs, x, y)
scalar_add_impl!(Vec2, u64, Vec2AddRhs, x, y)
scalar_add_impl!(Vec2, u32, Vec2AddRhs, x, y)
scalar_add_impl!(Vec2, u16, Vec2AddRhs, x, y)
scalar_add_impl!(Vec2, u8, Vec2AddRhs, x, y)
scalar_add_impl!(Vec2, i64, Vec2AddRhs, x, y)
scalar_add_impl!(Vec2, i32, Vec2AddRhs, x, y)
scalar_add_impl!(Vec2, i16, Vec2AddRhs, x, y)
scalar_add_impl!(Vec2, i8, Vec2AddRhs, x, y)
scalar_add_impl!(Vec2, uint, Vec2AddRhs, x, y)
scalar_add_impl!(Vec2, int, Vec2AddRhs, x, y)
scalar_sub_impl!(Vec2, f64, Vec2SubRhs, x, y)
scalar_sub_impl!(Vec2, f32, Vec2SubRhs, x, y)
scalar_sub_impl!(Vec2, u64, Vec2SubRhs, x, y)
scalar_sub_impl!(Vec2, u32, Vec2SubRhs, x, y)
scalar_sub_impl!(Vec2, u16, Vec2SubRhs, x, y)
scalar_sub_impl!(Vec2, u8, Vec2SubRhs, x, y)
scalar_sub_impl!(Vec2, i64, Vec2SubRhs, x, y)
scalar_sub_impl!(Vec2, i32, Vec2SubRhs, x, y)
scalar_sub_impl!(Vec2, i16, Vec2SubRhs, x, y)
scalar_sub_impl!(Vec2, i8, Vec2SubRhs, x, y)
scalar_sub_impl!(Vec2, uint, Vec2SubRhs, x, y)
scalar_sub_impl!(Vec2, int, Vec2SubRhs, x, y)
translation_impl!(Vec2)
norm_impl!(Vec2, x, y)
approx_eq_impl!(Vec2, x, y)
round_impl!(Vec2, x, y)
one_impl!(Vec2, x, y)
from_iterator_impl!(Vec2, iterator, iterator)
bounded_impl!(Vec2, x, y)
iterable_impl!(Vec2, 2)
iterable_mut_impl!(Vec2, 2)
to_homogeneous_impl!(Vec2, Vec3, z, x, y)
from_homogeneous_impl!(Vec2, Vec3, z, x, y)
translate_impl!(Vec2)
rotate_impl!(Vec2)
transform_impl!(Vec2)

/// Vector of dimension 3.
#[deriving(Eq, Encodable, Decodable, Clone, DeepClone, Hash, Rand, Zero, Show, ToStr)]
pub struct Vec3<N> {
    /// First component of the vector.
    x: N,
    /// Second component of the vector.
    y: N,
    /// Third component of the vector.
    z: N
}

double_dispatch_binop_decl_trait!(Vec3, Vec3MulRhs)
double_dispatch_binop_decl_trait!(Vec3, Vec3DivRhs)
double_dispatch_binop_decl_trait!(Vec3, Vec3AddRhs)
double_dispatch_binop_decl_trait!(Vec3, Vec3SubRhs)
double_dispatch_cast_decl_trait!(Vec3, Vec3Cast)
mul_redispatch_impl!(Vec3, Vec3MulRhs)
div_redispatch_impl!(Vec3, Vec3DivRhs)
add_redispatch_impl!(Vec3, Vec3AddRhs)
sub_redispatch_impl!(Vec3, Vec3SubRhs)
cast_redispatch_impl!(Vec3, Vec3Cast)
new_impl!(Vec3, x, y, z)
ord_impl!(Vec3, x, y, z)
vec_axis_impl!(Vec3, x, y, z)
vec_cast_impl!(Vec3, Vec3Cast, x, y, z)
indexable_impl!(Vec3, 3)
at_fast_impl!(Vec3, 3)
new_repeat_impl!(Vec3, val, x, y, z)
dim_impl!(Vec3, 3)
container_impl!(Vec3)
// (specialized) basis_impl!(Vec3, 1)
add_impl!(Vec3, Vec3AddRhs, x, y, z)
sub_impl!(Vec3, Vec3SubRhs, x, y, z)
mul_impl!(Vec3, Vec3MulRhs, x, y, z)
div_impl!(Vec3, Vec3DivRhs, x, y, z)
neg_impl!(Vec3, x, y, z)
dot_impl!(Vec3, x, y, z)
scalar_mul_impl!(Vec3, f64, Vec3MulRhs, x, y, z)
scalar_mul_impl!(Vec3, f32, Vec3MulRhs, x, y, z)
scalar_mul_impl!(Vec3, u64, Vec3MulRhs, x, y, z)
scalar_mul_impl!(Vec3, u32, Vec3MulRhs, x, y, z)
scalar_mul_impl!(Vec3, u16, Vec3MulRhs, x, y, z)
scalar_mul_impl!(Vec3, u8, Vec3MulRhs, x, y, z)
scalar_mul_impl!(Vec3, i64, Vec3MulRhs, x, y, z)
scalar_mul_impl!(Vec3, i32, Vec3MulRhs, x, y, z)
scalar_mul_impl!(Vec3, i16, Vec3MulRhs, x, y, z)
scalar_mul_impl!(Vec3, i8, Vec3MulRhs, x, y, z)
scalar_mul_impl!(Vec3, uint, Vec3MulRhs, x, y, z)
scalar_mul_impl!(Vec3, int, Vec3MulRhs, x, y, z)

scalar_div_impl!(Vec3, f64, Vec3DivRhs, x, y, z)
scalar_div_impl!(Vec3, f32, Vec3DivRhs, x, y, z)
scalar_div_impl!(Vec3, u64, Vec3DivRhs, x, y, z)
scalar_div_impl!(Vec3, u32, Vec3DivRhs, x, y, z)
scalar_div_impl!(Vec3, u16, Vec3DivRhs, x, y, z)
scalar_div_impl!(Vec3, u8, Vec3DivRhs, x, y, z)
scalar_div_impl!(Vec3, i64, Vec3DivRhs, x, y, z)
scalar_div_impl!(Vec3, i32, Vec3DivRhs, x, y, z)
scalar_div_impl!(Vec3, i16, Vec3DivRhs, x, y, z)
scalar_div_impl!(Vec3, i8, Vec3DivRhs, x, y, z)
scalar_div_impl!(Vec3, uint, Vec3DivRhs, x, y, z)
scalar_div_impl!(Vec3, int, Vec3DivRhs, x, y, z)

scalar_add_impl!(Vec3, f64, Vec3AddRhs, x, y, z)
scalar_add_impl!(Vec3, f32, Vec3AddRhs, x, y, z)
scalar_add_impl!(Vec3, u64, Vec3AddRhs, x, y, z)
scalar_add_impl!(Vec3, u32, Vec3AddRhs, x, y, z)
scalar_add_impl!(Vec3, u16, Vec3AddRhs, x, y, z)
scalar_add_impl!(Vec3, u8, Vec3AddRhs, x, y, z)
scalar_add_impl!(Vec3, i64, Vec3AddRhs, x, y, z)
scalar_add_impl!(Vec3, i32, Vec3AddRhs, x, y, z)
scalar_add_impl!(Vec3, i16, Vec3AddRhs, x, y, z)
scalar_add_impl!(Vec3, i8, Vec3AddRhs, x, y, z)
scalar_add_impl!(Vec3, uint, Vec3AddRhs, x, y, z)
scalar_add_impl!(Vec3, int, Vec3AddRhs, x, y, z)

scalar_sub_impl!(Vec3, f64, Vec3SubRhs, x, y, z)
scalar_sub_impl!(Vec3, f32, Vec3SubRhs, x, y, z)
scalar_sub_impl!(Vec3, u64, Vec3SubRhs, x, y, z)
scalar_sub_impl!(Vec3, u32, Vec3SubRhs, x, y, z)
scalar_sub_impl!(Vec3, u16, Vec3SubRhs, x, y, z)
scalar_sub_impl!(Vec3, u8, Vec3SubRhs, x, y, z)
scalar_sub_impl!(Vec3, i64, Vec3SubRhs, x, y, z)
scalar_sub_impl!(Vec3, i32, Vec3SubRhs, x, y, z)
scalar_sub_impl!(Vec3, i16, Vec3SubRhs, x, y, z)
scalar_sub_impl!(Vec3, i8, Vec3SubRhs, x, y, z)
scalar_sub_impl!(Vec3, uint, Vec3SubRhs, x, y, z)
scalar_sub_impl!(Vec3, int, Vec3SubRhs, x, y, z)
translation_impl!(Vec3)
norm_impl!(Vec3, x, y ,z)
approx_eq_impl!(Vec3, x, y, z)
round_impl!(Vec3, x, y, z)
one_impl!(Vec3, x, y, z)
from_iterator_impl!(Vec3, iterator, iterator, iterator)
bounded_impl!(Vec3, x, y, z)
iterable_impl!(Vec3, 3)
iterable_mut_impl!(Vec3, 3)
to_homogeneous_impl!(Vec3, Vec4, w, x, y, z)
from_homogeneous_impl!(Vec3, Vec4, w, x, y, z)
translate_impl!(Vec3)
rotate_impl!(Vec3)
transform_impl!(Vec3)


/// Vector of dimension 4.
#[deriving(Eq, Encodable, Decodable, Clone, DeepClone, Hash, Rand, Zero, Show, ToStr)]
pub struct Vec4<N> {
    /// First component of the vector.
    x: N,
    /// Second component of the vector.
    y: N,
    /// Third component of the vector.
    z: N,
    /// Fourth component of the vector.
    w: N
}

double_dispatch_binop_decl_trait!(Vec4, Vec4MulRhs)
double_dispatch_binop_decl_trait!(Vec4, Vec4DivRhs)
double_dispatch_binop_decl_trait!(Vec4, Vec4AddRhs)
double_dispatch_binop_decl_trait!(Vec4, Vec4SubRhs)
double_dispatch_cast_decl_trait!(Vec4, Vec4Cast)
mul_redispatch_impl!(Vec4, Vec4MulRhs)
div_redispatch_impl!(Vec4, Vec4DivRhs)
add_redispatch_impl!(Vec4, Vec4AddRhs)
sub_redispatch_impl!(Vec4, Vec4SubRhs)
cast_redispatch_impl!(Vec4, Vec4Cast)
new_impl!(Vec4, x, y, z, w)
ord_impl!(Vec4, x, y, z, w)
vec_axis_impl!(Vec4, x, y, z, w)
vec_cast_impl!(Vec4, Vec4Cast, x, y, z, w)
indexable_impl!(Vec4, 4)
at_fast_impl!(Vec4, 4)
new_repeat_impl!(Vec4, val, x, y, z, w)
dim_impl!(Vec4, 4)
container_impl!(Vec4)
basis_impl!(Vec4, Vec4MulRhs, 4)
add_impl!(Vec4, Vec4AddRhs, x, y, z, w)
sub_impl!(Vec4, Vec4SubRhs, x, y, z, w)
mul_impl!(Vec4, Vec4MulRhs, x, y, z, w)
div_impl!(Vec4, Vec4DivRhs, x, y, z, w)
neg_impl!(Vec4, x, y, z, w)
dot_impl!(Vec4, x, y, z, w)
scalar_mul_impl!(Vec4, f64, Vec4MulRhs, x, y, z, w)
scalar_mul_impl!(Vec4, f32, Vec4MulRhs, x, y, z, w)
scalar_mul_impl!(Vec4, u64, Vec4MulRhs, x, y, z, w)
scalar_mul_impl!(Vec4, u32, Vec4MulRhs, x, y, z, w)
scalar_mul_impl!(Vec4, u16, Vec4MulRhs, x, y, z, w)
scalar_mul_impl!(Vec4, u8, Vec4MulRhs, x, y, z, w)
scalar_mul_impl!(Vec4, i64, Vec4MulRhs, x, y, z, w)
scalar_mul_impl!(Vec4, i32, Vec4MulRhs, x, y, z, w)
scalar_mul_impl!(Vec4, i16, Vec4MulRhs, x, y, z, w)
scalar_mul_impl!(Vec4, i8, Vec4MulRhs, x, y, z, w)
scalar_mul_impl!(Vec4, uint, Vec4MulRhs, x, y, z, w)
scalar_mul_impl!(Vec4, int, Vec4MulRhs, x, y, z, w)
scalar_div_impl!(Vec4, f64, Vec4DivRhs, x, y, z, w)
scalar_div_impl!(Vec4, f32, Vec4DivRhs, x, y, z, w)
scalar_div_impl!(Vec4, u64, Vec4DivRhs, x, y, z, w)
scalar_div_impl!(Vec4, u32, Vec4DivRhs, x, y, z, w)
scalar_div_impl!(Vec4, u16, Vec4DivRhs, x, y, z, w)
scalar_div_impl!(Vec4, u8, Vec4DivRhs, x, y, z, w)
scalar_div_impl!(Vec4, i64, Vec4DivRhs, x, y, z, w)
scalar_div_impl!(Vec4, i32, Vec4DivRhs, x, y, z, w)
scalar_div_impl!(Vec4, i16, Vec4DivRhs, x, y, z, w)
scalar_div_impl!(Vec4, i8, Vec4DivRhs, x, y, z, w)
scalar_div_impl!(Vec4, uint, Vec4DivRhs, x, y, z, w)
scalar_div_impl!(Vec4, int, Vec4DivRhs, x, y, z, w)
scalar_add_impl!(Vec4, f64, Vec4AddRhs, x, y, z, w)
scalar_add_impl!(Vec4, f32, Vec4AddRhs, x, y, z, w)
scalar_add_impl!(Vec4, u64, Vec4AddRhs, x, y, z, w)
scalar_add_impl!(Vec4, u32, Vec4AddRhs, x, y, z, w)
scalar_add_impl!(Vec4, u16, Vec4AddRhs, x, y, z, w)
scalar_add_impl!(Vec4, u8, Vec4AddRhs, x, y, z, w)
scalar_add_impl!(Vec4, i64, Vec4AddRhs, x, y, z, w)
scalar_add_impl!(Vec4, i32, Vec4AddRhs, x, y, z, w)
scalar_add_impl!(Vec4, i16, Vec4AddRhs, x, y, z, w)
scalar_add_impl!(Vec4, i8, Vec4AddRhs, x, y, z, w)
scalar_add_impl!(Vec4, uint, Vec4AddRhs, x, y, z, w)
scalar_add_impl!(Vec4, int, Vec4AddRhs, x, y, z, w)
scalar_sub_impl!(Vec4, f64, Vec4SubRhs, x, y, z, w)
scalar_sub_impl!(Vec4, f32, Vec4SubRhs, x, y, z, w)
scalar_sub_impl!(Vec4, u64, Vec4SubRhs, x, y, z, w)
scalar_sub_impl!(Vec4, u32, Vec4SubRhs, x, y, z, w)
scalar_sub_impl!(Vec4, u16, Vec4SubRhs, x, y, z, w)
scalar_sub_impl!(Vec4, u8, Vec4SubRhs, x, y, z, w)
scalar_sub_impl!(Vec4, i64, Vec4SubRhs, x, y, z, w)
scalar_sub_impl!(Vec4, i32, Vec4SubRhs, x, y, z, w)
scalar_sub_impl!(Vec4, i16, Vec4SubRhs, x, y, z, w)
scalar_sub_impl!(Vec4, i8, Vec4SubRhs, x, y, z, w)
scalar_sub_impl!(Vec4, uint, Vec4SubRhs, x, y, z, w)
scalar_sub_impl!(Vec4, int, Vec4SubRhs, x, y, z, w)
translation_impl!(Vec4)
norm_impl!(Vec4, x, y, z, w)
approx_eq_impl!(Vec4, x, y, z, w)
round_impl!(Vec4, x, y, z, w)
one_impl!(Vec4, x, y, z, w)
from_iterator_impl!(Vec4, iterator, iterator, iterator, iterator)
bounded_impl!(Vec4, x, y, z, w)
iterable_impl!(Vec4, 4)
iterable_mut_impl!(Vec4, 4)
to_homogeneous_impl!(Vec4, Vec5, a, x, y, z, w)
from_homogeneous_impl!(Vec4, Vec5, a, x, y, z, w)
translate_impl!(Vec4)
rotate_impl!(Vec4)
transform_impl!(Vec4)

/// Vector of dimension 5.
#[deriving(Eq, Encodable, Decodable, Clone, DeepClone, Hash, Rand, Zero, Show, ToStr)]
pub struct Vec5<N> {
    /// First component of the vector.
    x: N,
    /// Second component of the vector.
    y: N,
    /// Third component of the vector.
    z: N,
    /// Fourth component of the vector.
    w: N,
    /// Fifth of the vector.
    a: N
}

double_dispatch_binop_decl_trait!(Vec5, Vec5MulRhs)
double_dispatch_binop_decl_trait!(Vec5, Vec5DivRhs)
double_dispatch_binop_decl_trait!(Vec5, Vec5AddRhs)
double_dispatch_binop_decl_trait!(Vec5, Vec5SubRhs)
double_dispatch_cast_decl_trait!(Vec5, Vec5Cast)
mul_redispatch_impl!(Vec5, Vec5MulRhs)
div_redispatch_impl!(Vec5, Vec5DivRhs)
add_redispatch_impl!(Vec5, Vec5AddRhs)
sub_redispatch_impl!(Vec5, Vec5SubRhs)
cast_redispatch_impl!(Vec5, Vec5Cast)
new_impl!(Vec5, x, y, z, w, a)
ord_impl!(Vec5, x, y, z, w, a)
vec_axis_impl!(Vec5, x, y, z, w, a)
vec_cast_impl!(Vec5, Vec5Cast, x, y, z, w, a)
indexable_impl!(Vec5, 5)
at_fast_impl!(Vec5, 5)
new_repeat_impl!(Vec5, val, x, y, z, w, a)
dim_impl!(Vec5, 5)
container_impl!(Vec5)
basis_impl!(Vec5, Vec5MulRhs, 5)
add_impl!(Vec5, Vec5AddRhs, x, y, z, w, a)
sub_impl!(Vec5, Vec5SubRhs, x, y, z, w, a)
mul_impl!(Vec5, Vec5MulRhs, x, y, z, w, a)
div_impl!(Vec5, Vec5DivRhs, x, y, z, w, a)
neg_impl!(Vec5, x, y, z, w, a)
dot_impl!(Vec5, x, y, z, w, a)
scalar_mul_impl!(Vec5, f64, Vec5MulRhs, x, y, z, w, a)
scalar_mul_impl!(Vec5, f32, Vec5MulRhs, x, y, z, w, a)
scalar_mul_impl!(Vec5, u64, Vec5MulRhs, x, y, z, w, a)
scalar_mul_impl!(Vec5, u32, Vec5MulRhs, x, y, z, w, a)
scalar_mul_impl!(Vec5, u16, Vec5MulRhs, x, y, z, w, a)
scalar_mul_impl!(Vec5, u8, Vec5MulRhs, x, y, z, w, a)
scalar_mul_impl!(Vec5, i64, Vec5MulRhs, x, y, z, w, a)
scalar_mul_impl!(Vec5, i32, Vec5MulRhs, x, y, z, w, a)
scalar_mul_impl!(Vec5, i16, Vec5MulRhs, x, y, z, w, a)
scalar_mul_impl!(Vec5, i8, Vec5MulRhs, x, y, z, w, a)
scalar_mul_impl!(Vec5, uint, Vec5MulRhs, x, y, z, w, a)
scalar_mul_impl!(Vec5, int, Vec5MulRhs, x, y, z, w, a)
scalar_div_impl!(Vec5, f64, Vec5DivRhs, x, y, z, w, a)
scalar_div_impl!(Vec5, f32, Vec5DivRhs, x, y, z, w, a)
scalar_div_impl!(Vec5, u64, Vec5DivRhs, x, y, z, w, a)
scalar_div_impl!(Vec5, u32, Vec5DivRhs, x, y, z, w, a)
scalar_div_impl!(Vec5, u16, Vec5DivRhs, x, y, z, w, a)
scalar_div_impl!(Vec5, u8, Vec5DivRhs, x, y, z, w, a)
scalar_div_impl!(Vec5, i64, Vec5DivRhs, x, y, z, w, a)
scalar_div_impl!(Vec5, i32, Vec5DivRhs, x, y, z, w, a)
scalar_div_impl!(Vec5, i16, Vec5DivRhs, x, y, z, w, a)
scalar_div_impl!(Vec5, i8, Vec5DivRhs, x, y, z, w, a)
scalar_div_impl!(Vec5, uint, Vec5DivRhs, x, y, z, w, a)
scalar_div_impl!(Vec5, int, Vec5DivRhs, x, y, z, w, a)
scalar_add_impl!(Vec5, f64, Vec5AddRhs, x, y, z, w, a)
scalar_add_impl!(Vec5, f32, Vec5AddRhs, x, y, z, w, a)
scalar_add_impl!(Vec5, u64, Vec5AddRhs, x, y, z, w, a)
scalar_add_impl!(Vec5, u32, Vec5AddRhs, x, y, z, w, a)
scalar_add_impl!(Vec5, u16, Vec5AddRhs, x, y, z, w, a)
scalar_add_impl!(Vec5, u8, Vec5AddRhs, x, y, z, w, a)
scalar_add_impl!(Vec5, i64, Vec5AddRhs, x, y, z, w, a)
scalar_add_impl!(Vec5, i32, Vec5AddRhs, x, y, z, w, a)
scalar_add_impl!(Vec5, i16, Vec5AddRhs, x, y, z, w, a)
scalar_add_impl!(Vec5, i8, Vec5AddRhs, x, y, z, w, a)
scalar_add_impl!(Vec5, uint, Vec5AddRhs, x, y, z, w, a)
scalar_add_impl!(Vec5, int, Vec5AddRhs, x, y, z, w, a)
scalar_sub_impl!(Vec5, f64, Vec5SubRhs, x, y, z, w, a)
scalar_sub_impl!(Vec5, f32, Vec5SubRhs, x, y, z, w, a)
scalar_sub_impl!(Vec5, u64, Vec5SubRhs, x, y, z, w, a)
scalar_sub_impl!(Vec5, u32, Vec5SubRhs, x, y, z, w, a)
scalar_sub_impl!(Vec5, u16, Vec5SubRhs, x, y, z, w, a)
scalar_sub_impl!(Vec5, u8, Vec5SubRhs, x, y, z, w, a)
scalar_sub_impl!(Vec5, i64, Vec5SubRhs, x, y, z, w, a)
scalar_sub_impl!(Vec5, i32, Vec5SubRhs, x, y, z, w, a)
scalar_sub_impl!(Vec5, i16, Vec5SubRhs, x, y, z, w, a)
scalar_sub_impl!(Vec5, i8, Vec5SubRhs, x, y, z, w, a)
scalar_sub_impl!(Vec5, uint, Vec5SubRhs, x, y, z, w, a)
scalar_sub_impl!(Vec5, int, Vec5SubRhs, x, y, z, w, a)
translation_impl!(Vec5)
norm_impl!(Vec5, x, y, z, w, a)
approx_eq_impl!(Vec5, x, y, z, w, a)
round_impl!(Vec5, x, y, z, w, a)
one_impl!(Vec5, x, y, z, w, a)
from_iterator_impl!(Vec5, iterator, iterator, iterator, iterator, iterator)
bounded_impl!(Vec5, x, y, z, w, a)
iterable_impl!(Vec5, 5)
iterable_mut_impl!(Vec5, 5)
to_homogeneous_impl!(Vec5, Vec6, b, x, y, z, w, a)
from_homogeneous_impl!(Vec5, Vec6, b, x, y, z, w, a)
translate_impl!(Vec5)
rotate_impl!(Vec5)
transform_impl!(Vec5)

/// Vector of dimension 6.
#[deriving(Eq, Encodable, Decodable, Clone, DeepClone, Hash, Rand, Zero, Show, ToStr)]
pub struct Vec6<N> {
    /// First component of the vector.
    x: N,
    /// Second component of the vector.
    y: N,
    /// Third component of the vector.
    z: N,
    /// Fourth component of the vector.
    w: N,
    /// Fifth of the vector.
    a: N,
    /// Sixth component of the vector.
    b: N
}

double_dispatch_binop_decl_trait!(Vec6, Vec6MulRhs)
double_dispatch_binop_decl_trait!(Vec6, Vec6DivRhs)
double_dispatch_binop_decl_trait!(Vec6, Vec6AddRhs)
double_dispatch_binop_decl_trait!(Vec6, Vec6SubRhs)
double_dispatch_cast_decl_trait!(Vec6, Vec6Cast)
mul_redispatch_impl!(Vec6, Vec6MulRhs)
div_redispatch_impl!(Vec6, Vec6DivRhs)
add_redispatch_impl!(Vec6, Vec6AddRhs)
sub_redispatch_impl!(Vec6, Vec6SubRhs)
cast_redispatch_impl!(Vec6, Vec6Cast)
new_impl!(Vec6, x, y, z, w, a, b)
ord_impl!(Vec6, x, y, z, w, a, b)
vec_axis_impl!(Vec6, x, y, z, w, a, b)
vec_cast_impl!(Vec6, Vec6Cast, x, y, z, w, a, b)
indexable_impl!(Vec6, 6)
at_fast_impl!(Vec6, 6)
new_repeat_impl!(Vec6, val, x, y, z, w, a, b)
dim_impl!(Vec6, 6)
container_impl!(Vec6)
basis_impl!(Vec6, Vec6MulRhs, 6)
add_impl!(Vec6, Vec6AddRhs, x, y, z, w, a, b)
sub_impl!(Vec6, Vec6SubRhs, x, y, z, w, a, b)
mul_impl!(Vec6, Vec6MulRhs, x, y, z, w, a, b)
div_impl!(Vec6, Vec6DivRhs, x, y, z, w, a, b)
neg_impl!(Vec6, x, y, z, w, a, b)
dot_impl!(Vec6, x, y, z, w, a, b)
scalar_mul_impl!(Vec6, f64, Vec6MulRhs, x, y, z, w, a, b)
scalar_mul_impl!(Vec6, f32, Vec6MulRhs, x, y, z, w, a, b)
scalar_mul_impl!(Vec6, u64, Vec6MulRhs, x, y, z, w, a, b)
scalar_mul_impl!(Vec6, u32, Vec6MulRhs, x, y, z, w, a, b)
scalar_mul_impl!(Vec6, u16, Vec6MulRhs, x, y, z, w, a, b)
scalar_mul_impl!(Vec6, u8, Vec6MulRhs, x, y, z, w, a, b)
scalar_mul_impl!(Vec6, i64, Vec6MulRhs, x, y, z, w, a, b)
scalar_mul_impl!(Vec6, i32, Vec6MulRhs, x, y, z, w, a, b)
scalar_mul_impl!(Vec6, i16, Vec6MulRhs, x, y, z, w, a, b)
scalar_mul_impl!(Vec6, i8, Vec6MulRhs, x, y, z, w, a, b)
scalar_mul_impl!(Vec6, uint, Vec6MulRhs, x, y, z, w, a, b)
scalar_mul_impl!(Vec6, int, Vec6MulRhs, x, y, z, w, a, b)
scalar_div_impl!(Vec6, f64, Vec6DivRhs, x, y, z, w, a, b)
scalar_div_impl!(Vec6, f32, Vec6DivRhs, x, y, z, w, a, b)
scalar_div_impl!(Vec6, u64, Vec6DivRhs, x, y, z, w, a, b)
scalar_div_impl!(Vec6, u32, Vec6DivRhs, x, y, z, w, a, b)
scalar_div_impl!(Vec6, u16, Vec6DivRhs, x, y, z, w, a, b)
scalar_div_impl!(Vec6, u8, Vec6DivRhs, x, y, z, w, a, b)
scalar_div_impl!(Vec6, i64, Vec6DivRhs, x, y, z, w, a, b)
scalar_div_impl!(Vec6, i32, Vec6DivRhs, x, y, z, w, a, b)
scalar_div_impl!(Vec6, i16, Vec6DivRhs, x, y, z, w, a, b)
scalar_div_impl!(Vec6, i8, Vec6DivRhs, x, y, z, w, a, b)
scalar_div_impl!(Vec6, uint, Vec6DivRhs, x, y, z, w, a, b)
scalar_div_impl!(Vec6, int, Vec6DivRhs, x, y, z, w, a, b)
scalar_add_impl!(Vec6, f64, Vec6AddRhs, x, y, z, w, a, b)
scalar_add_impl!(Vec6, f32, Vec6AddRhs, x, y, z, w, a, b)
scalar_add_impl!(Vec6, u64, Vec6AddRhs, x, y, z, w, a, b)
scalar_add_impl!(Vec6, u32, Vec6AddRhs, x, y, z, w, a, b)
scalar_add_impl!(Vec6, u16, Vec6AddRhs, x, y, z, w, a, b)
scalar_add_impl!(Vec6, u8, Vec6AddRhs, x, y, z, w, a, b)
scalar_add_impl!(Vec6, i64, Vec6AddRhs, x, y, z, w, a, b)
scalar_add_impl!(Vec6, i32, Vec6AddRhs, x, y, z, w, a, b)
scalar_add_impl!(Vec6, i16, Vec6AddRhs, x, y, z, w, a, b)
scalar_add_impl!(Vec6, i8, Vec6AddRhs, x, y, z, w, a, b)
scalar_add_impl!(Vec6, uint, Vec6AddRhs, x, y, z, w, a, b)
scalar_add_impl!(Vec6, int, Vec6AddRhs, x, y, z, w, a, b)
scalar_sub_impl!(Vec6, f64, Vec6SubRhs, x, y, z, w, a, b)
scalar_sub_impl!(Vec6, f32, Vec6SubRhs, x, y, z, w, a, b)
scalar_sub_impl!(Vec6, u64, Vec6SubRhs, x, y, z, w, a, b)
scalar_sub_impl!(Vec6, u32, Vec6SubRhs, x, y, z, w, a, b)
scalar_sub_impl!(Vec6, u16, Vec6SubRhs, x, y, z, w, a, b)
scalar_sub_impl!(Vec6, u8, Vec6SubRhs, x, y, z, w, a, b)
scalar_sub_impl!(Vec6, i64, Vec6SubRhs, x, y, z, w, a, b)
scalar_sub_impl!(Vec6, i32, Vec6SubRhs, x, y, z, w, a, b)
scalar_sub_impl!(Vec6, i16, Vec6SubRhs, x, y, z, w, a, b)
scalar_sub_impl!(Vec6, i8, Vec6SubRhs, x, y, z, w, a, b)
scalar_sub_impl!(Vec6, uint, Vec6SubRhs, x, y, z, w, a, b)
scalar_sub_impl!(Vec6, int, Vec6SubRhs, x, y, z, w, a, b)
translation_impl!(Vec6)
norm_impl!(Vec6, x, y, z, w, a, b)
approx_eq_impl!(Vec6, x, y, z, w, a, b)
round_impl!(Vec6, x, y, z, w, a, b)
one_impl!(Vec6, x, y, z, w, a, b)
from_iterator_impl!(Vec6, iterator, iterator, iterator, iterator, iterator, iterator)
bounded_impl!(Vec6, x, y, z, w, a, b)
iterable_impl!(Vec6, 6)
iterable_mut_impl!(Vec6, 6)
translate_impl!(Vec6)
rotate_impl!(Vec6)
transform_impl!(Vec6)
