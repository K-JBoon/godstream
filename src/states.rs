use crate::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    AssetLoading,
    PrepareStage,
    OnStage,
}
