use crate::behavior::order::Order;
use crate::behavior::ItemBehavior;
use crate::config::{MOVE_FAST_VELOCITY, MOVE_HIDE_VELOCITY, MOVE_VELOCITY};
use crate::ui::order::OrderMarker;
use crate::ScenePoint;
use std::f32::consts::FRAC_PI_2;

pub fn velocity_for_behavior(behavior: &ItemBehavior) -> Option<f32> {
    match behavior {
        ItemBehavior::MoveTo(_, _) => Some(MOVE_VELOCITY),
        ItemBehavior::MoveFastTo(_, _) => Some(MOVE_FAST_VELOCITY),
        ItemBehavior::HideTo(_, _) => Some(MOVE_HIDE_VELOCITY),
        _ => None,
    }
}

pub fn angle(to_point: ScenePoint, from_point: ScenePoint) -> f32 {
    f32::atan2(to_point.y - from_point.y, to_point.x - from_point.x) + FRAC_PI_2
}

pub fn order_maker_for_order(order: &Order) -> OrderMarker {
    match order {
        Order::MoveTo(move_to_scene_point) => OrderMarker::MoveTo(*move_to_scene_point),
        Order::MoveFastTo(move_to_scene_point) => OrderMarker::MoveFastTo(*move_to_scene_point),
        Order::HideTo(move_to_scene_point) => OrderMarker::HideTo(*move_to_scene_point),
    }
}
