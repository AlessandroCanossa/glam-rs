{% import "macros.rs" as macros %}

// Generated from {{template_path}} template. Edit the template, not the generated file.

{% if dim == 2 %}
    {% set deref_t = "XY::<" ~ scalar_t ~ ">" %}
{% elif dim == 3 %}
    {% set deref_t = "XYZ::<" ~ scalar_t ~ ">" %}
{% elif dim == 4 %}
    {% set deref_t = "XYZW::<" ~ scalar_t ~ ">" %}
{% endif %}

{% if is_scalar %}
    {% set mask_t = "BVec" ~ dim %}
{% else %}
    {% set is_simd = true %}
    {% if is_sse2 %}
        {% set simd_t = "__m128" %}
    {% elif is_wasm32 %}
        {% set simd_t = "v128" %}
    {% endif %}
    {% set mask_t = "BVec" ~ dim ~ "A" %}
{% endif %}

{% if scalar_t == "f32" or scalar_t == "f64" %}
    {% set is_signed = true %}
    {% set is_float = true %}
    {% if scalar_t == "f32" %}
        {% if dim == 3 and is_simd or is_align %}
            {% set self_t = "Vec3A" %}
        {% else %}
            {% set self_t = "Vec" ~ dim %}
        {% endif %}
        {% set vec2_t = "Vec2" %}
        {% set vec3_t = "Vec3" %}
        {% set vec3a_t = "Vec3A" %}
        {% set vec4_t = "Vec4" %}
    {% elif scalar_t == "f64" %}
        {% set self_t = "DVec" ~ dim %}
        {% set vec2_t = "DVec2" %}
        {% set vec3_t = "DVec3" %}
        {% set vec4_t = "DVec4" %}
    {% endif %}
{% elif scalar_t == "i32" %}
    {% set is_signed = true %}
    {% set is_float = false %}
    {% set self_t = "IVec" ~ dim %}
    {% set vec2_t = "IVec2" %}
    {% set vec3_t = "IVec3" %}
    {% set vec4_t = "IVec4" %}
{% else %}
    {% set is_signed = false %}
    {% set is_float = false %}
    {% set self_t = "UVec" ~ dim %}
    {% set vec2_t = "UVec2" %}
    {% set vec3_t = "UVec3" %}
    {% set vec4_t = "UVec4" %}
{% endif %}

{% set const_new = "const_" ~ self_t | lower %}

{% if scalar_t == "f64" or dim == 4 %}
    {% set cuda_align = 16 %}
{% elif dim == 2 %}
    {% set cuda_align = 8 %}
{% endif %}

{% set components = ["x", "y", "z", "w"] | slice(end = dim) %}
{% if is_float %}
    {% set one = "1.0" %}
    {% set zero = "0.0" %}
{% else %}
    {% set one = "1" %}
    {% set zero = "0" %}
{% endif %}
{% set unit_x = [one, zero, zero, zero] %}
{% set unit_y = [zero, one, zero, zero] %}
{% set unit_z = [zero, zero, one, zero] %}
{% set unit_w = [zero, zero, zero, one] %}
{% set identity = [unit_x, unit_y, unit_z, unit_w] %}

use crate::{
    {{ mask_t }},
    {% if self_t != vec2_t %}
        {{ vec2_t }},
    {% endif %}
    {% if self_t != vec3_t %}
        {{ vec3_t }},
    {% endif %}
    {% if self_t == "Vec3" or self_t == "Vec4" %}
        {{ vec3a_t }},
    {% endif %}
    {% if dim > 2 and self_t != vec4_t %}
        {{ vec4_t }},
    {% endif %}
};

#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::iter::{Product, Sum};
use core::{f32, ops::*};

{% if is_sse2 %}
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
{% elif is_wasm32 %}
use core::arch::wasm32::*;
{% endif %}

{% if is_float %}
#[cfg(not(feature = "std"))]
use num_traits::Float;
{% endif %}

{% if is_simd %}
union UnionCast {
    a: [f32; 4],
    v: {{ self_t }}
}
{% endif %}

/// Creates a {{ dim }}-dimensional vector.
#[inline(always)]
pub const fn {{ self_t | lower }}(
    {% for c in components %}
        {{ c }}: {{ scalar_t }},
    {% endfor %}
) -> {{ self_t }} {
    {{ self_t }}::new({{ components | join(sep=",") }})
}

{% if self_t == "Vec3A" %}
/// A 3-dimensional vector with SIMD support.
///
/// This type is 16 byte aligned. A SIMD vector type is used for storage on supported platforms for
/// better performance than the `Vec3` type.
///
/// It is possible to convert between `Vec3` and `Vec3A` types using `From` trait implementations.
{%- elif self_t == "Vec4" and is_simd %}
/// A 4-dimensional vector with SIMD support.
///
/// This type uses 16 byte aligned SIMD vector type for storage.
{%- else %}
/// A {{ dim }}-dimensional vector.
{%- endif %}
#[derive(Clone, Copy)]
{%- if self_t == "Vec3A" and is_scalar %}
#[repr(C, align(16))]
{%- elif self_t == "Vec4" and is_scalar %}
#[cfg_attr(
    any(
        not(any(feature = "scalar-math", target_arch = "spirv")),
        feature = "cuda"),
    repr(C, align(16))
)]
{%- elif dim != 3 and is_scalar %}
#[cfg_attr(feature = "cuda", repr(C, align({{ cuda_align }})))]
{%- endif %}
{%- if is_scalar %}
pub struct {{ self_t }}
{
    {% for c in components %}
        pub {{ c }}: {{ scalar_t }},
    {%- endfor %}
}
{% else %}
#[repr(transparent)]
pub struct {{ self_t }}(pub(crate) {{ simd_t }});
{% endif %}

impl {{ self_t }} {
    /// All zeroes.
    pub const ZERO: Self = Self::splat({{ zero }});

    /// All ones.
    pub const ONE: Self = Self::splat({{ one }});

{% if is_signed %}
    /// All negative ones.
    pub const NEG_ONE: Self = Self::splat(-{{ one }});
{% endif %}

{% if is_float %}
    /// All NAN.
    pub const NAN: Self = Self::splat({{ scalar_t }}::NAN);
{% endif %}

{% for i in range(end = dim) %}
    {% set C = components[i] | upper %}
    /// `[{{ identity[i] | slice(end = dim) | join(sep=", ") }}]`: a unit-length vector pointing along the positive {{ C }} axis.
    pub const {{ C }}: Self = Self::from_array({{ identity[i] | slice(end = dim) }});
{% endfor %}

    /// The unit axes.
    pub const AXES: [Self; {{ dim }}] = [
        {% for c in components %}
            Self::{{ c | upper }},
        {% endfor %}
    ];

    /// Creates a new vector.
    #[inline(always)]
    pub const fn new(
        {% for c in components %}
            {{ c }}: {{ scalar_t }},
        {% endfor %}
    ) -> Self {
        {% if is_scalar %}
            Self {
                {% for c in components %}
                    {{ c }},
                {%- endfor %}
            }
        {% elif is_sse2 %}
            unsafe {
                UnionCast { a: [
                    {% if dim == 3 %}
                        x, y, z, z
                    {% elif dim == 4 %}
                        x, y, z, w
                    {% endif %}
                ] }.v
            }
        {% elif is_wasm32 %}
            Self(f32x4(
                {% if dim == 3 %}
                    x, y, z, z
                {% elif dim == 4 %}
                    x, y, z, w
                {% endif %}
            ))
        {% endif %}
    }

    /// Creates a vector with all elements set to `v`.
    #[inline]
    pub const fn splat(v: {{ scalar_t }}) -> Self {
        {% if is_scalar %}
            Self {
                {% for c in components %}
                    {{ c }}: v,
                {% endfor %}
            }
        {% else %}
            unsafe { UnionCast { a: [v; 4] }.v }
        {% endif %}
    }

    /// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    /// for each element of `self`.
    ///
    /// A true element in the mask uses the corresponding element from `if_true`, and false
    /// uses the element from `if_false`.
    #[inline]
    pub fn select(mask: {{ mask_t }}, if_true: Self, if_false: Self) -> Self {
        {% if is_scalar %}
            Self {
                {% for c in components %}
                    {{ c  }}: if mask.{{ c }} { if_true.{{ c }} } else { if_false.{{ c }} },
                {%- endfor %}
            }
        {% elif is_sse2 %}
            Self(unsafe { _mm_or_ps(_mm_andnot_ps(mask.0, if_false.0), _mm_and_ps(if_true.0, mask.0)) })
        {% elif is_wasm32 %}
            Self(v128_bitselect(if_true.0, if_false.0, mask.0))
        {% endif %}
    }

