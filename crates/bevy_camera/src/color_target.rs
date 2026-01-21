use alloc::sync::Arc;
use bevy_asset::Handle;
use bevy_ecs::{component::Component, entity::Entity};
use bevy_image::Image;
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Component, Debug, Clone)]
pub struct MainColorTarget {
    pub main_a: Handle<Image>,
    pub main_b: Option<Handle<Image>>,
    pub multisampled: Option<Handle<Image>>,
    pub main_target_flag: Option<Arc<AtomicUsize>>,
}

impl MainColorTarget {
    pub fn new(
        main_a: Handle<Image>,
        main_b: Option<Handle<Image>>,
        multisampled: Option<Handle<Image>>,
    ) -> Self {
        let main_target = main_b.as_ref().map(|_| Arc::new(AtomicUsize::new(0)));
        Self {
            main_a,
            main_b,
            multisampled,
            main_target_flag: main_target,
        }
    }

    pub fn current_target(&self) -> &Handle<Image> {
        if let Some(main_target) = &self.main_target_flag
            && main_target.load(Ordering::SeqCst) == 1
        {
            self.main_b.as_ref().unwrap()
        } else {
            &self.main_a
        }
    }

    pub fn other_target(&self) -> Option<&Handle<Image>> {
        let Some(main_target) = &self.main_target_flag else {
            return None;
        };
        Some(if main_target.load(Ordering::SeqCst) == 1 {
            &self.main_a
        } else {
            self.main_b.as_ref().unwrap()
        })
    }
}

#[derive(Component)]
pub struct NoAutoConfiguredColorTarget;

#[derive(Component)]
#[relationship(relationship_target  = MainColorTargetCameras)]
pub struct UseMainColorTarget(pub Entity);

#[derive(Component)]
#[relationship_target(relationship  = UseMainColorTarget, linked_spawn)]
pub struct MainColorTargetCameras(Vec<Entity>);
