from dataclasses import dataclass
from itertools import permutations
from math import gcd

INPUT_DATA = [
    (-3, 15, -11),
    (3, 13, -19),
    (-13, 18, -2),
    (6, 0, -1),
]

PAIRS = list(permutations(range(4), 2))


@dataclass(frozen=False)
class Moon:
    x: int
    y: int
    z: int
    vx: int = 0
    vy: int = 0
    vz: int = 0

    def __post_init__(self):
        self._x = self.x
        self._y = self.y
        self._z = self.z

    def apply_gravity(self, o: "Moon") -> None:
        x, y, z = self.x, self.y, self.z
        ox, oy, oz = o.x, o.y, o.z
        self.vx = self.vx + 1 if x < ox else self.vx - 1 if x > ox else self.vx
        self.vy = self.vy + 1 if y < oy else self.vy - 1 if y > oy else self.vy
        self.vz = self.vz + 1 if z < oz else self.vz - 1 if z > oz else self.vz

    def apply_velocity(self) -> None:
        self.x += self.vx
        self.y += self.vy
        self.z += self.vz

    def potential_energy(self) -> int:
        return sum((abs(self.x), abs(self.y), abs(self.z)))

    def kinetic_energy(self) -> int:
        return sum((abs(self.vx), abs(self.vy), abs(self.vz)))

    def total_energy(self) -> int:
        return self.kinetic_energy() * self.potential_energy()

    def is_initial_x(self) -> bool:
        return self.x == self._x and self.vx == 0

    def is_initial_y(self) -> bool:
        return self.y == self._y and self.vy == 0

    def is_initial_z(self) -> bool:
        return self.z == self._z and self.vz == 0

    def reset(self) -> None:
        self.x = self._x
        self.y = self._y
        self.z = self._z
        self.vx = self.vy = self.vz = 0


moons = [Moon(*data) for data in INPUT_DATA]


def lcd(n1: int, n2: int) -> int:
    return abs(n1 * n2) // gcd(n1, n2)


def energy_after_n_steps(n: int) -> int:
    for _ in range(n):
        for m1, m2 in PAIRS:
            moons[m1].apply_gravity(moons[m2])
        for m in moons:
            m.apply_velocity()
    return sum(m.total_energy() for m in moons)


def steps_to_reach_initial_state():
    n = nx = ny = nz = 0
    while not (nx and ny and nz):
        for m1, m2 in PAIRS:
            moons[m1].apply_gravity(moons[m2])
        for m in moons:
            m.apply_velocity()
        n += 1
        if nx == 0 and all(m.is_initial_x() for m in moons):
            nx = n
        if ny == 0 and all(m.is_initial_y() for m in moons):
            ny = n
        if nz == 0 and all(m.is_initial_z() for m in moons):
            nz = n
    return lcd(lcd(nx, ny), nz)


def solve(_: str) -> None:
    # TODO: Parse input data instead of hardcoding it
    print(
        f"Total energy in the system after 1000 steps => {energy_after_n_steps(1000)}"
    )

    for m in moons:
        m.reset()

    print(
        "Number of steps to reach the initial state => "
        + f"{steps_to_reach_initial_state()}"
    )
