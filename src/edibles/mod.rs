use bevy::prelude::*;
use crate::edibles::dots::DotPlugin;
use crate::edibles::energizer::EnergizerPlugin;
use crate::edibles::fruit::FruitPlugin;
use crate::life_cycle::LifeCycle::Running;

pub mod dots;
pub mod fruit;
pub mod energizer;

pub struct EdiblePlugin;

impl Plugin for EdiblePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EAllEdiblesEaten>()
            .add_plugin(DotPlugin)
            .add_plugin(EnergizerPlugin)
            .add_plugin(FruitPlugin)
            .add_system_set(
                SystemSet::on_update(Running).with_system(check_if_all_edibles_eaten)
            )
        ;
    }
}

/// Component for everything in the maze that is edible.
#[derive(Component)]
pub struct Edible;

/// Event that gets fired when all edibles are eaten (or at least gone), so the maze is empty
pub struct EAllEdiblesEaten;

fn check_if_all_edibles_eaten(
    mut event_writer: EventWriter<EAllEdiblesEaten>,
    query: Query<&Edible>
) {
    if query.iter().count() == 0 {
        event_writer.send(EAllEdiblesEaten)
    }
}