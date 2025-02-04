use battle_core::{
    game::squad::{SquadStatusResume, SquadStatusesResume},
    state::battle::{phase::Phase, BattleState},
    types::WindowPoint,
};
use ggez::Context;
use glam::Vec2;

use crate::{engine::state::GuiState, ui::component::Component};

use super::{
    background::Background,
    battle::BattleButton,
    detail::{SquadDetail, SQUAD_DETAIL_WIDTH},
    minimap::Minimap,
    morale::{MoraleIndicator, MORALE_INDICATOR_HEIGHT},
    squad::SquadStatuses,
    Hud,
};

pub const MARGIN: f32 = 5.;
pub const RIGHT_BOX_WIDTH: f32 = 200.;
pub const BOTTOM_LINE_HEIGHT: f32 = 25.;

pub struct HudBuilder<'a> {
    gui_state: &'a GuiState,
    battle_state: &'a BattleState,
    point: WindowPoint,
    width: f32,
    height: f32,
}

impl<'a> HudBuilder<'a> {
    pub fn new(gui_state: &'a GuiState, battle_state: &'a BattleState) -> Self {
        Self {
            gui_state,
            battle_state,
            point: WindowPoint::new(0., 0.),
            width: 0.,
            height: 0.,
        }
    }

    pub fn point(mut self, point: WindowPoint) -> Self {
        self.point = point;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn build(&self, ctx: &Context) -> Hud {
        let right_column_start = self
            .point
            .apply(Vec2::new(self.width - RIGHT_BOX_WIDTH, MARGIN));
        let battle_button = self.battle_button(&right_column_start);
        let morale_indicator_start =
            right_column_start.apply(Vec2::new(battle_button.width(ctx), 0.));
        let morale_indicator = self.morale_indicator(&morale_indicator_start);
        let hud_interior_start = self.point.apply(Vec2::new(MARGIN, MARGIN));
        let squad_statuses = self.squad_statuses(&hud_interior_start);

        let squad_detail_start = right_column_start.apply(Vec2::new(-SQUAD_DETAIL_WIDTH, 0.));
        let squad_detail = self.squad_detail(&squad_detail_start);

        let minimap_start = right_column_start.apply(Vec2::new(0., MORALE_INDICATOR_HEIGHT));
        let minimap = self.minimap(&minimap_start);

        Hud::new(
            Background::new(self.point, self.width, self.height),
            battle_button,
            morale_indicator,
            squad_statuses,
            squad_detail,
            minimap,
        )
    }

    fn battle_button(&self, point: &WindowPoint) -> BattleButton {
        match self.battle_state.phase() {
            Phase::Placement => {
                let enabled = !self.battle_state.ready(self.gui_state.side());
                BattleButton::begin(*point, enabled)
            }
            // FIXME BS NOW : enabled computing
            Phase::Battle => BattleButton::end(*point, true),
            Phase::End(_, _) => BattleButton::end(*point, false),
        }
    }

    fn morale_indicator(&self, point: &WindowPoint) -> MoraleIndicator {
        MoraleIndicator::new(
            *point,
            self.battle_state.a_morale().clone(),
            self.battle_state.b_morale().clone(),
        )
    }

    fn squad_statuses(&self, point: &WindowPoint) -> SquadStatuses {
        SquadStatuses::new(
            SquadStatusesResume::from_battle_state(self.gui_state.side(), self.battle_state),
            *point,
            self.gui_state.selected_squads().1.clone(),
        )
    }

    fn squad_detail(&self, point: &WindowPoint) -> SquadDetail {
        if let Some(squad_uuid) = self.gui_state.selected_squads().1.first() {
            SquadDetail::new(
                *point,
                Some(SquadStatusResume::from_squad(self.battle_state, squad_uuid)),
                self.gui_state.selected_squads().0,
            )
        } else {
            SquadDetail::empty(*point)
        }
    }

    fn minimap(&self, point: &WindowPoint) -> Minimap {
        // FIXME BS NOW : all of this consume cpu (specially soldier_squad_is_visible_by_side)
        // So, reduce hud or minimap framerate (store it in gui state ?)
        let blue_positions = self
            .battle_state
            .squads()
            .iter()
            .map(|(s, _)| self.battle_state.squad(*s))
            .map(|s| self.battle_state.soldier(s.leader()))
            .filter(|s| s.side() == self.gui_state.side())
            .map(|s| s.world_point())
            .collect();
        let red_positions = self
            .battle_state
            .squads()
            .iter()
            .map(|(s, _)| self.battle_state.squad(*s))
            .map(|s| self.battle_state.soldier(s.leader()))
            .filter(|s| s.side() != self.gui_state.side())
            .filter(|s| {
                self.battle_state
                    .soldier_squad_is_visible_by_side(s, self.gui_state.side())
            })
            .map(|s| s.world_point())
            .collect();
        Minimap::new(
            *point,
            self.battle_state.map().visual_width() as f32,
            self.battle_state.map().visual_height() as f32,
            self.gui_state.display_scene_offset,
            self.gui_state.zoom.clone(),
            blue_positions,
            red_positions,
        )
    }
}
