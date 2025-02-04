use std::path::PathBuf;

use battle_core::game::Side;
use battle_core::map::Map;
use battle_core::order::PendingOrder;
use battle_core::physics::utils::DISTANCE_TO_METERS_COEFFICIENT;
use battle_core::types::{
    Distance, Offset, SoldierIndex, SquadUuid, WindowPoint, WorldPaths, WorldPoint,
};
use battle_core::utils::{DebugPoint, WindowShape, WorldShape};
use ggez::graphics::Rect;
use ggez::Context;

use crate::debug::{DebugPhysics, DebugTerrain};
use crate::graphics::qualified::Zoom;
use crate::ui::hud::HUD_HEIGHT;

use super::event::UIEvent;
use super::input::Control;
use super::message::GuiStateMessage;

pub struct GuiState {
    /// Printed frames since start of program
    frame_i: u64,
    /// Side of game instance
    side: Side,
    /// Offset to apply to battle scene by window relative
    pub display_scene_offset: Offset,
    /// Scale to apply to battle scene by window relative
    pub zoom: Zoom,
    /// Display or not decor (trees, etc)
    pub draw_decor: bool,
    /// Current debugs to apply
    pub debug_mouse: bool,
    pub debug_move_paths: bool,
    pub debug_formation_positions: bool,
    pub debug_scene_item_circles: bool,
    pub debug_areas: bool,
    pub debug_visibilities: bool,
    pub debug_targets: bool,
    pub debug_physics_areas: bool,
    /// Current debug terrain to apply
    pub debug_terrain: DebugTerrain,
    /// Current debug physics to apply
    debug_physics: DebugPhysics,
    /// Should display debug window ?
    display_debug_gui: bool,
    /// Is debug window currently covered
    pub debug_gui_hovered: bool,
    /// Current WindowPoint of mouse cursor
    current_cursor_point: WindowPoint,
    /// Last instant since cursor don't move
    last_cursor_move_frame: u64,
    /// WindowPoint where left click was down, if left click currently down
    left_click_down: Option<WindowPoint>,
    /// Vector representing current cursor drag
    current_cursor_vector: Option<(WindowPoint, WindowPoint)>,
    /// Vector of UIEvent (will be consumed)
    ui_events: Vec<UIEvent>,
    /// Selected squad ids
    selected_squads: (Option<SoldierIndex>, Vec<SquadUuid>),
    /// Possible currently displayed menu
    squad_menu: Option<(WindowPoint, Vec<SquadUuid>)>,
    /// Possible current player squad order
    pending_order: Vec<PendingOrder>,
    /// Paths to display
    display_paths: Vec<Vec<(WorldPaths, SquadUuid)>>,
    /// Used to know a path already search here last frame
    last_computed_path_point: Option<WorldPoint>,
    /// Debug points to display if debug mode
    debug_points: Vec<DebugPoint>,
    /// Contains current control mode
    control: Control,
    cursor_in_hud: bool,
    ///
    begin_click_on_soldier: Option<SoldierIndex>,
    dragged_squad: Option<SquadUuid>,
    //
    intro_ack: bool,
    //
    saves: Vec<PathBuf>,
    //
    map_width: f32,
    map_height: f32,
}

impl GuiState {
    pub fn new(side: Side, map: &Map) -> Self {
        Self {
            frame_i: 0,
            side,
            display_scene_offset: Offset::new(0., 0.),
            zoom: Zoom::default(),
            draw_decor: true,
            debug_mouse: false,
            debug_move_paths: false,
            debug_formation_positions: false,
            debug_scene_item_circles: false,
            debug_areas: false,
            debug_visibilities: false,
            debug_targets: false,
            debug_physics_areas: false,
            debug_terrain: DebugTerrain::None,
            debug_physics: DebugPhysics::None,
            display_debug_gui: false,
            debug_gui_hovered: false,
            current_cursor_point: WindowPoint::new(0., 0.),
            last_cursor_move_frame: 0,
            left_click_down: None,
            current_cursor_vector: None,
            ui_events: vec![],
            selected_squads: (None, vec![]),
            squad_menu: None,
            pending_order: vec![],
            display_paths: vec![],
            last_computed_path_point: None,
            debug_points: vec![],
            control: Control::Soldiers,
            cursor_in_hud: false,
            begin_click_on_soldier: None,
            dragged_squad: None,
            intro_ack: false,
            saves: vec![],
            map_width: map.visual_width() as f32,
            map_height: map.visual_height() as f32,
        }
    }

