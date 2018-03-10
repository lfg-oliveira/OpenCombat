# coding: utf-8
import datetime
import time

from freezegun import freeze_time
from synergine2.config import Config
from synergine2_xyz.simulation import XYZSimulation
from synergine2_xyz.subjects import XYZSubject

from opencombat.simulation.move import MoveWithRotationBehaviour, \
    SubjectStartRotationEvent, SubjectStartMoveEvent, SubjectFinishRotationEvent
from opencombat.simulation.subject import TankSubject


def test_move_behaviour__begin_rotate(config):
    simulation = XYZSimulation(config)
    subject = TankSubject(
        config,
        simulation,
        position=(0, 0),
    )

    move_behaviour = MoveWithRotationBehaviour(
        config=config,
        simulation=simulation,
        subject=subject,
    )

    intention_data = {
        'from': (0, 0),
        'to': (2, 1),
        'path': [
            (0, 0),
            (1, 1),
            (2, 1),
        ],
    }

    # Rotation required to begin move
    with freeze_time("2000-01-01 00:00:00", tz_offset=0):
        data = move_behaviour.run(intention_data)
        assert {
            'rotate_relative': 45,
            'rotate_absolute': 45,
        } == data

        events = move_behaviour.action(data)
        assert events
        assert 1 == len(events)
        assert isinstance(events[0], SubjectStartRotationEvent)
        assert 45.0 == events[0].rotate_relative
        assert 4.9995 == events[0].duration
        assert subject.position == (0, 0)
        assert subject.direction == 0
        assert subject.rotate_to == 45
        assert subject.start_rotation == 946684800.0

    # This is 1 second before end of rotation
    with freeze_time("2000-01-01 00:00:04", tz_offset=0):
        data = move_behaviour.run(intention_data)
        assert not data

    # We are now just after rotation duration, a move will start
    with freeze_time("2000-01-01 00:00:05", tz_offset=0):
        data = move_behaviour.run(intention_data)
        assert {
            'begin_move_to': (1, 1),
            'rotation_finished': True,
        } == data

        events = move_behaviour.action(data)
        assert 2 == len(events)
        assert isinstance(events[1], SubjectStartMoveEvent)
        assert isinstance(events[0], SubjectFinishRotationEvent)
        assert (1, 1) == events[1].move_to
        assert 9.0 == events[1].duration
        assert subject.position == (0, 0)
        assert subject.moving_to == (1, 1)
        assert subject.move_duration == 9.0
        assert subject.start_move == 946684805.0

    # We are during the move
    # We are after the move


def test_move_behaviour__begin_move(config):
    simulation = XYZSimulation(config)
    subject = TankSubject(
        config,
        simulation,
        position=(0, 0),
    )

    move_behaviour = MoveWithRotationBehaviour(
        config=config,
        simulation=simulation,
        subject=subject,
    )

    intention_data = {
        'from': (0, 0),
        'to': (2, 1),
        'path': [
            (0, 0),
            (1, 1),
            (2, 1),
        ],
    }

    with freeze_time("2000-01-01 00:00:00", tz_offset=0):
        # First run, rotation required
        data = move_behaviour.run(intention_data)
        pass  # TODO
