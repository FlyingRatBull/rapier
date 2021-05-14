use crate::harness::RunState;
use crate::physics::PhysicsEvents;
use crate::PhysicsState;

use rapier::gravity::Gravity;

pub trait HarnessPlugin {
    fn run_callbacks(
        &mut self,
        physics: &mut PhysicsState<Box<dyn Gravity>>,
        events: &PhysicsEvents,
        harness_state: &RunState,
    );
    fn step(&mut self, physics: &mut PhysicsState<Box<dyn Gravity>>, run_state: &RunState);
    fn profiling_string(&self) -> String;
}