    /// Creates a new vector from an array.
    #[inline]
    pub const fn from_array(a: [{{ scalar_t }}; {{ dim }}]) -> Self {
        Self::new(
            {% for c in components %}
                a[{{ loop.index0 }}],
            {%- endfor %}
        )
    }

    /// `[{{ components | join(sep=", ") }}]`
    #[inline]
    pub const fn to_array(&self) -> [{{ scalar_t }}; {{ dim }}] {
        {% if is_scalar %}
            [
                {% for c in components %}
                    self.{{ c }},
                {% endfor %}
            ]
        {% else %}
            unsafe { *(self as *const {{ self_t }} as *const [{{ scalar_t }}; {{ dim }}]) }
        {% endif %}
    }

    /// Creates a vector from the first N values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than N elements long.
    #[inline]
    pub const fn from_slice(slice: &[{{ scalar_t }}]) -> Self {
        Self::new(
            {% for c in components %}
                slice[{{ loop.index0 }}],
            {%- endfor %}
        )
    }

    /// Writes the elements of `self` to the first {{ dim }} elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than N elements long.
    #[inline]
    pub fn write_to_slice(self, slice: &mut [{{ scalar_t }}]) {
        {% if self_t == "Vec4" and is_sse2 %}
            unsafe {
                assert!(slice.len() >= 4);
                _mm_storeu_ps(slice.as_mut_ptr(), self.0);
            }
        {% else %}
            {% for c in components %}
                slice[{{ loop.index0 }}] = self.{{ c }};
            {%- endfor %}
        {% endif %}
    }

{% if dim == 2 %}
    /// Creates a 3D vector from `self` and the given `z` value.
    #[inline]
    pub const fn extend(self, z: {{ scalar_t }}) -> {{ vec3_t }} {
        {{ vec3_t }}::new(self.x, self.y, z)
    }
{% elif dim == 3 %}
    /// Internal method for creating a 3D vector from a 4D vector, discarding `w`.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn from_vec4(v: {{ vec4_t }}) -> Self {
        {% if is_scalar %}
            Self { x: v.x, y: v.y, z: v.z }
        {% else %}
            Self(v.0)
        {% endif %}
    }

    /// Creates a 4D vector from `self` and the given `w` value.
    #[inline]
    pub fn extend(self, w: {{ scalar_t }}) -> {{ vec4_t }} {
        {{ vec4_t }}::new(self.x, self.y, self.z, w)
    }

    /// Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
    ///
    /// Truncation may also be performed by using `self.xy()` or `{{ vec2_t }}::from()`.
    #[inline]
    pub fn truncate(self) -> {{ vec2_t }} {
        use crate::swizzles::Vec3Swizzles;
        self.xy()
    }
{% elif dim == 4 %}
    /// Creates a 2D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.
    ///
    /// Truncation to `{{ vec3_t }}` may also be performed by using `self.xyz()` or `{{ vec3_t }}::from()`.
{%- if scalar_t == "f32" %}
    ///
    /// To truncate to `Vec3A` use `Vec3A::from()`.
{%- endif %}
    #[inline]
    pub fn truncate(self) -> {{ vec3_t }} {
        use crate::swizzles::Vec4Swizzles;
        self.xyz()
    }
{% endif %}

    /// Computes the dot product of `self` and `rhs`.
    #[inline]
    pub fn dot(self, rhs: Self) -> {{ scalar_t }} {
        {% if is_scalar %}
            {% for c in components %}
                (self.{{ c }} * rhs.{{ c }}) {% if not loop.last %} + {% endif %}
            {%- endfor %}
        {% elif is_sse2 %}
            unsafe { crate::sse2::dot{{ dim }}(self.0, rhs.0) }
        {% elif is_wasm32 %}
            crate::wasm32::dot{{ dim }}(self.0, rhs.0)
        {% endif %}
    }

