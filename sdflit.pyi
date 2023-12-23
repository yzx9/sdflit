from typing import Tuple

__all__ = ["FrustumCone", "RoundCone", "Sphere"]

class FrustumCone:
    def __init__(
        self,
        a: Tuple[float, float, float],
        b: Tuple[float, float, float],
        ra: float,
        rb: float,
    ): ...
    def distance(self, p: Tuple[float, float, float]) -> float: ...
    def inside(self, p: Tuple[float, float, float]) -> bool: ...

class RoundCone:
    def __init__(
        self,
        a: Tuple[float, float, float],
        b: Tuple[float, float, float],
        ra: float,
        rb: float,
    ): ...
    def distance(self, p: Tuple[float, float, float]) -> float: ...
    def inside(self, p: Tuple[float, float, float]) -> bool: ...

class Sphere:
    def __init__(self, center: Tuple[float, float, float], radius: float): ...
    def distance(self, p: Tuple[float, float, float]) -> float: ...
    def inside(self, p: Tuple[float, float, float]) -> bool: ...
