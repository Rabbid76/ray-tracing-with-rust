<a href="https://stackexchange.com/users/7322082/rabbid76"><img src="https://stackexchange.com/users/flair/7322082.png" width="208" height="58" alt="profile for Rabbid76 on Stack Exchange, a network of free, community-driven Q&amp;A sites" title="profile for Rabbid76 on Stack Exchange, a network of free, community-driven Q&amp;A sites" /></a>

# Rust Ray Tracing

“Note that I avoid most “modern features” of C++, but inheritance and operator overloading are too useful for ray tracers to pass on.”  
― [Peter Shirley](https://research.nvidia.com/person/peter-shirley), [Ray Tracing in One Weekend](https://www.goodreads.com/book/show/28794030-ray-tracing-in-one-weekend)

Implemented with [Rust Programming Language](https://www.rust-lang.org/), based on [Peter Shirley's](https://research.nvidia.com/person/peter-shirley) books:

- ["Ray Tracing in One Weekend (Ray Tracing Minibooks Book 1)"](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
- ["Ray Tracing: the Next Week (Ray Tracing Minibooks Book 2)"](https://raytracing.github.io/books/RayTracingTheNextWeek.html)
- ["Ray Tracing: The Rest of Your Life (Ray Tracing Minibooks Book 3)"](https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html)

Crates:

- [Crate ray_tracing_core](https://docs.rs/ray_tracing_core/0.1.0/ray_tracing_core/)
- [Crate ray_tracing_utility](https://docs.rs/ray_tracing_utility/0.1.0/ray_tracing_utility/)
- [Crate ray_tracing_show_image](https://docs.rs/ray_tracing_show_image/0.1.0/ray_tracing_show_image/)

![cover scene - ray tracing 3](rendering/RoomGlassSphere_800x800_100000_samples.png)

## Ray Tracing in One Weekend

Materials

![materials](rendering/Materials1_800x400_10000_samples.png)

Defocus blur

![defocus  blur](rendering/DefocusBlur_800x400_10000_samples.png)

Motion blur

![motion  blur](rendering/MotionBlur_800x400_10000_samples.png)

Dielectric

![dielectric 1](rendering/MaterialDielectric1_800x400_10000_samples.png)

(Physically completely wrong, but a nice effect)  
![dielectric 2](rendering/MaterialDielectric2_800x400_10000_samples.png)

Blending material

![material blend](rendering/MaterialBlend_800x400_10000_samples.png)

Cover scene "Ray Tracing in One Weekend"

![cover scene - ray tracing 1](rendering/CoverSceneRT1_800x400_10000_samples.png)

Cover scene "Ray Tracing in One Weekend" with moving spheres

![cover scene - ray tracing 1 motion](rendering/CoverSceneRT1Motion_800x400_10000_samples.png)

Checker texture

![checker texture](rendering/TextureChecker_800x400_10000_samples.png)

Noise texture

![noise texture](rendering/TextureNoise_800x400_10000_samples.png)

Marble

![marble](rendering/TextureNoiseMarble_800x400_10000_samples.png)

Bitmap texture

![bitmap texture](rendering/TextureBitmap_800x400_10000_samples.png)

Bitmap texture with alpha mask (:construction: There is a normal vector bug on the backside of the sphere)

![bitmap texture alpha mask](rendering/TextureBitmapAlpha_800x400_10000_samples.png)

Diffuse light

![diffuse light](rendering/LightDiffuse_800x400_10000_samples.png)

Room

![room](rendering/Room_800x800_100000_samples.png)

Volume

![volume](rendering/Volume_800x800_100000_samples.png)

Mirror

![volume](rendering/RoomMirror_800x800_100000_samples.png)

Cover scene "Ray Tracing: the Next Week"

![cover scene - ray tracing 2](rendering/CoverSceneRT2_800x800_10000_samples.png)

---

Recommended

- [PeterShirley/raytracing.github.io](https://github.com/RayTracing/raytracing.github.io)
- [PeterShirley/RayTraycingInOneWeekend](https://github.com/RayTracing/InOneWeekend)
- [PeterShirley/RayTraycingTheNextWeek](https://github.com/RayTracing/TheNextWeek)
- [PeterShirley/RayTraycingTheRestOfYourLife](https://github.com/RayTracing/TheRestOfYourLife)
- [Ray Tracing in C#](https://github.com/Rabbid76/c_sharp_raytrace_examples)
- [PyGame Ray tracing](https://github.com/Rabbid76/PyGameRayTracing)