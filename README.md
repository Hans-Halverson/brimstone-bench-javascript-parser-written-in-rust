# Parser benchmark comparison between Brimstone, Ocx, and Swc

Forked from https://github.com/oxc-project/bench-javascript-parser-written-in-rust, adding support for Brimstone and removing Biome.

### typescript.js on a Mac Mini M4 Pro

As of 03/31/2025, https://github.com/Hans-Halverson/brimstone/commit/8dd0e60cf47926e9677ebe0102844918e362d131

|               | brimstone         | oxc                | swc                |
| ------------- | ----------------- | ------------------ | ------------------ |
| no-drop       | `35.7 ms` (1.00x) | `24.0 ms` (0.67x)  | `66.4 ms`  (1.86x) |
| parallel      | `57.5 ms` (1.00x) | `36.5 ms` (0.63x)  | `109.8 ms` (1.99x) |
| single-thread | `36.3 ms` (1.00x) | `23.8 ms` (0.66x)  | `72.1 ms`  (1.99x) |