use crate::{begin_shader_mode, end_shader_mode, get_shader_location, is_shader_valid, load_shader, load_shader_from_memory, set_shader_value, set_shader_value_matrix, set_shader_value_v, unload_shader, Color, Matrix, Texture2D, Transform};

/// Material map index
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialMapIndex {
    /// Albedo material (same as: MATERIAL_MAP_DIFFUSE)
    Albedo = 0,
    /// Metalness material (same as: MATERIAL_MAP_SPECULAR)
    Metalness,
    /// Normal material
    Normal,
    /// Roughness material
    Roughness,
    /// Ambient occlusion material
    Occlusion,
    /// Emission material
    Emission,
    /// Heightmap material
    Height,
    /// Cubemap material (NOTE: Uses GL_TEXTURE_CUBE_MAP)
    Cubemap,
    /// Irradiance material (NOTE: Uses GL_TEXTURE_CUBE_MAP)
    Irradiance,
    /// Prefilter material (NOTE: Uses GL_TEXTURE_CUBE_MAP)
    Prefilter,
    /// Brdf material
    Brdf,
}

impl MaterialMapIndex {
    #[allow(non_upper_case_globals)]
    pub const Diffuse: Self = Self::Albedo;
    #[allow(non_upper_case_globals)]
    pub const Specular: Self = Self::Metalness;
}

/// Shader location index
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderLocationIndex {
    /// Shader location: vertex attribute: position
    VertexPosition = 0,
    /// Shader location: vertex attribute: texCoord01
    VertexTexCoord01,
    /// Shader location: vertex attribute: texCoord02
    VertexTexCoord02,
    /// Shader location: vertex attribute: normal
    VertexNormal,
    /// Shader location: vertex attribute: tangent
    VertexTangent,
    /// Shader location: vertex attribute: color
    VertexColor,
    /// Shader location: matrix uniform: model-view-projection
    MatrixMvp,
    /// Shader location: matrix uniform: view (camera transform)
    MatrixView,
    /// Shader location: matrix uniform: projection
    MatrixProjection,
    /// Shader location: matrix uniform: model (transform)
    MatrixModel,
    /// Shader location: matrix uniform: normal
    MatrixNormal,
    /// Shader location: vector uniform: view
    VectorView,
    /// Shader location: vector uniform: diffuse color
    ColorDiffuse,
    /// Shader location: vector uniform: specular color
    ColorSpecular,
    /// Shader location: vector uniform: ambient color
    ColorAmbient,
    /// Shader location: sampler2d texture: albedo (same as: SHADER_LOC_MAP_DIFFUSE)
    MapAlbedo,
    /// Shader location: sampler2d texture: metalness (same as: SHADER_LOC_MAP_SPECULAR)
    MapMetalness,
    /// Shader location: sampler2d texture: normal
    MapNormal,
    /// Shader location: sampler2d texture: roughness
    MapRoughness,
    /// Shader location: sampler2d texture: occlusion
    MapOcclusion,
    /// Shader location: sampler2d texture: emission
    MapEmission,
    /// Shader location: sampler2d texture: height
    MapHeight,
    /// Shader location: samplerCube texture: cubemap
    MapCubemap,
    /// Shader location: samplerCube texture: irradiance
    MapIrradiance,
    /// Shader location: samplerCube texture: prefilter
    MapPrefilter,
    /// Shader location: sampler2d texture: brdf
    MapBrdf,
    /// Shader location: vertex attribute: boneIds
    VertexBoneIds,
    /// Shader location: vertex attribute: boneWeights
    VertexBoneWeights,
    /// Shader location: array of matrices uniform: boneMatrices
    BoneMatrices,
    /// Shader location: vertex attribute: instanceTransform
    VertexInstanceTx,
}

impl ShaderLocationIndex {
    #[allow(non_upper_case_globals)]
    pub const MapDiffuse: Self = Self::MapAlbedo;
    #[allow(non_upper_case_globals)]
    pub const MapSpecular: Self = Self::MapMetalness;
}

