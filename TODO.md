## todo
- actually implement area lights
- transmittance (volume integration) -- does this cover traveling through refractive substances?
- remove open triangle meshes -- it's kinda weird and doesn't play well with dpdu/dpdv

## features
- animate objects in scenes
- procedural textures - K
- procedural shapes - K
- procedural scenes - K
- 3D FRACTALS - K
- global illumination (photon mapping?) - K
- depth of field
- caustics
- subsurface scattering
- normal mapping
- interactivity
- revolving beziers
- water
- named objects, geometries in scenes
- subdivision surfaces
- refraction: attenuation over distance (think: dark but not opaque liquid)
- measured BRDFs

## performance/quality
- r-tree/bvh (would these be better than the kd-tree?)
- profile
- optimize kd-tree construction (how does pbrt assign things to left/right, specifically, things that are on the splitting plane??)
- audit usages of Clone/Copy derivations and reference parameters to see if we're too copy-happy (or will rustc automatically optimize extraneous copies into moves?)
- bounding boxes should maybe always be computed relative to the world for better bounds?
- display image as it is being created
- possible optimization heuristic: on each pixel, check intersection with the nearest object from the previous pixel, if any. if some, then chop down the ray range before running it through the spatial index. this may make performance worse if you have a small number of very complex objects (i.e. topo-earth) unless you can punch an analogous optimization through to them as well, or otherwise skip them during the normal collision checking (since you already checked it).
- partial intersection in geometries
- partial intersection in kd-tree
- loading the large images for the space pictures is sloooooow
- add Geometry::transformed_bound(&self, transform) so that some shapes can maybe compute better world-space bounds

## misc
- Add pixel-matching image tests
