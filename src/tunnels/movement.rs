use crate::board_dimensions::BoardDimensions;
use bevy::prelude::*;

use crate::capman::Capman;
use crate::common::position::Vec3Helper;
use crate::common::Direction;
use crate::ghosts::Ghost;
use crate::tunnels::{GhostPassedTunnel, Tunnel};

// TODO: There was some smarter way to get combinations of two equivalent queries. Use this
pub(in crate::tunnels) fn move_capman_through_tunnel(
    dimensions: Res<BoardDimensions>,
    tunnel_query_0: Query<(Entity, &Tunnel, &Transform, &Direction), Without<Capman>>,
    tunnel_query_1: Query<(Entity, &Tunnel, &Transform, &Direction), Without<Capman>>,
    mut capman_query: Query<(&mut Transform, &mut Direction), With<Capman>>,
) {
    for (entity_0, tunnel_0, tunnel_transform_0, tunnel_direction_0) in tunnel_query_0.iter() {
        for (mut transform, mut capman_direction) in capman_query.iter_mut() {
            if dimensions.trans_to_pos(&transform) != dimensions.trans_to_pos(tunnel_transform_0)
                || *capman_direction != *tunnel_direction_0
            {
                continue;
            }

            for (entity_1, tunnel_1, tunnel_transform_1, tunnel_direction_1) in
                tunnel_query_1.iter()
            {
                if entity_0 != entity_1 && **tunnel_0 == **tunnel_1 {
                    transform
                        .translation
                        .set_xy(&tunnel_transform_1.translation);
                    *capman_direction = tunnel_direction_1.opposite()
                }
            }
        }
    }
}

pub(in crate::tunnels) fn move_ghost_trough_tunnel(
    mut event_writer: EventWriter<GhostPassedTunnel>,
    dimensions: Res<BoardDimensions>,
    tunnel_query_0: Query<(Entity, &Tunnel, &Transform, &Direction), Without<Ghost>>,
    tunnel_query_1: Query<(Entity, &Tunnel, &Transform, &Direction), Without<Ghost>>,
    mut ghost_query: Query<(Entity, &mut Transform, &mut Direction), With<Ghost>>,
) {
    for (entity_0, tunnel_0, tunnel_transform_0, tunnel_direction_0) in tunnel_query_0.iter() {
        for (ghost_entity, mut transform, mut ghost_direction) in ghost_query.iter_mut() {
            if dimensions.trans_to_pos(&transform) != dimensions.trans_to_pos(tunnel_transform_0)
                || *ghost_direction != *tunnel_direction_0
            {
                continue;
            }

            for (entity_1, tunnel_1, tunnel_transform_1, tunnel_direction_1) in
                tunnel_query_1.iter()
            {
                if entity_0 != entity_1 && **tunnel_0 == **tunnel_1 {
                    transform
                        .translation
                        .set_xy(&tunnel_transform_1.translation);
                    *ghost_direction = tunnel_direction_1.opposite();
                    event_writer.send(GhostPassedTunnel(ghost_entity));
                }
            }
        }
    }
}
