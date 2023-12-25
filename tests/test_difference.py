import numpy.testing as npt
import pytest

from sdflit import Sphere, subtract


class TestMerge:
    @pytest.mark.parametrize(
        "c1, r1, c2, r2, p, expected",
        [
            # fmt:off
            ((0, 0, 0), 2, (2, 0, 0), 2, (0, 0, 0), 0),
            ((0, 0, 0), 2, (2, 0, 0), 2, (1, 0, 0), 1),
            ((0, 0, 0), 2, (2, 0, 0), 2, (2, 0, 0), 2),
            ((0, 0, 0), 2, (3, 0, 0), 2, (0, 0, 0), -1),
            ((0, 0, 0), 2, (3, 0, 0), 2, (1, 0, 0),  0),
            ((0, 0, 0), 2, (3, 0, 0), 2, (2, 0, 0),  1),
            ((0, 0, 0), 2, (3, 0, 0), 2, (3, 0, 0),  2),
            # fmt:on
        ],
    )
    def test_sphere_2(self, c1, r1, c2, r2, p, expected):
        s1 = Sphere(c1, r1)
        s2 = Sphere(c2, r2)
        sdf = subtract(s1.into(), s2.into())
        npt.assert_allclose(sdf.distance(p), expected)