{% if dim == 3 %}
    /// Computes the cross product of `self` and `rhs`.
    #[inline]
    pub fn cross(self, rhs: Self) -> Self {
        {% if is_scalar %}
            Self {
                x: self.y * rhs.z - rhs.y * self.z,
                y: self.z * rhs.x - rhs.z * self.x,
                z: self.x * rhs.y - rhs.x * self.y,
            }
        {% elif is_sse2 %}
            unsafe {
                // x  <-  a.y*b.z - a.z*b.y
                // y  <-  a.z*b.x - a.x*b.z
                // z  <-  a.x*b.y - a.y*b.x
                // We can save a shuffle by grouping it in this wacky order:
                // (self.zxy() * rhs - self * rhs.zxy()).zxy()
                let lhszxy = _mm_shuffle_ps(self.0, self.0, 0b01_01_00_10);
                let rhszxy = _mm_shuffle_ps(rhs.0, rhs.0, 0b01_01_00_10);
                let lhszxy_rhs = _mm_mul_ps(lhszxy, rhs.0);
                let rhszxy_lhs = _mm_mul_ps(rhszxy, self.0);
                let sub = _mm_sub_ps(lhszxy_rhs, rhszxy_lhs);
                Self(_mm_shuffle_ps(sub, sub, 0b01_01_00_10))
            }
        {% elif is_wasm32 %}
            let lhszxy = i32x4_shuffle::<2, 0, 1, 1>(self.0, self.0);
            let rhszxy = i32x4_shuffle::<2, 0, 1, 1>(rhs.0, rhs.0);
            let lhszxy_rhs = f32x4_mul(lhszxy, rhs.0);
            let rhszxy_lhs = f32x4_mul(rhszxy, self.0);
            let sub = f32x4_sub(lhszxy_rhs, rhszxy_lhs);
            Self(i32x4_shuffle::<2, 0, 1, 1>(sub, sub))
        {% endif %}
    }
{% endif %}

    /// Returns a vector containing the minimum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
    #[inline]
    pub fn min(self, rhs: Self) -> Self {
        {% if is_scalar %}
            Self {
                {% for c in components %}
                    {{ c }}: self.{{ c }}.min(rhs.{{ c }}),
                {%- endfor %}
            }
        {% elif is_sse2 %}
            Self(unsafe { _mm_min_ps(self.0, rhs.0) })
        {% elif is_wasm32 %}
            Self(f32x4_pmin(self.0, rhs.0))
        {% endif %}
    }

    /// Returns a vector containing the maximum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
    #[inline]
    pub fn max(self, rhs: Self) -> Self {
        {% if is_scalar %}
            Self {
                {% for c in components %}
                    {{ c }}: self.{{ c }}.max(rhs.{{ c }}),
                {%- endfor %}
            }
        {% elif is_sse2 %}
            Self(unsafe { _mm_max_ps(self.0, rhs.0) })
        {% elif is_wasm32 %}
            Self(f32x4_pmax(self.0, rhs.0))
        {% endif %}
    }

    /// Component-wise clamping of values, similar to [`f32::clamp`].
    ///
    /// Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    /// # Panics
    ///
    /// Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        glam_assert!(min.cmple(max).all(), "clamp: expected min <= max");
        self.max(min).min(max)
    }

    /// Returns the horizontal minimum of `self`.
    ///
    /// In other words this computes `min(x, y, ..)`.
    #[inline]
    pub fn min_element(self) -> {{ scalar_t }} {
        {% if is_scalar %}
            {% if dim == 2 %}
                self.x.min(self.y)
            {% elif dim == 3 %}
                self.x.min(self.y.min(self.z))
            {% elif dim == 4 %}
                self.x.min(self.y.min(self.z.min(self.w)))
            {% endif %}
        {% elif is_sse2 %}
            {% if dim == 3 %}
                unsafe {
                    let v = self.0;
                    let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b01_01_10_10));
                    let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b00_00_00_01));
                    _mm_cvtss_f32(v)
                }
            {% elif dim == 4 %}
                unsafe {
                    let v = self.0;
                    let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b00_00_11_10));
                    let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b00_00_00_01));
                    _mm_cvtss_f32(v)
                }
            {% endif %}
        {% elif is_wasm32 %}
            {% if dim == 3 %}
                let v = self.0;
                let v = f32x4_pmin(v, i32x4_shuffle::<2, 2, 1, 1>(v, v));
                let v = f32x4_pmin(v, i32x4_shuffle::<1, 0, 0, 0>(v, v));
                f32x4_extract_lane::<0>(v)
            {% elif dim == 4 %}
                let v = self.0;
                let v = f32x4_pmin(v, i32x4_shuffle::<2, 3, 0, 0>(v, v));
                let v = f32x4_pmin(v, i32x4_shuffle::<1, 0, 0, 0>(v, v));
                f32x4_extract_lane::<0>(v)
            {% endif %}
        {% endif %}
    }

    /// Returns the horizontal maximum of `self`.
    ///
    /// In other words this computes `max(x, y, ..)`.
    #[inline]
    pub fn max_element(self) -> {{ scalar_t }} {
        {% if is_scalar %}
            {% if dim == 2 %}
                self.x.max(self.y)
            {% elif dim == 3 %}
                self.x.max(self.y.max(self.z))
            {% elif dim == 4 %}
                self.x.max(self.y.max(self.z.max(self.w)))
            {% endif %}
        {% elif is_sse2 %}
            {% if dim == 3 %}
                unsafe {
                    let v = self.0;
                    let v = _mm_max_ps(v, _mm_shuffle_ps(v, v, 0b00_00_10_10));
                    let v = _mm_max_ps(v, _mm_shuffle_ps(v, v, 0b00_00_00_01));
                    _mm_cvtss_f32(v)
                }
            {% elif dim == 4 %}
                unsafe {
                    let v = self.0;
                    let v = _mm_max_ps(v, _mm_shuffle_ps(v, v, 0b00_00_11_10));
                    let v = _mm_max_ps(v, _mm_shuffle_ps(v, v, 0b00_00_00_01));
                    _mm_cvtss_f32(v)
                }
            {% endif %}
        {% elif is_wasm32 %}
            {% if dim == 3 %}
                let v = self.0;
                let v = f32x4_pmax(v, i32x4_shuffle::<2, 2, 0, 0>(v, v));
                let v = f32x4_pmax(v, i32x4_shuffle::<1, 0, 0, 0>(v, v));
                f32x4_extract_lane::<0>(v)
            {% elif dim == 4 %}
                let v = self.0;
                let v = f32x4_pmax(v, i32x4_shuffle::<2, 3, 0, 0>(v, v));
                let v = f32x4_pmax(v, i32x4_shuffle::<1, 0, 0, 0>(v, v));
                f32x4_extract_lane::<0>(v)
            {% endif %}
        {% endif %}
    }

    /// Returns a vector mask containing the result of a `==` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmpeq(self, rhs: Self) -> {{ mask_t }} {
        {% if is_scalar %}
            {{ mask_t }}::new(
                {% for c in components %}
                    self.{{ c }}.eq(&rhs.{{ c }}),
                {%- endfor %}
            )
        {% elif is_sse2 %}
            {{ mask_t }}(unsafe { _mm_cmpeq_ps(self.0, rhs.0) })
        {% elif is_wasm32 %}
            {{ mask_t }}(f32x4_eq(self.0, rhs.0))
        {% endif %}
    }

    /// Returns a vector mask containing the result of a `!=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmpne(self, rhs: Self) -> {{ mask_t }} {
        {% if is_scalar %}
            {{ mask_t }}::new(
                {% for c in components %}
                    self.{{ c }}.ne(&rhs.{{ c }}),
                {%- endfor %}
            )
        {% elif is_sse2 %}
            {{ mask_t }}(unsafe { _mm_cmpneq_ps(self.0, rhs.0) })
        {% elif is_wasm32 %}
            {{ mask_t }}(f32x4_ne(self.0, rhs.0))
        {% endif %}
    }

    /// Returns a vector mask containing the result of a `>=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmpge(self, rhs: Self) -> {{ mask_t }} {
        {% if is_scalar %}
            {{ mask_t }}::new(
                {% for c in components %}
                    self.{{ c }}.ge(&rhs.{{ c }}),
                {%- endfor %}
            )
        {% elif is_sse2 %}
            {{ mask_t }}(unsafe { _mm_cmpge_ps(self.0, rhs.0) })
        {% elif is_wasm32 %}
            {{ mask_t }}(f32x4_ge(self.0, rhs.0))
        {% endif %}
    }

    /// Returns a vector mask containing the result of a `>` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmpgt(self, rhs: Self) -> {{ mask_t }} {
        {% if is_scalar %}
            {{ mask_t }}::new(
                {% for c in components %}
                    self.{{ c }}.gt(&rhs.{{ c }}),
                {%- endfor %}
            )
        {% elif is_sse2 %}
            {{ mask_t }}(unsafe { _mm_cmpgt_ps(self.0, rhs.0) })
        {% elif is_wasm32 %}
            {{ mask_t }}(f32x4_gt(self.0, rhs.0))
        {% endif %}
    }

    /// Returns a vector mask containing the result of a `<=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmple(self, rhs: Self) -> {{ mask_t }} {
        {% if is_scalar %}
            {{ mask_t }}::new(
                {% for c in components %}
                    self.{{ c }}.le(&rhs.{{ c }}),
                {%- endfor %}
            )
        {% elif is_sse2 %}
            {{ mask_t }}(unsafe { _mm_cmple_ps(self.0, rhs.0) })
        {% elif is_wasm32 %}
            {{ mask_t }}(f32x4_le(self.0, rhs.0))
        {% endif %}
    }

    /// Returns a vector mask containing the result of a `<` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmplt(self, rhs: Self) -> {{ mask_t }} {
        {% if is_scalar %}
            {{ mask_t }}::new(
                {% for c in components %}
                    self.{{ c }}.lt(&rhs.{{ c }}),
                {%- endfor %}
            )
        {% elif is_sse2 %}
            {{ mask_t }}(unsafe { _mm_cmplt_ps(self.0, rhs.0) })
        {% elif is_wasm32 %}
            {{ mask_t }}(f32x4_lt(self.0, rhs.0))
        {% endif %}
    }

{% if is_signed %}
    /// Returns a vector containing the absolute value of each element of `self`.
    #[inline]
    pub fn abs(self) -> Self {
        {% if is_scalar %}
            Self {
                {% for c in components %}
                    {{ c }}: self.{{ c }}.abs(),
                {%- endfor %}
            }
        {% elif is_sse2 %}
            Self(unsafe { crate::sse2::m128_abs(self.0) })
        {% elif is_wasm32 %}
            Self(f32x4_abs(self.0))
        {% endif %}
    }

    /// Returns a vector with elements representing the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the number is `NAN`
    #[inline]
    pub fn signum(self) -> Self {
        {% if is_scalar %}
            Self {
                {% for c in components %}
                    {{ c }}: self.{{ c }}.signum(),
                {%- endfor %}
            }
        {% else %}
            let mask = self.cmpge(Self::ZERO);
            let result = Self::select(mask, Self::ONE, Self::NEG_ONE);
            let mask = self.is_nan_mask();
            Self::select(mask, self, result)
        {% endif %}
    }
{% endif %}

