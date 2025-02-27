use std::ops::{AddAssign, SubAssign, Mul, MulAssign};
use num_traits::{Float, Pow};
use std::ops::Div;

pub(crate) trait NumericOperations:
    Copy +
    Clone +
    AddAssign +
    MulAssign +
    SubAssign +
    Float +
    Mul<Output = Self> +
    Div<Output = Self> +
    Pow<Self, Output = Self> +
    rand::distr::uniform::SampleUniform +
    std::fmt::Debug
{
    fn to_f32(&self) -> f32;
    fn to_f64(&self) -> f64;
}

impl NumericOperations for f64 {
    fn to_f32(&self) -> f32 {
        *self as f32
    }

    fn to_f64(&self) -> f64 {
        *self
    }
}

impl NumericOperations for f32 {
    fn to_f32(&self) -> f32 {
        *self
    }

    fn to_f64(&self) -> f64 {
        *self as f64
    }
}
