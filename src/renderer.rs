pub mod context;
pub mod camera;
pub mod line;
pub mod mesh;
pub mod mesh_wireframe;
pub mod triangle;

pub use context::Context;
pub use camera::Camera;
pub use line::draw_line;
pub use mesh::{draw_mesh, draw_mesh_textured};
pub use mesh_wireframe::draw_mesh_wireframe;
pub use triangle::draw_triangle;

use triangle::{draw_3d_triangle, draw_3d_triangle_textured};
