from sdflit import ColoredMaterial, ObjectsScene, SDFObject, Sphere


class TestScene:
    def test_hit(self):
        sdf = Sphere((0, 0, 0), 1).into()
        material = ColoredMaterial((1, 1, 1)).into()
        obj = SDFObject(sdf, material).into()
        objs = ObjectsScene()
        objs.add_object(obj)
        objs.set_background((0.5, 0.5, 0.5))
        s = objs.into()

        assert s.hit((0, 0, 0)) == (1, 1, 1)
        assert s.hit((1, 1, 1)) == (0.5, 0.5, 0.5)
