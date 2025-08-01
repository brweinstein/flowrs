# flowai/flow_env.py

import gymnasium as gym
from gymnasium import spaces
import numpy as np
from pathlib import Path
from typing import Optional, Tuple, List, Dict
from collections import deque


def load_puzzles(
    dir_path: str
) -> List[Tuple[np.ndarray, Dict[str, List[Tuple[int, int]]]]]:
    """
    Parse every .txt in dir_path into:
      • grid: 2D np.ndarray of ints (0 = empty cell)
      • endpoints: dict[label] = list of (row, col) coords
    Upper-case letters mark endpoints. Dots are empties.
    """
    def parse_file(path: Path):
        lines = [l.strip() for l in path.read_text().splitlines() if l.strip()]
        n = len(lines)
        grid = np.zeros((n, n), dtype=int)
        endpoints: Dict[str, List[Tuple[int, int]]] = {}

        for r, line in enumerate(lines):
            if len(line) != n:
                raise ValueError(
                    f"{path.name}: line {r} length {len(line)}, expected {n}"
                )
            for c, ch in enumerate(line):
                if ch == '.':
                    continue
                elif ch.isalpha():
                    label = ch.upper()
                    endpoints.setdefault(label, []).append((r, c))
                else:
                    try:
                        grid[r, c] = int(ch)
                    except ValueError:
                        raise ValueError(f"Unrecognized char {ch!r} in {path.name}")

        # Ensure each label appears an even number of times
        for lbl, pts in endpoints.items():
            if len(pts) % 2 != 0:
                raise ValueError(
                    f"{path.name}: label '{lbl}' has odd count {len(pts)}"
                )

        return grid, endpoints

    puzzles: List[Tuple[np.ndarray, Dict[str, List[Tuple[int, int]]]]] = []
    for f in Path(dir_path).glob("*.txt"):
        puzzles.append(parse_file(f))

    if not puzzles:
        raise FileNotFoundError(f"No .txt puzzles found in {dir_path!r}")

    return puzzles


class SudokuEnv(gym.Env):
    """
    Gymnasium environment for Flow/River puzzles.

    Observation:
      - A square integer grid of size N×N (0 = empty cell).
    Action:
      - One of N*N discrete cells, encoded 0..(N*N-1).
    """
    metadata = {"render_modes": ["human"], "render_fps": 2}

    def __init__(self, puzzles_dir: str):
        super().__init__()
        self.puzzles = load_puzzles(puzzles_dir)

        # Derive grid size from the first puzzle
        init_grid, init_eps = self.puzzles[0]
        self.grid_size: int = init_grid.shape[0]

        # Observation: N×N grid, values 0..N
        self.observation_space = spaces.Box(
            low=0,
            high=self.grid_size,
            shape=(self.grid_size, self.grid_size),
            dtype=np.int64,
        )

        # Action: choose one of N*N cells
        self.action_space = spaces.Discrete(self.grid_size * self.grid_size)

        # Internal state
        self.current_grid: Optional[np.ndarray] = None
        self.current_eps: Dict[str, List[Tuple[int, int]]] = {}
        self.done: bool = True

    def reset(
        self,
        *,
        seed: Optional[int] = None,
        options: Optional[dict] = None
    ) -> Tuple[np.ndarray, dict]:
        super().reset(seed=seed)

        # Pick a random puzzle
        idx = np.random.randint(len(self.puzzles))
        grid, eps = self.puzzles[idx]

        self.current_grid = grid.copy()
        self.current_eps = eps
        self.done = False

        return self.current_grid.copy(), {}

    def step(self, action):
        """
        Maps action to (r,c), applies it, and returns:
        (obs, reward, terminated, truncated, info)
        """
        if self.current_grid is None:
            raise RuntimeError("Call reset() before step().")

        grid = self.current_grid
        from numpy import integer as _np_int
        if isinstance(action, (_np_int, int)):
            idx = int(action)
            r = idx // self.grid_size
            c = idx % self.grid_size
        else:
            r, c = action

        if self.done:
            raise RuntimeError("Environment already terminated; call reset().")

        # Base reward: paint empty=+1, repaint/overwrite=-1
        if grid[r, c] == 0:
            grid[r, c] = 1
            reward = 1.0
        else:
            reward = -1.0

        # Check termination: no zeros left
        terminated = not np.any(grid == 0)
        self.done = terminated

        # Extra reward shaping: correct vs. incorrect solve
        if terminated:
            if self.is_solved():
                reward += 100.0
            else:
                reward -= 20.0

        obs = grid.copy()
        truncated = False
        info: dict = {"solved": terminated and self.is_solved()}

        return obs, reward, terminated, truncated, info

    def is_solved(self) -> bool:
        """
        Returns True if:
        - All cells filled
        - Each endpoint pair is connected by BFS
        """
        if self.current_grid is None or not self.current_eps:
            return False

        if np.any(self.current_grid == 0):
            return False

        for label, endpoints in self.current_eps.items():
            if len(endpoints) != 2:
                return False
            if not self._connected(label, endpoints[0], endpoints[1]):
                return False

        return True

    def _connected(self, label: str, start: Tuple[int, int], end: Tuple[int, int]) -> bool:
        """
        BFS from start to end using same path value.
        """
        if self.current_grid is None:
            return False

        visited = set()
        queue = deque([start])
        visited.add(start)

        target_value = self.current_grid[start[0], start[1]]
        directions = [(-1, 0), (1, 0), (0, -1), (0, 1)]

        while queue:
            r, c = queue.popleft()
            if (r, c) == end:
                return True

            for dr, dc in directions:
                nr, nc = r + dr, c + dc
                if (
                    0 <= nr < self.grid_size and 
                    0 <= nc < self.grid_size and
                    (nr, nc) not in visited and
                    self.current_grid[nr, nc] == target_value
                ):
                    visited.add((nr, nc))
                    queue.append((nr, nc))

        return False

    def render(self):
        if self.current_grid is None:
            print("Call reset() before rendering.")
            return
        print(self.current_grid)


if __name__ == "__main__":
    env = SudokuEnv("puzzles")
    obs, _ = env.reset()
    print("Start grid:")
    env.render()

    obs, rew, terminated, truncated, info = env.step(0)
    print(f"\nAfter action 0 → reward={rew}, done={terminated}, info={info}")
    env.render()