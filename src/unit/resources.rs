use crate::*;

/// Used as a marker for the unit to spawn on the field
#[derive(Resource, Debug)]
pub struct SelectedAllyUnit(AllyUnit);