    pub fn frame_i(&self) -> u64 {
        self.frame_i
    }

    pub fn increment_frame_i(&mut self) {
        self.frame_i += 1;
    }

    pub fn side(&self) -> &Side {
        &self.side
    }

    pub fn opponent_side(&self) -> &Side {
        match self.side {
            Side::A => &Side::B,
            Side::B => &Side::A,
            _ => unreachable!(),
        }
    }

    pub fn debug_terrain(&self) -> &DebugTerrain {
        &self.debug_terrain
    }

    pub fn debug_physics(&self) -> &DebugPhysics {
        &self.debug_physics
    }

    pub fn debug_physics_mut(&mut self) -> &mut DebugPhysics {
        &mut self.debug_physics
    }

    pub fn current_cursor_window_point(&self) -> &WindowPoint {
        &self.current_cursor_point
    }

    pub fn current_cursor_world_point(&self) -> WorldPoint {
        self.world_point_from_window_point(self.current_cursor_point)
    }

    pub fn left_click_down_window_point(&self) -> &Option<WindowPoint> {
        &self.left_click_down
    }

    pub fn _left_click_down_world_point(&self) -> Option<WorldPoint> {
        self.left_click_down.map(|left_click_down| self.world_point_from_window_point(left_click_down))
    }

    pub fn current_cursor_vector_window_points(&self) -> &Option<(WindowPoint, WindowPoint)> {
        &self.current_cursor_vector
    }

    pub fn _current_cursor_vector_world_points(&self) -> Option<(WorldPoint, WorldPoint)> {
        if let Some((start, end)) = self.current_cursor_vector {
            let world_start = self.world_point_from_window_point(start);
            let world_end = self.world_point_from_window_point(end);
            Some((world_start, world_end))
        } else {
            None
        }
    }

    pub fn pop_ui_event(&mut self) -> Option<UIEvent> {
        self.ui_events.pop()
    }

    pub fn world_point_from_window_point(&self, window_point: WindowPoint) -> WorldPoint {
        WorldPoint::from(
            window_point
                .apply(-self.display_scene_offset.to_vec2())
                .to_vec2()
                / self.zoom.factor(),
        )
    }

    pub fn window_point_from_world_point(&self, world_point: WorldPoint) -> WindowPoint {
        WindowPoint::from(
            world_point
                .apply(self.display_scene_offset.to_vec2() / self.zoom.factor())
                .to_vec2()
                * self.zoom.factor(),
        )
    }

    pub fn window_rect_from_world_rect(&self, world_rect: Rect) -> Rect {
        let window_point =
            self.window_point_from_world_point(WorldPoint::new(world_rect.x, world_rect.y));
        Rect::new(
            window_point.x,
            window_point.y,
            world_rect.w * self.zoom.factor(),
            world_rect.h * self.zoom.factor(),
        )
    }

    pub fn window_shape_from_world_shape(&self, world_shape: &WorldShape) -> WindowShape {
        WindowShape {
            top_left: self.window_point_from_world_point(world_shape.top_left),
            top_right: self.window_point_from_world_point(world_shape.top_right),
            bottom_right: self.window_point_from_world_point(world_shape.bottom_right),
            bottom_left: self.window_point_from_world_point(world_shape.bottom_left),
        }
    }

    pub fn selected_squads(&self) -> &(Option<SoldierIndex>, Vec<SquadUuid>) {
        &self.selected_squads
    }

    pub fn squad_menu(&self) -> &Option<(WindowPoint, Vec<SquadUuid>)> {
        &self.squad_menu
    }

    pub fn last_cursor_move_frame(&self) -> u64 {
        self.last_cursor_move_frame
    }

    pub fn pending_order(&self) -> &Vec<PendingOrder> {
        &self.pending_order
    }

    pub fn display_paths(&self) -> &Vec<Vec<(WorldPaths, SquadUuid)>> {
        &self.display_paths
    }

    pub fn last_computed_path_point(&self) -> &Option<WorldPoint> {
        &self.last_computed_path_point
    }

