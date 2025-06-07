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

/// connecting lazers to beams
#[derive(Component, Debug)]
#[relationship(relationship_target = BeamOf)]
pub struct LazerBeam(pub Entity);

#[derive(Component, Debug)]
#[relationship_target(relationship = LazerBeam)]
pub struct BeamOf(Entity);

// /// connecting beams to rays
// #[derive(Component, Debug)]
// #[relationship(relationship_target = RayOf)]
// pub struct BeamRay(pub Entity);

// #[derive(Component, Debug)]
// #[relationship_target(relationship = BeamRay)]
// pub struct RayOf(Entity);

// fn connect_energy(mut connected: EventReader<EnergyConnected>, mut commands: Commands) {
//     for e in connected.read() {
//         commands
//             .entity(e.giver)
//             .entry::<EnergyGiver>()
//             .and_modify(|mut giver| giver.push(e.receiver))
//             .or_insert(EnergyGiver(vec![(e.receiver)]));
//     }
// }

// fn disconnect_energy(mut disconnected: EventReader<EnergyDisconnected>, mut commands: Commands) {
//     for e in disconnected.read() {
//         commands
//             .entity(e.giver)
//             .entry::<EnergyGiver>()
//             .and_modify(|mut giver| {
//                 if let Ok(index) = giver.0.iter().position(|r| r == e.receiver) {
//                     giver.0.remove(index);
//                 }
//             });
//     }
// }
