---
title: COLMAP
python: https://github.com/rerun-io/rerun/tree/latest/examples/python/colmap/main.py
---

![colmap example>](https://static.rerun.io/ab7edab1aabe2c80faab3f901512c7841279c488_colmap1.png)

An example using Rerun to log and visualize the output of COLMAP's sparse reconstruction.

[COLMAP](https://colmap.github.io/index.html) is a general-purpose Structure-from-Motion (SfM) and Multi-View Stereo (MVS) pipeline with a graphical and command-line interface.

In this example a short video clip has been processed offline by the COLMAP pipeline, and we use Rerun to visualize the individual camera frames, estimated camera poses, and resulting point clouds over time.


```bash
pip install -r examples/python/colmap/requirements.txt
python examples/python/colmap/main.py
```