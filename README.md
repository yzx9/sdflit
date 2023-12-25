# SDF-Lit

## Usage

```sh
pip install sdflit
```

Try it in Python:

```python
>>> import sdflit as sdf
>>> sphere = sdf.Sphere((0, 0, 0), 1)
>>> sphere.distance((0, 0, 0))
-1.0
>>> sphere.distance((1, 0, 0))
0.0
>>> sphere.distance((1, 1, 1))
0.7320507764816284
```

## Development

Setup Python venv:

```sh
python -m venv .env
source .env/bin/activate
pip install maturin
```

Build Python package after each modification:

```sh
maturin develop
```

## License

This work is licensed under a <a rel="license" href="https://www.apache.org/licenses/">Apache-2.0</a>.

Copyright (c) 2023-present, Zexin Yuan
