use bevy_ecs::{component::Component, entity::Entity};

#[derive(Component)]
pub struct RenderTargetDoubleBuffered {
    pub a: Entity,
    pub b: Entity,
}

#[derive(Component)]
pub struct NoAutoConfiguredColorTarget;

/// `RenderTarget` or `RenderTargetDoubleBuffered`.
#[derive(Component)]
#[relationship(relationship_target  = ColorTarget)]
pub struct ColorTargetOf(pub Entity);

/// `RenderTarget` or `RenderTargetDoubleBuffered`.
#[derive(Component)]
#[relationship(relationship_target  = MsaaColorTarget)]
pub struct MsaaColorTargetOf(pub Entity);

/// `RenderTarget` or `RenderTargetDoubleBuffered`.
#[derive(Component)]
#[relationship(relationship_target  = MsaaResolveTarget)]
pub struct MsaaResolveTargetOf(pub Entity);

/// `RenderTarget` or `RenderTargetDoubleBuffered`.
#[derive(Component)]
#[relationship(relationship_target  = OutputColorTarget)]
pub struct OutputColorTargetOf(pub Entity);

#[derive(Component)]
#[relationship_target(relationship  = ColorTargetOf, linked_spawn)]
pub struct ColorTarget(Vec<Entity>);

#[derive(Component)]
#[relationship_target(relationship  = MsaaColorTargetOf, linked_spawn)]
pub struct MsaaColorTarget(Vec<Entity>);

#[derive(Component)]
#[relationship_target(relationship  = MsaaResolveTargetOf, linked_spawn)]
pub struct MsaaResolveTarget(Vec<Entity>);

#[derive(Component)]
#[relationship_target(relationship  = OutputColorTargetOf, linked_spawn)]
pub struct OutputColorTarget(Vec<Entity>);
