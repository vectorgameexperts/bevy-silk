use bevy::{
    ecs::schedule::{ScheduleLabel, SystemSetConfigs},
    prelude::*,
};
use enum_display::EnumDisplay;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SilkSchedule;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, SystemSet, EnumDisplay)]
pub enum SilkSet {
    /// An exclusive system to analyze network traffic before use.
    Process,
    /// Apply updates before the main update.
    PreUpdate,
    /// Default stage for game updates.
    Update,
    /// Apply updates after the main update.
    PostUpdate,
}

impl SilkSet {
    pub fn sets() -> SystemSetConfigs {
        // Define the ordering of systems here
        (
            Self::Process,
            Self::PreUpdate,
            Self::Update,
            Self::PostUpdate,
        )
            .chain()
    }
}
