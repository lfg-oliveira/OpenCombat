# coding: utf-8
import typing

from synergine2.config import Config
from synergine2_cocos2d.interaction import BaseActorInteraction
from synergine2.simulation import SimulationBehaviour
from synergine2.simulation import Simulation
from synergine2.simulation import Event
from synergine2_cocos2d.actor import Actor
from synergine2_cocos2d.gl import draw_line
from synergine2_xyz.move.simulation import RequestMoveBehaviour as BaseRequestMoveBehaviour  # nopep8

from opencombat.simulation.move import TeamPlacer
from opencombat.user_action import UserAction


class RequestMoveBehaviour(BaseRequestMoveBehaviour):
    def __init__(
        self,
        config: Config,
        simulation: Simulation,
    ):
        super().__init__(config, simulation)
        self._team_placer = TeamPlacer(config, self.simulation)

    def action(self, data) -> typing.List[Event]:
        subject_id = data['subject_id']
        move_to = data['move_to']

        try:
            subject = self.simulation.subjects.index[subject_id]
            teammates = [
                self.simulation.subjects.index[teammate_id]
                for teammate_id in subject.teammate_ids
            ]
            subjects = teammates + [subject]
            subject_positions = self._team_placer.get_positions(
                subjects,
                move_to,
            )

            for subject, to_position in subject_positions:
                subject.intentions.set(self.move_intention_class(
                    to_position,
                    gui_action=data['gui_action'],
                ))
        except KeyError:
            # TODO: log error here
            pass

        return []


class BaseMoveActorInteraction(BaseActorInteraction):
    gui_action = None
    color = None
    request_move_behaviour_class = RequestMoveBehaviour

    def draw_pending(self) -> None:
        for actor in self.layer_manager.edit_layer.selection:
            grid_position = self.layer_manager\
                .grid_manager\
                .get_grid_position(actor.position)
            pixel_position = self.layer_manager\
                .grid_manager\
                .get_world_position_of_grid_position(grid_position)

            draw_line(
                self.layer_manager.scrolling_manager.world_to_screen(*pixel_position),
                self.layer_manager.edit_layer.screen_mouse,
                self.color,
            )

    def get_behaviour(
        self,
        actor: Actor,
        mouse_grid_position,
    ) -> typing.Tuple[typing.Type[SimulationBehaviour], dict]:
        return self.request_move_behaviour_class, {
            'subject_id': actor.subject.id,
            'move_to': mouse_grid_position,
            'gui_action': self.gui_action,
        }


class MoveActorInteraction(BaseMoveActorInteraction):
    gui_action = UserAction.ORDER_MOVE
    color = (0, 0, 255)


class MoveFastActorInteraction(BaseMoveActorInteraction):
    gui_action = UserAction.ORDER_MOVE_FAST
    color = (72, 244, 66)


class MoveCrawlActorInteraction(BaseMoveActorInteraction):
    gui_action = UserAction.ORDER_MOVE_CRAWL
    color = (235, 244, 66)
