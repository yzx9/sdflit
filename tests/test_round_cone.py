import numpy.testing as npt
import pytest

from sdflit import RoundCone


class TestRoundCone:
    @pytest.mark.parametrize(
        "a, b, ra, rb, p, expected",
        [
            # fmt: off
            ((0, 0, 0), (0, 0, 2), 2, 1, (0, 0, -2),  0  ),
            ((0, 0, 0), (0, 0, 2), 2, 1, (0, 0, -1), -1  ),
            ((0, 0, 0), (0, 0, 2), 2, 1, (0, 0,  0), -2  ),
            ((0, 0, 0), (0, 0, 2), 2, 1, (0, 0,  1), -1.5),
            ((0, 0, 0), (0, 0, 2), 2, 1, (0, 0,  2), -1  ),
            ((0, 0, 0), (0, 0, 2), 2, 1, (0, 0,  3),  0  ),
            ((0, 0, 0), (0, 0, 2), 2, 1, (0, 0,  4),  1  ),
            # fmt: on
        ],
    )
    def test_distance(self, a, b, ra, rb, p, expected):
        round_cone = RoundCone(a, b, ra, rb)
        npt.assert_allclose(round_cone.distance(p), expected)

    @pytest.mark.parametrize(
        "a, b, ra, rb, p, expected",
        [
            # fmt: off
            ((0, 0, 0), (0, 0, 2), 2, 1, (0, 0, -2), False),
            ((0, 0, 0), (0, 0, 2), 2, 1, (0, 0, -1), True ),
            ((0, 0, 0), (0, 0, 2), 2, 1, (0, 0,  0), True ),
            ((0, 0, 0), (0, 0, 2), 2, 1, (0, 0,  1), True ),
            ((0, 0, 0), (0, 0, 2), 2, 1, (0, 0,  2), True ),
            ((0, 0, 0), (0, 0, 2), 2, 1, (0, 0,  3), False),
            ((0, 0, 0), (0, 0, 2), 2, 1, (0, 0,  4), False),
            # fmt: on
        ],
    )
    def test_inside(self, a, b, ra, rb, p, expected):
        round_cone = RoundCone(a, b, ra, rb)
        npt.assert_allclose(round_cone.inside(p), expected)
