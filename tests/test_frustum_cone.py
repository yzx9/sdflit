import numpy.testing as npt
import pytest

from sdflit import FrustumCone


class TestFrustumCone:
    @pytest.mark.parametrize(
        "a, b, ra, rb, p, expected",
        [
            # fmt: off
            ((0, 0, 0), (0, 0, 2), 2, 1, (0  , 0, -1),  1  ),
            ((0, 0, 0), (0, 0, 2), 2, 1, (0  , 0,  0),  0  ),
            ((0, 0, 0), (0, 0, 2), 2, 1, (0  , 0,  1), -1  ),
            ((0, 0, 0), (0, 0, 2), 2, 1, (1.5, 0,  1),  0  ),
            ((0, 0, 0), (0, 0, 2), 2, 1, (0  , 0,  2),  0  ),
            ((0, 0, 0), (0, 0, 2), 2, 1, (0  , 0,  3),  1  ),
            # fmt: on
        ],
    )
    def test_distance(self, a, b, ra, rb, p, expected):
        frustum_cone = FrustumCone(a, b, ra, rb)
        npt.assert_allclose(frustum_cone.distance(p), expected)

    @pytest.mark.parametrize(
        "a, b, ra, rb, p, expected",
        [
            # fmt:off
            ((0, 0, 0), (0, 0, 2), 2, 1, (0  , 0, -1), False),
            ((0, 0, 0), (0, 0, 2), 2, 1, (0  , 0,  0), False),
            ((0, 0, 0), (0, 0, 2), 2, 1, (0  , 0,  1), True ),
            ((0, 0, 0), (0, 0, 2), 2, 1, (1.4, 0,  1), True ),
            ((0, 0, 0), (0, 0, 2), 2, 1, (1.5, 0,  1), False),
            ((0, 0, 0), (0, 0, 2), 2, 1, (0  , 0,  2), False),
            ((0, 0, 0), (0, 0, 2), 2, 1, (0  , 0,  3), False),
            # fmt:on
        ],
    )
    def test_inside(self, a, b, ra, rb, p, expected):
        frustum_cone = FrustumCone(a, b, ra, rb)
        npt.assert_allclose(frustum_cone.inside(p), expected)
