/*!
Rusty 2d graphics software renderer library.

```
// TODO
```
*/
#![doc(
html_root_url = "https://github.com/substation-beta/puny2d"
)]

// Error types.
pub mod error;
// Math extensions.
mod math;

// Types & operations for vector graphics.
pub mod vector;
// Types & operations for raster graphics (building upon vector graphics).
pub mod raster;