{% if is_float %}
    /// Returns `true` if, and only if, all elements are finite.  If any element is either
    /// `NaN`, positive or negative infinity, this will return `false`.
    #[inline]
    pub fn is_finite(self) -> bool {
        {% for c in components %}
            self.{{ c }}.is_finite() {% if not loop.last %} && {% endif %}
        {%- endfor %}
    }

    /// Returns `true` if any elements are `NaN`.
    #[inline]
    pub fn is_nan(self) -> bool {
        {% if is_scalar %}
            {% for c in components %}
                self.{{ c }}.is_nan() {% if not loop.last %} || {% endif %}
            {%- endfor %}
        {% else %}
            self.is_nan_mask().any()
        {% endif %}
    }

    /// Performs `is_nan` on each element of self, returning a vector mask of the results.
    ///
    /// In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
    #[inline]
    pub fn is_nan_mask(self) -> {{ mask_t }} {
        {% if is_scalar %}
            {{ mask_t }}::new(
                {% for c in components %}
                    self.{{ c }}.is_nan(),
                {%- endfor %}
            )
        {% elif is_sse2 %}
            {{ mask_t }}(unsafe { _mm_cmpunord_ps(self.0, self.0) })
        {% elif is_wasm32 %}
            {{ mask_t }}(f32x4_ne(self.0, self.0))
        {% endif %}
    }

    /// Computes the length of `self`.
    #[doc(alias = "magnitude")]
    #[inline]
    pub fn length(self) -> {{ scalar_t }} {
        {% if is_scalar %}
            self.dot(self).sqrt()
        {% elif is_sse2 %}
            unsafe {
                let dot = crate::sse2::dot{{ dim }}_in_x(self.0, self.0);
                _mm_cvtss_f32(_mm_sqrt_ps(dot))
            }
        {% elif is_wasm32 %}
            let dot = crate::wasm32::dot{{ dim }}_in_x(self.0, self.0);
            f32x4_extract_lane::<0>(f32x4_sqrt(dot))
        {% endif %}
    }

    /// Computes the squared length of `self`.
    ///
    /// This is faster than `length()` as it avoids a square root operation.
    #[doc(alias = "magnitude2")]
    #[inline]
    pub fn length_squared(self) -> {{ scalar_t }} {
        self.dot(self)
    }

    /// Computes `1.0 / length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    pub fn length_recip(self) -> {{ scalar_t }} {
        {% if is_scalar %}
            self.length().recip()
        {% elif is_sse2 %}
            unsafe {
                let dot = crate::sse2::dot{{ dim }}_in_x(self.0, self.0);
                _mm_cvtss_f32(_mm_div_ps(Self::ONE.0, _mm_sqrt_ps(dot)))
            }
        {% elif is_wasm32 %}
            let dot = crate::wasm32::dot{{ dim }}_in_x(self.0, self.0);
            f32x4_extract_lane::<0>(f32x4_div(Self::ONE.0, f32x4_sqrt(dot)))
        {% endif %}
    }

    /// Computes the Euclidean distance between two points in space.
    #[inline]
    pub fn distance(self, rhs: Self) -> {{ scalar_t }} {
        (self - rhs).length()
    }

    /// Compute the squared euclidean distance between two points in space.
    #[inline]
    pub fn distance_squared(self, rhs: Self) -> {{ scalar_t }} {
        (self - rhs).length_squared()
    }

    /// Returns `self` normalized to length 1.0.
    ///
    /// For valid results, `self` must _not_ be of length zero, nor very close to zero.
    ///
    /// See also [`Self::try_normalize`] and [`Self::normalize_or_zero`].
    ///
    /// Panics
    ///
    /// Will panic if `self` is zero length when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn normalize(self) -> Self {
        {% if is_scalar %}
            #[allow(clippy::let_and_return)]
            let normalized = self.mul(self.length_recip());
            glam_assert!(normalized.is_finite());
            normalized
        {% elif is_sse2 %}
            unsafe {
                let length = _mm_sqrt_ps(crate::sse2::dot{{ dim }}_into_m128(self.0, self.0));
                #[allow(clippy::let_and_return)]
                let normalized = Self(_mm_div_ps(self.0, length));
                glam_assert!(normalized.is_finite());
                normalized
            }
        {% elif is_wasm32 %}
            let length = f32x4_sqrt(crate::wasm32::dot{{ dim }}_into_v128(self.0, self.0));
            #[allow(clippy::let_and_return)]
            let normalized = Self(f32x4_div(self.0, length));
            glam_assert!(normalized.is_finite());
            normalized
        {% endif %}
    }

    /// Returns `self` normalized to length 1.0 if possible, else returns `None`.
    ///
    /// In particular, if the input is zero (or very close to zero), or non-finite,
    /// the result of this operation will be `None`.
    ///
    /// See also [`Self::normalize_or_zero`].
    #[must_use]
    #[inline]
    pub fn try_normalize(self) -> Option<Self> {
        let rcp = self.length_recip();
        if rcp.is_finite() && rcp > 0.0 {
            Some(self * rcp)
        } else {
            None
        }
    }

    /// Returns `self` normalized to length 1.0 if possible, else returns zero.
    ///
    /// In particular, if the input is zero (or very close to zero), or non-finite,
    /// the result of this operation will be zero.
    ///
    /// See also [`Self::try_normalize`].
    #[must_use]
    #[inline]
    pub fn normalize_or_zero(self) -> Self {
        let rcp = self.length_recip();
        if rcp.is_finite() && rcp > 0.0 {
            self * rcp
        } else {
            Self::ZERO
        }
    }

    /// Returns whether `self` is length `1.0` or not.
    ///
    /// Uses a precision threshold of `1e-6`.
    #[inline]
    pub fn is_normalized(self) -> bool {
        // TODO: do something with epsilon
        (self.length_squared() - 1.0).abs() <= 1e-4
    }

    /// Returns the vector projection of `self` onto `rhs`.
    ///
    /// `rhs` must be of non-zero length.
    ///
    /// # Panics
    ///
    /// Will panic if `rhs` is zero length when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn project_onto(self, rhs: Self) -> Self {
        let other_len_sq_rcp = rhs.dot(rhs).recip();
        glam_assert!(other_len_sq_rcp.is_finite());
        rhs * self.dot(rhs) * other_len_sq_rcp
    }

    /// Returns the vector rejection of `self` from `rhs`.
    ///
    /// The vector rejection is the vector perpendicular to the projection of `self` onto
    /// `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
    ///
    /// `rhs` must be of non-zero length.
    ///
    /// # Panics
    ///
    /// Will panic if `rhs` has a length of zero when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn reject_from(self, rhs: Self) -> Self {
        self - self.project_onto(rhs)
    }

    /// Returns the vector projection of `self` onto `rhs`.
    ///
    /// `rhs` must be normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `rhs` is not normalized when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn project_onto_normalized(self, rhs: Self) -> Self {
        glam_assert!(rhs.is_normalized());
        rhs * self.dot(rhs)
    }

    /// Returns the vector rejection of `self` from `rhs`.
    ///
    /// The vector rejection is the vector perpendicular to the projection of `self` onto
    /// `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
    ///
    /// `rhs` must be normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `rhs` is not normalized when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn reject_from_normalized(self, rhs: Self) -> Self {
        self - self.project_onto_normalized(rhs)
    }

    /// Returns a vector containing the nearest integer to a number for each element of `self`.
    /// Round half-way cases away from 0.0.
    #[inline]
    pub fn round(self) -> Self {
        {% if is_scalar %}
            Self {
                {% for c in components %}
                    {{ c }}: self.{{ c }}.round(),
                {%- endfor %}
            }
        {% elif is_sse2 %}
            Self(unsafe { crate::sse2::m128_round(self.0) })
        {% elif is_wasm32 %}
            Self(f32x4_nearest(self.0))
        {% endif %}
    }

    /// Returns a vector containing the largest integer less than or equal to a number for each
    /// element of `self`.
    #[inline]
    pub fn floor(self) -> Self {
        {% if is_scalar %}
            Self {
                {% for c in components %}
                    {{ c }}: self.{{ c }}.floor(),
                {%- endfor %}
            }
        {% elif is_sse2 %}
            Self(unsafe { crate::sse2::m128_floor(self.0) })
        {% elif is_wasm32 %}
            Self(f32x4_floor(self.0))
        {% endif %}
    }

    /// Returns a vector containing the smallest integer greater than or equal to a number for
    /// each element of `self`.
    #[inline]
    pub fn ceil(self) -> Self {
        {% if is_scalar %}
            Self {
                {% for c in components %}
                    {{ c }}: self.{{ c }}.ceil(),
                {%- endfor %}
            }
        {% elif is_sse2 %}
            Self(unsafe { crate::sse2::m128_ceil(self.0) })
        {% elif is_wasm32 %}
            Self(f32x4_ceil(self.0))
        {% endif %}
    }

    /// Returns a vector containing the fractional part of the vector, e.g. `self -
    /// self.floor()`.
    ///
    /// Note that this is fast but not precise for large numbers.
    #[inline]
    pub fn fract(self) -> Self {
        self - self.floor()
    }

    /// Returns a vector containing `e^self` (the exponential function) for each element of
    /// `self`.
    #[inline]
    pub fn exp(self) -> Self {
        Self::new(
            {% for c in components %}
                self.{{ c }}.exp(),
            {%- endfor %}
        )
    }

    /// Returns a vector containing each element of `self` raised to the power of `n`.
    #[inline]
    pub fn powf(self, n: {{ scalar_t }}) -> Self {
        Self::new(
            {% for c in components %}
                self.{{ c }}.powf(n),
            {%- endfor %}
        )
    }

    /// Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
    #[inline]
    pub fn recip(self) -> Self {
        {% if is_scalar %}
            Self {
                {% for c in components %}
                    {{ c }}: self.{{ c }}.recip(),
                {%- endfor %}
            }
        {% elif is_sse2 %}
            Self(unsafe { _mm_div_ps(Self::ONE.0, self.0) })
        {% elif is_wasm32 %}
            Self(f32x4_div(Self::ONE.0, self.0))
        {% endif %}
    }

    /// Performs a linear interpolation between `self` and `rhs` based on the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
    /// will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
    /// extrapolated.
    #[doc(alias = "mix")]
    #[inline]
    pub fn lerp(self, rhs: Self, s: {{ scalar_t }}) -> Self {
        self + ((rhs - self) * s)
    }

    /// Returns true if the absolute difference of all elements between `self` and `rhs` is
    /// less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two vectors contain similar elements. It works best when
    /// comparing with a known value. The `max_abs_diff` that should be used used depends on
    /// the values being compared against.
    ///
    /// For more see
    /// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[inline]
    pub fn abs_diff_eq(self, rhs: Self, max_abs_diff: {{ scalar_t }}) -> bool {
        self.sub(rhs).abs().cmple(Self::splat(max_abs_diff)).all()
    }

    /// Returns a vector with a length no less than `min` and no more than `max`
    ///
    /// # Panics
    ///
    /// Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[inline]
    pub fn clamp_length(self, min: {{ scalar_t }}, max: {{ scalar_t }}) -> Self {
        glam_assert!(min <= max);
        let length_sq = self.length_squared();
        if length_sq < min * min {
            self * (length_sq.sqrt().recip() * min)
        } else if length_sq > max * max {
            self * (length_sq.sqrt().recip() * max)
        } else {
            self
        }
    }

    /// Returns a vector with a length no more than `max`
    pub fn clamp_length_max(self, max: {{ scalar_t }}) -> Self {
        let length_sq = self.length_squared();
        if length_sq > max * max {
            self * (length_sq.sqrt().recip() * max)
        } else {
            self
        }
    }

    /// Returns a vector with a length no less than `min`
    pub fn clamp_length_min(self, min: {{ scalar_t }}) -> Self {
        let length_sq = self.length_squared();
        if length_sq < min * min {
            self * (length_sq.sqrt().recip() * min)
        } else {
            self
        }
    }

    /// Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
    /// error, yielding a more accurate result than an unfused multiply-add.
    ///
    /// Using `mul_add` *may* be more performant than an unfused multiply-add if the target
    /// architecture has a dedicated fma CPU instruction. However, this is not always true,
    /// and will be heavily dependant on designing algorithms with specific target hardware in
    /// mind.
    #[inline]
    pub fn mul_add(self, a: Self, b: Self) -> Self {
        {% if is_sse2 %}
            #[cfg(target_feature = "fma")]
            unsafe { _mm_fmadd_ps(self, b, c) }
            #[cfg(not(target_feature = "fma"))]
        {% endif %}
        Self::new(
            {% for c in components %}
                self.{{ c }}.mul_add(a.{{ c }}, b.{{ c }}),
            {%- endfor %}
        )
    }

