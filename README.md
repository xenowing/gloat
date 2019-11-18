# gloat

Initial rasterizer tests for the [xenowing](https://github.com/yupferris/xenowing/) project.

## status

Currently, gloat is a basic software renderer with an OpenGL 1.2.1(ish) subset frontend. This allows us to test a full 3D rendering pipeline with some early 3D accelerated demoscene productions, particularly those by [Haujobb](https://www.pouet.net/groups.php?which=31), instead of authoring content up-front. Note that this OpenGL implementation is very barebones and the test demos don't render correctly in several ways, but this is OK, since the goal is not to support a full OpenGL implementation, but rather to nail down how the xenowing's hardware rasterizer is going to work. It's also likely that the initial hardware implementation will only cover the core rasterizer, and all triangle setup etc will be done in software. Who knows.

The rasterization algorithm is based on the classic [Pineda paper](https://www.cs.drexel.edu/~david/Classes/Papers/comp175-06-pineda.pdf).

## stuff

Note that this list isn't necessarily exhaustive, but should help me keep track of different things I'm exploring:

- [x] Texture mapping (will likely only support POT textures up to 128x128, maybe less to reduce texture cache bandwidth)
- [x] Perspective-correct s, t coords
- [x] Blending (lots of modes missing, flesh this out!)
- [ ] Proper top/left fill rule to tie-break sample coverage for polygons that share edges
- [ ] Proper view frustum clipping/culling
- [ ] There seem to be some coverage issues in some cases, with visible holes between polygons. Perhaps the triangle bounding boxes aren't conservative enough?
- [ ] Separate rasterizer model from OpenGL frontend (both for ease of further testing, but also to make it clear which parts of the project are for which purpose)
- [ ] Better traversal algorithm, so we don't sample the entire bounding box of each triangle
- [ ] Move entire rasterizer to fixed-point arithmetic
- [ ] Proper handling of back/front face and cull toggle (this will probably end up entirely as triangle setup details, and not rasterizer details)
- [ ] Tiled rendering (again, most of this ends up being triangle setup, and some smarter culling as an optimization). Needs to include explicit color/depth buffer copies from/to main memory.
- [ ] Reduce color depth (currently thinking doing everything with rgb565, and then have support for separate alpha maps. Hopefully there's not tons of overhead transferring textures in/out of cache..)
