# Ideas for 'toy-sw-renderer'

The primary motivation is obviously to implement the software renderer, which takes in some specifications of the scene and generates an image file with the rendered scene.

TODOs:
- Work out the `swrender` crate API to balance generality with ease of use in example binaries

Possible extensions beyond the course material:
- Render to some window in real-time rather than to an image file
- Have the scene be animated (e.g. rotating around Z axis)
- Allow camera control with mouse
- Do rendering on GPU with CUDA/OpenCL (might be a dumb idea, but why not)

