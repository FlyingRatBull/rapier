//! Structure for combining the various physics components to perform an actual simulation.

pub use collision_pipeline::CollisionPipeline;
pub use event_handler::{ChannelEventCollector, EventHandler};
pub use physics_hooks::{
    ContactModificationContext, PairFilterContext, PhysicsHooks, PhysicsHooksFlags,
};
pub use physics_pipeline::PhysicsPipeline;
pub use query_pipeline::{QueryPipeline, QueryPipelineMode};

mod collision_pipeline;
mod event_handler;
mod physics_hooks;
mod physics_pipeline;
mod query_pipeline;
mod user_changes;
