use crate::ball::Ball;
use crate::vector2d::{Vector2D};
use crate::constraint::{NumericOperations};
use crate::ball::new_random_ball;

pub(crate) trait InteractionForce<T>{
    fn calculate_force(&self, first: &Ball<T>, second: &Ball<T>) -> Vector2D<T>;
}

pub(crate) struct NegativeInteraction<T>{
    pub(crate) r: T
}

impl<T: NumericOperations> InteractionForce<T> for NegativeInteraction<T> {
    fn calculate_force(&self, first: &Ball<T>, second: &Ball<T>) -> Vector2D<T>{
        let diff = second.position - first.position;
        let mut dist = diff.length() * diff.length();
        if dist < T::from(0.0001).unwrap(){
            dist = T::from(0.0001).unwrap();
        }
        diff * (self.r * first.mass * second.mass) / dist
    }
}

pub(crate) struct PositiveInteraction<T: NumericOperations>{
    pub(crate) g: T
}

impl<T: NumericOperations> InteractionForce<T> for PositiveInteraction<T> {
    fn calculate_force(&self, first: &Ball<T>, second: &Ball<T>) -> Vector2D<T>{
        let diff = second.position - first.position;
        let mut dist = diff.length() * diff.length() * diff.length();
        if dist < T::from(0.0001).unwrap(){
            dist = T::from(0.0001).unwrap();
        }
        diff * (self.g * first.mass * second.mass) / dist
    }
}

#[cfg(test)]
mod interaction_force_test {
    use super::*;

    #[test]
    fn test_neg_interaction_force() {
        let first = new_random_ball::<f64>();
        let second = new_random_ball::<f64>();
        let force = NegativeInteraction::<f64>{
            r:-4.3
        };
        let val = force.calculate_force(&first, &second);
    }

    #[test]
    fn test_pos_interaction_force() {
        let first = new_random_ball::<f64>();
        let second = new_random_ball::<f64>();
        let force = PositiveInteraction::<f64>{
            g:-4.3
        };
        let val = force.calculate_force(&first, &second);
    }

}