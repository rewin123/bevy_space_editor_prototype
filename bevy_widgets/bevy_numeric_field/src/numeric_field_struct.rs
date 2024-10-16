use bevy::prelude::*;
use num_traits::{Bounded, CheckedAdd, CheckedSub, NumCast};
use std::cmp::PartialOrd;
use std::ops::{Add, Sub};
use std::str::FromStr;

/// Represents a numeric field with optional constraints
#[derive(Component)]
pub struct NumericField<T: NumericFieldValue> {
    /// Current value
    pub value: T,
    /// Minimum allowed value
    pub min: Option<T>,
    /// Maximum allowed value
    pub max: Option<T>,
    /// Value change per logical pixel during mouse drag
    pub drag_step: Option<f64>,
}

/// Internal state for numeric field
#[derive(Component)]
pub(crate) struct InnerNumericField<T: NumericFieldValue> {
    /// Last valid value
    pub last_val: T,
    /// Flag for failed string to number conversion
    pub failed_convert: bool,
    /// Flag to ignore text input changes
    pub ignore_text_changes: bool,
    /// Accumulated drag delta
    pub accumulated_delta: f64,
}

/// Trait defining requirements for numeric field values
pub trait NumericFieldValue:
    Copy
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + NumCast
    + PartialEq
    + Send
    + Sync
    + 'static
    + ToString
    + FromStr
    + Bounded
{
    /// Default change per logical pixel during dragging
    fn default_drag_step() -> f64;
    /// Chars allowed in text field for this type
    fn allowed_chars() -> Vec<char>;

    /// Checked addition
    fn checked_add(&self, rhs: &Self) -> Option<Self>;
    /// Checked subtraction
    fn checked_sub(&self, rhs: &Self) -> Option<Self>;
}

impl<T> NumericField<T>
where
    T: NumericFieldValue,
{
    /// Create new numeric field with default min/max values
    pub fn new(value: T) -> Self {
        NumericField {
            value,
            min: None,
            max: None,
            drag_step: Some(T::default_drag_step()),
        }
    }

    /// Set a new value, ensuring it's within the allowed range
    pub(crate) fn set_value(&mut self, new_value: T) {
        let new_value = if let Some(min) = self.min {
            if new_value < min {
                min
            } else {
                new_value
            }
        } else {
            new_value
        };

        let new_value = if let Some(max) = self.max {
            if new_value > max {
                max
            } else {
                new_value
            }
        } else {
            new_value
        };

        self.value = new_value;
    }
}

// Macro to implement NumericFieldValue for signed integer types
macro_rules! impl_signed_numeric_field_value {
    ($($t:ty),*) => {
        $(
            impl NumericFieldValue for $t {
                fn default_drag_step() -> f64 { 0.1 }
                fn allowed_chars() -> Vec<char> {
                    vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-']
                }

                fn checked_add(&self, rhs: &Self) -> Option<Self> {
                    <Self as CheckedAdd>::checked_add(self, rhs)
                }
                fn checked_sub(&self, rhs: &Self) -> Option<Self> {
                    <Self as CheckedSub>::checked_sub(self, rhs)
                }
            }
        )*
    }
}

// Macro to implement NumericFieldValue for unsigned integer types
macro_rules! impl_unsigned_numeric_field_value {
    ($($t:ty),*) => {
        $(
            impl NumericFieldValue for $t {
                fn default_drag_step() -> f64 { 0.1 }
                fn allowed_chars() -> Vec<char> {
                    vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
                }

                fn checked_add(&self, rhs: &Self) -> Option<Self> {
                    <Self as CheckedAdd>::checked_add(self, rhs)
                }
                fn checked_sub(&self, rhs: &Self) -> Option<Self> {
                    <Self as CheckedSub>::checked_sub(self, rhs)
                }
            }
        )*
    }
}

// Macro to implement NumericFieldValue for floating-point types
macro_rules! impl_float_numeric_field_value {
    ($($t:ty),*) => {
        $(
            impl NumericFieldValue for $t {
                fn default_drag_step() -> f64 { 0.1 }
                fn allowed_chars() -> Vec<char> {
                    vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.', '-']
                }

                fn checked_add(&self, rhs: &Self) -> Option<Self> {
                    Some(*self + *rhs)
                }

                fn checked_sub(&self, rhs: &Self) -> Option<Self> {
                    Some(*self - *rhs)
                }
            }
        )*
    }
}

// Implement NumericFieldValue for signed and unsigned integer types
impl_signed_numeric_field_value!(i8, i16, i32, i64, i128);
impl_unsigned_numeric_field_value!(u8, u16, u32, u64, u128);

// Implement NumericFieldValue for floating-point types
impl_float_numeric_field_value!(f32, f64);
