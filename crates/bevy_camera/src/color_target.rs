use alloc::sync::Arc;
use bevy_ecs::{component::Component, entity::Entity};
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::{NormalizedRenderTarget, RenderTarget};

#[derive(Component, Debug)]
pub struct RenderTargetDoubleBuffered {
    pub a: RenderTarget,
    pub b: RenderTarget,
    pub main_texture: Option<Arc<AtomicUsize>>,
}

impl RenderTargetDoubleBuffered {
    pub fn new(a: RenderTarget, b: Option<RenderTarget>) -> Self {
        let main_texture = b.as_ref().map(|_| Arc::new(AtomicUsize::new(0)));
        let b = b.unwrap_or(RenderTarget::None {
            size: Default::default(),
        });
        Self { a, b, main_texture }
    }

    pub fn current_target(&self) -> &RenderTarget {
        if let Some(main_texture) = &self.main_texture
            && main_texture.load(Ordering::SeqCst) == 1
        {
            &self.b
        } else {
            &self.a
        }
    }

    pub fn other_target(&self) -> Option<&RenderTarget> {
        let Some(main_texture) = &self.main_texture else {
            return None;
        };
        Some(if main_texture.load(Ordering::SeqCst) == 1 {
            &self.a
        } else {
            &self.b
        })
    }

    pub fn swap(&mut self) -> Option<usize> {
        let Some(main_texture) = &mut self.main_texture else {
            return None;
        };
        Some(main_texture.fetch_xor(1, Ordering::SeqCst))
    }

    pub fn normalize(
        &self,
        primary_window: Option<Entity>,
    ) -> Option<NormalizedRenderTargetDoubleBuffered> {
        let Some(a) = self.a.normalize(primary_window) else {
            return None;
        };
        let Some(b) = self.b.normalize(primary_window) else {
            return None;
        };
        Some(NormalizedRenderTargetDoubleBuffered {
            a,
            b,
            main_texture: self.main_texture.clone(),
        })
    }
}

#[derive(Debug)]
pub struct NormalizedRenderTargetDoubleBuffered {
    pub a: NormalizedRenderTarget,
    pub b: NormalizedRenderTarget,
    pub main_texture: Option<Arc<AtomicUsize>>,
}

impl NormalizedRenderTargetDoubleBuffered {
    pub fn current_target(&self) -> &NormalizedRenderTarget {
        if let Some(main_texture) = &self.main_texture
            && main_texture.load(Ordering::SeqCst) == 1
        {
            &self.b
        } else {
            &self.a
        }
    }

    pub fn other_target(&self) -> Option<&NormalizedRenderTarget> {
        let Some(main_texture) = &self.main_texture else {
            return None;
        };
        Some(if main_texture.load(Ordering::SeqCst) == 1 {
            &self.a
        } else {
            &self.b
        })
    }
}

impl From<NormalizedRenderTarget> for NormalizedRenderTargetDoubleBuffered {
    fn from(value: NormalizedRenderTarget) -> Self {
        Self {
            a: value,
            b: NormalizedRenderTarget::None {
                width: 0,
                height: 0,
            },
            main_texture: None,
        }
    }
}

#[derive(Component)]
pub struct NoAutoConfiguredColorTarget;

#[derive(Component)]
#[relationship(relationship_target  = ColorTarget)]
pub struct ColorTargetOf(pub Entity);

#[derive(Component)]
#[relationship(relationship_target  = MsaaColorTarget)]
pub struct MsaaColorTargetOf(pub Entity);

#[derive(Component)]
#[relationship(relationship_target  = MsaaResolveTarget)]
pub struct MsaaResolveTargetOf(pub Entity);

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