{% if dim == 2 %}
    /// Creates a 2D vector containing `[angle.cos(), angle.sin()]`. This can be used in
    /// conjunction with the `rotate` method, e.g. `Vec2::from_angle(PI).rotate(Vec2::Y)` will
    /// create the vector [-1, 0] and rotate `Vec2::Y` around it returning `-Vec2::Y`.
    #[inline]
    pub fn from_angle(angle: {{ scalar_t }}) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self {
            x: cos,
            y: sin,
        }
    }

    /// Returns the angle (in radians) between `self` and `rhs`.
    ///
    /// The input vectors do not need to be unit length however they must be non-zero.
    #[inline]
    pub fn angle_between(self, rhs: Self) -> {{ scalar_t }} {
        use crate::FloatEx;
        let angle = (self.dot(rhs) / (self.length_squared() * rhs.length_squared()).sqrt())
            .acos_approx();

        angle * self.perp_dot(rhs).signum()
    }
{% elif dim == 3 %}
    /// Returns the angle (in radians) between two vectors.
    ///
    /// The input vectors do not need to be unit length however they must be non-zero.
    #[inline]
    pub fn angle_between(self, rhs: Self) -> {{ scalar_t }} {
        use crate::FloatEx;
        self.dot(rhs)
            .div(self.length_squared().mul(rhs.length_squared()).sqrt())
            .acos_approx()
    }

    /// Returns some vector that is orthogonal to the given one.
    ///
    /// The input vector must be finite and non-zero.
    ///
    /// The output vector is not necessarily unit-length.
    /// For that use [`Self::any_orthonormal_vector`] instead.
    #[inline]
    pub fn any_orthogonal_vector(&self) -> Self {
        // This can probably be optimized
        if self.x.abs() > self.y.abs() {
            Self::new(-self.z, 0.0, self.x) // self.cross(Self::Y)
        } else {
            Self::new(0.0, self.z, -self.y) // self.cross(Self::X)
        }
    }

    /// Returns any unit-length vector that is orthogonal to the given one.
    /// The input vector must be finite and non-zero.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn any_orthonormal_vector(&self) -> Self {
        glam_assert!(self.is_normalized());
        // From https://graphics.pixar.com/library/OrthonormalB/paper.pdf
        #[cfg(feature = "std")]
        let sign = (1.0_{{ scalar_t }}).copysign(self.z);
        #[cfg(not(feature = "std"))]
        let sign = self.z.signum();
        let a = -1.0 / (sign + self.z);
        let b = self.x * self.y * a;
        Self::new(b, sign + self.y * self.y * a, -self.y)
    }

    /// Given a unit-length vector return two other vectors that together form an orthonormal
    /// basis.  That is, all three vectors are orthogonal to each other and are normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn any_orthonormal_pair(&self) -> (Self, Self) {
        glam_assert!(self.is_normalized());
        // From https://graphics.pixar.com/library/OrthonormalB/paper.pdf
        #[cfg(feature = "std")]
        let sign = (1.0_{{ scalar_t }}).copysign(self.z);
        #[cfg(not(feature = "std"))]
        let sign = self.z.signum();
        let a = -1.0 / (sign + self.z);
        let b = self.x * self.y * a;
        (
            Self::new(1.0 + sign * self.x * self.x * a, sign * b, -sign * self.x),
            Self::new(b, sign + self.y * self.y * a, -self.y),
        )
    }
{% endif %}
{% endif %}

{% if is_signed and dim == 2 %}
    /// Returns a vector that is equal to `self` rotated by 90 degrees.
    #[inline]
    pub fn perp(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    /// The perpendicular dot product of `self` and `rhs`.
    /// Also known as the wedge product, 2D cross product, and determinant.
    #[doc(alias = "wedge")]
    #[doc(alias = "cross")]
    #[doc(alias = "determinant")]
    #[inline]
    pub fn perp_dot(self, rhs: Self) -> {{ scalar_t }} {
        (self.x * rhs.y) - (self.y * rhs.x)
    }

    /// Returns `rhs` rotated by the angle of `self`. If `self` is normalized,
    /// then this just rotation. This is what you usually want. Otherwise,
    /// it will be like a rotation with a multiplication by `self`'s length.
    #[must_use]
    #[inline]
    pub fn rotate(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x - self.y * rhs.y,
            y: self.y * rhs.x + self.x * rhs.y,
        }
    }
{% endif %}

{% if scalar_t != "f32" %}
    {% if dim == 2 %}
    /// Casts all elements of `self` to `f32`.
    #[inline]
    pub fn as_vec2(&self) -> crate::Vec2 {
        crate::Vec2::new(self.x as f32, self.y as f32)
    }
    {% elif dim == 3 %}
    /// Casts all elements of `self` to `f32`.
    #[inline]
    pub fn as_vec3(&self) -> crate::Vec3 {
        crate::Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }

    /// Casts all elements of `self` to `f32`.
    #[inline]
    pub fn as_vec3a(&self) -> crate::Vec3A {
        crate::Vec3A::new(self.x as f32, self.y as f32, self.z as f32)
    }
    {% elif dim == 4 %}
    /// Casts all elements of `self` to `f32`.
    #[inline]
    pub fn as_vec4(&self) -> crate::Vec4 {
        crate::Vec4::new(self.x as f32, self.y as f32, self.z as f32, self.w as f32)
    }
    {% endif %}
{% endif %}
{% if scalar_t != "f64" %}
    {% if dim == 2 %}
    /// Casts all elements of `self` to `f64`.
    #[inline]
    pub fn as_dvec2(&self) -> crate::DVec2 {
        crate::DVec2::new(self.x as f64, self.y as f64)
    }
    {% elif dim == 3 %}
    /// Casts all elements of `self` to `f64`.
    #[inline]
    pub fn as_dvec3(&self) -> crate::DVec3 {
        crate::DVec3::new(self.x as f64, self.y as f64, self.z as f64)
    }
    {% elif dim == 4 %}
    /// Casts all elements of `self` to `f64`.
    #[inline]
    pub fn as_dvec4(&self) -> crate::DVec4 {
        crate::DVec4::new(self.x as f64, self.y as f64, self.z as f64, self.w as f64)
    }
    {% endif %}
{% endif %}
{% if scalar_t != "i32" %}
    {% if dim == 2 %}
    /// Casts all elements of `self` to `i32`.
    #[inline]
    pub fn as_ivec2(&self) -> crate::IVec2 {
        crate::IVec2::new(self.x as i32, self.y as i32)
    }
    {% elif dim == 3 %}
    /// Casts all elements of `self` to `i32`.
    #[inline]
    pub fn as_ivec3(&self) -> crate::IVec3 {
        crate::IVec3::new(self.x as i32, self.y as i32, self.z as i32)
    }
    {% elif dim == 4 %}
    /// Casts all elements of `self` to `i32`.
    #[inline]
    pub fn as_ivec4(&self) -> crate::IVec4 {
        crate::IVec4::new(self.x as i32, self.y as i32, self.z as i32, self.w as i32)
    }
    {% endif %}
{% endif %}
{% if scalar_t != "u32" %}
    {% if dim == 2 %}
    /// Casts all elements of `self` to `u32`.
    #[inline]
    pub fn as_uvec2(&self) -> crate::UVec2 {
        crate::UVec2::new(self.x as u32, self.y as u32)
    }
    {% elif dim == 3 %}
    /// Casts all elements of `self` to `u32`.
    #[inline]
    pub fn as_uvec3(&self) -> crate::UVec3 {
        crate::UVec3::new(self.x as u32, self.y as u32, self.z as u32)
    }
    {% elif dim == 4 %}
    /// Casts all elements of `self` to `u32`.
    #[inline]
    pub fn as_uvec4(&self) -> crate::UVec4 {
        crate::UVec4::new(self.x as u32, self.y as u32, self.z as u32, self.w as u32)
    }
    {% endif %}
{% endif %}
}

