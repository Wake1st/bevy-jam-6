use bevy::prelude::*;

pub struct RelationshipPlugin;

impl Plugin for RelationshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ModuleAttached>()
            .add_event::<ModuleRemoved>();
        app.add_systems(Update, (attach_module, remove_module));
    }
}

/// connecting hubs to modules
#[derive(Component, Debug)]
#[relationship(relationship_target = ModuleOf)]
pub struct HubHolder(pub Entity);

#[derive(Component, Debug)]
#[relationship_target(relationship = HubHolder)]
pub struct ModuleOf(Entity);

#[derive(Event, Debug)]
pub struct ModuleAttached {
    pub hub: Entity,
    pub module: Entity,
}

fn attach_module(mut attached: EventReader<ModuleAttached>, mut commands: Commands) {
    for e in attached.read() {
        commands.entity(e.hub).insert(HubHolder(e.module));
    }
}

#[derive(Event, Debug)]
pub struct ModuleRemoved {
    pub module: Entity,
}

fn remove_module(mut removed: EventReader<ModuleRemoved>, mut commands: Commands) {
    for e in removed.read() {
        // commands.entity(e.module).try_remove::<ModuleOf>();
        commands.entity(e.module).remove::<ModuleOf>();
    }
}

/// connecting energy dispensors to hubs
#[derive(Component, Debug)]
#[relationship(relationship_target = EnergyReceiver)]
pub struct EnergyGiver(Entity);

#[derive(Component, Debug)]
#[relationship_target(relationship = EnergyGiver)]
pub struct EnergyReceiver(Entity);
