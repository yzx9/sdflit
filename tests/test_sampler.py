import numpy as np
import numpy.testing as npt

from sdflit import (
    ColoredMaterial,
    ObjectsScene,
    RangeSampler,
    SDFObject,
    Sphere,
    UniformSampler,
)


class TestUniformSampler:
    def test_sample(self):
        sdf = Sphere((0, 0, 0), 1).into()
        material = ColoredMaterial((1, 1, 1)).into()
        obj = SDFObject(sdf, material).into()

        scene = ObjectsScene()
        scene.add_object(obj)
        scene.set_background((0, 0, 0))

        N = 1_000_000
        sampler = UniformSampler((-1, -1, -1), (1, 1, 1))
        samples = sampler.sample(scene.into(), N)

        assert samples.shape == (1_000_000, 3)
        assert samples.dtype == np.float32

        rate = samples[..., 0].sum() / N
        npt.assert_allclose(rate, np.pi / 6, atol=5e-2)


class TestRangeSampler:
    def test_sample(self):
        sdf = Sphere((0, 0, 0), 1).into()
        material = ColoredMaterial((1, 1, 1)).into()
        obj = SDFObject(sdf, material).into()

        scene = ObjectsScene()
        scene.add_object(obj)
        scene.set_background((0, 0, 0))

        stride = (1e-2, 1e-2, 1e-2)
        rmin = (-1 + stride[0] / 2, -1 + stride[1] / 2, -1 + stride[2] / 2)
        sampler = RangeSampler(rmin, (1, 1, 1), stride)
        samples = sampler.sample(scene.into())

        assert samples.shape == (200, 200, 200, 3)
        assert samples.dtype == np.float32

        rate = samples[..., 0].sum() / (200 * 200 * 200)
        npt.assert_allclose(rate, np.pi / 6, atol=5e-2)
