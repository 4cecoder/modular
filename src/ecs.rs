//! Entity Component System (ECS) module
//!
//! This module provides the core ECS functionality using the Specs crate.

use specs::{Builder, Component, Entity, World, WorldExt};

/// Core time resource
#[derive(Debug, Clone, Default)]
pub struct Time {
    pub delta: f32,
    pub elapsed: f32,
}

/// Input state resource
#[derive(Debug, Clone, Default)]
pub struct InputState {
    pub mouse_position: (f32, f32),
    pub mouse_delta: (f32, f32),
    pub keys_pressed: std::collections::HashSet<winit::event::VirtualKeyCode>,
}

/// ECS World extension methods
pub trait GameWorldExt {
    fn create_entity_with_components(&mut self) -> EntityBuilder;
}

impl GameWorldExt for World {
    fn create_entity_with_components(&mut self) -> EntityBuilder {
        let entity = self.create_entity().build();
        EntityBuilder {
            world: self,
            entity,
        }
    }
}

/// Entity builder for fluent component addition
pub struct EntityBuilder<'a> {
    world: &'a mut World,
    entity: Entity,
}

impl<'a> EntityBuilder<'a> {
    pub fn with<C: Component + Send + Sync>(self, component: C) -> Self {
        {
            let mut storage = self.world.write_storage::<C>();
            storage.insert(self.entity, component).unwrap();
        }
        self
    }

    pub fn build(self) -> Entity {
        self.entity
    }
}

/// System trait extension for easier system creation
pub trait SystemExt<'a> {
    fn name(&self) -> &'static str;
}

impl<'a, T> SystemExt<'a> for T
where
    T: specs::System<'a>,
{
    fn name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}
