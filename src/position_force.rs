use crate::field::Field;
use crate::ball::Ball;
use crate::vector2d::Vector2D;
use crate::constraint::NumericOperations;

#[cfg(deprecated)]
pub(crate) trait PositionForce<T: NumericOperations>{
    fn calculate_force(&self, first: Ball<T>, field: &Field<T>) -> Vector2D<T>;
}

//TODO implement!!!
#[cfg(deprecated)]
pub(crate) struct CircleForce<T>{
    intention_radius: T
}
#[cfg(deprecated)]
pub(crate) fn new<T: NumericOperations>(field: &Field<T>) -> CircleForce<T> {
    CircleForce {
        intention_radius: field.width.clone(),
    }
}

#[cfg(deprecated)]
impl<T: NumericOperations> PositionForce<T> for CircleForce<T>{
    fn calculate_force(&self, first: Ball<T>, field: &Field<T>) -> Vector2D<T>{
        //TODO
        Vector2D::new(T::zero(), T::zero())
    }
}

#[cfg(deprecated)]
#[cfg(test)]
mod position_force_tests {
    #[test]
    fn test_neg_interaction_force() {
        // let first = new_random_ball::<f64>();
        // let field = new_random_field::<f64>();
        // let force = crate::PositionForce::CircleForce::<f64> {R:-4.3};
        // let val = force.calculate_force(first, field);
    }

}