    pub fn debug_points_mut(&mut self) -> &mut Vec<DebugPoint> {
        &mut self.debug_points
    }

    pub fn set_debug_points(&mut self, debug_points: Vec<DebugPoint>) {
        self.debug_points = debug_points
    }

    pub fn display_debug_gui(&self) -> bool {
        self.display_debug_gui
    }

    pub fn distance_pixels(&self, distance: &Distance) -> f32 {
        ((distance.millimeters() as f32) / 1000.) / DISTANCE_TO_METERS_COEFFICIENT
    }

    pub fn set_scene_offset_to(&mut self, ctx: &mut Context, target_point: &WorldPoint) {
        let (window_width, window_height) = ctx.gfx.drawable_size();
        let half_window = WindowPoint::new(window_width / 2., (window_height - HUD_HEIGHT) / 2.);
        let needed_offset = Offset::new(
            half_window.x - target_point.x * self.zoom.factor(),
            half_window.y - target_point.y * self.zoom.factor(),
        );
        self.display_scene_offset = needed_offset;
    }

    pub fn ensure_correct_scene_offset(&mut self, ctx: &mut Context) {
        let (window_width, window_height) = ctx.gfx.drawable_size();

        let top = window_width / 2.;
        let left = window_height / 2.;
        let right = -((self.map_width * self.zoom.factor()) - window_width / 2.);
        let bottom = -((self.map_height * self.zoom.factor()) - window_height / 2.);

        self.display_scene_offset.x = self.display_scene_offset.x.min(top);
        self.display_scene_offset.y = self.display_scene_offset.y.min(left);
        self.display_scene_offset.x = self.display_scene_offset.x.max(right);
        self.display_scene_offset.y = self.display_scene_offset.y.max(bottom);
    }

    pub fn react(&mut self, message: &GuiStateMessage, ctx: &mut Context) {
        match message {
            GuiStateMessage::SetCursorPoint(point) => {
                //
                self.current_cursor_point = *point;
                self.last_cursor_move_frame = self.frame_i;
            }
            GuiStateMessage::ApplyOnDisplaySceneOffset(offset) => {
                //
                self.display_scene_offset =
                    Offset::from_vec2(self.display_scene_offset.to_vec2() + offset.to_vec2());
                self.ensure_correct_scene_offset(ctx);
            }
            GuiStateMessage::SetDisplaySceneOffset(offset) => {
                //
                self.display_scene_offset = *offset;
                self.ensure_correct_scene_offset(ctx);
            }
            GuiStateMessage::SetDebugTerrain(value) => {
                //
                self.debug_terrain = value.clone();
            }
            GuiStateMessage::SetDebugPhysics(level) => self.debug_physics = level.clone(),
            GuiStateMessage::SetLeftClickDown(point) => {
                //
                self.left_click_down = *point
            }
            GuiStateMessage::SetCurrentCursorVector(vector) => {
                //
                self.current_cursor_vector = *vector
            }
            GuiStateMessage::PushUIEvent(event) => {
                //
                self.ui_events.push(event.clone())
            }
            GuiStateMessage::SetSelectedSquads(selected_soldier_index, selected_squads) => {
                //
                self.selected_squads = (*selected_soldier_index, selected_squads.clone())
            }
            GuiStateMessage::SetSquadMenu(squad_menu) => {
                //
                self.squad_menu = squad_menu.clone()
            }
            GuiStateMessage::SetPendingOrders(pending_orders) => {
                //
                self.pending_order = pending_orders.clone()
            }
            GuiStateMessage::SetDisplayPaths(display_paths) => {
                //
                self.display_paths = display_paths.clone();
                self.last_computed_path_point = None;
            }
            GuiStateMessage::AddCachePointToPendingOrder(new_point) => {
                for pending_order in &mut self.pending_order {
                    pending_order.push_cache_point(*new_point);
                }
            }
            GuiStateMessage::PushDebugPoint(debug_point) => {
                //
                self.debug_points.push(debug_point.clone())
            }
            GuiStateMessage::ChangeSide => match self.side {
                Side::A => self.side = Side::B,
                Side::B => self.side = Side::A,
                Side::All => unreachable!("Side All is excluded from ChangeSide"),
            },
            GuiStateMessage::SetZoom(scale, point) => {
                //
                self.zoom = scale.clone();
                self.set_scene_offset_to(ctx, point);
                self.ensure_correct_scene_offset(ctx);
            }
            GuiStateMessage::SetControl(new_control) => {
                //
                self.control = new_control.clone();
            }
            GuiStateMessage::SetDebugGuiHovered(value) => {
                //
                self.debug_gui_hovered = *value
            }
            GuiStateMessage::SetDisplayDebugGui(value) => {
                //
                self.display_debug_gui = *value
            }
            GuiStateMessage::SetDragSquad(squad_index) => {
                //
                self.dragged_squad = *squad_index;
            }
            GuiStateMessage::SetBeginClickOnSoldier(soldier_index) => {
                self.begin_click_on_soldier = *soldier_index
            }
            GuiStateMessage::SetCursorInHud(value) => {
                //
                self.cursor_in_hud = *value
            }
            GuiStateMessage::SetIntroAck(value) => {
                //
                self.intro_ack = *value
            }
            GuiStateMessage::SetSavesList(saves) => {
                //
                self.saves = saves.clone()
            }
            GuiStateMessage::CenterSceneOn(point) => {
                //
                self.set_scene_offset_to(ctx, point);
                self.ensure_correct_scene_offset(ctx);
            }
        }
    }

