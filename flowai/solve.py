#!/usr/bin/env python3
# flowai/solve.py

import argparse
import numpy as np
from pathlib import Path
from gymnasium import spaces
from stable_baselines3 import PPO
from stable_baselines3.common.env_util import make_vec_env
from stable_baselines3.common.callbacks import BaseCallback
from flowai.flow_env import SudokuEnv, load_puzzles


class SizedSudokuEnv(SudokuEnv):
    """
    Filters loaded puzzles down to exactly one grid_size,
    then rebuilds obs/action spaces using gymnasium.spaces.
    """
    def __init__(self, puzzles_dir: str, grid_size: int):
        super().__init__(puzzles_dir)

        # Filter only puzzles with the requested shape
        self.puzzles = [
            (grid, meta) for grid, meta in self.puzzles
            if grid.shape[0] == grid_size
        ]
        if not self.puzzles:
            raise ValueError(
                f"No {grid_size}×{grid_size} puzzles in {puzzles_dir!r}"
            )

        # Override to the fixed size
        self.grid_size = grid_size
        self.observation_space = spaces.Box(
            low=0,
            high=self.grid_size,
            shape=(grid_size, grid_size),
            dtype=np.int32,
        )
        self.action_space = spaces.Discrete(grid_size * grid_size)


class PuzzleSolvedLogger(BaseCallback):
    """
    A callback that logs when any environment solves a puzzle
    during training rollouts.
    """
    def _on_step(self) -> bool:
        # Reads the `solved` attribute from each sub‐env
        solved_flags = self.training_env.get_attr("solved")
        if any(solved_flags):
            print("✅ Puzzle solved in this rollout!")
        return True


def train(
    puzzles_dir: Path,
    timesteps: int = 100_000,
    target_sizes: list[int] | None = None,
):
    # 1) Validate puzzles_dir
    if not puzzles_dir.is_dir():
        raise FileNotFoundError(f"No puzzles folder at {puzzles_dir!r}")

    # 2) Discover all sizes available
    raw = load_puzzles(str(puzzles_dir))
    all_sizes = sorted({grid.shape[0] for grid, _ in raw})
    print(f"Found puzzle sizes: {all_sizes}")

    # 3) Filter by user‐requested sizes (if any)
    if target_sizes:
        sizes = [s for s in target_sizes if s in all_sizes]
        missing = set(target_sizes) - set(sizes)
        if missing:
            raise ValueError(f"Requested sizes not found: {sorted(missing)}")
    else:
        sizes = all_sizes

    # 4) Train one PPO per size
    for size in sizes:
        print(f"\nTraining on {size}×{size} puzzles")
        env = make_vec_env(
            lambda: SizedSudokuEnv(str(puzzles_dir), grid_size=size),
            n_envs=1,
        )

        model = PPO("MlpPolicy", env, verbose=1)
        model.learn(
            total_timesteps=timesteps,
            callback=PuzzleSolvedLogger()
        )

        out_dir = Path(__file__).parent / "models"
        out_dir.mkdir(parents=True, exist_ok=True)
        model_path = out_dir / f"ppo_sudoku_{size}x{size}"
        model.save(str(model_path))
        print(f"Saved {size}×{size} model to {model_path}.zip")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Train PPO on Sudoku‐style puzzles of arbitrary sizes."
    )
    parser.add_argument(
        "--puzzles-dir",
        type=Path,
        default=Path(__file__).resolve().parent.parent / "puzzles",
        help="Folder with your .txt puzzle files.",
    )
    parser.add_argument(
        "--timesteps",
        type=int,
        default=200_000,
        help="Timesteps per model.",
    )
    parser.add_argument(
        "--sizes",
        type=int,
        nargs="+",
        help="Which sizes to train on (e.g. --sizes 9 12). Defaults to all found.",
    )

    args = parser.parse_args()
    train(args.puzzles_dir, timesteps=args.timesteps, target_sizes=args.sizes)