impl Default for {{ self_t }} {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl PartialEq for {{ self_t }} {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.cmpeq(*rhs).all()
    }
}

impl Div<{{ self_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        {% if is_scalar %}
            Self {
                {% for c in components %}
                    {{ c }}: self.{{ c }}.div(rhs.{{ c }}),
                {%- endfor %}
            }
        {% elif is_sse2 %}
            Self(unsafe { _mm_div_ps(self.0, rhs.0) })
        {% elif is_wasm32 %}
            Self(f32x4_div(self.0, rhs.0))
        {% endif %}
    }
}

impl DivAssign<{{ self_t }}> for {{ self_t }} {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        {% if is_scalar %}
            {% for c in components %}
                self.{{ c }}.div_assign(rhs.{{ c }});
            {%- endfor %}
        {% elif is_sse2 %}
            self.0 = unsafe { _mm_div_ps(self.0, rhs.0) };
        {% elif is_wasm32 %}
            self.0 = f32x4_div(self.0, rhs.0);
        {% endif %}
    }
}

impl Div<{{ scalar_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn div(self, rhs: {{ scalar_t }}) -> Self {
        {% if is_scalar %}
            Self {
                {% for c in components %}
                    {{ c }}: self.{{ c }}.div(rhs),
                {%- endfor %}
            }
        {% elif is_sse2 %}
            Self(unsafe { _mm_div_ps(self.0, _mm_set1_ps(rhs)) })
        {% elif is_wasm32 %}
            Self(f32x4_div(self.0, f32x4_splat(rhs)))
        {% endif %}
    }
}

impl DivAssign<{{ scalar_t }}> for {{ self_t }} {
    #[inline]
    fn div_assign(&mut self, rhs: {{ scalar_t }}) {
        {% if is_scalar %}
            {% for c in components %}
                self.{{ c }}.div_assign(rhs);
            {%- endfor %}
        {% elif is_sse2 %}
            self.0 = unsafe { _mm_div_ps(self.0, _mm_set1_ps(rhs)) };
        {% elif is_wasm32 %}
            self.0 = f32x4_div(self.0, f32x4_splat(rhs))
        {% endif %}
    }
}

impl Div<{{ self_t }}> for {{ scalar_t }} {
    type Output = {{ self_t }};
    #[inline]
    fn div(self, rhs: {{ self_t }}) -> {{ self_t }} {
        {% if is_scalar %}
            {{ self_t }} {
                {% for c in components %}
                    {{ c }}: self.div(rhs.{{ c }}),
                {%- endfor %}
            }
        {% elif is_sse2 %}
            {{ self_t }}(unsafe { _mm_div_ps(_mm_set1_ps(self), rhs.0) })
        {% elif is_wasm32 %}
            {{ self_t }}(f32x4_div(f32x4_splat(self), rhs.0))
        {% endif %}
    }
}

impl Mul<{{ self_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        {% if is_scalar %}
            Self {
                {% for c in components %}
                    {{ c }}: self.{{ c }}.mul(rhs.{{ c }}),
                {%- endfor %}
            }
        {% elif is_sse2 %}
            Self(unsafe { _mm_mul_ps(self.0, rhs.0) })
        {% elif is_wasm32 %}
            Self(f32x4_mul(self.0, rhs.0))
        {% endif %}
    }
}

impl MulAssign<{{ self_t }}> for {{ self_t }} {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        {% if is_scalar %}
            {% for c in components %}
                self.{{ c }}.mul_assign(rhs.{{ c }});
            {%- endfor %}
        {% elif is_sse2 %}
            self.0 = unsafe { _mm_mul_ps(self.0, rhs.0) };
        {% elif is_wasm32 %}
            self.0 = f32x4_mul(self.0, rhs.0);
        {% endif %}
    }
}

impl Mul<{{ scalar_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: {{ scalar_t }}) -> Self {
        {% if is_scalar %}
            Self {
                {% for c in components %}
                    {{ c }}: self.{{ c }}.mul(rhs),
                {%- endfor %}
            }
        {% elif is_sse2 %}
            Self(unsafe { _mm_mul_ps(self.0, _mm_set1_ps(rhs)) })
        {% elif is_wasm32 %}
            Self(f32x4_mul(self.0, f32x4_splat(rhs)))
        {% endif %}
    }
}

impl MulAssign<{{ scalar_t }}> for {{ self_t }} {
    #[inline]
    fn mul_assign(&mut self, rhs: {{ scalar_t }}) {
        {% if is_scalar %}
            {% for c in components %}
                self.{{ c }}.mul_assign(rhs);
            {%- endfor %}
        {% elif is_sse2 %}
            self.0 = unsafe { _mm_mul_ps(self.0, _mm_set1_ps(rhs)) };
        {% elif is_wasm32 %}
            self.0 = f32x4_mul(self.0, f32x4_splat(rhs))
        {% endif %}
    }
}

impl Mul<{{ self_t }}> for {{ scalar_t }} {
    type Output = {{ self_t }};
    #[inline]
    fn mul(self, rhs: {{ self_t }}) -> {{ self_t }} {
        {% if is_scalar %}
            {{ self_t }} {
                {% for c in components %}
                    {{ c }}: self.mul(rhs.{{ c }}),
                {%- endfor %}
            }
        {% elif is_sse2 %}
            {{ self_t }}(unsafe { _mm_mul_ps(_mm_set1_ps(self), rhs.0) })
        {% elif is_wasm32 %}
            {{ self_t }}(f32x4_mul(f32x4_splat(self), rhs.0))
        {% endif %}
    }
}

impl Add<{{ self_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        {% if is_scalar %}
            Self {
                {% for c in components %}
                    {{ c }}: self.{{ c }}.add(rhs.{{ c }}),
                {%- endfor %}
            }
        {% elif is_sse2 %}
            Self(unsafe { _mm_add_ps(self.0, rhs.0) })
        {% elif is_wasm32 %}
            Self(f32x4_add(self.0, rhs.0))
        {% endif %}
    }
}

impl AddAssign<{{ self_t }}> for {{ self_t }} {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        {% if is_scalar %}
            {% for c in components %}
                self.{{ c }}.add_assign(rhs.{{ c }});
            {%- endfor %}
        {% elif is_sse2 %}
            self.0 = unsafe { _mm_add_ps(self.0, rhs.0) };
        {% elif is_wasm32 %}
            self.0 = f32x4_add(self.0, rhs.0);
        {% endif %}
    }
}

