
# Performance

```lang-none
cargo doc --workspace --no-deps
```

## Bugs

normal vector on the inside of a sphere (TextureBitmapAlpha.json)  
floating point overflow in percentage computation?

## TODO

Cuboid: `pdf_value`, `random`

Batch renderer (render all scenes in a directory)

Investigate [https://github.com/JiayinCao/SORT]
The CSG rendering library [http://www.opencsg.org/]

### Scenes

...

### Serializer values

- Polar vector

### Deserializer

- Abstract the deserialization of child nodes, interface segregation
- Make it extendable for customer. Add `CustomerTexture`, `CustomerMaterial`, `CustomerHitAble` and `CustomerEnvironment`. The input to the deserialization methods should be a dictionary (`HashMap`). The deserialization classes should be registerable for the general case and for special cases depending on a type property.

### Texture

- wood texture, add scene with wooden cube/sphere?
  https://shaderfrog.com/app/view/123 Procedural Wood

### Materials

- Dielectric with attenuation by distance in the volume
- Correct color mapping of dielectric with refraction dependent on the wavelength of the light

### Shapes

- Matrix44
- Tetraeder, Hexaeder, Oktaeder, Dodekaeder, Ikosaeder 
- Sphere with thickness, use with (Ornament)
- Ray marching bodies
- AND, OR, MINUS
- 2 Sight, 3 sight body
- curvature of triangle projected on a sphere

## Ask Stack Overflow Questions

### Performance of BVH-Tree:

```rust
pub sturct BVHNode {
    pub left: Arc<dyn HitAble>,
    pub right: Arc<syn HitAble> 
}

impl HitAble for BVHNode {}
```

