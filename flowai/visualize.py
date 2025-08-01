#!/usr/bin/env python3
# visualize.py

import argparse
import time

import torch
import numpy as np

from flowai.flow_env import SudokuEnv  # or the path to your Env class


def run_episode(env, policy, delay: float):
   """Roll out one puzzle, printing each step."""
   obs, _ = env.reset()
   done = False

   while not done:
      # Console render of current grid
      env.render()
      time.sleep(delay)

      # Prepare obs → tensor: shape [1, 1, H, W]
      obs_t = torch.from_numpy(obs).float().unsqueeze(0).unsqueeze(0)

      # Forward pass, pick greedy action
      with torch.no_grad():
         logits = policy(obs_t)            # expecting shape [1, N_actions]
         action = int(logits.argmax(-1))   # scalar index

      # Step the env
      obs, reward, done, _, _ = env.step(action)

   # Final render & summary
   env.render()
   print("→ Episode done! Total reward:", np.sum(obs != 0).item(), "\n")


def main():
   p = argparse.ArgumentParser(
      description="Visualize a TorchScript policy solving Flow/River puzzles"
   )
   p.add_argument(
      "--model",
      type=str,
      required=True,
      help="Path to your scripted model (e.g. flow_policy_scripted.pt)",
   )
   p.add_argument(
      "--puzzles",
      type=str,
      default="puzzles",
      help="Directory with your .txt puzzles",
   )
   p.add_argument(
      "--n",
      type=int,
      default=3,
      help="How many puzzles to visualize",
   )
   p.add_argument(
      "--delay",
      type=float,
      default=0.5,
      help="Seconds to wait between steps",
   )
   args = p.parse_args()

   # 1) Load env & policy
   env = SudokuEnv(args.puzzles)
   policy = torch.jit.load(args.model)
   policy.eval()

   # 2) Roll out N episodes
   for i in range(args.n):
      print(f"\n=== Puzzle {i+1}/{args.n} ===")
      run_episode(env, policy, args.delay)


if __name__ == "__main__":
   main()