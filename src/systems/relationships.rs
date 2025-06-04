use bevy::prelude::*;

pub struct RelationshipPlugin;

impl Plugin for RelationshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ModuleAttached>()
            .add_event::<ModuleRemoved>()
            .add_event::<EnergyConnected>()
            .add_event::<EnergyDisconnected>();
        app.add_systems(
            Update,
            (
                attach_module,
                remove_module,
                connect_energy,
                disconnect_energy,
            ),
        );
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

#[derive(Event, Debug)]
pub struct ModuleRemoved {
    pub module: Entity,
}

fn attach_module(mut attached: EventReader<ModuleAttached>, mut commands: Commands) {
    for e in attached.read() {
        commands.entity(e.hub).insert(HubHolder(e.module));
    }
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
pub struct EnergyGiver(pub Vec<Entity>);

#[derive(Component, Debug)]
#[relationship_target(relationship = EnergyGiver)]
pub struct EnergyReceiver(Vec<Entity>);

#[derive(Event, Debug)]
pub struct EnergyConnected {
    pub giver: Entity,
    pub receiver: Entity,
}

#[derive(Event, Debug)]
pub struct EnergyDisconnected {
    pub giver: Entity,
    pub receiver: Entity,
}

fn connect_energy(mut connected: EventReader<EnergyConnected>, mut commands: Commands) {
    for e in connected.read() {
        commands
            .entity(e.giver)
            .entry::<EnergyGiver>()
            .and_modify(|mut giver| giver.push(e.receiver))
            .or_insert(EnergyGiver(vec![(e.receiver)]));
    }
}

fn disconnect_energy(mut disconnected: EventReader<EnergyDisconnected>, mut commands: Commands) {
    for e in disconnected.read() {
        commands
            .entity(e.giver)
            .entry::<EnergyGiver>()
            .and_modify(|mut giver| {
                if let Ok(index) = giver.0.iter().position(|r| r == e.receiver) {
                    giver.0.remove(index);
                }
            });
    }
}
