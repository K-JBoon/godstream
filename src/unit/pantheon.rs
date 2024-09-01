use crate::*;

#[derive(Component, Clone, Debug, serde::Deserialize, Reflect)]
pub enum Pantheon {
    Greek,
    Norse,
    Egyptian,
}
