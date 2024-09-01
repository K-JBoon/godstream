use crate::*;

#[derive(Component, Clone, Debug, serde::Deserialize)]
pub enum Pantheon {
    Greek,
    Norse,
    Egyptian,
}
