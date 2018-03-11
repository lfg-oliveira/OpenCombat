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
from synergine2_xyz.utils import get_angle


class SubjectStartRotationEvent(Event):
    def __init__(
        self,
        rotate_relative: float,
        duration: float,
    ) -> None:
        self.rotate_relative = rotate_relative
        self.duration = duration


class SubjectFinishRotationEvent(Event):
    pass


class SubjectStartMoveEvent(Event):
    def __init__(
        self,
        move_to: typing.Tuple[int, int],
        duration: float,
    ) -> None:
        self.move_to = move_to
        self.duration = duration


class MoveWithRotationBehaviour(SubjectBehaviour):
    def run(self, data) -> object:
        """
        FIXME: implement this:
        1. if it is end of movement: return move_to_finished: x, y
        2. if it is moving:
          2a: not finished: return moving_to: x, y
          2b: finished: fee data with tile_move_to_finished: x, y
        3. if it is rotating:
          3a: not finished: return rotate_to: x
          3b: finished: feed data with rotate_to_finished: x
        4. if next move need rotation: feed data with rotate_to: x, return data
        5. feed data with tile_move_to: x, y

        """
        from_ = data['from']  # type: typing.Tuple(int, int)
        to = data['to']  # type: typing.Tuple(int, int)
        path = data['path']  # type: typing.List[typing.Tuple(int, int)]
        path_index = path.index(self.subject.position)
        data = {}

        # Test if finish move
        if path_index == len(path) - 1:
            raise NotImplementedError('Code it, move finished')

        # Prepare data
        next_position = path[path_index + 1]
        now = time.time()

        # Test if need rotation
        next_position_direction = get_angle(self.subject.position, next_position)

        # Check if rotation is in process
        if self.subject.rotate_to == next_position_direction:
            # If it is not finished
            if self.subject.start_rotation + self.subject.rotate_duration > now:
                # Let rotation do it's job
                return None
            # rotation finish
            data['rotation_finished'] = True
        elif self.subject.direction != next_position_direction:
            return {
                'rotate_relative': next_position_direction - self.subject.direction,
                'rotate_absolute': next_position_direction,
            }

        # Begin or finish move
        # compete data dict
        # TODO: manage if movement is in process / finished

        # Subject is moving
        if self.subject.moving_to == next_position:
            if self.subject.start_move + self.subject.move_duration > now:
                # Let moving
                return None
            data['move_finished'] = True
            # TODO: Can start here a new move or rotation !
            raise NotImplementedError('TODO')

        # else start a move
        data['begin_move_to'] = next_position
        return data

    def action(self, data) -> [Event]:
        events = []

        if data.get('rotate_relative'):
            duration = self.subject.get_rotate_duration(angle=data['rotate_relative'])
            self.subject.rotate_to = data['rotate_absolute']
            self.subject.rotate_duration = duration
            self.subject.start_rotation = time.time()

            return [SubjectStartRotationEvent(
                rotate_relative=data['rotate_relative'],
                duration=duration,
            )]

        if data.get('rotation_finished'):
            self.subject.rotate_to = -1
            self.subject.rotate_duration = -1
            self.subject.start_rotation = -1
            events.append(SubjectFinishRotationEvent())

        if data.get('begin_move_to'):
            duration = self.subject.walk_duration
            self.subject.moving_to = data['begin_move_to']
            # TODO: duration must be computed
            self.subject.move_duration = duration
            self.subject.start_move = time.time()
            events.append(SubjectStartMoveEvent(
                move_to=data['begin_move_to'],
                duration=duration,
            ))
            return events

        if data.get('move_finished'):
            # TODO: Can start here a new move or rotation !
            raise NotImplementedError('TODO')

        raise NotImplementedError('This case not managed yet')
