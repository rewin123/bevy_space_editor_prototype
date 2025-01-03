//! This crate provides a entity inspector pane for Bevy Editor

use bevy::prelude::*;
use bevy_collapsing_header::CollapsingHeaderPlugin;
use bevy_field_forms::FieldFormsPlugin;
use render::ChangeComponentField;
use render_impl::RenderStorage;

pub mod render;
pub mod render_impl;

/// Plugin for the entity inspector.
pub struct EntityInspectorPlugin;

impl Plugin for EntityInspectorPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<FieldFormsPlugin>() {
            app.add_plugins(FieldFormsPlugin);
        }

        if !app.is_plugin_added::<CollapsingHeaderPlugin>() {
            app.add_plugins(CollapsingHeaderPlugin);
        }

        if !app.is_plugin_added::<bevy_editor_styles::StylesPlugin>() {
            app.add_plugins(bevy_editor_styles::StylesPlugin);
        }

        app.add_event::<ChangeComponentField>();

        app.init_resource::<RenderStorage>();
        app.add_plugins(render_impl::RenderImplPlugin);

        app.add_systems(PreUpdate, render::render_entity_inspector);
        app.add_systems(PreUpdate, render::render_component_inspector);

        app.add_observer(render::on_change_component_field);
    }
}

/// A marker for node in whicj entity inspector will render sub-tree

#[derive(Component)]
pub struct EntityInspector;

/// Component for marking an entity as being inspected.
#[derive(Component)]
pub struct InspectedEntity;
