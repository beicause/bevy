use alloc::sync::Arc;
use bevy_ecs::{component::Component, entity::Entity};
use bevy_math::UVec2;
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::{NormalizedRenderTarget, RenderTarget};

#[derive(Component, Debug)]
pub struct RenderColorTarget {
    pub main_a: RenderTarget,
    pub main_b: RenderTarget,
    pub multisampled: Option<RenderTarget>,
    pub main_target: Option<Arc<AtomicUsize>>,
}

impl RenderColorTarget {
    pub fn new(
        main_a: RenderTarget,
        main_b: Option<RenderTarget>,
        multisampled: Option<RenderTarget>,
    ) -> Self {
        let main_target = main_b.as_ref().map(|_| Arc::new(AtomicUsize::new(0)));
        let main_b = main_b.unwrap_or(RenderTarget::None { size: UVec2::ZERO });
        Self {
            main_a,
            main_b,
            multisampled,
            main_target,
        }
    }

    pub fn current_target(&self) -> &RenderTarget {
        if let Some(main_target) = &self.main_target
            && main_target.load(Ordering::SeqCst) == 1
        {
            &self.main_b
        } else {
            &self.main_a
        }
    }

    pub fn other_target(&self) -> Option<&RenderTarget> {
        let Some(main_target) = &self.main_target else {
            return None;
        };
        Some(if main_target.load(Ordering::SeqCst) == 1 {
            &self.main_a
        } else {
            &self.main_b
        })
    }

    pub fn swap(&mut self) -> Option<usize> {
        let Some(main_target) = &mut self.main_target else {
            return None;
        };
        Some(main_target.fetch_xor(1, Ordering::SeqCst))
    }

    pub fn normalize(&self, primary_window: Option<Entity>) -> Option<NormalizedRenderColorTarget> {
        let Some(main_a) = self.main_a.normalize(primary_window) else {
            return None;
        };
        let Some(main_b) = self.main_b.normalize(primary_window) else {
            return None;
        };
        let multisampled = self
            .multisampled
            .as_ref()
            .and_then(|t| t.normalize(primary_window));
        Some(NormalizedRenderColorTarget {
            main_a,
            main_b,
            multisampled,
            main_target_flag: self.main_target.clone(),
        })
    }
}

#[derive(Debug)]
pub struct NormalizedRenderColorTarget {
    pub main_a: NormalizedRenderTarget,
    pub main_b: NormalizedRenderTarget,
    pub multisampled: Option<NormalizedRenderTarget>,
    pub main_target_flag: Option<Arc<AtomicUsize>>,
}

impl NormalizedRenderColorTarget {
    pub fn current_target(&self) -> &NormalizedRenderTarget {
        if let Some(main_target) = &self.main_target_flag
            && main_target.load(Ordering::SeqCst) == 1
        {
            &self.main_b
        } else {
            &self.main_a
        }
    }

    pub fn other_target(&self) -> Option<&NormalizedRenderTarget> {
        let Some(main_target) = &self.main_target_flag else {
            return None;
        };
        Some(if main_target.load(Ordering::SeqCst) == 1 {
            &self.main_a
        } else {
            &self.main_b
        })
    }
}

impl From<NormalizedRenderTarget> for NormalizedRenderColorTarget {
    fn from(value: NormalizedRenderTarget) -> Self {
        Self {
            main_a: value,
            main_b: NormalizedRenderTarget::None {
                width: 0,
                height: 0,
            },
            multisampled: None,
            main_target_flag: None,
        }
    }
}

#[derive(Component)]
pub struct NoAutoConfiguredColorTarget;

#[derive(Component)]
#[relationship(relationship_target  = MainColorTarget)]
pub struct MainColorTargetOf(pub Entity);

#[derive(Component)]
#[relationship(relationship_target  = OutputColorTarget)]
pub struct OutputColorTargetOf(pub Entity);

#[derive(Component)]
#[relationship_target(relationship  = MainColorTargetOf, linked_spawn)]
pub struct MainColorTarget(Vec<Entity>);

#[derive(Component)]
#[relationship_target(relationship  = OutputColorTargetOf, linked_spawn)]
pub struct OutputColorTarget(Vec<Entity>);