impl Add<{{ scalar_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn add(self, rhs: {{ scalar_t }}) -> Self {
        {% if is_scalar %}
            Self {
                {% for c in components %}
                    {{ c }}: self.{{ c }}.add(rhs),
                {%- endfor %}
            }
        {% elif is_sse2 %}
            Self(unsafe { _mm_add_ps(self.0, _mm_set1_ps(rhs)) })
        {% elif is_wasm32 %}
            Self(f32x4_add(self.0, f32x4_splat(rhs)))
        {% endif %}
    }
}

impl AddAssign<{{ scalar_t }}> for {{ self_t }} {
    #[inline]
    fn add_assign(&mut self, rhs: {{ scalar_t }}) {
        {% if is_scalar %}
            {% for c in components %}
                self.{{ c }}.add_assign(rhs);
            {%- endfor %}
        {% elif is_sse2 %}
            self.0 = unsafe { _mm_add_ps(self.0, _mm_set1_ps(rhs)) };
        {% elif is_wasm32 %}
            self.0 = f32x4_add(self.0, f32x4_splat(rhs))
        {% endif %}
    }
}

impl Add<{{ self_t }}> for {{ scalar_t }} {
    type Output = {{ self_t }};
    #[inline]
    fn add(self, rhs: {{ self_t }}) -> {{ self_t }} {
        {% if is_scalar %}
            {{ self_t }} {
                {% for c in components %}
                    {{ c }}: self.add(rhs.{{ c }}),
                {%- endfor %}
            }
        {% elif is_sse2 %}
            {{ self_t }}(unsafe { _mm_add_ps(_mm_set1_ps(self), rhs.0) })
        {% elif is_wasm32 %}
            {{ self_t }}(f32x4_add(f32x4_splat(self), rhs.0))
        {% endif %}
    }
}

impl Sub<{{ self_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        {% if is_scalar %}
            Self {
                {% for c in components %}
                    {{ c }}: self.{{ c }}.sub(rhs.{{ c }}),
                {%- endfor %}
            }
        {% elif is_sse2 %}
            Self(unsafe { _mm_sub_ps(self.0, rhs.0) })
        {% elif is_wasm32 %}
            Self(f32x4_sub(self.0, rhs.0))
        {% endif %}
    }
}

impl SubAssign<{{ self_t }}> for {{ self_t }} {
    #[inline]
    fn sub_assign(&mut self, rhs: {{ self_t }}) {
        {% if is_scalar %}
            {% for c in components %}
                self.{{ c }}.sub_assign(rhs.{{ c }});
            {%- endfor %}
        {% elif is_sse2 %}
            self.0 = unsafe { _mm_sub_ps(self.0, rhs.0) };
        {% elif is_wasm32 %}
            self.0 = f32x4_sub(self.0, rhs.0);
        {% endif %}
    }
}

impl Sub<{{ scalar_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: {{ scalar_t }}) -> Self {
        {% if is_scalar %}
            Self {
                {% for c in components %}
                    {{ c }}: self.{{ c }}.sub(rhs),
                {%- endfor %}
            }
        {% elif is_sse2 %}
            Self(unsafe { _mm_sub_ps(self.0, _mm_set1_ps(rhs)) })
        {% elif is_wasm32 %}
            Self(f32x4_sub(self.0, f32x4_splat(rhs)))
        {% endif %}
    }
}

impl SubAssign<{{ scalar_t }}> for {{ self_t }} {
    #[inline]
    fn sub_assign(&mut self, rhs: {{ scalar_t }}) {
        {% if is_scalar %}
            {% for c in components %}
                self.{{ c }}.sub_assign(rhs);
            {%- endfor %}
        {% elif is_sse2 %}
            self.0 = unsafe { _mm_sub_ps(self.0, _mm_set1_ps(rhs)) };
        {% elif is_wasm32 %}
            self.0 = f32x4_sub(self.0, f32x4_splat(rhs))
        {% endif %}
    }
}

impl Sub<{{ self_t }}> for {{ scalar_t }} {
    type Output = {{ self_t }};
    #[inline]
    fn sub(self, rhs: {{ self_t }}) -> {{ self_t }} {
        {% if is_scalar %}
            {{ self_t }} {
                {% for c in components %}
                    {{ c }}: self.sub(rhs.{{ c }}),
                {%- endfor %}
            }
        {% elif is_sse2 %}
            {{ self_t }}(unsafe { _mm_sub_ps(_mm_set1_ps(self), rhs.0) })
        {% elif is_wasm32 %}
            {{ self_t }}(f32x4_sub(f32x4_splat(self), rhs.0))
        {% endif %}
    }
}

impl Rem<{{ self_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        {% if is_scalar %}
            Self {
                {% for c in components %}
                    {{ c }}: self.{{ c }}.rem(rhs.{{ c }}),
                {%- endfor %}
            }
        {% elif is_sse2 %}
            unsafe {
                let n = crate::sse2::m128_floor(_mm_div_ps(self.0, rhs.0));
                Self(_mm_sub_ps(self.0, _mm_mul_ps(n, rhs.0)))
            }
        {% elif is_wasm32 %}
            let n = f32x4_floor(f32x4_div(self.0, rhs.0));
            Self(f32x4_sub(self.0, f32x4_mul(n, rhs.0)))
        {% endif %}
    }
}

impl RemAssign<{{ self_t }}> for {{ self_t }} {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        {% if is_scalar %}
            {% for c in components %}
                self.{{ c }}.rem_assign(rhs.{{ c }});
            {%- endfor %}
        {% else %}
            *self = self.rem(rhs);
        {% endif %}
    }
}

impl Rem<{{ scalar_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: {{ scalar_t }}) -> Self {
        {% if is_scalar %}
            Self {
                {% for c in components %}
                    {{ c }}: self.{{ c }}.rem(rhs),
                {%- endfor %}
            }
        {% else %}
            self.rem(Self::splat(rhs))
        {% endif %}
    }
}

impl RemAssign<{{ scalar_t }}> for {{ self_t }} {
    #[inline]
    fn rem_assign(&mut self, rhs: {{ scalar_t }}) {
        {% if is_scalar %}
            {% for c in components %}
                self.{{ c }}.rem_assign(rhs);
            {%- endfor %}
        {% else %}
            *self = self.rem(Self::splat(rhs));
        {% endif %}
    }
}

impl Rem<{{ self_t }}> for {{ scalar_t }} {
    type Output = {{ self_t }};
    #[inline]
    fn rem(self, rhs: {{ self_t }}) -> {{ self_t }} {
        {% if is_scalar %}
            {{ self_t }} {
                {% for c in components %}
                    {{ c }}: self.rem(rhs.{{ c }}),
                {%- endfor %}
            }
        {% else %}
            {{ self_t }}::splat(self).rem(rhs)
        {% endif %}
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[{{ scalar_t }}; {{ dim }}]> for {{ self_t }} {
    #[inline]
    fn as_ref(&self) -> &[{{ scalar_t }}; {{ dim }}] {
        unsafe { &*(self as *const {{ self_t }} as *const [{{ scalar_t }}; {{ dim }}]) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsMut<[{{ scalar_t }}; {{ dim }}]> for {{ self_t }} {
    #[inline]
    fn as_mut(&mut self) -> &mut [{{ scalar_t }}; {{ dim }}] {
        unsafe { &mut *(self as *mut {{ self_t }} as *mut [{{ scalar_t }}; {{ dim }}]) }
    }
}

impl<'a> Sum<&'a Self> for {{ self_t }} {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl<'a> Product<&'a Self> for {{ self_t }} {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, &b| Self::mul(a, b))
    }
}

{% if is_signed %}
impl Neg for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        {% if is_scalar %}
            Self {
                {% for c in components %}
                    {{ c }}: self.{{ c }}.neg(),
                {%- endfor %}
            }
        {% elif is_sse2 %}
            Self(unsafe { _mm_sub_ps(Self::ZERO.0, self.0) })
        {% elif is_wasm32 %}
            Self(f32x4_neg(self.0))
        {% endif %}
    }
}
{% endif %}

{% if not is_float %}
impl Eq for {{ self_t }} {}

#[cfg(not(target_arch = "spirv"))]
impl core::hash::Hash for {{ self_t }} {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let inner: &[{{ scalar_t }}; {{ dim }}] = self.as_ref();
        inner.hash(state);
    }
}

impl Not for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        Self {
            {% for c in components %}
                {{ c }}: self.{{ c }}.not(),
            {%- endfor %}
        }
    }
}

impl BitAnd for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            {% for c in components %}
                {{ c }}: self.{{ c }}.bitand(rhs.{{ c }}),
            {%- endfor %}
        }
    }
}

impl BitOr for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            {% for c in components %}
                {{ c }}: self.{{ c }}.bitor(rhs.{{ c }}),
            {%- endfor %}
        }
    }
}

impl BitXor for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            {% for c in components %}
                {{ c }}: self.{{ c }}.bitxor(rhs.{{ c }}),
            {%- endfor %}
        }
    }
}

impl BitAnd<{{ scalar_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: {{ scalar_t }}) -> Self::Output {
        Self {
            {% for c in components %}
                {{ c }}: self.{{ c }}.bitand(rhs),
            {%- endfor %}
        }
    }
}

impl BitOr<{{ scalar_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: {{ scalar_t }}) -> Self::Output {
        Self {
            {% for c in components %}
                {{ c }}: self.{{ c }}.bitor(rhs),
            {%- endfor %}
        }
    }
}

impl BitXor<{{ scalar_t }}> for {{ self_t }} {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: {{ scalar_t }}) -> Self::Output {
        Self {
            {% for c in components %}
                {{ c }}: self.{{ c }}.bitxor(rhs),
            {%- endfor %}
        }
    }
}

