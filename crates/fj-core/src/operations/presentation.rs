//! Operations to control the presentation of objects

use fj_interop::Color;

use crate::{
    objects::{IsObject, Region},
    storage::Handle,
};

/// Set the color of an object
pub trait SetColor: IsObject {
    /// Set the color of the object
    fn set_color(&self, color: impl Into<Color>) -> Self::BareObject;
}

impl SetColor for Handle<Region> {
    fn set_color(&self, color: impl Into<Color>) -> Self::BareObject {
        Region::new(
            self.exterior().clone(),
            self.interiors().into_iter().cloned(),
            Some(color.into()),
        )
    }
}
