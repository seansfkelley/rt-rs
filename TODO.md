## features
- triangle mesh normals and texture mapping
- animate objects in scenes
- procedural textures - K
- procedural shapes - K
- procedural scenes - K
- 3D FRACTALS - K
- BRDF
- global illumination (photon mapping?) - K
- depth of field
- caustics
- subsurface scattering
- normal mapping
- attenuation
- interactivity
- displacement mapping
- Revolving beziers
- use any geometry as a light source
- water
- named objects, geometries in scenes
- Gouraud shading

## performance/quality
- spatial indexing (kd-tree, r-tree?) - K
- triangle mesh optimizations
- profile
- parallelization
- audit usages of Clone/Copy derivations and reference parameters to see if we're too copy-happy (or will rustc automatically optimize extraneous copies into moves?)
- bounding boxes should maybe always be computed relative to the world for better bounds?
- display image as it is being created

## misc
- More scenes
- add "include" directive for scene files for copy-pasting purposes
- make matrix a typedef of [[f64; 4]; 4] and then implement things for it?
