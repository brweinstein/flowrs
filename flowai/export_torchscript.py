#!/usr/bin/env python3

import torch
from pathlib import Path
from stable_baselines3 import PPO


class OnnxablePolicy(torch.nn.Module):
   """
   Wraps a SB3 ActorCriticPolicy so its forward()
   can be exported to ONNX.
   """
   def __init__(self, policy: torch.nn.Module):
      super().__init__()
      self.policy = policy

   def forward(self, obs: torch.Tensor):
      # SB3 policies return (action, value, log_prob)
      action, value, _log_prob = self.policy(obs, deterministic=True)
      return action, value


def main():
   # 1) Load the exact .zip you trained
   model_path = Path("flowai/models/ppo_sudoku_9x9.zip")
   if not model_path.exists():
      raise FileNotFoundError(f"Model not found: {model_path}")
   model = PPO.load(str(model_path))
   policy: torch.nn.Module = model.policy.cpu().eval()

   # 2) Pull out the obs‐space shape (should never be None at runtime)
   obs_space = policy.observation_space  # type: ignore
   if not hasattr(obs_space, "shape") or obs_space.shape is None:
      raise RuntimeError("Policy.observation_space.shape is missing")
   height, width = obs_space.shape

   # 3) Build a dummy 1×HxW tensor
   dummy_obs = torch.zeros((1, height, width), dtype=torch.float32)

   # 4) TorchScript: trace_module returns a ScriptModule
   ts: torch.jit.ScriptModule = torch.jit.trace_module(
      policy, {"forward": dummy_obs}
   )
   ts_path = model_path.parent / "sudoku_ts.pt"
   ts.save(str(ts_path))
   print(f"TorchScript saved to {ts_path}")

   # 5) ONNX: wrap policy and export with args=(dummy_obs,)
   onnx_wrapper = OnnxablePolicy(policy).eval()
   onnx_path = model_path.parent / "sudoku.onnx"
   torch.onnx.export(
      onnx_wrapper,
      (dummy_obs,),
      str(onnx_path),
      input_names=["obs"],
      output_names=["action", "value"],
      opset_version=12,
   )
   print(f"ONNX model saved to {onnx_path}")


if __name__ == "__main__":
   main()