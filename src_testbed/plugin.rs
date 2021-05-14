use crate::harness::RunState;
use crate::physics::PhysicsState;
use kiss3d::window::Window;
use na::Point3;
use rapier::gravity::Gravity;

pub trait TestbedPlugin {
    fn init_graphics(&mut self, window: &mut Window, gen_color: &mut dyn FnMut() -> Point3<f32>);
    fn clear_graphics(&mut self, window: &mut Window);
    fn run_callbacks(
        &mut self,
        window: &mut Window,
        physics: &mut PhysicsState<Box<dyn Gravity>>,
        run_state: &RunState,
    );
    fn step(&mut self, physics: &mut PhysicsState<Box<dyn Gravity>>);
    fn draw(&mut self);
    fn profiling_string(&self) -> String;
}
