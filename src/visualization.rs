use piston_window::*;
use crate::simulation::Simulation;
use std::time::{Instant, Duration};
use crate::constraint::NumericOperations;

pub struct Visualization<T: NumericOperations> {
    window_size: [u32; 2],
    scale_factor: T,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: NumericOperations + 'static> Visualization<T> {
    pub fn new(window_size: [u32; 2], field_width: f64, field_height: f64) -> Self {
        let scale_x = window_size[0] as f64 / field_width;
        let scale_y = window_size[1] as f64 / field_height;
        let scale_factor = T::from(scale_x.min(scale_y)).unwrap();

        Visualization {
            window_size,
            scale_factor,
           _phantom: std::marker::PhantomData,
        }
    }

    pub fn run(&mut self, mut simulation: Simulation<T>) {
        // Создаем окно
        let mut window: PistonWindow = WindowSettings::new(
            "Симуляция шаров",
            self.window_size
        )
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| panic!("Не удалось создать окно: {}", e));

        let mut events = Events::new(EventSettings::new().max_fps(60));

        let mut avr_time = Duration::new(0, 0);
        let mut count = 0;

        let mut precomputed_balls: Vec<(T, T, T, [f32; 4])> = simulation.field.balls.iter()
            .map(|ball| {
                let x = ball.position.x * self.scale_factor;
                let y = T::from(self.window_size[1]).unwrap()  - (ball.position.y * self.scale_factor);
                let radius = ball.radius * self.scale_factor;

                let mass_ratio = NumericOperations::to_f32(&(ball.mass / T::from(10.0).unwrap()).min(T::from(1.0).unwrap()));
                let color = [mass_ratio, 0.2, 1.0 - mass_ratio, 1.0];

                (x, y, radius, color)
            })
            .collect();


        while let Some(event) = events.next(&mut window) {
            if let Some(render_args) = event.render_args() {
                window.draw_2d(&event, |context, graphics, _device| {
                    clear([1.0, 1.0, 1.0, 1.0], graphics);

                    for (x, y, radius, color) in &precomputed_balls {
                        let circle = ellipse::circle(NumericOperations::to_f64(x), NumericOperations::to_f64(y), NumericOperations::to_f64(radius));
                        ellipse(*color, circle, context.transform, graphics);
                    }
                });
            }

            if let Some(_) = event.update_args() {
                count += 1;
                let start = Instant::now();
                simulation.step();
                let duration = start.elapsed();
                avr_time += duration;

                for (i, ball) in simulation.field.balls.iter().enumerate() {
                    let x = ball.position.x * self.scale_factor;
                    let y = T::from(self.window_size[1]).unwrap() - (ball.position.y * self.scale_factor);
                    let radius = ball.radius * self.scale_factor;

                    let mass_ratio = NumericOperations::to_f32(&(ball.mass / T::from(10.0).unwrap()).min(T::from(1.0).unwrap()));
                    let color = [mass_ratio, 0.2, 1.0 - mass_ratio, 1.0];

                    if let Some(precomputed) = precomputed_balls.get_mut(i) {
                        *precomputed = (x, y, radius, color);
                    }
                }

                if count % 100 == 0 {
                    println!(
                        "{:?} avr_time = {:?}\n",
                        count,
                        avr_time.as_micros() as f64 / count as f64,
                    );
                }
            }
        }
    }
}