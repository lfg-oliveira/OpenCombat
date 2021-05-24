use crate::behavior::ItemBehavior;
use crate::map::Map;
use crate::scene::item::{SceneItem, SceneItemModifier};
use crate::SceneItemId;

const DEFAULT_FRAMES_TO_ACQUIRE: u32 = 120;

pub fn digest_engage_scene_item_behavior(
    frame_i: u32,
    scene_item: &SceneItem,
    engage_scene_item_id: SceneItemId,
    _map: &Map,
) -> Vec<SceneItemModifier> {
    let mut scene_item_modifiers: Vec<SceneItemModifier> = vec![];

    if let Some(visibility) = scene_item.visibility_for(engage_scene_item_id) {
        // Always acquire a target before fire
        if let Some(acquiring_since) = scene_item.acquiring_since {
            if frame_i - acquiring_since >= DEFAULT_FRAMES_TO_ACQUIRE {
                scene_item_modifiers.push(SceneItemModifier::FireOnSceneItem(
                    engage_scene_item_id,
                    visibility.to_scene_point,
                ))
            }
        } else {
            scene_item_modifiers.push(SceneItemModifier::BeginAcquire)
        }
    } else {
        // TODO: Disangage modifier ?
    }

    scene_item_modifiers
}
