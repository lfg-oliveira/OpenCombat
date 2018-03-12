# coding: utf-8
import time
import typing

from synergine2.simulation import SubjectComposedBehaviour, SubjectBehaviour
from synergine2.simulation import BehaviourStep
from synergine2.simulation import Event


# class MoveWithRotationBehaviour(SubjectComposedBehaviour):
#     step_classes = [
#
#     ]
from synergine2_xyz.move.intention import MoveToIntention
from synergine2_xyz.utils import get_angle


class SubjectStartRotationEvent(Event):
    def __init__(
        self,
        rotate_relative: float,
        duration: float,
    ) -> None:
        self.rotate_relative = rotate_relative
        self.duration = duration


class SubjectContinueRotationEvent(Event):
    def __init__(
        self,
        rotate_relative: float,
        duration: float,
    ) -> None:
        self.rotate_relative = rotate_relative
        self.duration = duration


class SubjectFinishRotationEvent(Event):
    def __init__(
        self,
        rotation_absolute: float,
    ) -> None:
        self.rotation_absolute = rotation_absolute


class SubjectStartTileMoveEvent(Event):
    def __init__(
        self,
        move_to: typing.Tuple[int, int],
        duration: float,
    ) -> None:
        self.move_to = move_to
        self.duration = duration


class SubjectFinishTileMoveEvent(Event):
    def __init__(
        self,
        move_to: typing.Tuple[int, int],
    ) -> None:
        self.move_to = move_to


class SubjectFinishMoveEvent(Event):
    def __init__(
        self,
        move_to: typing.Tuple[int, int],
    ) -> None:
        self.move_to = move_to


class MoveWithRotationBehaviour(SubjectBehaviour):
    def run(self, data) -> object:
        """
        FIXME: implement this:
        1. if it is end of movement: return move_to_finished: x, y
        1bis: if first start: feed data with computed path
        2. if it is moving:
          2a: not finished: return moving_to: x, y
          2b: finished: fee data with tile_move_to_finished: x, y
        3. if it is rotating:
          3a: not finished: return rotate_to: x
          3b: finished: feed data with rotate_to_finished: x
        4. if next move need rotation: feed data with rotate_to: x, return data
        5. feed data with tile_move_to: x, y

        """
        # Prepare data
        from_ = data['from']  # type: typing.Tuple(int, int)
        to = data['to']  # type: typing.Tuple(int, int)
        return_data = {}
        now = time.time()

        # Test if it's first time
        if not data.get('path'):
            return_data['path'] = self.simulation.physics.found_path(
                start=self.subject.position,
                end=to,
                subject=self.subject,
            )
            # find path algorithm can skip start position, add it if not in
            if return_data['path'][0] != self.subject.position:
                return_data['path'] = [self.subject.position] + return_data['path']
            data['path'] = return_data['path']

        # Prepare data
        path = data['path']  # type: typing.List[typing.Tuple(int, int)]
        path_index = path.index(self.subject.position)
        next_position = path[path_index + 1]
        next_position_direction = get_angle(self.subject.position, next_position)
        rotate_relative = next_position_direction - self.subject.direction

        # Test if finish move
        if path_index == len(path) - 1:
            return {
                'move_finished': to,
            }

        # Check if moving
        if self.subject.moving_to == next_position:
            if self.subject.start_move + self.subject.move_duration > now:
                # Let moving
                return {
                    'tile_moving_to': next_position,
                }
            return_data['tile_move_to_finished'] = self.subject.moving_to

        # Check if rotating
        if self.subject.rotate_to != -1:
            # If it is not finished
            if self.subject.start_rotation + self.subject.rotate_duration > now:
                # Let rotation do it's job
                return {
                    'rotate_relative': rotate_relative,
                    'rotate_absolute': next_position_direction,
                }
            # rotation finish
            return_data['rotate_to_finished'] = self.subject.rotate_to

        # Check if need to rotate
        if self.subject.direction != next_position_direction:
            return_data.update({
                'start_rotate_relative': rotate_relative,
                'start_rotate_absolute': next_position_direction,
            })
            return return_data

        # Need to move to next tile
        return_data['tile_move_to'] = next_position
        return return_data

    def action(self, data) -> [Event]:
        events = []

        if data.get('path'):
            move = self.subject.intentions.get(MoveToIntention)
            move.path = data['path']
            self.subject.intentions.set(move)

        if data.get('tile_move_to_finished'):
            self.subject.position = data['tile_move_to_finished']
            events.append(SubjectFinishTileMoveEvent(
                move_to=data['tile_move_to_finished'],
            ))

        if data.get('move_to_finished'):
            self.subject.position = data['move_to_finished']
            events.append(SubjectFinishMoveEvent(
                move_to=data['move_to_finished'],
            ))

        if data.get('start_rotate_relative'):
            duration = self.subject.get_rotate_duration(
                angle=data['start_rotate_relative'],
            )
            self.subject.rotate_to = data['start_rotate_absolute']
            self.subject.rotate_duration = duration
            self.subject.start_rotation = time.time()

            events.append(SubjectStartRotationEvent(
                rotate_relative=data['start_rotate_relative'],
                duration=duration,
            ))

        if data.get('rotate_to_finished'):
            self.subject.rotate_to = -1
            self.subject.rotate_duration = -1
            self.subject.start_rotation = -1
            self.subject.direction = data['rotate_to_finished']

            return events.append(SubjectFinishRotationEvent(
                rotation_absolute=data['rotate_to_finished'],
            ))

        if data.get('rotate_relative'):
            duration = self.subject.get_rotate_duration(angle=data['rotate_relative'])
            self.subject.rotate_to = data['rotate_absolute']
            self.subject.rotate_duration = duration
            self.subject.start_rotation = time.time()
            # FIXME: we not have info about actuel direction

            return [SubjectContinueRotationEvent(
                rotate_relative=data['rotate_relative'],
                duration=duration,
            )]

        if data.get('tile_move_to'):
            # TODO: duration must be computed
            duration = self.subject.walk_duration
            self.subject.moving_to = data['tile_move_to']
            self.subject.move_duration = duration
            self.subject.start_move = time.time()
            events.append(SubjectStartTileMoveEvent(
                move_to=data['tile_move_to'],
                duration=duration,
            ))

        return events
