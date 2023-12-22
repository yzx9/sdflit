# SDF-Lit

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

Try it in Python:

```python
>>> import swc2mask
>>> swc2mask.sum_as_string(5, 20)
... 25
```

## License

[MIT](https://opensource.org/license/mit/)

Copyright (c) 2023-present, Zexin Yuan
