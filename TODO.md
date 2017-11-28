## features
- animate objects in scenes
- procedural textures - K
- procedural shapes - K
- 3D FRACTALS - K
- BRDF
- global illumination (photon mapping?) - K
- depth of field
- refraction (and total internal reflection) - K
- caustics
- subsurface scattering
- normal mapping
- attenuation
- interactivity
- displacement mapping
- Revolving beziers
- use any geometry as a light source
- water

## performance/quality
- bounding boxes
- spatial indexing (kd-tree, r-tree?) - K
- triangle mesh optimizations
- profile
- parallelization
- audit usages of Clone/Copy derivations and reference parameters to see if we're too copy-happy (or will rustc automatically optimize extraneous copies into moves?)

## misc
- More scenes
- add "include" directive for scene files for copy-pasting purposes
- make matrix a typedef of [[f64; 4]; 4] and then implement things for it?

## known issues
- Difference only sort of works (need better test case)