/// Shader uniform data type
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderUniformDataType {
    /// Shader uniform type: float
    Float = 0,
    /// Shader uniform type: vec2 (2 float)
    Vec2,
    /// Shader uniform type: vec3 (3 float)
    Vec3,
    /// Shader uniform type: vec4 (4 float)
    Vec4,
    /// Shader uniform type: int
    Int,
    /// Shader uniform type: ivec2 (2 int)
    Ivec2,
    /// Shader uniform type: ivec3 (3 int)
    Ivec3,
    /// Shader uniform type: ivec4 (4 int)
    Ivec4,
    /// Shader uniform type: unsigned int
    Uint,
    /// Shader uniform type: uivec2 (2 unsigned int)
    Uivec2,
    /// Shader uniform type: uivec3 (3 unsigned int)
    Uivec3,
    /// Shader uniform type: uivec4 (4 unsigned int)
    Uivec4,
    /// Shader uniform type: sampler2d
    Sampler2d,
}

/// Shader attribute data types
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderAttributeDataType {
    /// Shader attribute type: float
    Float = 0,
    /// Shader attribute type: vec2 (2 float)
    Vec2,
    /// Shader attribute type: vec3 (3 float)
    Vec3,
    /// Shader attribute type: vec4 (4 float)
    Vec4,
}

/// Mesh, vertex data and vao/vbo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Mesh {
    /// Number of vertices stored in arrays
    pub vertex_count: i32,
    /// Number of triangles stored (indexed or not)
    pub triangle_count: i32,

    /// Vertex and Attribute data
    /// Vertex position (XYZ - 3 components per vertex) (shader-location = 0)
    pub vertices: *mut f32,
    /// Vertex texture coordinates (UV - 2 components per vertex) (shader-location = 1)
    pub tex_coords: *mut f32,
    /// Vertex texture second coordinates (UV - 2 components per vertex) (shader-location = 5)
    pub tex_coords2: *mut f32,
    /// Vertex normals (XYZ - 3 components per vertex) (shader-location = 2)
    pub normals: *mut f32,
    /// Vertex tangents (XYZW - 4 components per vertex) (shader-location = 4)
    pub tangents: *mut f32,
    /// Vertex colors (RGBA - 4 components per vertex) (shader-location = 3)
    pub colors: *mut u8,
    /// Vertex indices (in case vertex data comes indexed)
    pub indices: *mut u16,

    /// Animation vertex data
    /// Animated vertex positions (after bones transformations)
    pub anim_vertices: *mut f32,
    /// Animated normals (after bones transformations)
    pub anim_normals: *mut f32,
    /// Vertex bone ids, max 255 bone ids, up to 4 bones influence by vertex (skinning) (shader-location = 6)
    pub bone_ids: *mut u8,
    /// Vertex bone weight, up to 4 bones influence by vertex (skinning) (shader-location = 7)
    pub bone_weights: *mut f32,
    /// Bones animated transformation matrices
    pub bone_matrices: *mut Matrix,
    /// Number of bones
    pub bone_count: i32,

    /// OpenGL identifiers
    /// OpenGL Vertex Array Object id
    pub vao_id: u32,
    /// OpenGL Vertex Buffer Objects id (default vertex data)
    pub vbo_id: *mut u32,
}

pub struct ShaderLocation(i32);

impl From<i32> for ShaderLocation {
    fn from(location: i32) -> Self {
        return Self(location);
    }
}

impl Into<i32> for ShaderLocation {
    fn into(self) -> i32 {
        return self.0;
    }
}

/// Shader
#[repr(C)]
#[derive(Debug)]
pub struct Shader {
    /// Shader program id
    pub id: u32,
    /// Shader locations array (RL_MAX_SHADER_LOCATIONS)
    pub locs: *mut u32,
}

impl Shader {
    pub fn load(vs_filename: &str, fs_filename: &str) -> Shader {
        return load_shader(vs_filename, fs_filename);
    }

