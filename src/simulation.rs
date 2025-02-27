use rand::Rng;
use crate::field::Field;
use crate::field::new_random;
use crate::vector2d::Vector2D;
use crate::constraint::NumericOperations;
use crate::interaction_force::{InteractionForce, NegativeInteraction, PositiveInteraction};
use crate::force_calculation;

pub struct Simulation<T: NumericOperations>{
    pub field: Field<T>,
    interaction_forces: Vec<Box<dyn InteractionForce<T>>>,
    //positional_forces: Vec<Box<dyn PositionForce<T>>>,
}

impl <T: NumericOperations + 'static> Simulation<T>{

    fn new(
        field: Field<T>,
        interaction_forces: Vec<Box<dyn InteractionForce<T>>>,
        //positional_forces: Vec<Box<dyn PositionForce<T>>>
    ) -> Simulation<T>{
        Simulation{
            field: field.clone(),
            interaction_forces,
            //positional_forces,
        }
    }

    pub fn new_random(width: T, height: T) -> Simulation<T>{
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(6..20);
        Simulation{
            field: new_random(width, height, T::from(0.001).unwrap(), random_number),
            interaction_forces: vec![
                Box::new(NegativeInteraction::<T>{ r: T::from(50000.).unwrap() }),
                Box::new(PositiveInteraction::<T>{ g: T::from(-5000000.).unwrap() }),
            ],
            //positional_forces: vec![],
        }
    }

    pub fn step(&mut self) {
        //interaction_step

        force_calculation::interaction_step(&mut self.field.balls, self.field.dt, &self.interaction_forces);
        //ForceCalculation::positional_step(&mut self.field, &self.positional_forces);

        //update
        // for index in 0..self.field.balls.len() {
        //     // self.field.balls[index].add_force(
        //     //     self.force_intentions[index],
        //     //     self.field.dt
        //     // );
        //
        //     //check border value
        //     // if self.field.balls[index].position.x > self.field.width{ //TODO return
        //     //     self.field.balls[index].velocity.x *= T::from(-1.0).unwrap();
        //     // }
        //     // if self.field.balls[index].position.y > self.field.height {
        //     //     self.field.balls[index].velocity.y *= T::from(-1.0).unwrap();
        //     // }
        // }
    }

}

#[cfg(test)]
mod simulator_tests {
    use super::*;

    #[test]
    fn step_test() {
        let mut simulation = Simulation::<f64>::new_random(1., 1.);
        let mut count = 0;
        loop{
            println!("count: {:?}----------------------------------//\n", count);
            simulation.step();
            for (index, current_ball) in simulation.field.balls.iter().enumerate(){
                println!(
                    "{:?} (x: {:?}, y: {:?})\n",
                    index,
                    current_ball.position.x,
                    current_ball.position.y,
                );
            }
            count += 1;
        }
    }

}