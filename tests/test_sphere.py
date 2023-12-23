import numpy as np
import numpy.testing as npt
import pytest

from sdflit import Sphere


class TestSphere:
    @pytest.mark.parametrize(
        "center, raidus, p, expected",
        [
            # fmt:off
            ((0, 0, 0), 1, (0, 0, 0), -1),
            ((0, 0, 0), 1, (1, 0, 0),  0),
            ((0, 0, 0), 1, (2, 0, 0),  1),
            ((1, 1, 1), 1, (0, 0, 0), np.sqrt(3) - 1),
            ((1, 1, 1), 1, (1, 0, 0), np.sqrt(2) - 1),
            # fmt:on
        ],
    )
    def test_distance(self, center, raidus, p, expected):
        sphere = Sphere(center, raidus)
        npt.assert_allclose(sphere.distance(p), expected)

    @pytest.mark.parametrize(
        "center, raidus, p, expected",
        [
            # fmt:off
            ((0, 0, 0), 1, (0, 0, 0), True ),
            ((0, 0, 0), 1, (1, 0, 0), False),
            ((0, 0, 0), 1, (2, 0, 0), False),
            ((1, 1, 1), 1, (0, 0, 0), False),
            ((1, 1, 1), 1, (1, 0, 0), False),
            # fmt:on
        ],
    )
    def test_inside(self, center, raidus, p, expected):
        sphere = Sphere(center, raidus)
        npt.assert_equal(sphere.inside(p), expected)
