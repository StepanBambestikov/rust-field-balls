use crate::vector2d::Vector2D;
use crate::constraint::NumericOperations;
use std::error::Error;
use std::fmt;
use rand::Rng;

#[derive(Debug)]
pub enum BallError {
    ZeroMass,
    ZeroRadius,
}

#[derive(Clone, Debug)]
pub(crate) struct Ball<T> {
    pub(crate) position: Vector2D<T>,
    pub(crate) mass: T,
    pub(crate) radius: T,
}

impl fmt::Display for BallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BallError::ZeroMass => write!(f, "mass cannot be zero"),
            BallError::ZeroRadius => write!(f, "radius cannot be zero"),
        }
    }
}
impl Error for BallError {}


pub(crate) fn new_random_ball<T>() -> Ball<T>
where T: NumericOperations{
    let mut rng = rand::thread_rng();
    let pos_x = T::from(rng.random_range(0.0..10.0)).unwrap();
    let pos_y = T::from(rng.random_range(0.0..10.0)).unwrap();
    let mass = T::from(rng.random_range(1.0..10.0)).unwrap();
    let radius = T::from(rng.random_range(0.1..1.0)).unwrap();

    Ball {
        position: Vector2D::new(pos_x, pos_y),
        mass,
        radius,
    }
}

impl<T> Ball<T>
where
    T: NumericOperations{
    pub(crate) fn new(
        position: Vector2D<T>,
        mass: T,
        radius: T
    ) -> Result<Ball<T>, BallError> {
        if mass <= T::zero() {
            return Err(BallError::ZeroMass);
        }
        if radius <= T::zero() {
            return Err(BallError::ZeroRadius);
        }

        Ok(Ball {
            position,
            mass,
            radius,
        })
    }

    pub(crate) fn add_force(&mut self, force: Vector2D<T>, dt: T){
        let a = force / self.mass;
        self.position += a * dt * dt;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ball_creation() {
        let position = Vector2D::new(1.0, 1.0);
        let velocity = Vector2D::new(0.0, 0.0);
        let mass = 1.0;
        let radius = 0.5;

        let result = Ball::new(position, mass, radius);
        assert!(result.is_ok());

        let ball = result.unwrap();
        assert_eq!(ball.mass, 1.0);
        assert_eq!(ball.radius, 0.5);
    }

    #[test]
    fn test_zero_mass_error() {
        let position = Vector2D::new(1.0, 1.0);
        let velocity = Vector2D::new(0.0, 0.0);
        let mass = 0.0;
        let radius = 0.5;

        let result = Ball::new(position, mass, radius);
        assert!(result.is_err());

        match result {
            Err(BallError::ZeroMass) => (),
            _ => panic!("Expected ZeroMass error"),
        }
    }

    #[test]
    fn test_zero_radius_error() {
        let position = Vector2D::new(1.0, 1.0);
        let velocity = Vector2D::new(0.0, 0.0);
        let mass = 1.0;
        let radius = 0.0;

        let result = Ball::new(position, mass, radius);
        assert!(result.is_err());

        match result {
            Err(BallError::ZeroRadius) => (),
            _ => panic!("Expected ZeroRadius error"),
        }
    }

    #[test]
    fn test_error_messages() {
        let zero_mass = BallError::ZeroMass.to_string();
        let zero_radius = BallError::ZeroRadius.to_string();

        assert_eq!(zero_mass, "mass cannot be zero");
        assert_eq!(zero_radius, "radius cannot be zero");
    }

    #[test]
    fn test_valid_ball_force_changing() {
        let position = Vector2D::new(1.0, 1.0);
        let velocity = Vector2D::new(0.0, 0.0);
        let mass = 1.0;
        let radius = 0.5;

        let result = Ball::new(position, mass, radius);
        assert!(result.is_ok());

        let mut ball = result.unwrap();
        let dt = 0.1;
        ball.add_force(Vector2D::new(1.0, 2.0), dt);
    }
}