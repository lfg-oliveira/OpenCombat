# coding: utf-8
import time
import typing

from synergine2.simulation import SubjectBehaviour
from synergine2.simulation import Event
from synergine2_xyz.move.intention import MoveToIntention
from synergine2_xyz.simulation import XYZSimulation
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


class SubjectContinueTileMoveEvent(Event):
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
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.simulation = typing.cast(XYZSimulation, self.simulation)

    def run(self, data) -> object:
        """
        Comptue data relative to move
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
                'move_to_finished': to,
            }

        # Check if moving
        if self.subject.moving_to == next_position:
            if self.subject.start_move + self.subject.move_duration > now:
                # Let moving
                return {
                    'tile_move_to': next_position,
                }
            return_data['tile_move_to_finished'] = self.subject.moving_to
            # Must consider new position of subject
            path_index = path.index(return_data['tile_move_to_finished'])
            if path_index == len(path) - 1:
                return {
                    'move_to_finished': to,
                }
            next_position = path[path_index + 1]
            next_position_direction = get_angle(
                return_data['tile_move_to_finished'],
                next_position,
            )
            rotate_relative = next_position_direction - self.subject.direction

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
        if not return_data.get('rotate_to_finished') \
                and self.subject.direction != next_position_direction:
            return_data.update({
                'rotate_relative': rotate_relative,
                'rotate_absolute': next_position_direction,
            })
            return return_data

        # Need to move to next tile
        return_data['tile_move_to'] = next_position
        return return_data

    def action(self, data) -> [Event]:
        events = []
        now = time.time()

        if data.get('path'):
            move = self.subject.intentions.get(MoveToIntention)
            move.path = data['path']
            self.subject.intentions.set(move)

        if data.get('tile_move_to_finished'):
            self.subject.position = data['tile_move_to_finished']
            self.subject.moving_to = (-1, -1)
            self.subject.start_move = -1
            self.subject.move_duration = -1
            events.append(SubjectFinishTileMoveEvent(
                move_to=data['tile_move_to_finished'],
            ))

        if data.get('move_to_finished'):
            self.subject.position = data['move_to_finished']
            self.subject.moving_to = (-1, -1)
            self.subject.start_move = -1
            self.subject.move_duration = -1
            self.subject.intentions.remove(MoveToIntention)
            events.append(SubjectFinishMoveEvent(
                move_to=data['move_to_finished'],
            ))

        if data.get('rotate_to_finished'):
            self.subject.rotate_to = -1
            self.subject.rotate_duration = -1
            self.subject.start_rotation = -1
            self.subject.direction = data['rotate_to_finished']

            events.append(SubjectFinishRotationEvent(
                rotation_absolute=data['rotate_to_finished'],
            ))

        if data.get('rotate_relative'):
            # Test if rotation is already started
            if self.subject.rotate_to == data['rotate_absolute']:
                # look at progression
                rotate_since = now - self.subject.start_rotation
                rotate_progress = rotate_since / self.subject.rotate_duration
                rotation_to_do = self.subject.rotate_to - self.subject.direction
                rotation_done = rotation_to_do * rotate_progress
                self.subject.direction = self.subject.direction + rotation_done
                rotation_left = self.subject.rotate_to - self.subject.direction
                duration = self.subject.get_rotate_duration(angle=rotation_left)
                self.subject.rotate_duration = duration
                self.subject.start_rotation = now

                return [SubjectContinueRotationEvent(
                    rotate_relative=rotation_left,
                    duration=duration,
                )]
            else:
                duration = self.subject.get_rotate_duration(angle=data['rotate_relative'])
                self.subject.rotate_to = data['rotate_absolute']
                self.subject.rotate_duration = duration
                self.subject.start_rotation = time.time()

                events.append(SubjectStartRotationEvent(
                    rotate_relative=data['rotate_relative'],
                    duration=duration,
                ))

        if data.get('tile_move_to'):
            # It is already moving ?
            if self.subject.moving_to == data.get('tile_move_to'):
                # look at progression
                move_since = now - self.subject.start_move
                move_progress = move_since / self.subject.move_duration
                move_done = self.subject.move_duration * move_progress
                duration = self.subject.move_duration - move_done
                self.subject.move_duration = duration

                return [SubjectContinueTileMoveEvent(
                    move_to=data['tile_move_to'],
                    duration=duration,
                )]
            else:
                move = self.subject.intentions.get(MoveToIntention)
                move_type_duration = self.subject.get_move_duration(move)
                # FIXME: duration depend next tile type, etc
                duration = move_type_duration * 1
                self.subject.moving_to = data['tile_move_to']
                self.subject.move_duration = duration
                self.subject.start_move = time.time()
                events.append(SubjectStartTileMoveEvent(
                    move_to=data['tile_move_to'],
                    duration=duration,
                ))

        return events