{% for rhs_t in ["i8", "i16", "i32", "u8", "u16", "u32"] %}
    impl Shl<{{ rhs_t }}> for {{ self_t }} {
        type Output = Self;
        #[inline]
        fn shl(self, rhs: {{ rhs_t }}) -> Self::Output {
            Self {
                {% for c in components %}
                    {{ c }}: self.{{ c }}.shl(rhs),
                {%- endfor %}
            }
        }
    }

    impl Shr<{{ rhs_t }}> for {{ self_t }} {
        type Output = Self;
        #[inline]
        fn shr(self, rhs: {{ rhs_t }}) -> Self::Output {
            Self {
                {% for c in components %}
                    {{ c }}: self.{{ c }}.shr(rhs),
                {%- endfor %}
            }
        }
    }
{% endfor %}

{% for rhs_t in ["crate::IVec" ~ dim, "crate::UVec" ~ dim] %}
        impl Shl<{{ rhs_t }}> for {{ self_t }} {
            type Output = Self;
            #[inline]
            fn shl(self, rhs: {{ rhs_t }}) -> Self::Output {
                Self {
                    {% for c in components %}
                        {{ c }}: self.{{ c }}.shl(rhs.{{ c }}),
                    {%- endfor %}
                }
            }
        }

        impl Shr<{{ rhs_t }}> for {{ self_t }} {
            type Output = Self;
            #[inline]
            fn shr(self, rhs: {{ rhs_t }}) -> Self::Output {
                Self {
                    {% for c in components %}
                        {{ c }}: self.{{ c }}.shr(rhs.{{ c }}),
                    {%- endfor %}
                }
            }
        }
{% endfor %}
{% endif %}

impl Index<usize> for {{ self_t }} {
    type Output = {{ scalar_t }};
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            {% for c in components %}
                {{ loop.index0 }} => &self.{{ c }},
            {%- endfor %}
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for {{ self_t }} {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            {% for c in components %}
                {{ loop.index0 }} => &mut self.{{ c }},
            {%- endfor %}
            _ => panic!("index out of bounds"),
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for {{ self_t }} {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        {% if dim == 2 %}
            write!(f, "[{}, {}]", self.x, self.y)
        {% elif dim == 3 %}
            write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
        {% elif dim == 4 %}
            write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
        {% endif %}
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for {{ self_t }} {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!({{ self_t }}))
            {% for c in components %}
                .field(&self.{{ c }})
            {% endfor %}
            .finish()
    }
}

{% if not is_scalar %}
impl From<{{ self_t }}> for {{ simd_t }} {
    #[inline]
    fn from(t: {{ self_t }}) -> Self {
        t.0
    }
}

impl From<{{ simd_t }}> for {{ self_t }} {
    #[inline]
    fn from(t: {{ simd_t }}) -> Self {
        Self(t)
    }
}
{% endif %}

impl From<[{{ scalar_t }}; {{ dim }}]> for {{ self_t }} {
    #[inline]
    fn from(a: [{{ scalar_t }}; {{ dim }}]) -> Self {
        {% if self_t == "Vec4" and is_sse2 %}
            Self(unsafe { _mm_loadu_ps(a.as_ptr()) })
        {% else %}
            Self::new(
                {% for c in components %}
                    a[{{ loop.index0 }}],
                {%- endfor %}
            )
        {% endif %}
    }
}

impl From<{{ self_t }}> for [{{ scalar_t }}; {{ dim }}] {
    #[inline]
    fn from(v: {{ self_t }}) -> Self {
        {% if is_scalar %}
            [
                {% for c in components %}
                    v.{{ c }},
                {%- endfor %}
            ]
        {% elif is_sse2 %}
            use core::mem::MaybeUninit;
            use crate::Align16;
            let mut out: MaybeUninit<Align16<[f32; {{ dim }}]>> = MaybeUninit::uninit();
            unsafe {
                _mm_store_ps(out.as_mut_ptr().cast(), v.0);
                out.assume_init().0
            }
        {% elif is_wasm32 %}
            // TODO: can probably simplify this?
            use core::mem::MaybeUninit;
            let mut out: MaybeUninit<v128> = MaybeUninit::uninit();
            unsafe {
                v128_store(out.as_mut_ptr(), v.0);
                *(&out.assume_init() as *const v128 as *const [f32; {{ dim }}])
            }
        {% endif %}
    }
}

impl From<{{ macros::make_tuple_t(t=scalar_t, n=dim) }}> for {{ self_t }} {
    #[inline]
    fn from(t: {{ macros::make_tuple_t(t=scalar_t, n=dim) }}) -> Self {
        Self::new(
            {% for c in components %}
                t.{{ loop.index0 }},
            {%- endfor %}
        )
    }
}

impl From<{{ self_t }}> for {{ macros::make_tuple_t(t=scalar_t, n=dim) }} {
    #[inline]
    fn from(v: {{ self_t }}) -> Self {
        {% if is_scalar %}
            (
                {% for c in components %}
                    v.{{ c }},
                {%- endfor %}
            )
        {% elif is_sse2 %}
            use core::mem::MaybeUninit;
            use crate::Align16;
            let mut out: MaybeUninit<Align16<Self>> = MaybeUninit::uninit();
            unsafe {
                _mm_store_ps(out.as_mut_ptr().cast(), v.0);
                out.assume_init().0
            }
        {% elif is_wasm32 %}
            // TODO: can probably simplify this
            use core::mem::MaybeUninit;
            let mut out: MaybeUninit<v128> = MaybeUninit::uninit();
            unsafe {
                v128_store(out.as_mut_ptr(), v.0);
                *(&out.assume_init() as *const v128 as *const Self)
            }
        {% endif %}
    }
}

{% if self_t == "Vec3A" %}
impl From<Vec3> for Vec3A {
    #[inline]
    fn from(v: Vec3) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl From<Vec4> for Vec3A {
    /// Creates a `Vec3A` from the `x`, `y` and `z` elements of `self` discarding `w`.
    ///
    /// On architectures where SIMD is supported such as SSE2 on `x86_64` this conversion is a noop.
    #[inline]
    fn from(v: Vec4) -> Self {
        {% if is_scalar %}
            Self {
                x: v.x,
                y: v.y,
                z: v.z,
            }
        {% else %}
            Self(v.0)
        {% endif %}
    }
}
{% elif self_t == "Vec3" %}
impl From<Vec3A> for Vec3 {
    #[inline]
    fn from(v: Vec3A) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}
{% elif self_t == "Vec4" %}
impl From<(Vec3A, f32)> for Vec4 {
    #[inline]
    fn from((v, w): (Vec3A, f32)) -> Self {
        v.extend(w)
    }
}

impl From<(f32, Vec3A)> for Vec4 {
    #[inline]
    fn from((x, v): (f32, Vec3A)) -> Self {
        Self::new(x, v.x, v.y, v.z)
    }
}
{% endif %}

{% if dim == 3 %}
impl From<({{ vec2_t }}, {{ scalar_t }})> for {{ self_t }} {
    #[inline]
    fn from((v, z): ({{ vec2_t }}, {{ scalar_t }})) -> Self {
        Self::new(v.x, v.y, z)
    }
}
{% elif dim == 4 %}
impl From<({{ vec3_t }}, {{ scalar_t }})> for {{ self_t }} {
    #[inline]
    fn from((v, w): ({{ vec3_t }}, {{ scalar_t }})) -> Self {
        Self::new(v.x, v.y, v.z, w)
    }
}

impl From<({{ scalar_t }}, {{ vec3_t }})> for {{ self_t }} {
    #[inline]
    fn from((x, v): ({{ scalar_t }}, {{ vec3_t }})) -> Self {
        Self::new(x, v.x, v.y, v.z)
    }
}

impl From<({{ vec2_t }}, {{ scalar_t }}, {{ scalar_t }})> for {{ self_t }} {
    #[inline]
    fn from((v, z, w): ({{ vec2_t }}, {{ scalar_t }}, {{ scalar_t }})) -> Self {
        Self::new(v.x, v.y, z, w)
    }
}

impl From<({{ vec2_t }}, {{ vec2_t }})> for {{ self_t }} {
    #[inline]
    fn from((v, u): ({{ vec2_t }}, {{ vec2_t }})) -> Self {
        Self::new(v.x, v.y, u.x, u.y)
    }
}
{% endif %}

{% if not is_scalar %}
impl Deref for {{ self_t }} {
    type Target = crate::deref::{{ deref_t }};
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self).cast() }
    }
}

impl DerefMut for {{ self_t }} {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self).cast() }
    }
}
{% endif %}