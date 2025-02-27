use rand::distr::{Uniform};
use crate::ball::Ball;
use crate::vector2d::Vector2D;
use crate::constraint::NumericOperations;
use rand::distr::Distribution;

#[derive(Clone, Debug)]
pub(crate) struct Field<T: NumericOperations> {
    pub(crate) width: T,
    pub(crate) height: T,
    pub(crate) dt: T,
    pub(crate) balls: Vec<Ball<T>>
}
pub(crate) fn new<T: NumericOperations>(width: T, height: T, dt: T, balls: Vec<Ball<T>>) -> Field<T> {
    Field {
        width,
        height,
        dt,
        balls,
    }
}

pub fn new_random<T>(width: T, height: T, dt: T, num_balls: usize) -> Field<T>
where T: NumericOperations
{
    let mut rng = rand::thread_rng();
    let x_range = Uniform::new(width * T::from(0.3).unwrap(), width * T::from(0.7).unwrap()).unwrap();
    let y_range = Uniform::new(height * T::from(0.3).unwrap(), height * T::from(0.7).unwrap()).unwrap();
    // let mass_range = Uniform::new(T::from(100.0).unwrap(), T::from(2000.0).unwrap()).unwrap();
    // let radius_range = Uniform::new(T::from(5.0).unwrap(), T::from(10.0).unwrap()).unwrap();

    let mut balls = Vec::with_capacity(num_balls);
    balls.reserve(num_balls);
    for _ in 0..num_balls {
        let position = Vector2D::new(
            x_range.sample(&mut rng),
            y_range.sample(&mut rng)
        );

        //let mass = mass_range.sample(&mut rng);
        let mass = T::from(10000.0).unwrap();

        // let radius = radius_range.sample(&mut rng);
        let radius = T::from(10.0).unwrap();

        let ball = Ball::new(position, mass, radius).unwrap();
        balls.push(ball);
    }

    Field {
        width,
        height,
        dt,
        balls,
    }
}

