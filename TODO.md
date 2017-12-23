## features
- animate objects in scenes
- procedural textures - K
- procedural shapes - K
- procedural scenes - K
- 3D FRACTALS - K
- BRDF
- use any geometry as a light source
- global illumination (photon mapping?) - K
- depth of field
- caustics
- subsurface scattering
- normal mapping
- attenuation
- interactivity
- revolving beziers
- water
- named objects, geometries in scenes
- subdivision surfaces
- refraction: attenuation over distance (think: dark but not opaque liquid)
- rect_prism UVs

## performance/quality
- r-tree/bvh (would these be better than the kd-tree?)
- profile
- optimize & parallelize kd-tree construction (how does pbrt assign things to left/right, specifically, things that are on the splitting plane??)
- audit usages of Clone/Copy derivations and reference parameters to see if we're too copy-happy (or will rustc automatically optimize extraneous copies into moves?)
- bounding boxes should maybe always be computed relative to the world for better bounds?
- display image as it is being created

## misc
- add "include" directive for scene files for copy-pasting purposes
- make matrix a typedef of [[f64; 4]; 4] and then implement things for it?
- update rayon to 0.9.0 (version compatibility issues?)
