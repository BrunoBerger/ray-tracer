# ray-tracer
For the "Advanced Computer Graphics" module<br>
[GitHub-Repository](https://github.com/BrunoBerger/ray-tracer)

## Install
1. Install rust and cargo, preferably with [rustup](https://www.rust-lang.org/tools/install)
2. In the project root, run ```cargo build --release``` <br>
This will download and compile the dependencies for random numbers and png-encoding
3. ```cargo run --release``` to build and/or run the raytracer

Optional:
- The program can also take 1 or 2 arguments to define the image resolution
e.g.: ``` cargo run --release 1000``` or ``` cargo run --release 1920 1080```
- build with ```--profile=release-with-debug``` to preserve debug-symbols


## Features
- Spheres and infinite planes
- Phong shading on diffuse materials
- Perfectly reflecting materials
- Shadows from 1 light source
- Randomly created sample scene
