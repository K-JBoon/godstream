use crate::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    LoadingScreen,
    MainMenu,
    InGame,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    OnStage,
}
