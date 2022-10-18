use crate::capman::ECapmanDead;
use crate::edibles::EAllEdiblesEaten;
use crate::game_assets::EAllAssetsLoaded;
use crate::interactions::{ECapmanHit, EGhostEaten};
use crate::lives::Life;
use bevy::prelude::*;
use LifeCycle::*;

/// All lifecycle states of the app. See ./resources/lifecycle.png for a visualization.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum LifeCycle {
    Loading,
    Start,
    Ready,
    Running,
    CapmanHit,
    CapmanDying,
    CapmanDead,
    GameOver,
    LevelTransition,
    GhostEatenPause,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(Loading)
            .add_system_set(
                SystemSet::on_update(Loading).with_system(start_game_when_all_assets_loaded),
            )
            .add_system_set(SystemSet::on_enter(Start).with_system(start_state_timer))
            .add_system_set(
                SystemSet::on_update(Start).with_system(switch_state_when_state_timer_finished),
            )
            .add_system_set(SystemSet::on_enter(Ready).with_system(start_state_timer))
            .add_system_set(
                SystemSet::on_update(Ready).with_system(switch_state_when_state_timer_finished),
            )
            .add_system_set(
                SystemSet::on_update(Running)
                    .with_system(switch_to_dying_when_capman_was_hit)
                    .with_system(switch_to_level_transition_when_all_edibles_eaten)
                    .with_system(switch_to_ghost_eaten_pause_when_ghost_was_eaten),
            )
            .add_system_set(SystemSet::on_enter(CapmanHit).with_system(start_state_timer))
            .add_system_set(
                SystemSet::on_update(CapmanHit).with_system(switch_state_when_state_timer_finished),
            )
            .add_system_set(
                SystemSet::on_update(CapmanDying).with_system(switch_to_dead_when_capman_is_dead),
            )
            .add_system_set(SystemSet::on_enter(CapmanDead).with_system(start_state_timer))
            .add_system_set(
                SystemSet::on_update(CapmanDead).with_system(switch_dead_state_when_timer_finished),
            )
            .add_system_set(SystemSet::on_enter(LevelTransition).with_system(start_state_timer))
            .add_system_set(
                SystemSet::on_update(LevelTransition)
                    .with_system(switch_state_when_state_timer_finished),
            )
            .add_system_set(SystemSet::on_enter(GhostEatenPause).with_system(start_state_timer))
            .add_system_set(
                SystemSet::on_update(GhostEatenPause)
                    .with_system(switch_state_when_state_timer_finished),
            );
    }
}

/// Some lifecycle states just wait for a few seconds before switching. This timer and the related systems
/// handle these states
#[derive(Deref, DerefMut)]
struct StateTimer(Timer);

fn start_game_when_all_assets_loaded(
    mut life_cycle: ResMut<State<LifeCycle>>,
    mut event_reader: EventReader<EAllAssetsLoaded>,
) {
    for _ in event_reader.iter() {
        life_cycle.set(Start).unwrap()
    }
}

fn start_state_timer(mut commands: Commands, life_cycle: Res<State<LifeCycle>>) {
    let state_time = match life_cycle.current() {
        Start => 2.0,
        Ready => 2.5,
        CapmanHit => 1.0,
        CapmanDead => 1.0,
        LevelTransition => 3.0,
        GhostEatenPause => 1.0,
        _ => return,
    };

    commands.insert_resource(StateTimer(Timer::from_seconds(state_time, false)));
}

fn switch_state_when_state_timer_finished(
    mut commands: Commands,
    time: Res<Time>,
    mut state_timer: ResMut<StateTimer>,
    mut life_cycle: ResMut<State<LifeCycle>>,
) {
    state_timer.tick(time.delta());

    if state_timer.finished() {
        commands.remove_resource::<StateTimer>();

        let next_state = match life_cycle.current() {
            Start => Ready,
            Ready => Running,
            CapmanHit => CapmanDying,
            LevelTransition => Ready,
            GhostEatenPause => Running,
            _ => return,
        };

        life_cycle.set(next_state).unwrap()
    }
}

fn switch_to_dying_when_capman_was_hit(
    mut event_reader: EventReader<ECapmanHit>,
    mut game_state: ResMut<State<LifeCycle>>,
) {
    for _ in event_reader.iter() {
        game_state.set(CapmanHit).unwrap()
    }
}

fn switch_to_dead_when_capman_is_dead(
    mut event_reader: EventReader<ECapmanDead>,
    mut game_state: ResMut<State<LifeCycle>>,
) {
    for _ in event_reader.iter() {
        game_state.set(CapmanDead).unwrap()
    }
}

fn switch_dead_state_when_timer_finished(
    mut commands: Commands,
    time: Res<Time>,
    mut state_timer: ResMut<StateTimer>,
    mut life_cycle: ResMut<State<LifeCycle>>,
    query: Query<&Life>,
) {
    state_timer.tick(time.delta());

    if state_timer.finished() {
        commands.remove_resource::<StateTimer>();

        if query.iter().count() > 0 {
            life_cycle.set(Ready).unwrap()
        } else {
            life_cycle.set(GameOver).unwrap()
        }
    }
}

fn switch_to_level_transition_when_all_edibles_eaten(
    mut event_reader: EventReader<EAllEdiblesEaten>,
    mut life_cycle: ResMut<State<LifeCycle>>,
) {
    for _ in event_reader.iter() {
        life_cycle.set(LevelTransition).unwrap()
    }
}

fn switch_to_ghost_eaten_pause_when_ghost_was_eaten(
    mut event_reader: EventReader<EGhostEaten>,
    mut life_cycle: ResMut<State<LifeCycle>>,
) {
    for _ in event_reader.iter() {
        life_cycle.set(GhostEatenPause).unwrap()
    }
}
