use std::ops::{AddAssign, SubAssign, Mul, MulAssign, Neg, Sub};
use num_traits::{Float, Pow};
use std::ops::Div;
use constraint::NumericOperations;
use crate::constraint;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Vector2D<T> {
    pub(crate) x: T,
    pub(crate) y: T
}

impl<T> Div<T> for Vector2D<T> where T: NumericOperations,
{
    type Output = Vector2D<T>;
    fn div(self, rhs: T) -> Self::Output {
        Vector2D {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T> Sub for Vector2D<T> where T: NumericOperations,
{
    type Output = Vector2D<T>;
    fn sub(self, rhs: Vector2D<T>) -> Self::Output {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Neg for Vector2D<T> where T: NumericOperations,
{
    type Output = Vector2D<T>;
    fn neg(self) -> Self::Output {
        Vector2D {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T> Mul<T> for Vector2D<T> where T: NumericOperations,
{
    type Output = Vector2D<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Vector2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> MulAssign for Vector2D<T> where T: NumericOperations,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T> MulAssign<T> for Vector2D<T> where T: NumericOperations,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T> AddAssign for Vector2D<T> where T: NumericOperations,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl<T> SubAssign for Vector2D<T> where T: NumericOperations,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
impl<T> Vector2D<T> where T: NumericOperations {
    pub(crate) fn new(x: T, y: T) -> Vector2D<T> {
        Vector2D { x, y }
    }

    pub(crate) fn length(&self) -> T{
        self.square().sqrt()
    }

    pub(crate) fn square(&self) -> T{
        self.x * self.x + self.y * self.y
    }
}

#[cfg(test)]
mod vector2d_tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut a = Vector2D::new(1.0, 2.0);
        a += Vector2D::new(3.0, 4.0);
        assert_eq!(a, Vector2D::new(4.0, 6.0));
    }

    #[test]
    fn test_mul() {
        let mut a = Vector2D::new(1.0, 2.0);
        a = a * 2.0;
        assert_eq!(a, Vector2D::new(2.0, 4.0));
    }

    #[test]
    fn test_length() {
        let a = Vector2D::new(3.0, 4.0);
        assert_eq!(a.length(), 5.0);
    }

    #[test]
    fn test_square() {
        let a = Vector2D::new(3.0, 4.0);
        assert_eq!(a.square(), 25.0);
    }

}