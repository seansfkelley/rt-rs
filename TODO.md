## probable bugs
- stuff.scene -- what are those weird reflections on both the spheres?
- fix difference geometries
- BRDF issues
  - topo-earth is too dark and not shiny enough -- is PhongMaterial working correctly?
  - refraction.scene appears to have the right geometry but the refracted straw is desaturated
  - is the translation of dpdu/dpdv into u_axis and v_axis correctly, especially given the left-handed nature of the the brdf local coordinate space?

## tests
- PointKdTree
  - test-only code to count the number of nodes traversed to ensure it is proportional to lgn — how to compile extra code and how to accurately check proportionality?
    - best way I can think to implement this is as a macro that implements the declaration, sans return type, and the body, but the macro takes four arguments: a return type, a statement to execute at the beginning of the function, a statement to execute on each iteration, and a statement to produce the return type
    - or stmt_expr_attributes

## features
- glossy specular BRDFs so Phong materials can work right
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
- transmittance (volume integration) -- does this cover traveling through refractive substances?
- area lights
- reflectance textures
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
