use crate::ball::Ball;
use crate::field::Field;
use crate::constraint::NumericOperations;
use crate::interaction_force::InteractionForce;

pub(crate) fn interaction_step<T: NumericOperations>(
    balls: &mut Vec<Ball<T>>,
    dt: T,
    forces: &Vec<Box<dyn InteractionForce<T>>>
){
    for i in 0..balls.len()-1{
        for j in i+1..balls.len(){
            for current_force in forces.iter() {
                let force_i_to_j = current_force.calculate_force(
                    &balls[i],
                    &balls[j]
                );

                balls[i].add_force(
                    force_i_to_j,
                    dt
                );
                balls[j].add_force(
                    -force_i_to_j,
                    dt
                );
            }
        }
    }
}

#[cfg(deprecated)]
pub(crate) fn positional_step<T: NumericOperations>(
    field: &Field<T>,
    positional_forces: &Vec<Box<dyn PositionForce<T>>>
){
    for (index, current_ball) in field.balls.iter().enumerate() {
        // for current_force in field.forces.iter() {
        //     let added = current_force.calculate_force(current_ball.clone(), &field);
        //     // force_intentions[index] += added; TODO
        // }
    }
}