    pub fn from_memory(vs_code: &str, fs_code: &str) -> Shader {
        return load_shader_from_memory(vs_code, fs_code);
    }

    pub fn begin_mode(&self) {
        begin_shader_mode(self.clone());
    }

    pub fn end_mode(&self) {
        end_shader_mode();
    }

    pub fn is_valid(&self) -> bool {
        return is_shader_valid(self.clone());
    }

    pub fn uniform_location(&self, uniform_name: &str) -> ShaderLocation {
        return get_shader_location(self.clone(), uniform_name);
    }

    pub fn attrib_location(&self, attrib_name: &str) -> ShaderLocation {
        return get_shader_location(self.clone(), attrib_name);
    }

    pub fn set<T>(&self, location: ShaderLocation, value: &T, uniform_type: ShaderUniformDataType) {
        set_shader_value(self.clone(), location, value, uniform_type);
    }

    pub fn set_vector<T>(&self, location: ShaderLocation, value: &[T], uniform_type: ShaderUniformDataType) {
        set_shader_value_v(self.clone(), location, value, uniform_type);
    }

    pub fn set_matrix(&self, location: ShaderLocation, mat: Matrix) {
        set_shader_value_matrix(self.clone(), location, mat);
    }
}

impl Clone for Shader {
    fn clone(&self) -> Self {
        return Self{ id: self.id, locs: self.locs };
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unload_shader(self.clone());
    }
}

/// MaterialMap
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MaterialMap {
    /// Material map texture
    pub texture: Texture2D,
    /// Material map color
    pub color: Color,
    /// Material map value
    pub value: f32,
}

/// Material, includes shader and maps
#[repr(C)]
#[derive(Debug)]
pub struct Material {
    /// Material shader
    pub shader: Shader,
    /// Material maps array
    pub maps: *mut MaterialMap,
    /// Material generic parameters (if required)
    pub params: [f32; 4],
}

impl Clone for Material {
    fn clone(&self) -> Self {
        return Self { shader: self.shader.clone(), maps: self.maps, params: self.params };
    }
}

/// Bone, skeletal animation bone
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BoneInfo {
    /// Bone name
    pub name: [u8; 32],
    /// Bone parent
    pub parent: i32,
}

/// Model, meshes, materials and animation data
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Model {
    /// Local transform matrix
    pub transform: Matrix,

    /// Number of meshes
    pub mesh_count: i32,
    /// Number of materials
    pub material_count: i32,
    /// Meshes array
    pub meshes: *mut Mesh,
    /// Materials array
    pub materials: *mut Material,
    /// Mesh material number
    pub mesh_material: i32,

    /// Animation data
    /// Number of bones
    pub bone_count: i32,
    /// Bones information (skeleton)
    pub bones: *mut BoneInfo,
    /// Bones base transformation (pose)
    pub bind_pose: *mut Transform,
}

/// ModelAnimation
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ModelAnimation {
    /// Number of bones
    pub bone_count: i32,
    /// Number of animation frames
    pub frame_count: i32,
    /// Bones information (skeleton)
    pub bones: *mut BoneInfo,
    /// Poses array by frame
    pub frame_poses: *mut Transform,
    /// Animation name
    pub name: [u8; 32],
}

/// Color blending modes (pre-defined)
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    /// Blend textures considering alpha (default)
    Alpha = 0,
    /// Blend textures adding colors
    Additive,
    /// Blend textures multiplying colors
    Multiplied,
    /// Blend textures adding colors (alternative)
    AddColors,
    /// Blend textures subtracting colors (alternative)
    SubtractColors,
    /// Blend premultiplied textures considering alpha
    AlphaPremultiply,
    /// Blend textures using custom src/dst factors (use rlSetBlendFactors())
    Custom,
    /// Blend textures using custom rgb/alpha separate src/dst factors (use rlSetBlendFactorsSeparate())
    CustomSeparate,
}
