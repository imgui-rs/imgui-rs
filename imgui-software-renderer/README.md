# Software renderer for imgui-rs

A renderer backend for imgui-rs to allow easy capture of an "dear imgui" interface to a file, without requiring any graphics hardware or complex dependencies.

Some use cases:

1. In a test case, run an application in a "headless" mode, interacting with the interface programatically. If an assertion fails, save the rendered image to a PNG file for debugging purposes.
2. In a test case, render a widget and compare it again a "known good" reference image to check a complex custom widget hasn't been altered
3. Use the renderer to automatically generate screenshots of an application or widget for use in documentation/tutorials.


## Notes/performance

Performance is not a high priority for this project.

The renderer is reasonably fast (the basic example with the "Dear ImGUI demo" nad a few other windows renders in around 6ms per frame in release mode), but this is almost entirely thanks to the speed of [`tiny_skia`](https://github.com/RazrFalcon/tiny-skia) used for rasterisation.

The renderer is inspired by [this C++ implementation](https://github.com/emilk/imgui_software_renderer/blob/master/src/imgui_sw.cpp) which contains many optimisations (e.g combining the polygons for each text character into a single square).

The primary goals of this renderer are:

1. Small, simple to follow code base
2. Consistent output - the same draw list should produce the same pixel data


## Usage

See the `examples/` directory.