    pub fn is_controlling(&self, control: &Control) -> bool {
        &self.control == control
    }

    pub fn controlling(&self) -> &Control {
        &self.control
    }

    pub fn debug_lines(&self) -> Vec<(String, String)> {
        let mut lines = vec![];

        lines.push(("FrameI".to_string(), self.frame_i.to_string()));

        lines.push(("Side".to_string(), self.side.to_string()));

        lines.push((
            "DisplaySceneOffset".to_string(),
            format!(
                "{}x{}",
                self.display_scene_offset.x.ceil(),
                self.display_scene_offset.y.ceil()
            ),
        ));

        lines.push((
            "DisplaySceneScale".to_string(),
            format!("{:.3}x{:.3}", self.zoom.factor(), self.zoom.factor()),
        ));

        lines.push((
            "CurrentCursorPoint".to_string(),
            format!(
                "{}x{}",
                self.current_cursor_point.x.ceil(),
                self.current_cursor_point.y.ceil()
            ),
        ));

        lines.push((
            "LastCursorMoveFrame".to_string(),
            self.last_cursor_move_frame.to_string(),
        ));

        let current_cursor_vector_text =
            if let Some(current_cursor_vector) = self.current_cursor_vector {
                format!(
                    "{}x{} -> {}x{}",
                    current_cursor_vector.0.x.ceil(),
                    current_cursor_vector.0.y.ceil(),
                    current_cursor_vector.1.x.ceil(),
                    current_cursor_vector.1.y.ceil()
                )
            } else {
                "".to_string()
            };
        lines.push((
            "CurrentCursorVector".to_string(),
            current_cursor_vector_text,
        ));

        let squad_menu_text = if self.squad_menu.is_some() {
            "Yes".to_string()
        } else {
            "No".to_string()
        };
        lines.push(("SquadMenu".to_string(), squad_menu_text));

        if !self.pending_order.is_empty() {
            for pending_order in &self.pending_order {
                lines.push(("PendingOrder".to_string(), pending_order.to_string()));
            }
        } else {
            lines.push(("PendingOrder".to_string(), "n/a".to_string()));
        }

        lines.push((
            "DebugPoints (len)".to_string(),
            self.debug_points.len().to_string(),
        ));

        lines.push(("Control".to_string(), self.control.to_string()));

        lines
    }

    pub fn dragged_squad(&self) -> &Option<SquadUuid> {
        &self.dragged_squad
    }

    pub fn begin_click_on_soldier(&self) -> Option<SoldierIndex> {
        self.begin_click_on_soldier
    }

    pub fn cursor_in_hud(&self) -> bool {
        self.cursor_in_hud
    }

    pub fn intro_ack(&self) -> bool {
        self.intro_ack
    }

    pub fn set_saves(&mut self, saves: Vec<PathBuf>) {
        self.saves = saves;
    }

    pub fn saves(&self) -> &Vec<PathBuf> {
        &self.saves
    }

    pub fn saves_mut(&mut self) -> &mut Vec<PathBuf> {
        &mut self.saves
    }
}
