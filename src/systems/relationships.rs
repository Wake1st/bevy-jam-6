use bevy::prelude::*;

/// connecting hubs to modules
#[derive(Component, Debug)]
#[relationship(relationship_target = HubHolder)]
pub struct ModuleOf(Entity);

#[derive(Component, Debug)]
#[relationship_target(relationship = ModuleOf)]
pub struct HubHolder(Entity);

/// connecting energy dispensors to hubs
#[derive(Component, Debug)]
#[relationship(relationship_target = EnergyReceiver)]
pub struct EnergyGiver(Entity);

#[derive(Component, Debug)]
#[relationship_target(relationship = EnergyGiver)]
pub struct EnergyReceiver(Vec<Entity>);
