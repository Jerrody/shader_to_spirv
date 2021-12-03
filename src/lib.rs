pub enum ShaderType {
    Wgsl,
    Glsl,
}

pub enum ShaderStage {
    Vertex,
    Fragment,
    Compute,
}

pub struct ShaderInfo {
    pub shader_stage: ShaderStage,
    pub entry_point: String,
}

impl ShaderInfo {
    pub fn new(shader_stage: ShaderStage, entry_point: &str) -> Self {
        Self {
            shader_stage,
            entry_point: entry_point.to_string(),
        }
    }
}

pub struct ShaderOptions {
    pub validation_flags: naga::valid::ValidationFlags,
    pub capabilities: naga::valid::Capabilities,
    pub options: naga::back::spv::Options,
}

impl ShaderOptions {
    pub fn new(
        validation_flags: naga::valid::ValidationFlags,
        capabilities: naga::valid::Capabilities,
        options: naga::back::spv::Options,
    ) -> Self {
        Self {
            validation_flags,
            capabilities,
            options,
        }
    }
}

impl Default for ShaderOptions {
    fn default() -> Self {
        Self {
            validation_flags: naga::valid::ValidationFlags::all(),
            capabilities: naga::valid::Capabilities::empty(),
            options: Default::default(),
        }
    }
}

struct Spirv<P: AsRef<std::path::Path>> {
    path: P,
    shader_type: ShaderType,
    shader_info: ShaderInfo,
    shader_options: ShaderOptions,
}

impl<P: AsRef<std::path::Path>> Spirv<P> {
    fn new(path: P, shader_type: ShaderType, shader_info: ShaderInfo) -> Self {
        Self {
            path,
            shader_type,
            shader_info,
            shader_options: Default::default(),
        }
    }
}

pub struct SpirvBuilder<P: AsRef<std::path::Path>>(Spirv<P>);

impl<P: AsRef<std::path::Path>> SpirvBuilder<P> {
    pub fn new(path: P, shader_type: ShaderType, shader_info: ShaderInfo) -> Self {
        Self(Spirv::new(path, shader_type, shader_info))
    }

    /// set path to shader in `fs`
    pub fn with_path(&mut self, path: P) {
        self.0.path = path;
    }

    /// set [`ShaderType`](ShaderType)
    pub fn with_shader_type(&mut self, shader_type: ShaderType) {
        self.0.shader_type = shader_type;
    }

    /// set [`ShaderInfo`](ShaderInfo) info which includes shader stage and entry point
    pub fn with_shader_info(&mut self, shader_info: ShaderInfo) {
        self.0.shader_info = shader_info;
    }

    /// set [`ShaderOptions`](ShaderOptions)
    pub fn with_shader_options(&mut self, shader_options: ShaderOptions) {
        self.0.shader_options = shader_options;
    }

    /// moves an instance and creates SPIR-V
    pub fn build(self) -> Result<Vec<u32>, naga::back::spv::Error> {
        let shader_file = std::fs::read_to_string(self.0.path).unwrap();
        let shader_stage = match self.0.shader_info.shader_stage {
            ShaderStage::Vertex => naga::ShaderStage::Vertex,
            ShaderStage::Fragment => naga::ShaderStage::Fragment,
            ShaderStage::Compute => naga::ShaderStage::Compute,
        };

        let module = match self.0.shader_type {
            ShaderType::Wgsl => naga::front::wgsl::parse_str(&shader_file).unwrap(),
            ShaderType::Glsl => naga::front::glsl::Parser::default()
                .parse(
                    &naga::front::glsl::Options::from(shader_stage),
                    &shader_file,
                )
                .unwrap(),
        };

        let info = naga::valid::Validator::new(
            self.0.shader_options.validation_flags,
            self.0.shader_options.capabilities,
        )
        .validate(&module)
        .unwrap();

        naga::back::spv::write_vec(
            &module,
            &info,
            &naga::back::spv::Options::default(),
            Some(&naga::back::spv::PipelineOptions {
                shader_stage,
                entry_point: self.0.shader_info.entry_point,
            }),
        )
    }
}
