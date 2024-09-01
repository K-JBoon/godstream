use crate::*;

#[derive(Component, Clone, Debug, serde::Deserialize, Reflect)]
pub enum Domain {
    Death,
    Fertility,
    Fire,
    Healing,
    Hunting,
    Justice,
    Light,
    Love,
    Magic,
    Protection,
    Strength,
    Thunder,
    Trickery,
    War,
    Water,
    Wealth,
    Wisdom,
}
