use std::str::FromStr;

use bevy::{prelude::*, utils::HashSet};
use bevy_text_editing::*;
use child_traversal::FirstChildTraversal;

/// Plugin for validated input fields with a generic value type
pub struct ValidatedInputFieldPlugin<T: Validable> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: Validable> Default for ValidatedInputFieldPlugin<T> {
    fn default() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Validable> Plugin for ValidatedInputFieldPlugin<T> {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EditableTextLinePlugin>() {
            app.add_plugins(EditableTextLinePlugin);
        }

        app.add_event::<ValidationChanged>();
        app.add_event::<ValidatedValueChanged<T>>();
        app.add_event::<SetValidatedValue<T>>();

        app.add_systems(PreUpdate, spawn_system::<T>);

        app.add_observer(on_text_changed::<T>);
    }
}

/// A text field with input validation
/// It will not contain special style updates for validation state, because it's expected that it will be
/// combined with other widgets to form a custom UI.
#[derive(Component)]
#[require(FirstChildTraversal)]
pub struct ValidatedInputField<T: Validable> {
    /// The last valid value
    pub value: T,
    /// The current validation state
    pub validation_state: ValidationState,
    /// If true, this text field will not update its value automatically
    /// and will require an external update call to update the value.
    pub controlled: bool,
}

impl<T: Validable> ValidatedInputField<T> {
    /// Create a new validated input field with the given value
    pub fn new(value: T) -> Self {
        Self {
            value,
            validation_state: ValidationState::Unchecked,
            controlled: false,
        }
    }
}

/// A trait for types that can be validated from a string input.
///
/// Types implementing this trait can be used with `ValidatedInputField`.
pub trait Validable: Send + Sync + Default + PartialEq + Clone + ToString + 'static {
    /// Attempts to validate and convert a string into this type.
    ///
    /// # Arguments
    ///
    /// * `text` - The input string to validate and convert.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` if the input is valid and can be converted to this type.
    /// * `Err(String)` with an error message if the input is invalid.
    fn validate(text: &str) -> Result<Self, String>;
}

/// The current state of the text field validation
#[derive(Default, Clone, Debug)]
pub enum ValidationState {
    /// No validation has been performed yet
    #[default]
    Unchecked,
    /// The field content is valid
    Valid,
    /// The field content is invalid
    Invalid(String),
}

/// Event that is emitted when the validation state changes
#[derive(Event)]
pub struct ValidationChanged(pub ValidationState);

/// Event that is emitted when the value changes
#[derive(Event)]
pub struct ValidatedValueChanged<T: Validable>(pub T);

/// This event is used to set the value of the validated input field.
#[derive(Event)]
pub struct SetValidatedValue<T: Validable>(pub T);

fn spawn_system<T: Validable>(
    mut commands: Commands,
    query: Query<(Entity, &ValidatedInputField<T>), Without<EditableTextLine>>,
) {
}

fn on_text_changed<T: Validable>(
    mut trigger: Trigger<TextChanged>,
    mut commands: Commands,
    mut q_validated_input_fields: Query<&mut ValidatedInputField<T>>,
) {
    let entity = trigger.entity();
    let Ok(mut field) = q_validated_input_fields.get_mut(entity) else {
        return;
    };

    let new_text = trigger.new_text.clone();
    trigger.propagate(false);

    match T::validate(&new_text) {
        Ok(value) => {
            commands.trigger_targets(ValidatedValueChanged(value.clone()), entity);
            commands.trigger_targets(ValidationChanged(ValidationState::Valid), entity);
            // Update the value only if the field is not controlled
            if !field.controlled {
                // Update the text in the EditableTextLine
                commands.trigger_targets(SetText(value.to_string()), entity);
                field.value = value;
            }
        }
        Err(error) => {
            commands.trigger_targets(ValidationChanged(ValidationState::Invalid(error)), entity);
        }
    }
}
