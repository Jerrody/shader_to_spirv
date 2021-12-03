# `shader_to_spirv` usage

`shader_to_spirv` crate provides very easy way to create a SPIR-V's output.

## Examples

```rust
let vs_shader: Vec<u32> = SpirvBuilder::new(
    "../path/to/my_shader.vert",
    ShaderType::Glsl,
    ShaderInfo::new(ShaderStage::Vertex, "main"),
    )
    .unwrap();
let fs_shader: Vec<u32> = SpirvBuilder::new(
    "../path/to/my_shader.wgsl",
    ShaderType::Wgsl,
    ShaderInfo::new(ShaderStage::Fragment, "fs_main"),
    )
    .unwrap();
```

Also, you can do like this:

```rust
let vs_shader: Vec<u32> = SpirvBuilder::new(
    "../path/to/my_shader.frag",
    ShaderType::Glsl,
    ShaderInfo::new(ShaderStage::Vertex, "main"),
    )
    .with_shader_type(ShaderStage::Fragment)
    .unwrap();
```

## Supported Shader Languages
- [x] WGSL
- [x] GLSL
- [ ] HLSL
