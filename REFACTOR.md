# Currently
Rewriting the 2d library and example binaries. Fixing the library code so that example `threelines` works. Moving `flip_vertically` function to `Bitmap` and re-organizing the TGA module to only have `write_tga` and `read_tga` in public API.

# Refactor TODO
## 2D library
* [x] Extract Color and Bitmap
* [x] Implement Framebuffer for Bitmap
* [ ] Make TGA into a persistence util library
* [ ] Errors for the 2D portion of the library
* [ ] Update the 2D example binaries

## 3D library
* [ ] Implement Scalar trait in math
* [ ] Rewrite math structs to be generic over Scalar implementers, not over random collections of Ops traits
* [ ] Implement homogeneous coords, only with f32
* [ ] Implement elements of data module: scene, light, geometry, camera
* [ ] Modularize triangle drawing to clearly define data flow, especially changes in math representation from normal to homogeneous coords and back
* [ ] Implement perspective projection
