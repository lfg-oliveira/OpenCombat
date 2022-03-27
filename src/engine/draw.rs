use ggez::{
    graphics::{DrawParam, MeshBuilder},
    GameResult,
};

use crate::{
    order::{marker::OrderMarker, PendingOrder},
    types::*,
};

use super::Engine;

impl Engine {
    // TODO : don't generate sprites of non visible entities (hidden enemy, outside screen, etc)
    pub fn generate_entities_sprites(&mut self) -> GameResult {
        for (i, entity) in self.shared_state.entities().iter().enumerate() {
            for sprite in self.graphics.entity_sprites(EntityIndex(i), entity) {
                let sprite_ = sprite.dest(entity.get_world_point().to_vec2());
                self.graphics.append_sprites_batch(sprite_);
            }
        }

        Ok(())
    }

    pub fn generate_map_sprites(&self, _draw_decor: bool) -> GameResult {
        // Note : Background sprites have been prepared once for map_background_batch
        // Note : Decor sprites have been prepared once for map_background_batch
        Ok(())
    }

    pub fn generate_menu_sprites(&mut self) -> GameResult {
        if let Some((to_point, squad_id)) = self.local_state.get_squad_menu() {
            for sprite in self.graphics.squad_menu_sprites(
                *to_point,
                *self.local_state.get_current_cursor_window_point(),
                *squad_id,
            ) {
                self.graphics.append_ui_batch(sprite);
            }
        }

        Ok(())
    }

    pub fn generate_selection_meshes(&self, mesh_builder: &mut MeshBuilder) -> GameResult {
        self.generate_selected_entities_meshes(mesh_builder)?;

        Ok(())
    }

    pub fn generate_display_paths_meshes(&self, mesh_builder: &mut MeshBuilder) -> GameResult {
        for (display_path, _) in self.local_state.get_display_paths() {
            self.generate_display_path_meshes(display_path, mesh_builder)?
        }

        Ok(())
    }

    pub fn generate_debug_meshes(&self, mesh_builder: &mut MeshBuilder) -> GameResult {
        if self.local_state.get_debug().mouse() {
            self.generate_debug_mouse_meshes(mesh_builder)?;
        }

        // TODO : It is not the right place
        if self.local_state.get_pending_order().is_none() {
            self.generate_select_rectangle_meshes(mesh_builder)?;
        }

        if self.local_state.get_debug().move_paths() {
            self.generate_move_paths_meshes(mesh_builder)?
        }

        Ok(())
    }

    pub fn generate_pending_order_sprites(
        &self,
        pending_order: &PendingOrder,
        squad_id: SquadUuid,
        cached_points: &Vec<WorldPoint>,
    ) -> Vec<DrawParam> {
        let mut draw_params = vec![];
        let order_marker = pending_order.marker();
        let sprite_infos = order_marker.sprite_info();
        for (draw_to, angle, offset) in
            self.get_pending_order_params(pending_order, squad_id, cached_points)
        {
            draw_params.push(sprite_infos.as_draw_params(draw_to, angle, offset))
        }
        draw_params
    }

    pub fn generate_order_marker_sprites(
        &self,
        order_marker: &OrderMarker,
        point: WindowPoint,
    ) -> Vec<DrawParam> {
        let sprite_infos = order_marker.sprite_info();
        let offset = order_marker.offset();
        vec![sprite_infos.as_draw_params(point, Angle(0.), offset)]
    }
}
