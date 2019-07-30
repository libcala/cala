#![allow(dead_code, unused_parens, unused_imports)]

extern crate stdweb;

use self::stdweb::unstable::{TryFrom, TryInto};
use self::stdweb::web::html_element::CanvasElement;
use self::stdweb::web::{ArrayBuffer, RenderingContext, TypedArray};
use self::stdweb::{InstanceOf, JsSerialize, Reference, UnsafeTypedArray, Value};

type ConversionError = <Reference as TryFrom<Value>>::Error;

pub trait AsTypedArray<'a, T> {
    type Result: JsSerialize;

    unsafe fn as_typed_array(self) -> Self::Result;
}

pub trait AsArrayBufferView<'a> {
    type Result: JsSerialize;

    unsafe fn as_array_buffer_view(self) -> Self::Result;
}

pub trait Extension: TryFrom<Value> {
    const NAME: &'static str;
}

macro_rules! define_array {
    ($elem:ty) => {
        impl<'a> AsTypedArray<'a, $elem> for &'a TypedArray<$elem> {
            type Result = Self;

            unsafe fn as_typed_array(self) -> Self::Result {
                self
            }
        }

        impl<'a> AsTypedArray<'a, $elem> for &'a [$elem] {
            type Result = UnsafeTypedArray<'a, $elem>;

            unsafe fn as_typed_array(self) -> Self::Result {
                UnsafeTypedArray::new(self)
            }
        }

        impl<'a> AsArrayBufferView<'a> for &'a TypedArray<$elem> {
            type Result = Self;

            unsafe fn as_array_buffer_view(self) -> Self::Result {
                self
            }
        }

        impl<'a> AsArrayBufferView<'a> for &'a [$elem] {
            type Result = UnsafeTypedArray<'a, $elem>;

            unsafe fn as_array_buffer_view(self) -> Self::Result {
                UnsafeTypedArray::new(self)
            }
        }
    };
}

define_array!(i8);
define_array!(u8);
define_array!(i16);
define_array!(u16);
define_array!(i32);
define_array!(u32);
define_array!(f32);
define_array!(f64);

pub type Float32List = TypedArray<f32>;
pub type GLbitfield = u32;
pub type GLboolean = bool;
pub type GLbyte = i8;
pub type GLclampf = f32;
pub type GLenum = u32;
pub type GLfloat = f32;
pub type GLint = i32;
pub type GLint64 = i64;
pub type GLintptr = i64;
pub type GLshort = i16;
pub type GLsizei = i32;
pub type GLsizeiptr = i64;
pub type GLubyte = u8;
pub type GLuint = u32;
pub type GLuint64 = u64;
pub type GLushort = u16;
pub type Int32List = TypedArray<i32>;
pub type TexImageSource = Value;
pub type Uint32List = TypedArray<u32>;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WebGLPowerPreference {
    #[serde(rename = "default")]
    Default,

    #[serde(rename = "high-performance")]
    HighPerformance,

    #[serde(rename = "low-power")]
    LowPower,
}
js_deserializable!(WebGLPowerPreference);
js_serializable!(WebGLPowerPreference);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebGLContextAttributes {
    alpha: GLboolean,

    antialias: GLboolean,

    depth: GLboolean,

    #[serde(rename = "failIfMajorPerformanceCaveat")]
    fail_if_major_performance_caveat: GLboolean,

    #[serde(rename = "powerPreference")]
    power_preference: WebGLPowerPreference,

    #[serde(rename = "premultipliedAlpha")]
    premultiplied_alpha: GLboolean,

    #[serde(rename = "preserveDrawingBuffer")]
    preserve_drawing_buffer: GLboolean,

    stencil: GLboolean,
}
js_deserializable!(WebGLContextAttributes);
js_serializable!(WebGLContextAttributes);

#[derive(Debug, Clone, ReferenceType)]
pub struct GLContext(Reference);

impl GLContext {
    pub const ACTIVE_ATTRIBUTES: GLenum = 35721;
    pub const ACTIVE_TEXTURE: GLenum = 34016;
    pub const ACTIVE_UNIFORMS: GLenum = 35718;
    pub const ACTIVE_UNIFORM_BLOCKS: GLenum = 35382;
    pub const ALIASED_LINE_WIDTH_RANGE: GLenum = 33902;
    pub const ALIASED_POINT_SIZE_RANGE: GLenum = 33901;
    pub const ALPHA: GLenum = 6406;
    pub const ALPHA_BITS: GLenum = 3413;
    pub const ALREADY_SIGNALED: GLenum = 37146;
    pub const ALWAYS: GLenum = 519;
    pub const ANY_SAMPLES_PASSED: GLenum = 35887;
    pub const ANY_SAMPLES_PASSED_CONSERVATIVE: GLenum = 36202;
    pub const ARRAY_BUFFER: GLenum = 34962;
    pub const ARRAY_BUFFER_BINDING: GLenum = 34964;
    pub const ATTACHED_SHADERS: GLenum = 35717;
    pub const BACK: GLenum = 1029;
    pub const BLEND: GLenum = 3042;
    pub const BLEND_COLOR: GLenum = 32773;
    pub const BLEND_DST_ALPHA: GLenum = 32970;
    pub const BLEND_DST_RGB: GLenum = 32968;
    pub const BLEND_EQUATION: GLenum = 32777;
    pub const BLEND_EQUATION_ALPHA: GLenum = 34877;
    pub const BLEND_EQUATION_RGB: GLenum = 32777;
    pub const BLEND_SRC_ALPHA: GLenum = 32971;
    pub const BLEND_SRC_RGB: GLenum = 32969;
    pub const BLUE_BITS: GLenum = 3412;
    pub const BOOL: GLenum = 35670;
    pub const BOOL_VEC2: GLenum = 35671;
    pub const BOOL_VEC3: GLenum = 35672;
    pub const BOOL_VEC4: GLenum = 35673;
    pub const BROWSER_DEFAULT_WEBGL: GLenum = 37444;
    pub const BUFFER_SIZE: GLenum = 34660;
    pub const BUFFER_USAGE: GLenum = 34661;
    pub const BYTE: GLenum = 5120;
    pub const CCW: GLenum = 2305;
    pub const CLAMP_TO_EDGE: GLenum = 33071;
    pub const COLOR: GLenum = 6144;
    pub const COLOR_ATTACHMENT0: GLenum = 36064;
    pub const COLOR_ATTACHMENT1: GLenum = 36065;
    pub const COLOR_ATTACHMENT10: GLenum = 36074;
    pub const COLOR_ATTACHMENT11: GLenum = 36075;
    pub const COLOR_ATTACHMENT12: GLenum = 36076;
    pub const COLOR_ATTACHMENT13: GLenum = 36077;
    pub const COLOR_ATTACHMENT14: GLenum = 36078;
    pub const COLOR_ATTACHMENT15: GLenum = 36079;
    pub const COLOR_ATTACHMENT2: GLenum = 36066;
    pub const COLOR_ATTACHMENT3: GLenum = 36067;
    pub const COLOR_ATTACHMENT4: GLenum = 36068;
    pub const COLOR_ATTACHMENT5: GLenum = 36069;
    pub const COLOR_ATTACHMENT6: GLenum = 36070;
    pub const COLOR_ATTACHMENT7: GLenum = 36071;
    pub const COLOR_ATTACHMENT8: GLenum = 36072;
    pub const COLOR_ATTACHMENT9: GLenum = 36073;
    pub const COLOR_BUFFER_BIT: GLenum = 16384;
    pub const COLOR_CLEAR_VALUE: GLenum = 3106;
    pub const COLOR_WRITEMASK: GLenum = 3107;
    pub const COMPARE_REF_TO_TEXTURE: GLenum = 34894;
    pub const COMPILE_STATUS: GLenum = 35713;
    pub const COMPRESSED_TEXTURE_FORMATS: GLenum = 34467;
    pub const CONDITION_SATISFIED: GLenum = 37148;
    pub const CONSTANT_ALPHA: GLenum = 32771;
    pub const CONSTANT_COLOR: GLenum = 32769;
    pub const CONTEXT_LOST_WEBGL: GLenum = 37442;
    pub const COPY_READ_BUFFER: GLenum = 36662;
    pub const COPY_READ_BUFFER_BINDING: GLenum = 36662;
    pub const COPY_WRITE_BUFFER: GLenum = 36663;
    pub const COPY_WRITE_BUFFER_BINDING: GLenum = 36663;
    pub const CULL_FACE: GLenum = 2884;
    pub const CULL_FACE_MODE: GLenum = 2885;
    pub const CURRENT_PROGRAM: GLenum = 35725;
    pub const CURRENT_QUERY: GLenum = 34917;
    pub const CURRENT_VERTEX_ATTRIB: GLenum = 34342;
    pub const CW: GLenum = 2304;
    pub const DECR: GLenum = 7683;
    pub const DECR_WRAP: GLenum = 34056;
    pub const DELETE_STATUS: GLenum = 35712;
    pub const DEPTH: GLenum = 6145;
    pub const DEPTH24_STENCIL8: GLenum = 35056;
    pub const DEPTH32F_STENCIL8: GLenum = 36013;
    pub const DEPTH_ATTACHMENT: GLenum = 36096;
    pub const DEPTH_BITS: GLenum = 3414;
    pub const DEPTH_BUFFER_BIT: GLenum = 256;
    pub const DEPTH_CLEAR_VALUE: GLenum = 2931;
    pub const DEPTH_COMPONENT: GLenum = 6402;
    pub const DEPTH_COMPONENT16: GLenum = 33189;
    pub const DEPTH_COMPONENT24: GLenum = 33190;
    pub const DEPTH_COMPONENT32F: GLenum = 36012;
    pub const DEPTH_FUNC: GLenum = 2932;
    pub const DEPTH_RANGE: GLenum = 2928;
    pub const DEPTH_STENCIL: GLenum = 34041;
    pub const DEPTH_STENCIL_ATTACHMENT: GLenum = 33306;
    pub const DEPTH_TEST: GLenum = 2929;
    pub const DEPTH_WRITEMASK: GLenum = 2930;
    pub const DITHER: GLenum = 3024;
    pub const DONT_CARE: GLenum = 4352;
    pub const DRAW_BUFFER0: GLenum = 34853;
    pub const DRAW_BUFFER1: GLenum = 34854;
    pub const DRAW_BUFFER10: GLenum = 34863;
    pub const DRAW_BUFFER11: GLenum = 34864;
    pub const DRAW_BUFFER12: GLenum = 34865;
    pub const DRAW_BUFFER13: GLenum = 34866;
    pub const DRAW_BUFFER14: GLenum = 34867;
    pub const DRAW_BUFFER15: GLenum = 34868;
    pub const DRAW_BUFFER2: GLenum = 34855;
    pub const DRAW_BUFFER3: GLenum = 34856;
    pub const DRAW_BUFFER4: GLenum = 34857;
    pub const DRAW_BUFFER5: GLenum = 34858;
    pub const DRAW_BUFFER6: GLenum = 34859;
    pub const DRAW_BUFFER7: GLenum = 34860;
    pub const DRAW_BUFFER8: GLenum = 34861;
    pub const DRAW_BUFFER9: GLenum = 34862;
    pub const DRAW_FRAMEBUFFER: GLenum = 36009;
    pub const DRAW_FRAMEBUFFER_BINDING: GLenum = 36006;
    pub const DST_ALPHA: GLenum = 772;
    pub const DST_COLOR: GLenum = 774;
    pub const DYNAMIC_COPY: GLenum = 35050;
    pub const DYNAMIC_DRAW: GLenum = 35048;
    pub const DYNAMIC_READ: GLenum = 35049;
    pub const ELEMENT_ARRAY_BUFFER: GLenum = 34963;
    pub const ELEMENT_ARRAY_BUFFER_BINDING: GLenum = 34965;
    pub const EQUAL: GLenum = 514;
    pub const FASTEST: GLenum = 4353;
    pub const FLOAT: GLenum = 5126;
    pub const FLOAT_32_UNSIGNED_INT_24_8_REV: GLenum = 36269;
    pub const FLOAT_MAT2: GLenum = 35674;
    pub const FLOAT_MAT2X3: GLenum = 35685;
    pub const FLOAT_MAT2X4: GLenum = 35686;
    pub const FLOAT_MAT3: GLenum = 35675;
    pub const FLOAT_MAT3X2: GLenum = 35687;
    pub const FLOAT_MAT3X4: GLenum = 35688;
    pub const FLOAT_MAT4: GLenum = 35676;
    pub const FLOAT_MAT4X2: GLenum = 35689;
    pub const FLOAT_MAT4X3: GLenum = 35690;
    pub const FLOAT_VEC2: GLenum = 35664;
    pub const FLOAT_VEC3: GLenum = 35665;
    pub const FLOAT_VEC4: GLenum = 35666;
    pub const FRAGMENT_SHADER: GLenum = 35632;
    pub const FRAGMENT_SHADER_DERIVATIVE_HINT: GLenum = 35723;
    pub const FRAMEBUFFER: GLenum = 36160;
    pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: GLenum = 33301;
    pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: GLenum = 33300;
    pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: GLenum = 33296;
    pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: GLenum = 33297;
    pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: GLenum = 33302;
    pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: GLenum = 33299;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLenum = 36049;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLenum = 36048;
    pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: GLenum = 33298;
    pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: GLenum = 33303;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLenum = 36051;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: GLenum = 36052;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLenum = 36050;
    pub const FRAMEBUFFER_BINDING: GLenum = 36006;
    pub const FRAMEBUFFER_COMPLETE: GLenum = 36053;
    pub const FRAMEBUFFER_DEFAULT: GLenum = 33304;
    pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = 36054;
    pub const FRAMEBUFFER_INCOMPLETE_DIMENSIONS: GLenum = 36057;
    pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = 36055;
    pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: GLenum = 36182;
    pub const FRAMEBUFFER_UNSUPPORTED: GLenum = 36061;
    pub const FRONT: GLenum = 1028;
    pub const FRONT_AND_BACK: GLenum = 1032;
    pub const FRONT_FACE: GLenum = 2886;
    pub const FUNC_ADD: GLenum = 32774;
    pub const FUNC_REVERSE_SUBTRACT: GLenum = 32779;
    pub const FUNC_SUBTRACT: GLenum = 32778;
    pub const GENERATE_MIPMAP_HINT: GLenum = 33170;
    pub const GEQUAL: GLenum = 518;
    pub const GREATER: GLenum = 516;
    pub const GREEN_BITS: GLenum = 3411;
    pub const HALF_FLOAT: GLenum = 5131;
    pub const HIGH_FLOAT: GLenum = 36338;
    pub const HIGH_INT: GLenum = 36341;
    pub const IMPLEMENTATION_COLOR_READ_FORMAT: GLenum = 35739;
    pub const IMPLEMENTATION_COLOR_READ_TYPE: GLenum = 35738;
    pub const INCR: GLenum = 7682;
    pub const INCR_WRAP: GLenum = 34055;
    pub const INT: GLenum = 5124;
    pub const INTERLEAVED_ATTRIBS: GLenum = 35980;
    pub const INT_2_10_10_10_REV: GLenum = 36255;
    pub const INT_SAMPLER_2D: GLenum = 36298;
    pub const INT_SAMPLER_2D_ARRAY: GLenum = 36303;
    pub const INT_SAMPLER_3D: GLenum = 36299;
    pub const INT_SAMPLER_CUBE: GLenum = 36300;
    pub const INT_VEC2: GLenum = 35667;
    pub const INT_VEC3: GLenum = 35668;
    pub const INT_VEC4: GLenum = 35669;
    pub const INVALID_ENUM: GLenum = 1280;
    pub const INVALID_FRAMEBUFFER_OPERATION: GLenum = 1286;
    pub const INVALID_INDEX: GLenum = 4294967295;
    pub const INVALID_OPERATION: GLenum = 1282;
    pub const INVALID_VALUE: GLenum = 1281;
    pub const INVERT: GLenum = 5386;
    pub const KEEP: GLenum = 7680;
    pub const LEQUAL: GLenum = 515;
    pub const LESS: GLenum = 513;
    pub const LINEAR: GLenum = 9729;
    pub const LINEAR_MIPMAP_LINEAR: GLenum = 9987;
    pub const LINEAR_MIPMAP_NEAREST: GLenum = 9985;
    pub const LINES: GLenum = 1;
    pub const LINE_LOOP: GLenum = 2;
    pub const LINE_STRIP: GLenum = 3;
    pub const LINE_WIDTH: GLenum = 2849;
    pub const LINK_STATUS: GLenum = 35714;
    pub const LOW_FLOAT: GLenum = 36336;
    pub const LOW_INT: GLenum = 36339;
    pub const LUMINANCE: GLenum = 6409;
    pub const LUMINANCE_ALPHA: GLenum = 6410;
    pub const MAX: GLenum = 32776;
    pub const MAX_3D_TEXTURE_SIZE: GLenum = 32883;
    pub const MAX_ARRAY_TEXTURE_LAYERS: GLenum = 35071;
    pub const MAX_CLIENT_WAIT_TIMEOUT_WEBGL: GLenum = 37447;
    pub const MAX_COLOR_ATTACHMENTS: GLenum = 36063;
    pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 35379;
    pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = 35661;
    pub const MAX_COMBINED_UNIFORM_BLOCKS: GLenum = 35374;
    pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GLenum = 35377;
    pub const MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = 34076;
    pub const MAX_DRAW_BUFFERS: GLenum = 34852;
    pub const MAX_ELEMENTS_INDICES: GLenum = 33001;
    pub const MAX_ELEMENTS_VERTICES: GLenum = 33000;
    pub const MAX_ELEMENT_INDEX: GLenum = 36203;
    pub const MAX_FRAGMENT_INPUT_COMPONENTS: GLenum = 37157;
    pub const MAX_FRAGMENT_UNIFORM_BLOCKS: GLenum = 35373;
    pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 35657;
    pub const MAX_FRAGMENT_UNIFORM_VECTORS: GLenum = 36349;
    pub const MAX_PROGRAM_TEXEL_OFFSET: GLenum = 35077;
    pub const MAX_RENDERBUFFER_SIZE: GLenum = 34024;
    pub const MAX_SAMPLES: GLenum = 36183;
    pub const MAX_SERVER_WAIT_TIMEOUT: GLenum = 37137;
    pub const MAX_TEXTURE_IMAGE_UNITS: GLenum = 34930;
    pub const MAX_TEXTURE_LOD_BIAS: GLenum = 34045;
    pub const MAX_TEXTURE_SIZE: GLenum = 3379;
    pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: GLenum = 35978;
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: GLenum = 35979;
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: GLenum = 35968;
    pub const MAX_UNIFORM_BLOCK_SIZE: GLenum = 35376;
    pub const MAX_UNIFORM_BUFFER_BINDINGS: GLenum = 35375;
    pub const MAX_VARYING_COMPONENTS: GLenum = 35659;
    pub const MAX_VARYING_VECTORS: GLenum = 36348;
    pub const MAX_VERTEX_ATTRIBS: GLenum = 34921;
    pub const MAX_VERTEX_OUTPUT_COMPONENTS: GLenum = 37154;
    pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = 35660;
    pub const MAX_VERTEX_UNIFORM_BLOCKS: GLenum = 35371;
    pub const MAX_VERTEX_UNIFORM_COMPONENTS: GLenum = 35658;
    pub const MAX_VERTEX_UNIFORM_VECTORS: GLenum = 36347;
    pub const MAX_VIEWPORT_DIMS: GLenum = 3386;
    pub const MEDIUM_FLOAT: GLenum = 36337;
    pub const MEDIUM_INT: GLenum = 36340;
    pub const MIN: GLenum = 32775;
    pub const MIN_PROGRAM_TEXEL_OFFSET: GLenum = 35076;
    pub const MIRRORED_REPEAT: GLenum = 33648;
    pub const NEAREST: GLenum = 9728;
    pub const NEAREST_MIPMAP_LINEAR: GLenum = 9986;
    pub const NEAREST_MIPMAP_NEAREST: GLenum = 9984;
    pub const NEVER: GLenum = 512;
    pub const NICEST: GLenum = 4354;
    pub const NONE: GLenum = 0;
    pub const NOTEQUAL: GLenum = 517;
    pub const NO_ERROR: GLenum = 0;
    pub const OBJECT_TYPE: GLenum = 37138;
    pub const ONE: GLenum = 1;
    pub const ONE_MINUS_CONSTANT_ALPHA: GLenum = 32772;
    pub const ONE_MINUS_CONSTANT_COLOR: GLenum = 32770;
    pub const ONE_MINUS_DST_ALPHA: GLenum = 773;
    pub const ONE_MINUS_DST_COLOR: GLenum = 775;
    pub const ONE_MINUS_SRC_ALPHA: GLenum = 771;
    pub const ONE_MINUS_SRC_COLOR: GLenum = 769;
    pub const OUT_OF_MEMORY: GLenum = 1285;
    pub const PACK_ALIGNMENT: GLenum = 3333;
    pub const PACK_ROW_LENGTH: GLenum = 3330;
    pub const PACK_SKIP_PIXELS: GLenum = 3332;
    pub const PACK_SKIP_ROWS: GLenum = 3331;
    pub const PIXEL_PACK_BUFFER: GLenum = 35051;
    pub const PIXEL_PACK_BUFFER_BINDING: GLenum = 35053;
    pub const PIXEL_UNPACK_BUFFER: GLenum = 35052;
    pub const PIXEL_UNPACK_BUFFER_BINDING: GLenum = 35055;
    pub const POINTS: GLenum = 0;
    pub const POLYGON_OFFSET_FACTOR: GLenum = 32824;
    pub const POLYGON_OFFSET_FILL: GLenum = 32823;
    pub const POLYGON_OFFSET_UNITS: GLenum = 10752;
    pub const QUERY_RESULT: GLenum = 34918;
    pub const QUERY_RESULT_AVAILABLE: GLenum = 34919;
    pub const R11F_G11F_B10F: GLenum = 35898;
    pub const R16F: GLenum = 33325;
    pub const R16I: GLenum = 33331;
    pub const R16UI: GLenum = 33332;
    pub const R32F: GLenum = 33326;
    pub const R32I: GLenum = 33333;
    pub const R32UI: GLenum = 33334;
    pub const R8: GLenum = 33321;
    pub const R8I: GLenum = 33329;
    pub const R8UI: GLenum = 33330;
    pub const R8_SNORM: GLenum = 36756;
    pub const RASTERIZER_DISCARD: GLenum = 35977;
    pub const READ_BUFFER: GLenum = 3074;
    pub const READ_FRAMEBUFFER: GLenum = 36008;
    pub const READ_FRAMEBUFFER_BINDING: GLenum = 36010;
    pub const RED: GLenum = 6403;
    pub const RED_BITS: GLenum = 3410;
    pub const RED_INTEGER: GLenum = 36244;
    pub const RENDERBUFFER: GLenum = 36161;
    pub const RENDERBUFFER_ALPHA_SIZE: GLenum = 36179;
    pub const RENDERBUFFER_BINDING: GLenum = 36007;
    pub const RENDERBUFFER_BLUE_SIZE: GLenum = 36178;
    pub const RENDERBUFFER_DEPTH_SIZE: GLenum = 36180;
    pub const RENDERBUFFER_GREEN_SIZE: GLenum = 36177;
    pub const RENDERBUFFER_HEIGHT: GLenum = 36163;
    pub const RENDERBUFFER_INTERNAL_FORMAT: GLenum = 36164;
    pub const RENDERBUFFER_RED_SIZE: GLenum = 36176;
    pub const RENDERBUFFER_SAMPLES: GLenum = 36011;
    pub const RENDERBUFFER_STENCIL_SIZE: GLenum = 36181;
    pub const RENDERBUFFER_WIDTH: GLenum = 36162;
    pub const RENDERER: GLenum = 7937;
    pub const REPEAT: GLenum = 10497;
    pub const REPLACE: GLenum = 7681;
    pub const RG: GLenum = 33319;
    pub const RG16F: GLenum = 33327;
    pub const RG16I: GLenum = 33337;
    pub const RG16UI: GLenum = 33338;
    pub const RG32F: GLenum = 33328;
    pub const RG32I: GLenum = 33339;
    pub const RG32UI: GLenum = 33340;
    pub const RG8: GLenum = 33323;
    pub const RG8I: GLenum = 33335;
    pub const RG8UI: GLenum = 33336;
    pub const RG8_SNORM: GLenum = 36757;
    pub const RGB: GLenum = 6407;
    pub const RGB10_A2: GLenum = 32857;
    pub const RGB10_A2UI: GLenum = 36975;
    pub const RGB16F: GLenum = 34843;
    pub const RGB16I: GLenum = 36233;
    pub const RGB16UI: GLenum = 36215;
    pub const RGB32F: GLenum = 34837;
    pub const RGB32I: GLenum = 36227;
    pub const RGB32UI: GLenum = 36209;
    pub const RGB565: GLenum = 36194;
    pub const RGB5_A1: GLenum = 32855;
    pub const RGB8: GLenum = 32849;
    pub const RGB8I: GLenum = 36239;
    pub const RGB8UI: GLenum = 36221;
    pub const RGB8_SNORM: GLenum = 36758;
    pub const RGB9_E5: GLenum = 35901;
    pub const RGBA: GLenum = 6408;
    pub const RGBA16F: GLenum = 34842;
    pub const RGBA16I: GLenum = 36232;
    pub const RGBA16UI: GLenum = 36214;
    pub const RGBA32F: GLenum = 34836;
    pub const RGBA32I: GLenum = 36226;
    pub const RGBA32UI: GLenum = 36208;
    pub const RGBA4: GLenum = 32854;
    pub const RGBA8: GLenum = 32856;
    pub const RGBA8I: GLenum = 36238;
    pub const RGBA8UI: GLenum = 36220;
    pub const RGBA8_SNORM: GLenum = 36759;
    pub const RGBA_INTEGER: GLenum = 36249;
    pub const RGB_INTEGER: GLenum = 36248;
    pub const RG_INTEGER: GLenum = 33320;
    pub const SAMPLER_2D: GLenum = 35678;
    pub const SAMPLER_2D_ARRAY: GLenum = 36289;
    pub const SAMPLER_2D_ARRAY_SHADOW: GLenum = 36292;
    pub const SAMPLER_2D_SHADOW: GLenum = 35682;
    pub const SAMPLER_3D: GLenum = 35679;
    pub const SAMPLER_BINDING: GLenum = 35097;
    pub const SAMPLER_CUBE: GLenum = 35680;
    pub const SAMPLER_CUBE_SHADOW: GLenum = 36293;
    pub const SAMPLES: GLenum = 32937;
    pub const SAMPLE_ALPHA_TO_COVERAGE: GLenum = 32926;
    pub const SAMPLE_BUFFERS: GLenum = 32936;
    pub const SAMPLE_COVERAGE: GLenum = 32928;
    pub const SAMPLE_COVERAGE_INVERT: GLenum = 32939;
    pub const SAMPLE_COVERAGE_VALUE: GLenum = 32938;
    pub const SCISSOR_BOX: GLenum = 3088;
    pub const SCISSOR_TEST: GLenum = 3089;
    pub const SEPARATE_ATTRIBS: GLenum = 35981;
    pub const SHADER_TYPE: GLenum = 35663;
    pub const SHADING_LANGUAGE_VERSION: GLenum = 35724;
    pub const SHORT: GLenum = 5122;
    pub const SIGNALED: GLenum = 37145;
    pub const SIGNED_NORMALIZED: GLenum = 36764;
    pub const SRC_ALPHA: GLenum = 770;
    pub const SRC_ALPHA_SATURATE: GLenum = 776;
    pub const SRC_COLOR: GLenum = 768;
    pub const SRGB: GLenum = 35904;
    pub const SRGB8: GLenum = 35905;
    pub const SRGB8_ALPHA8: GLenum = 35907;
    pub const STATIC_COPY: GLenum = 35046;
    pub const STATIC_DRAW: GLenum = 35044;
    pub const STATIC_READ: GLenum = 35045;
    pub const STENCIL: GLenum = 6146;
    pub const STENCIL_ATTACHMENT: GLenum = 36128;
    pub const STENCIL_BACK_FAIL: GLenum = 34817;
    pub const STENCIL_BACK_FUNC: GLenum = 34816;
    pub const STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = 34818;
    pub const STENCIL_BACK_PASS_DEPTH_PASS: GLenum = 34819;
    pub const STENCIL_BACK_REF: GLenum = 36003;
    pub const STENCIL_BACK_VALUE_MASK: GLenum = 36004;
    pub const STENCIL_BACK_WRITEMASK: GLenum = 36005;
    pub const STENCIL_BITS: GLenum = 3415;
    pub const STENCIL_BUFFER_BIT: GLenum = 1024;
    pub const STENCIL_CLEAR_VALUE: GLenum = 2961;
    pub const STENCIL_FAIL: GLenum = 2964;
    pub const STENCIL_FUNC: GLenum = 2962;
    pub const STENCIL_INDEX8: GLenum = 36168;
    pub const STENCIL_PASS_DEPTH_FAIL: GLenum = 2965;
    pub const STENCIL_PASS_DEPTH_PASS: GLenum = 2966;
    pub const STENCIL_REF: GLenum = 2967;
    pub const STENCIL_TEST: GLenum = 2960;
    pub const STENCIL_VALUE_MASK: GLenum = 2963;
    pub const STENCIL_WRITEMASK: GLenum = 2968;
    pub const STREAM_COPY: GLenum = 35042;
    pub const STREAM_DRAW: GLenum = 35040;
    pub const STREAM_READ: GLenum = 35041;
    pub const SUBPIXEL_BITS: GLenum = 3408;
    pub const SYNC_CONDITION: GLenum = 37139;
    pub const SYNC_FENCE: GLenum = 37142;
    pub const SYNC_FLAGS: GLenum = 37141;
    pub const SYNC_FLUSH_COMMANDS_BIT: GLenum = 1;
    pub const SYNC_GPU_COMMANDS_COMPLETE: GLenum = 37143;
    pub const SYNC_STATUS: GLenum = 37140;
    pub const TEXTURE: GLenum = 5890;
    pub const TEXTURE0: GLenum = 33984;
    pub const TEXTURE1: GLenum = 33985;
    pub const TEXTURE10: GLenum = 33994;
    pub const TEXTURE11: GLenum = 33995;
    pub const TEXTURE12: GLenum = 33996;
    pub const TEXTURE13: GLenum = 33997;
    pub const TEXTURE14: GLenum = 33998;
    pub const TEXTURE15: GLenum = 33999;
    pub const TEXTURE16: GLenum = 34000;
    pub const TEXTURE17: GLenum = 34001;
    pub const TEXTURE18: GLenum = 34002;
    pub const TEXTURE19: GLenum = 34003;
    pub const TEXTURE2: GLenum = 33986;
    pub const TEXTURE20: GLenum = 34004;
    pub const TEXTURE21: GLenum = 34005;
    pub const TEXTURE22: GLenum = 34006;
    pub const TEXTURE23: GLenum = 34007;
    pub const TEXTURE24: GLenum = 34008;
    pub const TEXTURE25: GLenum = 34009;
    pub const TEXTURE26: GLenum = 34010;
    pub const TEXTURE27: GLenum = 34011;
    pub const TEXTURE28: GLenum = 34012;
    pub const TEXTURE29: GLenum = 34013;
    pub const TEXTURE3: GLenum = 33987;
    pub const TEXTURE30: GLenum = 34014;
    pub const TEXTURE31: GLenum = 34015;
    pub const TEXTURE4: GLenum = 33988;
    pub const TEXTURE5: GLenum = 33989;
    pub const TEXTURE6: GLenum = 33990;
    pub const TEXTURE7: GLenum = 33991;
    pub const TEXTURE8: GLenum = 33992;
    pub const TEXTURE9: GLenum = 33993;
    pub const TEXTURE_2D: GLenum = 3553;
    pub const TEXTURE_2D_ARRAY: GLenum = 35866;
    pub const TEXTURE_3D: GLenum = 32879;
    pub const TEXTURE_BASE_LEVEL: GLenum = 33084;
    pub const TEXTURE_BINDING_2D: GLenum = 32873;
    pub const TEXTURE_BINDING_2D_ARRAY: GLenum = 35869;
    pub const TEXTURE_BINDING_3D: GLenum = 32874;
    pub const TEXTURE_BINDING_CUBE_MAP: GLenum = 34068;
    pub const TEXTURE_COMPARE_FUNC: GLenum = 34893;
    pub const TEXTURE_COMPARE_MODE: GLenum = 34892;
    pub const TEXTURE_CUBE_MAP: GLenum = 34067;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = 34070;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = 34072;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = 34074;
    pub const TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = 34069;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = 34071;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = 34073;
    pub const TEXTURE_IMMUTABLE_FORMAT: GLenum = 37167;
    pub const TEXTURE_IMMUTABLE_LEVELS: GLenum = 33503;
    pub const TEXTURE_MAG_FILTER: GLenum = 10240;
    pub const TEXTURE_MAX_LEVEL: GLenum = 33085;
    pub const TEXTURE_MAX_LOD: GLenum = 33083;
    pub const TEXTURE_MIN_FILTER: GLenum = 10241;
    pub const TEXTURE_MIN_LOD: GLenum = 33082;
    pub const TEXTURE_WRAP_R: GLenum = 32882;
    pub const TEXTURE_WRAP_S: GLenum = 10242;
    pub const TEXTURE_WRAP_T: GLenum = 10243;
    pub const TIMEOUT_EXPIRED: GLenum = 37147;
    pub const TIMEOUT_IGNORED: GLint64 = -1;
    pub const TRANSFORM_FEEDBACK: GLenum = 36386;
    pub const TRANSFORM_FEEDBACK_ACTIVE: GLenum = 36388;
    pub const TRANSFORM_FEEDBACK_BINDING: GLenum = 36389;
    pub const TRANSFORM_FEEDBACK_BUFFER: GLenum = 35982;
    pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: GLenum = 35983;
    pub const TRANSFORM_FEEDBACK_BUFFER_MODE: GLenum = 35967;
    pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: GLenum = 35973;
    pub const TRANSFORM_FEEDBACK_BUFFER_START: GLenum = 35972;
    pub const TRANSFORM_FEEDBACK_PAUSED: GLenum = 36387;
    pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: GLenum = 35976;
    pub const TRANSFORM_FEEDBACK_VARYINGS: GLenum = 35971;
    pub const TRIANGLES: GLenum = 4;
    pub const TRIANGLE_FAN: GLenum = 6;
    pub const TRIANGLE_STRIP: GLenum = 5;
    pub const UNIFORM_ARRAY_STRIDE: GLenum = 35388;
    pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: GLenum = 35394;
    pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: GLenum = 35395;
    pub const UNIFORM_BLOCK_BINDING: GLenum = 35391;
    pub const UNIFORM_BLOCK_DATA_SIZE: GLenum = 35392;
    pub const UNIFORM_BLOCK_INDEX: GLenum = 35386;
    pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 35398;
    pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: GLenum = 35396;
    pub const UNIFORM_BUFFER: GLenum = 35345;
    pub const UNIFORM_BUFFER_BINDING: GLenum = 35368;
    pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: GLenum = 35380;
    pub const UNIFORM_BUFFER_SIZE: GLenum = 35370;
    pub const UNIFORM_BUFFER_START: GLenum = 35369;
    pub const UNIFORM_IS_ROW_MAJOR: GLenum = 35390;
    pub const UNIFORM_MATRIX_STRIDE: GLenum = 35389;
    pub const UNIFORM_OFFSET: GLenum = 35387;
    pub const UNIFORM_SIZE: GLenum = 35384;
    pub const UNIFORM_TYPE: GLenum = 35383;
    pub const UNPACK_ALIGNMENT: GLenum = 3317;
    pub const UNPACK_COLORSPACE_CONVERSION_WEBGL: GLenum = 37443;
    pub const UNPACK_FLIP_Y_WEBGL: GLenum = 37440;
    pub const UNPACK_IMAGE_HEIGHT: GLenum = 32878;
    pub const UNPACK_PREMULTIPLY_ALPHA_WEBGL: GLenum = 37441;
    pub const UNPACK_ROW_LENGTH: GLenum = 3314;
    pub const UNPACK_SKIP_IMAGES: GLenum = 32877;
    pub const UNPACK_SKIP_PIXELS: GLenum = 3316;
    pub const UNPACK_SKIP_ROWS: GLenum = 3315;
    pub const UNSIGNALED: GLenum = 37144;
    pub const UNSIGNED_BYTE: GLenum = 5121;
    pub const UNSIGNED_INT: GLenum = 5125;
    pub const UNSIGNED_INT_10F_11F_11F_REV: GLenum = 35899;
    pub const UNSIGNED_INT_24_8: GLenum = 34042;
    pub const UNSIGNED_INT_2_10_10_10_REV: GLenum = 33640;
    pub const UNSIGNED_INT_5_9_9_9_REV: GLenum = 35902;
    pub const UNSIGNED_INT_SAMPLER_2D: GLenum = 36306;
    pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: GLenum = 36311;
    pub const UNSIGNED_INT_SAMPLER_3D: GLenum = 36307;
    pub const UNSIGNED_INT_SAMPLER_CUBE: GLenum = 36308;
    pub const UNSIGNED_INT_VEC2: GLenum = 36294;
    pub const UNSIGNED_INT_VEC3: GLenum = 36295;
    pub const UNSIGNED_INT_VEC4: GLenum = 36296;
    pub const UNSIGNED_NORMALIZED: GLenum = 35863;
    pub const UNSIGNED_SHORT: GLenum = 5123;
    pub const UNSIGNED_SHORT_4_4_4_4: GLenum = 32819;
    pub const UNSIGNED_SHORT_5_5_5_1: GLenum = 32820;
    pub const UNSIGNED_SHORT_5_6_5: GLenum = 33635;
    pub const VALIDATE_STATUS: GLenum = 35715;
    pub const VENDOR: GLenum = 7936;
    pub const VERSION: GLenum = 7938;
    pub const VERTEX_ARRAY_BINDING: GLenum = 34229;
    pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = 34975;
    pub const VERTEX_ATTRIB_ARRAY_DIVISOR: GLenum = 35070;
    pub const VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = 34338;
    pub const VERTEX_ATTRIB_ARRAY_INTEGER: GLenum = 35069;
    pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = 34922;
    pub const VERTEX_ATTRIB_ARRAY_POINTER: GLenum = 34373;
    pub const VERTEX_ATTRIB_ARRAY_SIZE: GLenum = 34339;
    pub const VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = 34340;
    pub const VERTEX_ATTRIB_ARRAY_TYPE: GLenum = 34341;
    pub const VERTEX_SHADER: GLenum = 35633;
    pub const VIEWPORT: GLenum = 2978;
    pub const WAIT_FAILED: GLenum = 37149;
    pub const ZERO: GLenum = 0;

    pub fn active_texture(&self, texture: GLenum) {
        js!( @(no_return) @{self}.activeTexture(@{texture}); );
    }

    pub fn attach_shader(&self, program: &WebGLProgram, shader: &WebGLShader) {
        js!( @(no_return) @{self}.attachShader(@{program}, @{shader}); );
    }

    pub fn begin_query(&self, target: GLenum, query: &WebGLQuery) {
        js!( @(no_return) @{self}.beginQuery(@{target}, @{query}); );
    }

    pub fn begin_transform_feedback(&self, primitive_mode: GLenum) {
        js!( @(no_return) @{self}.beginTransformFeedback(@{primitive_mode}); );
    }

    pub fn bind_attrib_location(&self, program: &WebGLProgram, index: GLuint, name: &str) {
        js!( @(no_return) @{self}.bindAttribLocation(@{program}, @{index}, @{name}); );
    }

    pub fn bind_buffer(&self, target: GLenum, buffer: Option<&WebGLBuffer>) {
        js!( @(no_return) @{self}.bindBuffer(@{target}, @{buffer}); );
    }

    pub fn bind_buffer_base(&self, target: GLenum, index: GLuint, buffer: Option<&WebGLBuffer>) {
        js!( @(no_return) @{self}.bindBufferBase(@{target}, @{index}, @{buffer}); );
    }

    pub fn bind_buffer_range(
        &self,
        target: GLenum,
        index: GLuint,
        buffer: Option<&WebGLBuffer>,
        offset: GLintptr,
        size: GLsizeiptr,
    ) {
        js!( @(no_return) @{self}.bindBufferRange(@{target}, @{index}, @{buffer}, @{(offset as f64)}, @{(size as f64)}); );
    }

    pub fn bind_framebuffer(&self, target: GLenum, framebuffer: Option<&WebGLFramebuffer>) {
        js!( @(no_return) @{self}.bindFramebuffer(@{target}, @{framebuffer}); );
    }

    pub fn bind_renderbuffer(&self, target: GLenum, renderbuffer: Option<&WebGLRenderbuffer>) {
        js!( @(no_return) @{self}.bindRenderbuffer(@{target}, @{renderbuffer}); );
    }

    pub fn bind_sampler(&self, unit: GLuint, sampler: Option<&WebGLSampler>) {
        js!( @(no_return) @{self}.bindSampler(@{unit}, @{sampler}); );
    }

    pub fn bind_texture(&self, target: GLenum, texture: Option<&WebGLTexture>) {
        js!( @(no_return) @{self}.bindTexture(@{target}, @{texture}); );
    }

    pub fn bind_transform_feedback(&self, target: GLenum, tf: Option<&WebGLTransformFeedback>) {
        js!( @(no_return) @{self}.bindTransformFeedback(@{target}, @{tf}); );
    }

    pub fn bind_vertex_array(&self, array: Option<&WebGLVertexArrayObject>) {
        js!( @(no_return) @{self}.bindVertexArray(@{array}); );
    }

    pub fn blend_color(&self, red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
        js!( @(no_return) @{self}.blendColor(@{red}, @{green}, @{blue}, @{alpha}); );
    }

    pub fn blend_equation(&self, mode: GLenum) {
        js!( @(no_return) @{self}.blendEquation(@{mode}); );
    }

    pub fn blend_equation_separate(&self, mode_rgb: GLenum, mode_alpha: GLenum) {
        js!( @(no_return) @{self}.blendEquationSeparate(@{mode_rgb}, @{mode_alpha}); );
    }

    pub fn blend_func(&self, sfactor: GLenum, dfactor: GLenum) {
        js!( @(no_return) @{self}.blendFunc(@{sfactor}, @{dfactor}); );
    }

    pub fn blend_func_separate(
        &self,
        src_rgb: GLenum,
        dst_rgb: GLenum,
        src_alpha: GLenum,
        dst_alpha: GLenum,
    ) {
        js!( @(no_return) @{self}.blendFuncSeparate(@{src_rgb}, @{dst_rgb}, @{src_alpha}, @{dst_alpha}); );
    }

    pub fn blit_framebuffer(
        &self,
        src_x0: GLint,
        src_y0: GLint,
        src_x1: GLint,
        src_y1: GLint,
        dst_x0: GLint,
        dst_y0: GLint,
        dst_x1: GLint,
        dst_y1: GLint,
        mask: GLbitfield,
        filter: GLenum,
    ) {
        js!( @(no_return) @{self}.blitFramebuffer(@{src_x0}, @{src_y0}, @{src_x1}, @{src_y1}, @{dst_x0}, @{dst_y0}, @{dst_x1}, @{dst_y1}, @{mask}, @{filter}); );
    }

    pub fn buffer_data(&self, target: GLenum, size: GLsizeiptr, usage: GLenum) {
        js!( @(no_return) @{self}.bufferData(@{target}, @{(size as f64)}, @{usage}); );
    }

    pub fn buffer_data_1(&self, target: GLenum, src_data: Option<&ArrayBuffer>, usage: GLenum) {
        js!( @(no_return) @{self}.bufferData(@{target}, @{src_data}, @{usage}); );
    }

    pub fn buffer_data_2<'a0, T0>(
        &self,
        target: GLenum,
        src_data: T0,
        usage: GLenum,
        src_offset: GLuint,
        length: GLuint,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.bufferData(@{target}, @{unsafe { src_data.as_array_buffer_view() }}, @{usage}, @{src_offset}, @{length}); );
    }

    pub fn buffer_sub_data(
        &self,
        target: GLenum,
        dst_byte_offset: GLintptr,
        src_data: &ArrayBuffer,
    ) {
        js!( @(no_return) @{self}.bufferSubData(@{target}, @{(dst_byte_offset as f64)}, @{src_data}); );
    }

    pub fn buffer_sub_data_1<'a0, T0>(
        &self,
        target: GLenum,
        dst_byte_offset: GLintptr,
        src_data: T0,
        src_offset: GLuint,
        length: GLuint,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.bufferSubData(@{target}, @{(dst_byte_offset as f64)}, @{unsafe { src_data.as_array_buffer_view() }}, @{src_offset}, @{length}); );
    }

    pub fn canvas(&self) -> CanvasElement {
        (js! { return @{self}.canvas; }).try_into().unwrap()
    }

    pub fn check_framebuffer_status(&self, target: GLenum) -> GLenum {
        (js! { return @{self}.checkFramebufferStatus(@{target}); })
            .try_into()
            .unwrap()
    }

    pub fn clear(&self, mask: GLbitfield) {
        js!( @(no_return) @{self}.clear(@{mask}); );
    }

    pub fn clear_bufferfi(
        &self,
        buffer: GLenum,
        drawbuffer: GLint,
        depth: GLfloat,
        stencil: GLint,
    ) {
        js!( @(no_return) @{self}.clearBufferfi(@{buffer}, @{drawbuffer}, @{depth}, @{stencil}); );
    }

    pub fn clear_bufferfv<'a0, T0>(
        &self,
        buffer: GLenum,
        drawbuffer: GLint,
        values: T0,
        src_offset: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.clearBufferfv(@{buffer}, @{drawbuffer}, @{unsafe { values.as_typed_array() }}, @{src_offset}); );
    }

    pub fn clear_bufferiv<'a0, T0>(
        &self,
        buffer: GLenum,
        drawbuffer: GLint,
        values: T0,
        src_offset: GLuint,
    ) where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @(no_return) @{self}.clearBufferiv(@{buffer}, @{drawbuffer}, @{unsafe { values.as_typed_array() }}, @{src_offset}); );
    }

    pub fn clear_bufferuiv<'a0, T0>(
        &self,
        buffer: GLenum,
        drawbuffer: GLint,
        values: T0,
        src_offset: GLuint,
    ) where
        T0: AsTypedArray<'a0, u32>,
    {
        js!( @(no_return) @{self}.clearBufferuiv(@{buffer}, @{drawbuffer}, @{unsafe { values.as_typed_array() }}, @{src_offset}); );
    }

    pub fn clear_color(&self, red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
        js!( @(no_return) @{self}.clearColor(@{red}, @{green}, @{blue}, @{alpha}); );
    }

    pub fn clear_depth(&self, depth: GLclampf) {
        js!( @(no_return) @{self}.clearDepth(@{depth}); );
    }

    pub fn clear_stencil(&self, s: GLint) {
        js!( @(no_return) @{self}.clearStencil(@{s}); );
    }

    pub fn client_wait_sync(
        &self,
        sync: &WebGLSync,
        flags: GLbitfield,
        timeout: GLuint64,
    ) -> GLenum {
        (js! { return @{self}.clientWaitSync(@{sync}, @{flags}, @{(timeout as f64)}); })
            .try_into()
            .unwrap()
    }

    pub fn color_mask(&self, red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) {
        js!( @(no_return) @{self}.colorMask(@{red}, @{green}, @{blue}, @{alpha}); );
    }

    pub fn compile_shader(&self, shader: &WebGLShader) {
        js!( @(no_return) @{self}.compileShader(@{shader}); );
    }

    pub fn compressed_tex_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        image_size: GLsizei,
        offset: GLintptr,
    ) {
        js!( @(no_return) @{self}.compressedTexImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{image_size}, @{(offset as f64)}); );
    }

    pub fn compressed_tex_image2_d_1<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        src_data: T0,
        src_offset: GLuint,
        src_length_override: GLuint,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.compressedTexImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{unsafe { src_data.as_array_buffer_view() }}, @{src_offset}, @{src_length_override}); );
    }

    pub fn compressed_tex_image2_d_2<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        data: T0,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.compressedTexImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{unsafe { data.as_array_buffer_view() }}); );
    }

    pub fn compressed_tex_image3_d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        image_size: GLsizei,
        offset: GLintptr,
    ) {
        js!( @(no_return) @{self}.compressedTexImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{image_size}, @{(offset as f64)}); );
    }

    pub fn compressed_tex_image3_d_1<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        src_data: T0,
        src_offset: GLuint,
        src_length_override: GLuint,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.compressedTexImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{unsafe { src_data.as_array_buffer_view() }}, @{src_offset}, @{src_length_override}); );
    }

    pub fn compressed_tex_sub_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        image_size: GLsizei,
        offset: GLintptr,
    ) {
        js!( @(no_return) @{self}.compressedTexSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{image_size}, @{(offset as f64)}); );
    }

    pub fn compressed_tex_sub_image2_d_1<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        src_data: T0,
        src_offset: GLuint,
        src_length_override: GLuint,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.compressedTexSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{unsafe { src_data.as_array_buffer_view() }}, @{src_offset}, @{src_length_override}); );
    }

    pub fn compressed_tex_sub_image2_d_2<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        data: T0,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.compressedTexSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{unsafe { data.as_array_buffer_view() }}); );
    }

    pub fn compressed_tex_sub_image3_d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        image_size: GLsizei,
        offset: GLintptr,
    ) {
        js!( @(no_return) @{self}.compressedTexSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{width}, @{height}, @{depth}, @{format}, @{image_size}, @{(offset as f64)}); );
    }

    pub fn compressed_tex_sub_image3_d_1<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        src_data: T0,
        src_offset: GLuint,
        src_length_override: GLuint,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.compressedTexSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{width}, @{height}, @{depth}, @{format}, @{unsafe { src_data.as_array_buffer_view() }}, @{src_offset}, @{src_length_override}); );
    }

    pub fn copy_buffer_sub_data(
        &self,
        read_target: GLenum,
        write_target: GLenum,
        read_offset: GLintptr,
        write_offset: GLintptr,
        size: GLsizeiptr,
    ) {
        js!( @(no_return) @{self}.copyBufferSubData(@{read_target}, @{write_target}, @{(read_offset as f64)}, @{(write_offset as f64)}, @{(size as f64)}); );
    }

    pub fn copy_tex_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
    ) {
        js!( @(no_return) @{self}.copyTexImage2D(@{target}, @{level}, @{internalformat}, @{x}, @{y}, @{width}, @{height}, @{border}); );
    }

    pub fn copy_tex_sub_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @(no_return) @{self}.copyTexSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{x}, @{y}, @{width}, @{height}); );
    }

    pub fn copy_tex_sub_image3_d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @(no_return) @{self}.copyTexSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{x}, @{y}, @{width}, @{height}); );
    }

    pub fn create_buffer(&self) -> Option<WebGLBuffer> {
        (js! { return @{self}.createBuffer(); }).try_into().ok()
    }

    pub fn create_framebuffer(&self) -> Option<WebGLFramebuffer> {
        (js! { return @{self}.createFramebuffer(); })
            .try_into()
            .ok()
    }

    pub fn create_program(&self) -> Option<WebGLProgram> {
        (js! { return @{self}.createProgram(); }).try_into().ok()
    }

    pub fn create_query(&self) -> Option<WebGLQuery> {
        (js! { return @{self}.createQuery(); }).try_into().ok()
    }

    pub fn create_renderbuffer(&self) -> Option<WebGLRenderbuffer> {
        (js! { return @{self}.createRenderbuffer(); })
            .try_into()
            .ok()
    }

    pub fn create_sampler(&self) -> Option<WebGLSampler> {
        (js! { return @{self}.createSampler(); }).try_into().ok()
    }

    pub fn create_shader(&self, type_: GLenum) -> Option<WebGLShader> {
        (js! { return @{self}.createShader(@{type_}); })
            .try_into()
            .ok()
    }

    pub fn create_texture(&self) -> Option<WebGLTexture> {
        (js! { return @{self}.createTexture(); }).try_into().ok()
    }

    pub fn create_transform_feedback(&self) -> Option<WebGLTransformFeedback> {
        (js! { return @{self}.createTransformFeedback(); })
            .try_into()
            .ok()
    }

    pub fn create_vertex_array(&self) -> Option<WebGLVertexArrayObject> {
        (js! { return @{self}.createVertexArray(); })
            .try_into()
            .ok()
    }

    pub fn cull_face(&self, mode: GLenum) {
        js!( @(no_return) @{self}.cullFace(@{mode}); );
    }

    pub fn delete_buffer(&self, buffer: Option<&WebGLBuffer>) {
        js!( @(no_return) @{self}.deleteBuffer(@{buffer}); );
    }

    pub fn delete_framebuffer(&self, framebuffer: Option<&WebGLFramebuffer>) {
        js!( @(no_return) @{self}.deleteFramebuffer(@{framebuffer}); );
    }

    pub fn delete_program(&self, program: Option<&WebGLProgram>) {
        js!( @(no_return) @{self}.deleteProgram(@{program}); );
    }

    pub fn delete_query(&self, query: Option<&WebGLQuery>) {
        js!( @(no_return) @{self}.deleteQuery(@{query}); );
    }

    pub fn delete_renderbuffer(&self, renderbuffer: Option<&WebGLRenderbuffer>) {
        js!( @(no_return) @{self}.deleteRenderbuffer(@{renderbuffer}); );
    }

    pub fn delete_sampler(&self, sampler: Option<&WebGLSampler>) {
        js!( @(no_return) @{self}.deleteSampler(@{sampler}); );
    }

    pub fn delete_shader(&self, shader: Option<&WebGLShader>) {
        js!( @(no_return) @{self}.deleteShader(@{shader}); );
    }

    pub fn delete_sync(&self, sync: Option<&WebGLSync>) {
        js!( @(no_return) @{self}.deleteSync(@{sync}); );
    }

    pub fn delete_texture(&self, texture: Option<&WebGLTexture>) {
        js!( @(no_return) @{self}.deleteTexture(@{texture}); );
    }

    pub fn delete_transform_feedback(&self, tf: Option<&WebGLTransformFeedback>) {
        js!( @(no_return) @{self}.deleteTransformFeedback(@{tf}); );
    }

    pub fn delete_vertex_array(&self, vertex_array: Option<&WebGLVertexArrayObject>) {
        js!( @(no_return) @{self}.deleteVertexArray(@{vertex_array}); );
    }

    pub fn depth_func(&self, func: GLenum) {
        js!( @(no_return) @{self}.depthFunc(@{func}); );
    }

    pub fn depth_mask(&self, flag: GLboolean) {
        js!( @(no_return) @{self}.depthMask(@{flag}); );
    }

    pub fn depth_range(&self, z_near: GLclampf, z_far: GLclampf) {
        js!( @(no_return) @{self}.depthRange(@{z_near}, @{z_far}); );
    }

    pub fn detach_shader(&self, program: &WebGLProgram, shader: &WebGLShader) {
        js!( @(no_return) @{self}.detachShader(@{program}, @{shader}); );
    }

    pub fn disable(&self, cap: GLenum) {
        js!( @(no_return) @{self}.disable(@{cap}); );
    }

    pub fn disable_vertex_attrib_array(&self, index: GLuint) {
        js!( @(no_return) @{self}.disableVertexAttribArray(@{index}); );
    }

    pub fn draw_arrays(&self, mode: GLenum, first: GLint, count: GLsizei) {
        js!( @(no_return) @{self}.drawArrays(@{mode}, @{first}, @{count}); );
    }

    pub fn draw_arrays_instanced(
        &self,
        mode: GLenum,
        first: GLint,
        count: GLsizei,
        instance_count: GLsizei,
    ) {
        js!( @(no_return) @{self}.drawArraysInstanced(@{mode}, @{first}, @{count}, @{instance_count}); );
    }

    pub fn draw_buffers(&self, buffers: &[GLenum]) {
        js!( @(no_return) @{self}.drawBuffers(@{buffers}); );
    }

    pub fn draw_elements(&self, mode: GLenum, count: GLsizei, type_: GLenum, offset: GLintptr) {
        js!( @(no_return) @{self}.drawElements(@{mode}, @{count}, @{type_}, @{(offset as f64)}); );
    }

    pub fn draw_elements_instanced(
        &self,
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        offset: GLintptr,
        instance_count: GLsizei,
    ) {
        js!( @(no_return) @{self}.drawElementsInstanced(@{mode}, @{count}, @{type_}, @{(offset as f64)}, @{instance_count}); );
    }

    pub fn draw_range_elements(
        &self,
        mode: GLenum,
        start: GLuint,
        end: GLuint,
        count: GLsizei,
        type_: GLenum,
        offset: GLintptr,
    ) {
        js!( @(no_return) @{self}.drawRangeElements(@{mode}, @{start}, @{end}, @{count}, @{type_}, @{(offset as f64)}); );
    }

    pub fn drawing_buffer_height(&self) -> GLsizei {
        (js! { return @{self}.drawingBufferHeight; })
            .try_into()
            .unwrap()
    }

    pub fn drawing_buffer_width(&self) -> GLsizei {
        (js! { return @{self}.drawingBufferWidth; })
            .try_into()
            .unwrap()
    }

    pub fn enable(&self, cap: GLenum) {
        js!( @(no_return) @{self}.enable(@{cap}); );
    }

    pub fn enable_vertex_attrib_array(&self, index: GLuint) {
        js!( @(no_return) @{self}.enableVertexAttribArray(@{index}); );
    }

    pub fn end_query(&self, target: GLenum) {
        js!( @(no_return) @{self}.endQuery(@{target}); );
    }

    pub fn end_transform_feedback(&self) {
        js!( @(no_return) @{self}.endTransformFeedback(); );
    }

    pub fn fence_sync(&self, condition: GLenum, flags: GLbitfield) -> Option<WebGLSync> {
        (js! { return @{self}.fenceSync(@{condition}, @{flags}); })
            .try_into()
            .ok()
    }

    pub fn finish(&self) {
        js!( @(no_return) @{self}.finish(); );
    }

    pub fn flush(&self) {
        js!( @(no_return) @{self}.flush(); );
    }

    pub fn framebuffer_renderbuffer(
        &self,
        target: GLenum,
        attachment: GLenum,
        renderbuffertarget: GLenum,
        renderbuffer: Option<&WebGLRenderbuffer>,
    ) {
        js!( @(no_return) @{self}.framebufferRenderbuffer(@{target}, @{attachment}, @{renderbuffertarget}, @{renderbuffer}); );
    }

    pub fn framebuffer_texture2_d(
        &self,
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: Option<&WebGLTexture>,
        level: GLint,
    ) {
        js!( @(no_return) @{self}.framebufferTexture2D(@{target}, @{attachment}, @{textarget}, @{texture}, @{level}); );
    }

    pub fn framebuffer_texture_layer(
        &self,
        target: GLenum,
        attachment: GLenum,
        texture: Option<&WebGLTexture>,
        level: GLint,
        layer: GLint,
    ) {
        js!( @(no_return) @{self}.framebufferTextureLayer(@{target}, @{attachment}, @{texture}, @{level}, @{layer}); );
    }

    pub fn front_face(&self, mode: GLenum) {
        js!( @(no_return) @{self}.frontFace(@{mode}); );
    }

    pub fn generate_mipmap(&self, target: GLenum) {
        js!( @(no_return) @{self}.generateMipmap(@{target}); );
    }

    pub fn get_active_attrib(
        &self,
        program: &WebGLProgram,
        index: GLuint,
    ) -> Option<WebGLActiveInfo> {
        (js! { return @{self}.getActiveAttrib(@{program}, @{index}); })
            .try_into()
            .ok()
    }

    pub fn get_active_uniform(
        &self,
        program: &WebGLProgram,
        index: GLuint,
    ) -> Option<WebGLActiveInfo> {
        (js! { return @{self}.getActiveUniform(@{program}, @{index}); })
            .try_into()
            .ok()
    }

    pub fn get_active_uniform_block_name(
        &self,
        program: &WebGLProgram,
        uniform_block_index: GLuint,
    ) -> Option<String> {
        (js! { return @{self}.getActiveUniformBlockName(@{program}, @{uniform_block_index}); })
            .try_into()
            .ok()
    }

    pub fn get_active_uniform_block_parameter(
        &self,
        program: &WebGLProgram,
        uniform_block_index: GLuint,
        pname: GLenum,
    ) -> Value {
        (js! { return @{self}.getActiveUniformBlockParameter(@{program}, @{uniform_block_index}, @{pname}); } ).try_into().unwrap()
    }

    pub fn get_active_uniforms(
        &self,
        program: &WebGLProgram,
        uniform_indices: &[GLuint],
        pname: GLenum,
    ) -> Value {
        (js! { return @{self}.getActiveUniforms(@{program}, @{uniform_indices}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_attached_shaders(&self, program: &WebGLProgram) -> Option<Vec<WebGLShader>> {
        (js! { return @{self}.getAttachedShaders(@{program}); })
            .try_into()
            .ok()
    }

    pub fn get_attrib_location(&self, program: &WebGLProgram, name: &str) -> GLint {
        (js! { return @{self}.getAttribLocation(@{program}, @{name}); })
            .try_into()
            .unwrap()
    }

    pub fn get_buffer_parameter(&self, target: GLenum, pname: GLenum) -> Value {
        (js! { return @{self}.getBufferParameter(@{target}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_buffer_sub_data<'a0, T0>(
        &self,
        target: GLenum,
        src_byte_offset: GLintptr,
        dst_buffer: T0,
        dst_offset: GLuint,
        length: GLuint,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.getBufferSubData(@{target}, @{(src_byte_offset as f64)}, @{unsafe { dst_buffer.as_array_buffer_view() }}, @{dst_offset}, @{length}); );
    }

    pub fn get_context_attributes(&self) -> Option<WebGLContextAttributes> {
        (js! { return @{self}.getContextAttributes(); })
            .try_into()
            .ok()
    }

    pub fn get_error(&self) -> GLenum {
        (js! { return @{self}.getError(); }).try_into().unwrap()
    }

    pub fn get_extension<E: Extension>(&self) -> Option<E> {
        (js! { return @{self}.getExtension(@{E::NAME}); })
            .try_into()
            .ok()
    }

    pub fn get_frag_data_location(&self, program: &WebGLProgram, name: &str) -> GLint {
        (js! { return @{self}.getFragDataLocation(@{program}, @{name}); })
            .try_into()
            .unwrap()
    }

    pub fn get_framebuffer_attachment_parameter(
        &self,
        target: GLenum,
        attachment: GLenum,
        pname: GLenum,
    ) -> Value {
        (js! { return @{self}.getFramebufferAttachmentParameter(@{target}, @{attachment}, @{pname}); } ).try_into().unwrap()
    }

    pub fn get_indexed_parameter(&self, target: GLenum, index: GLuint) -> Value {
        (js! { return @{self}.getIndexedParameter(@{target}, @{index}); })
            .try_into()
            .unwrap()
    }

    pub fn get_internalformat_parameter(
        &self,
        target: GLenum,
        internalformat: GLenum,
        pname: GLenum,
    ) -> Value {
        (js! { return @{self}.getInternalformatParameter(@{target}, @{internalformat}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_parameter(&self, pname: GLenum) -> Value {
        (js! { return @{self}.getParameter(@{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_program_info_log(&self, program: &WebGLProgram) -> Option<String> {
        (js! { return @{self}.getProgramInfoLog(@{program}); })
            .try_into()
            .ok()
    }

    pub fn get_program_parameter(&self, program: &WebGLProgram, pname: GLenum) -> Value {
        (js! { return @{self}.getProgramParameter(@{program}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_query(&self, target: GLenum, pname: GLenum) -> Option<WebGLQuery> {
        (js! { return @{self}.getQuery(@{target}, @{pname}); })
            .try_into()
            .ok()
    }

    pub fn get_query_parameter(&self, query: &WebGLQuery, pname: GLenum) -> Value {
        (js! { return @{self}.getQueryParameter(@{query}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_renderbuffer_parameter(&self, target: GLenum, pname: GLenum) -> Value {
        (js! { return @{self}.getRenderbufferParameter(@{target}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_sampler_parameter(&self, sampler: &WebGLSampler, pname: GLenum) -> Value {
        (js! { return @{self}.getSamplerParameter(@{sampler}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_shader_info_log(&self, shader: &WebGLShader) -> Option<String> {
        (js! { return @{self}.getShaderInfoLog(@{shader}); })
            .try_into()
            .ok()
    }

    pub fn get_shader_parameter(&self, shader: &WebGLShader, pname: GLenum) -> Value {
        (js! { return @{self}.getShaderParameter(@{shader}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_shader_precision_format(
        &self,
        shadertype: GLenum,
        precisiontype: GLenum,
    ) -> Option<WebGLShaderPrecisionFormat> {
        (js! { return @{self}.getShaderPrecisionFormat(@{shadertype}, @{precisiontype}); })
            .try_into()
            .ok()
    }

    pub fn get_shader_source(&self, shader: &WebGLShader) -> Option<String> {
        (js! { return @{self}.getShaderSource(@{shader}); })
            .try_into()
            .ok()
    }

    pub fn get_supported_extensions(&self) -> Option<Vec<String>> {
        (js! { return @{self}.getSupportedExtensions(); })
            .try_into()
            .ok()
    }

    pub fn get_sync_parameter(&self, sync: &WebGLSync, pname: GLenum) -> Value {
        (js! { return @{self}.getSyncParameter(@{sync}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_tex_parameter(&self, target: GLenum, pname: GLenum) -> Value {
        (js! { return @{self}.getTexParameter(@{target}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_transform_feedback_varying(
        &self,
        program: &WebGLProgram,
        index: GLuint,
    ) -> Option<WebGLActiveInfo> {
        (js! { return @{self}.getTransformFeedbackVarying(@{program}, @{index}); })
            .try_into()
            .ok()
    }

    pub fn get_uniform(&self, program: &WebGLProgram, location: &WebGLUniformLocation) -> Value {
        (js! { return @{self}.getUniform(@{program}, @{location}); })
            .try_into()
            .unwrap()
    }

    pub fn get_uniform_block_index(
        &self,
        program: &WebGLProgram,
        uniform_block_name: &str,
    ) -> GLuint {
        (js! { return @{self}.getUniformBlockIndex(@{program}, @{uniform_block_name}); })
            .try_into()
            .unwrap()
    }

    pub fn get_uniform_indices(
        &self,
        program: &WebGLProgram,
        uniform_names: &[&str],
    ) -> Option<Vec<GLuint>> {
        (js! { return @{self}.getUniformIndices(@{program}, @{uniform_names}); })
            .try_into()
            .ok()
    }

    pub fn get_uniform_location(
        &self,
        program: &WebGLProgram,
        name: &str,
    ) -> Option<WebGLUniformLocation> {
        (js! { return @{self}.getUniformLocation(@{program}, @{name}); })
            .try_into()
            .ok()
    }

    pub fn get_vertex_attrib(&self, index: GLuint, pname: GLenum) -> Value {
        (js! { return @{self}.getVertexAttrib(@{index}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_vertex_attrib_offset(&self, index: GLuint, pname: GLenum) -> GLintptr {
        (js! { return @{self}.getVertexAttribOffset(@{index}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn hint(&self, target: GLenum, mode: GLenum) {
        js!( @(no_return) @{self}.hint(@{target}, @{mode}); );
    }

    pub fn invalidate_framebuffer(&self, target: GLenum, attachments: &[GLenum]) {
        js!( @(no_return) @{self}.invalidateFramebuffer(@{target}, @{attachments}); );
    }

    pub fn invalidate_sub_framebuffer(
        &self,
        target: GLenum,
        attachments: &[GLenum],
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @(no_return) @{self}.invalidateSubFramebuffer(@{target}, @{attachments}, @{x}, @{y}, @{width}, @{height}); );
    }

    pub fn is_buffer(&self, buffer: Option<&WebGLBuffer>) -> GLboolean {
        (js! { return @{self}.isBuffer(@{buffer}); })
            .try_into()
            .unwrap()
    }

    pub fn is_context_lost(&self) -> bool {
        (js! { return @{self}.isContextLost(); })
            .try_into()
            .unwrap()
    }

    pub fn is_enabled(&self, cap: GLenum) -> GLboolean {
        (js! { return @{self}.isEnabled(@{cap}); })
            .try_into()
            .unwrap()
    }

    pub fn is_framebuffer(&self, framebuffer: Option<&WebGLFramebuffer>) -> GLboolean {
        (js! { return @{self}.isFramebuffer(@{framebuffer}); })
            .try_into()
            .unwrap()
    }

    pub fn is_program(&self, program: Option<&WebGLProgram>) -> GLboolean {
        (js! { return @{self}.isProgram(@{program}); })
            .try_into()
            .unwrap()
    }

    pub fn is_query(&self, query: Option<&WebGLQuery>) -> GLboolean {
        (js! { return @{self}.isQuery(@{query}); })
            .try_into()
            .unwrap()
    }

    pub fn is_renderbuffer(&self, renderbuffer: Option<&WebGLRenderbuffer>) -> GLboolean {
        (js! { return @{self}.isRenderbuffer(@{renderbuffer}); })
            .try_into()
            .unwrap()
    }

    pub fn is_sampler(&self, sampler: Option<&WebGLSampler>) -> GLboolean {
        (js! { return @{self}.isSampler(@{sampler}); })
            .try_into()
            .unwrap()
    }

    pub fn is_shader(&self, shader: Option<&WebGLShader>) -> GLboolean {
        (js! { return @{self}.isShader(@{shader}); })
            .try_into()
            .unwrap()
    }

    pub fn is_sync(&self, sync: Option<&WebGLSync>) -> GLboolean {
        (js! { return @{self}.isSync(@{sync}); })
            .try_into()
            .unwrap()
    }

    pub fn is_texture(&self, texture: Option<&WebGLTexture>) -> GLboolean {
        (js! { return @{self}.isTexture(@{texture}); })
            .try_into()
            .unwrap()
    }

    pub fn is_transform_feedback(&self, tf: Option<&WebGLTransformFeedback>) -> GLboolean {
        (js! { return @{self}.isTransformFeedback(@{tf}); })
            .try_into()
            .unwrap()
    }

    pub fn is_vertex_array(&self, vertex_array: Option<&WebGLVertexArrayObject>) -> GLboolean {
        (js! { return @{self}.isVertexArray(@{vertex_array}); })
            .try_into()
            .unwrap()
    }

    pub fn line_width(&self, width: GLfloat) {
        js!( @(no_return) @{self}.lineWidth(@{width}); );
    }

    pub fn link_program(&self, program: &WebGLProgram) {
        js!( @(no_return) @{self}.linkProgram(@{program}); );
    }

    pub fn pause_transform_feedback(&self) {
        js!( @(no_return) @{self}.pauseTransformFeedback(); );
    }

    pub fn pixel_storei(&self, pname: GLenum, param: GLint) {
        js!( @(no_return) @{self}.pixelStorei(@{pname}, @{param}); );
    }

    pub fn polygon_offset(&self, factor: GLfloat, units: GLfloat) {
        js!( @(no_return) @{self}.polygonOffset(@{factor}, @{units}); );
    }

    pub fn read_buffer(&self, src: GLenum) {
        js!( @(no_return) @{self}.readBuffer(@{src}); );
    }

    pub fn read_pixels<'a0, T0>(
        &self,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        dst_data: Option<T0>,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.readPixels(@{x}, @{y}, @{width}, @{height}, @{format}, @{type_}, @{dst_data.map(|inner| unsafe { inner.as_array_buffer_view() })}); );
    }

    pub fn read_pixels_1(
        &self,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        offset: GLintptr,
    ) {
        js!( @(no_return) @{self}.readPixels(@{x}, @{y}, @{width}, @{height}, @{format}, @{type_}, @{(offset as f64)}); );
    }

    pub fn read_pixels_2<'a0, T0>(
        &self,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        dst_data: T0,
        dst_offset: GLuint,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.readPixels(@{x}, @{y}, @{width}, @{height}, @{format}, @{type_}, @{unsafe { dst_data.as_array_buffer_view() }}, @{dst_offset}); );
    }

    pub fn renderbuffer_storage(
        &self,
        target: GLenum,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @(no_return) @{self}.renderbufferStorage(@{target}, @{internalformat}, @{width}, @{height}); );
    }

    pub fn renderbuffer_storage_multisample(
        &self,
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @(no_return) @{self}.renderbufferStorageMultisample(@{target}, @{samples}, @{internalformat}, @{width}, @{height}); );
    }

    pub fn resume_transform_feedback(&self) {
        js!( @(no_return) @{self}.resumeTransformFeedback(); );
    }

    pub fn sample_coverage(&self, value: GLclampf, invert: GLboolean) {
        js!( @(no_return) @{self}.sampleCoverage(@{value}, @{invert}); );
    }

    pub fn sampler_parameterf(&self, sampler: &WebGLSampler, pname: GLenum, param: GLfloat) {
        js!( @(no_return) @{self}.samplerParameterf(@{sampler}, @{pname}, @{param}); );
    }

    pub fn sampler_parameteri(&self, sampler: &WebGLSampler, pname: GLenum, param: GLint) {
        js!( @(no_return) @{self}.samplerParameteri(@{sampler}, @{pname}, @{param}); );
    }

    pub fn scissor(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        js!( @(no_return) @{self}.scissor(@{x}, @{y}, @{width}, @{height}); );
    }

    pub fn shader_source(&self, shader: &WebGLShader, source: &str) {
        js!( @(no_return) @{self}.shaderSource(@{shader}, @{source}); );
    }

    pub fn stencil_func(&self, func: GLenum, ref_: GLint, mask: GLuint) {
        js!( @(no_return) @{self}.stencilFunc(@{func}, @{ref_}, @{mask}); );
    }

    pub fn stencil_func_separate(&self, face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) {
        js!( @(no_return) @{self}.stencilFuncSeparate(@{face}, @{func}, @{ref_}, @{mask}); );
    }

    pub fn stencil_mask(&self, mask: GLuint) {
        js!( @(no_return) @{self}.stencilMask(@{mask}); );
    }

    pub fn stencil_mask_separate(&self, face: GLenum, mask: GLuint) {
        js!( @(no_return) @{self}.stencilMaskSeparate(@{face}, @{mask}); );
    }

    pub fn stencil_op(&self, fail: GLenum, zfail: GLenum, zpass: GLenum) {
        js!( @(no_return) @{self}.stencilOp(@{fail}, @{zfail}, @{zpass}); );
    }

    pub fn stencil_op_separate(&self, face: GLenum, fail: GLenum, zfail: GLenum, zpass: GLenum) {
        js!( @(no_return) @{self}.stencilOpSeparate(@{face}, @{fail}, @{zfail}, @{zpass}); );
    }

    pub fn tex_image2_d<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: Option<T0>,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{format}, @{type_}, @{pixels.map(|inner| unsafe { inner.as_array_buffer_view() })}); );
    }

    pub fn tex_image2_d_1<T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @(no_return) @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_image2_d_2(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pbo_offset: GLintptr,
    ) {
        js!( @(no_return) @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{format}, @{type_}, @{(pbo_offset as f64)}); );
    }

    pub fn tex_image2_d_3<T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @(no_return) @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_image2_d_4<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        src_data: T0,
        src_offset: GLuint,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{format}, @{type_}, @{unsafe { src_data.as_array_buffer_view() }}, @{src_offset}); );
    }

    pub fn tex_image3_d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pbo_offset: GLintptr,
    ) {
        js!( @(no_return) @{self}.texImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{format}, @{type_}, @{(pbo_offset as f64)}); );
    }

    pub fn tex_image3_d_1<T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @(no_return) @{self}.texImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_image3_d_2<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        src_data: Option<T0>,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.texImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{format}, @{type_}, @{src_data.map(|inner| unsafe { inner.as_array_buffer_view() })}); );
    }

    pub fn tex_image3_d_3<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        src_data: T0,
        src_offset: GLuint,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.texImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{format}, @{type_}, @{unsafe { src_data.as_array_buffer_view() }}, @{src_offset}); );
    }

    pub fn tex_parameterf(&self, target: GLenum, pname: GLenum, param: GLfloat) {
        js!( @(no_return) @{self}.texParameterf(@{target}, @{pname}, @{param}); );
    }

    pub fn tex_parameteri(&self, target: GLenum, pname: GLenum, param: GLint) {
        js!( @(no_return) @{self}.texParameteri(@{target}, @{pname}, @{param}); );
    }

    pub fn tex_storage2_d(
        &self,
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @(no_return) @{self}.texStorage2D(@{target}, @{levels}, @{internalformat}, @{width}, @{height}); );
    }

    pub fn tex_storage3_d(
        &self,
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
    ) {
        js!( @(no_return) @{self}.texStorage3D(@{target}, @{levels}, @{internalformat}, @{width}, @{height}, @{depth}); );
    }

    pub fn tex_sub_image2_d<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: Option<T0>,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{type_}, @{pixels.map(|inner| unsafe { inner.as_array_buffer_view() })}); );
    }

    pub fn tex_sub_image2_d_1<T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @(no_return) @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_sub_image2_d_2(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pbo_offset: GLintptr,
    ) {
        js!( @(no_return) @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{type_}, @{(pbo_offset as f64)}); );
    }

    pub fn tex_sub_image2_d_3<T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @(no_return) @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_sub_image2_d_4<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        src_data: T0,
        src_offset: GLuint,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{type_}, @{unsafe { src_data.as_array_buffer_view() }}, @{src_offset}); );
    }

    pub fn tex_sub_image3_d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        type_: GLenum,
        pbo_offset: GLintptr,
    ) {
        js!( @(no_return) @{self}.texSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{width}, @{height}, @{depth}, @{format}, @{type_}, @{(pbo_offset as f64)}); );
    }

    pub fn tex_sub_image3_d_1<T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @(no_return) @{self}.texSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{width}, @{height}, @{depth}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_sub_image3_d_2<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        type_: GLenum,
        src_data: Option<T0>,
        src_offset: GLuint,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.texSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{width}, @{height}, @{depth}, @{format}, @{type_}, @{src_data.map(|inner| unsafe { inner.as_array_buffer_view() })}, @{src_offset}); );
    }

    pub fn transform_feedback_varyings(
        &self,
        program: &WebGLProgram,
        varyings: &[&str],
        buffer_mode: GLenum,
    ) {
        js!( @(no_return) @{self}.transformFeedbackVaryings(@{program}, @{varyings}, @{buffer_mode}); );
    }

    pub fn uniform1f(&self, location: Option<&WebGLUniformLocation>, x: GLfloat) {
        js!( @(no_return) @{self}.uniform1f(@{location}, @{x}); );
    }

    pub fn uniform1fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniform1fv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform1fv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniform1fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform1i(&self, location: Option<&WebGLUniformLocation>, x: GLint) {
        js!( @(no_return) @{self}.uniform1i(@{location}, @{x}); );
    }

    pub fn uniform1iv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @(no_return) @{self}.uniform1iv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform1iv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @(no_return) @{self}.uniform1iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform1ui(&self, location: Option<&WebGLUniformLocation>, v0: GLuint) {
        js!( @(no_return) @{self}.uniform1ui(@{location}, @{v0}); );
    }

    pub fn uniform1uiv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, u32>,
    {
        js!( @(no_return) @{self}.uniform1uiv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform2f(&self, location: Option<&WebGLUniformLocation>, x: GLfloat, y: GLfloat) {
        js!( @(no_return) @{self}.uniform2f(@{location}, @{x}, @{y}); );
    }

    pub fn uniform2fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniform2fv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform2fv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniform2fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform2i(&self, location: Option<&WebGLUniformLocation>, x: GLint, y: GLint) {
        js!( @(no_return) @{self}.uniform2i(@{location}, @{x}, @{y}); );
    }

    pub fn uniform2iv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @(no_return) @{self}.uniform2iv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform2iv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @(no_return) @{self}.uniform2iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform2ui(&self, location: Option<&WebGLUniformLocation>, v0: GLuint, v1: GLuint) {
        js!( @(no_return) @{self}.uniform2ui(@{location}, @{v0}, @{v1}); );
    }

    pub fn uniform2uiv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, u32>,
    {
        js!( @(no_return) @{self}.uniform2uiv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform3f(
        &self,
        location: Option<&WebGLUniformLocation>,
        x: GLfloat,
        y: GLfloat,
        z: GLfloat,
    ) {
        js!( @(no_return) @{self}.uniform3f(@{location}, @{x}, @{y}, @{z}); );
    }

    pub fn uniform3fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniform3fv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform3fv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniform3fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform3i(&self, location: Option<&WebGLUniformLocation>, x: GLint, y: GLint, z: GLint) {
        js!( @(no_return) @{self}.uniform3i(@{location}, @{x}, @{y}, @{z}); );
    }

    pub fn uniform3iv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @(no_return) @{self}.uniform3iv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform3iv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @(no_return) @{self}.uniform3iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform3ui(
        &self,
        location: Option<&WebGLUniformLocation>,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
    ) {
        js!( @(no_return) @{self}.uniform3ui(@{location}, @{v0}, @{v1}, @{v2}); );
    }

    pub fn uniform3uiv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, u32>,
    {
        js!( @(no_return) @{self}.uniform3uiv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform4f(
        &self,
        location: Option<&WebGLUniformLocation>,
        x: GLfloat,
        y: GLfloat,
        z: GLfloat,
        w: GLfloat,
    ) {
        js!( @(no_return) @{self}.uniform4f(@{location}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn uniform4fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniform4fv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform4fv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniform4fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform4i(
        &self,
        location: Option<&WebGLUniformLocation>,
        x: GLint,
        y: GLint,
        z: GLint,
        w: GLint,
    ) {
        js!( @(no_return) @{self}.uniform4i(@{location}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn uniform4iv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @(no_return) @{self}.uniform4iv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform4iv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @(no_return) @{self}.uniform4iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform4ui(
        &self,
        location: Option<&WebGLUniformLocation>,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
        v3: GLuint,
    ) {
        js!( @(no_return) @{self}.uniform4ui(@{location}, @{v0}, @{v1}, @{v2}, @{v3}); );
    }

    pub fn uniform4uiv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, u32>,
    {
        js!( @(no_return) @{self}.uniform4uiv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_block_binding(
        &self,
        program: &WebGLProgram,
        uniform_block_index: GLuint,
        uniform_block_binding: GLuint,
    ) {
        js!( @(no_return) @{self}.uniformBlockBinding(@{program}, @{uniform_block_index}, @{uniform_block_binding}); );
    }

    pub fn uniform_matrix2fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniformMatrix2fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix2fv_1<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        value: T0,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniformMatrix2fv(@{location}, @{transpose}, @{unsafe { value.as_typed_array() }}); );
    }

    pub fn uniform_matrix2x3fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniformMatrix2x3fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix2x4fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniformMatrix2x4fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix3fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniformMatrix3fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix3fv_1<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        value: T0,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniformMatrix3fv(@{location}, @{transpose}, @{unsafe { value.as_typed_array() }}); );
    }

    pub fn uniform_matrix3x2fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniformMatrix3x2fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix3x4fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniformMatrix3x4fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix4fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniformMatrix4fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix4fv_1<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        value: T0,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniformMatrix4fv(@{location}, @{transpose}, @{unsafe { value.as_typed_array() }}); );
    }

    pub fn uniform_matrix4x2fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniformMatrix4x2fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix4x3fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniformMatrix4x3fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn use_program(&self, program: Option<&WebGLProgram>) {
        js!( @(no_return) @{self}.useProgram(@{program}); );
    }

    pub fn validate_program(&self, program: &WebGLProgram) {
        js!( @(no_return) @{self}.validateProgram(@{program}); );
    }

    pub fn vertex_attrib1f(&self, index: GLuint, x: GLfloat) {
        js!( @(no_return) @{self}.vertexAttrib1f(@{index}, @{x}); );
    }

    pub fn vertex_attrib1fv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.vertexAttrib1fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib2f(&self, index: GLuint, x: GLfloat, y: GLfloat) {
        js!( @(no_return) @{self}.vertexAttrib2f(@{index}, @{x}, @{y}); );
    }

    pub fn vertex_attrib2fv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.vertexAttrib2fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib3f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
        js!( @(no_return) @{self}.vertexAttrib3f(@{index}, @{x}, @{y}, @{z}); );
    }

    pub fn vertex_attrib3fv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.vertexAttrib3fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib4f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
        js!( @(no_return) @{self}.vertexAttrib4f(@{index}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn vertex_attrib4fv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.vertexAttrib4fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib_divisor(&self, index: GLuint, divisor: GLuint) {
        js!( @(no_return) @{self}.vertexAttribDivisor(@{index}, @{divisor}); );
    }

    pub fn vertex_attrib_i4i(&self, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) {
        js!( @(no_return) @{self}.vertexAttribI4i(@{index}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn vertex_attrib_i4iv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @(no_return) @{self}.vertexAttribI4iv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib_i4ui(&self, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) {
        js!( @(no_return) @{self}.vertexAttribI4ui(@{index}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn vertex_attrib_i4uiv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, u32>,
    {
        js!( @(no_return) @{self}.vertexAttribI4uiv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib_i_pointer(
        &self,
        index: GLuint,
        size: GLint,
        type_: GLenum,
        stride: GLsizei,
        offset: GLintptr,
    ) {
        js!( @(no_return) @{self}.vertexAttribIPointer(@{index}, @{size}, @{type_}, @{stride}, @{(offset as f64)}); );
    }

    pub fn vertex_attrib_pointer(
        &self,
        index: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        offset: GLintptr,
    ) {
        js!( @(no_return) @{self}.vertexAttribPointer(@{index}, @{size}, @{type_}, @{normalized}, @{stride}, @{(offset as f64)}); );
    }

    pub fn viewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        js!( @(no_return) @{self}.viewport(@{x}, @{y}, @{width}, @{height}); );
    }

    pub fn wait_sync(&self, sync: &WebGLSync, flags: GLbitfield, timeout: GLint64) {
        js!( @(no_return) @{self}.waitSync(@{sync}, @{flags}, @{(timeout as f64)}); );
    }
}

impl InstanceOf for GLContext {
    #[inline]
    fn instance_of(reference: &Reference) -> bool {
        js!(
            return [WebGLRenderingContext, WebGL2RenderingContext].includes(@{{reference}}.constructor);
        ).try_into().unwrap()
    }
}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGL2RenderingContext")]
pub struct WebGL2RenderingContext(Reference);

impl WebGL2RenderingContext {
    pub const ACTIVE_ATTRIBUTES: GLenum = 35721;
    pub const ACTIVE_TEXTURE: GLenum = 34016;
    pub const ACTIVE_UNIFORMS: GLenum = 35718;
    pub const ACTIVE_UNIFORM_BLOCKS: GLenum = 35382;
    pub const ALIASED_LINE_WIDTH_RANGE: GLenum = 33902;
    pub const ALIASED_POINT_SIZE_RANGE: GLenum = 33901;
    pub const ALPHA: GLenum = 6406;
    pub const ALPHA_BITS: GLenum = 3413;
    pub const ALREADY_SIGNALED: GLenum = 37146;
    pub const ALWAYS: GLenum = 519;
    pub const ANY_SAMPLES_PASSED: GLenum = 35887;
    pub const ANY_SAMPLES_PASSED_CONSERVATIVE: GLenum = 36202;
    pub const ARRAY_BUFFER: GLenum = 34962;
    pub const ARRAY_BUFFER_BINDING: GLenum = 34964;
    pub const ATTACHED_SHADERS: GLenum = 35717;
    pub const BACK: GLenum = 1029;
    pub const BLEND: GLenum = 3042;
    pub const BLEND_COLOR: GLenum = 32773;
    pub const BLEND_DST_ALPHA: GLenum = 32970;
    pub const BLEND_DST_RGB: GLenum = 32968;
    pub const BLEND_EQUATION: GLenum = 32777;
    pub const BLEND_EQUATION_ALPHA: GLenum = 34877;
    pub const BLEND_EQUATION_RGB: GLenum = 32777;
    pub const BLEND_SRC_ALPHA: GLenum = 32971;
    pub const BLEND_SRC_RGB: GLenum = 32969;
    pub const BLUE_BITS: GLenum = 3412;
    pub const BOOL: GLenum = 35670;
    pub const BOOL_VEC2: GLenum = 35671;
    pub const BOOL_VEC3: GLenum = 35672;
    pub const BOOL_VEC4: GLenum = 35673;
    pub const BROWSER_DEFAULT_WEBGL: GLenum = 37444;
    pub const BUFFER_SIZE: GLenum = 34660;
    pub const BUFFER_USAGE: GLenum = 34661;
    pub const BYTE: GLenum = 5120;
    pub const CCW: GLenum = 2305;
    pub const CLAMP_TO_EDGE: GLenum = 33071;
    pub const COLOR: GLenum = 6144;
    pub const COLOR_ATTACHMENT0: GLenum = 36064;
    pub const COLOR_ATTACHMENT1: GLenum = 36065;
    pub const COLOR_ATTACHMENT10: GLenum = 36074;
    pub const COLOR_ATTACHMENT11: GLenum = 36075;
    pub const COLOR_ATTACHMENT12: GLenum = 36076;
    pub const COLOR_ATTACHMENT13: GLenum = 36077;
    pub const COLOR_ATTACHMENT14: GLenum = 36078;
    pub const COLOR_ATTACHMENT15: GLenum = 36079;
    pub const COLOR_ATTACHMENT2: GLenum = 36066;
    pub const COLOR_ATTACHMENT3: GLenum = 36067;
    pub const COLOR_ATTACHMENT4: GLenum = 36068;
    pub const COLOR_ATTACHMENT5: GLenum = 36069;
    pub const COLOR_ATTACHMENT6: GLenum = 36070;
    pub const COLOR_ATTACHMENT7: GLenum = 36071;
    pub const COLOR_ATTACHMENT8: GLenum = 36072;
    pub const COLOR_ATTACHMENT9: GLenum = 36073;
    pub const COLOR_BUFFER_BIT: GLenum = 16384;
    pub const COLOR_CLEAR_VALUE: GLenum = 3106;
    pub const COLOR_WRITEMASK: GLenum = 3107;
    pub const COMPARE_REF_TO_TEXTURE: GLenum = 34894;
    pub const COMPILE_STATUS: GLenum = 35713;
    pub const COMPRESSED_TEXTURE_FORMATS: GLenum = 34467;
    pub const CONDITION_SATISFIED: GLenum = 37148;
    pub const CONSTANT_ALPHA: GLenum = 32771;
    pub const CONSTANT_COLOR: GLenum = 32769;
    pub const CONTEXT_LOST_WEBGL: GLenum = 37442;
    pub const COPY_READ_BUFFER: GLenum = 36662;
    pub const COPY_READ_BUFFER_BINDING: GLenum = 36662;
    pub const COPY_WRITE_BUFFER: GLenum = 36663;
    pub const COPY_WRITE_BUFFER_BINDING: GLenum = 36663;
    pub const CULL_FACE: GLenum = 2884;
    pub const CULL_FACE_MODE: GLenum = 2885;
    pub const CURRENT_PROGRAM: GLenum = 35725;
    pub const CURRENT_QUERY: GLenum = 34917;
    pub const CURRENT_VERTEX_ATTRIB: GLenum = 34342;
    pub const CW: GLenum = 2304;
    pub const DECR: GLenum = 7683;
    pub const DECR_WRAP: GLenum = 34056;
    pub const DELETE_STATUS: GLenum = 35712;
    pub const DEPTH: GLenum = 6145;
    pub const DEPTH24_STENCIL8: GLenum = 35056;
    pub const DEPTH32F_STENCIL8: GLenum = 36013;
    pub const DEPTH_ATTACHMENT: GLenum = 36096;
    pub const DEPTH_BITS: GLenum = 3414;
    pub const DEPTH_BUFFER_BIT: GLenum = 256;
    pub const DEPTH_CLEAR_VALUE: GLenum = 2931;
    pub const DEPTH_COMPONENT: GLenum = 6402;
    pub const DEPTH_COMPONENT16: GLenum = 33189;
    pub const DEPTH_COMPONENT24: GLenum = 33190;
    pub const DEPTH_COMPONENT32F: GLenum = 36012;
    pub const DEPTH_FUNC: GLenum = 2932;
    pub const DEPTH_RANGE: GLenum = 2928;
    pub const DEPTH_STENCIL: GLenum = 34041;
    pub const DEPTH_STENCIL_ATTACHMENT: GLenum = 33306;
    pub const DEPTH_TEST: GLenum = 2929;
    pub const DEPTH_WRITEMASK: GLenum = 2930;
    pub const DITHER: GLenum = 3024;
    pub const DONT_CARE: GLenum = 4352;
    pub const DRAW_BUFFER0: GLenum = 34853;
    pub const DRAW_BUFFER1: GLenum = 34854;
    pub const DRAW_BUFFER10: GLenum = 34863;
    pub const DRAW_BUFFER11: GLenum = 34864;
    pub const DRAW_BUFFER12: GLenum = 34865;
    pub const DRAW_BUFFER13: GLenum = 34866;
    pub const DRAW_BUFFER14: GLenum = 34867;
    pub const DRAW_BUFFER15: GLenum = 34868;
    pub const DRAW_BUFFER2: GLenum = 34855;
    pub const DRAW_BUFFER3: GLenum = 34856;
    pub const DRAW_BUFFER4: GLenum = 34857;
    pub const DRAW_BUFFER5: GLenum = 34858;
    pub const DRAW_BUFFER6: GLenum = 34859;
    pub const DRAW_BUFFER7: GLenum = 34860;
    pub const DRAW_BUFFER8: GLenum = 34861;
    pub const DRAW_BUFFER9: GLenum = 34862;
    pub const DRAW_FRAMEBUFFER: GLenum = 36009;
    pub const DRAW_FRAMEBUFFER_BINDING: GLenum = 36006;
    pub const DST_ALPHA: GLenum = 772;
    pub const DST_COLOR: GLenum = 774;
    pub const DYNAMIC_COPY: GLenum = 35050;
    pub const DYNAMIC_DRAW: GLenum = 35048;
    pub const DYNAMIC_READ: GLenum = 35049;
    pub const ELEMENT_ARRAY_BUFFER: GLenum = 34963;
    pub const ELEMENT_ARRAY_BUFFER_BINDING: GLenum = 34965;
    pub const EQUAL: GLenum = 514;
    pub const FASTEST: GLenum = 4353;
    pub const FLOAT: GLenum = 5126;
    pub const FLOAT_32_UNSIGNED_INT_24_8_REV: GLenum = 36269;
    pub const FLOAT_MAT2: GLenum = 35674;
    pub const FLOAT_MAT2X3: GLenum = 35685;
    pub const FLOAT_MAT2X4: GLenum = 35686;
    pub const FLOAT_MAT3: GLenum = 35675;
    pub const FLOAT_MAT3X2: GLenum = 35687;
    pub const FLOAT_MAT3X4: GLenum = 35688;
    pub const FLOAT_MAT4: GLenum = 35676;
    pub const FLOAT_MAT4X2: GLenum = 35689;
    pub const FLOAT_MAT4X3: GLenum = 35690;
    pub const FLOAT_VEC2: GLenum = 35664;
    pub const FLOAT_VEC3: GLenum = 35665;
    pub const FLOAT_VEC4: GLenum = 35666;
    pub const FRAGMENT_SHADER: GLenum = 35632;
    pub const FRAGMENT_SHADER_DERIVATIVE_HINT: GLenum = 35723;
    pub const FRAMEBUFFER: GLenum = 36160;
    pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: GLenum = 33301;
    pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: GLenum = 33300;
    pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: GLenum = 33296;
    pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: GLenum = 33297;
    pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: GLenum = 33302;
    pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: GLenum = 33299;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLenum = 36049;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLenum = 36048;
    pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: GLenum = 33298;
    pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: GLenum = 33303;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLenum = 36051;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: GLenum = 36052;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLenum = 36050;
    pub const FRAMEBUFFER_BINDING: GLenum = 36006;
    pub const FRAMEBUFFER_COMPLETE: GLenum = 36053;
    pub const FRAMEBUFFER_DEFAULT: GLenum = 33304;
    pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = 36054;
    pub const FRAMEBUFFER_INCOMPLETE_DIMENSIONS: GLenum = 36057;
    pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = 36055;
    pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: GLenum = 36182;
    pub const FRAMEBUFFER_UNSUPPORTED: GLenum = 36061;
    pub const FRONT: GLenum = 1028;
    pub const FRONT_AND_BACK: GLenum = 1032;
    pub const FRONT_FACE: GLenum = 2886;
    pub const FUNC_ADD: GLenum = 32774;
    pub const FUNC_REVERSE_SUBTRACT: GLenum = 32779;
    pub const FUNC_SUBTRACT: GLenum = 32778;
    pub const GENERATE_MIPMAP_HINT: GLenum = 33170;
    pub const GEQUAL: GLenum = 518;
    pub const GREATER: GLenum = 516;
    pub const GREEN_BITS: GLenum = 3411;
    pub const HALF_FLOAT: GLenum = 5131;
    pub const HIGH_FLOAT: GLenum = 36338;
    pub const HIGH_INT: GLenum = 36341;
    pub const IMPLEMENTATION_COLOR_READ_FORMAT: GLenum = 35739;
    pub const IMPLEMENTATION_COLOR_READ_TYPE: GLenum = 35738;
    pub const INCR: GLenum = 7682;
    pub const INCR_WRAP: GLenum = 34055;
    pub const INT: GLenum = 5124;
    pub const INTERLEAVED_ATTRIBS: GLenum = 35980;
    pub const INT_2_10_10_10_REV: GLenum = 36255;
    pub const INT_SAMPLER_2D: GLenum = 36298;
    pub const INT_SAMPLER_2D_ARRAY: GLenum = 36303;
    pub const INT_SAMPLER_3D: GLenum = 36299;
    pub const INT_SAMPLER_CUBE: GLenum = 36300;
    pub const INT_VEC2: GLenum = 35667;
    pub const INT_VEC3: GLenum = 35668;
    pub const INT_VEC4: GLenum = 35669;
    pub const INVALID_ENUM: GLenum = 1280;
    pub const INVALID_FRAMEBUFFER_OPERATION: GLenum = 1286;
    pub const INVALID_INDEX: GLenum = 4294967295;
    pub const INVALID_OPERATION: GLenum = 1282;
    pub const INVALID_VALUE: GLenum = 1281;
    pub const INVERT: GLenum = 5386;
    pub const KEEP: GLenum = 7680;
    pub const LEQUAL: GLenum = 515;
    pub const LESS: GLenum = 513;
    pub const LINEAR: GLenum = 9729;
    pub const LINEAR_MIPMAP_LINEAR: GLenum = 9987;
    pub const LINEAR_MIPMAP_NEAREST: GLenum = 9985;
    pub const LINES: GLenum = 1;
    pub const LINE_LOOP: GLenum = 2;
    pub const LINE_STRIP: GLenum = 3;
    pub const LINE_WIDTH: GLenum = 2849;
    pub const LINK_STATUS: GLenum = 35714;
    pub const LOW_FLOAT: GLenum = 36336;
    pub const LOW_INT: GLenum = 36339;
    pub const LUMINANCE: GLenum = 6409;
    pub const LUMINANCE_ALPHA: GLenum = 6410;
    pub const MAX: GLenum = 32776;
    pub const MAX_3D_TEXTURE_SIZE: GLenum = 32883;
    pub const MAX_ARRAY_TEXTURE_LAYERS: GLenum = 35071;
    pub const MAX_CLIENT_WAIT_TIMEOUT_WEBGL: GLenum = 37447;
    pub const MAX_COLOR_ATTACHMENTS: GLenum = 36063;
    pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 35379;
    pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = 35661;
    pub const MAX_COMBINED_UNIFORM_BLOCKS: GLenum = 35374;
    pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GLenum = 35377;
    pub const MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = 34076;
    pub const MAX_DRAW_BUFFERS: GLenum = 34852;
    pub const MAX_ELEMENTS_INDICES: GLenum = 33001;
    pub const MAX_ELEMENTS_VERTICES: GLenum = 33000;
    pub const MAX_ELEMENT_INDEX: GLenum = 36203;
    pub const MAX_FRAGMENT_INPUT_COMPONENTS: GLenum = 37157;
    pub const MAX_FRAGMENT_UNIFORM_BLOCKS: GLenum = 35373;
    pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 35657;
    pub const MAX_FRAGMENT_UNIFORM_VECTORS: GLenum = 36349;
    pub const MAX_PROGRAM_TEXEL_OFFSET: GLenum = 35077;
    pub const MAX_RENDERBUFFER_SIZE: GLenum = 34024;
    pub const MAX_SAMPLES: GLenum = 36183;
    pub const MAX_SERVER_WAIT_TIMEOUT: GLenum = 37137;
    pub const MAX_TEXTURE_IMAGE_UNITS: GLenum = 34930;
    pub const MAX_TEXTURE_LOD_BIAS: GLenum = 34045;
    pub const MAX_TEXTURE_SIZE: GLenum = 3379;
    pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: GLenum = 35978;
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: GLenum = 35979;
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: GLenum = 35968;
    pub const MAX_UNIFORM_BLOCK_SIZE: GLenum = 35376;
    pub const MAX_UNIFORM_BUFFER_BINDINGS: GLenum = 35375;
    pub const MAX_VARYING_COMPONENTS: GLenum = 35659;
    pub const MAX_VARYING_VECTORS: GLenum = 36348;
    pub const MAX_VERTEX_ATTRIBS: GLenum = 34921;
    pub const MAX_VERTEX_OUTPUT_COMPONENTS: GLenum = 37154;
    pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = 35660;
    pub const MAX_VERTEX_UNIFORM_BLOCKS: GLenum = 35371;
    pub const MAX_VERTEX_UNIFORM_COMPONENTS: GLenum = 35658;
    pub const MAX_VERTEX_UNIFORM_VECTORS: GLenum = 36347;
    pub const MAX_VIEWPORT_DIMS: GLenum = 3386;
    pub const MEDIUM_FLOAT: GLenum = 36337;
    pub const MEDIUM_INT: GLenum = 36340;
    pub const MIN: GLenum = 32775;
    pub const MIN_PROGRAM_TEXEL_OFFSET: GLenum = 35076;
    pub const MIRRORED_REPEAT: GLenum = 33648;
    pub const NEAREST: GLenum = 9728;
    pub const NEAREST_MIPMAP_LINEAR: GLenum = 9986;
    pub const NEAREST_MIPMAP_NEAREST: GLenum = 9984;
    pub const NEVER: GLenum = 512;
    pub const NICEST: GLenum = 4354;
    pub const NONE: GLenum = 0;
    pub const NOTEQUAL: GLenum = 517;
    pub const NO_ERROR: GLenum = 0;
    pub const OBJECT_TYPE: GLenum = 37138;
    pub const ONE: GLenum = 1;
    pub const ONE_MINUS_CONSTANT_ALPHA: GLenum = 32772;
    pub const ONE_MINUS_CONSTANT_COLOR: GLenum = 32770;
    pub const ONE_MINUS_DST_ALPHA: GLenum = 773;
    pub const ONE_MINUS_DST_COLOR: GLenum = 775;
    pub const ONE_MINUS_SRC_ALPHA: GLenum = 771;
    pub const ONE_MINUS_SRC_COLOR: GLenum = 769;
    pub const OUT_OF_MEMORY: GLenum = 1285;
    pub const PACK_ALIGNMENT: GLenum = 3333;
    pub const PACK_ROW_LENGTH: GLenum = 3330;
    pub const PACK_SKIP_PIXELS: GLenum = 3332;
    pub const PACK_SKIP_ROWS: GLenum = 3331;
    pub const PIXEL_PACK_BUFFER: GLenum = 35051;
    pub const PIXEL_PACK_BUFFER_BINDING: GLenum = 35053;
    pub const PIXEL_UNPACK_BUFFER: GLenum = 35052;
    pub const PIXEL_UNPACK_BUFFER_BINDING: GLenum = 35055;
    pub const POINTS: GLenum = 0;
    pub const POLYGON_OFFSET_FACTOR: GLenum = 32824;
    pub const POLYGON_OFFSET_FILL: GLenum = 32823;
    pub const POLYGON_OFFSET_UNITS: GLenum = 10752;
    pub const QUERY_RESULT: GLenum = 34918;
    pub const QUERY_RESULT_AVAILABLE: GLenum = 34919;
    pub const R11F_G11F_B10F: GLenum = 35898;
    pub const R16F: GLenum = 33325;
    pub const R16I: GLenum = 33331;
    pub const R16UI: GLenum = 33332;
    pub const R32F: GLenum = 33326;
    pub const R32I: GLenum = 33333;
    pub const R32UI: GLenum = 33334;
    pub const R8: GLenum = 33321;
    pub const R8I: GLenum = 33329;
    pub const R8UI: GLenum = 33330;
    pub const R8_SNORM: GLenum = 36756;
    pub const RASTERIZER_DISCARD: GLenum = 35977;
    pub const READ_BUFFER: GLenum = 3074;
    pub const READ_FRAMEBUFFER: GLenum = 36008;
    pub const READ_FRAMEBUFFER_BINDING: GLenum = 36010;
    pub const RED: GLenum = 6403;
    pub const RED_BITS: GLenum = 3410;
    pub const RED_INTEGER: GLenum = 36244;
    pub const RENDERBUFFER: GLenum = 36161;
    pub const RENDERBUFFER_ALPHA_SIZE: GLenum = 36179;
    pub const RENDERBUFFER_BINDING: GLenum = 36007;
    pub const RENDERBUFFER_BLUE_SIZE: GLenum = 36178;
    pub const RENDERBUFFER_DEPTH_SIZE: GLenum = 36180;
    pub const RENDERBUFFER_GREEN_SIZE: GLenum = 36177;
    pub const RENDERBUFFER_HEIGHT: GLenum = 36163;
    pub const RENDERBUFFER_INTERNAL_FORMAT: GLenum = 36164;
    pub const RENDERBUFFER_RED_SIZE: GLenum = 36176;
    pub const RENDERBUFFER_SAMPLES: GLenum = 36011;
    pub const RENDERBUFFER_STENCIL_SIZE: GLenum = 36181;
    pub const RENDERBUFFER_WIDTH: GLenum = 36162;
    pub const RENDERER: GLenum = 7937;
    pub const REPEAT: GLenum = 10497;
    pub const REPLACE: GLenum = 7681;
    pub const RG: GLenum = 33319;
    pub const RG16F: GLenum = 33327;
    pub const RG16I: GLenum = 33337;
    pub const RG16UI: GLenum = 33338;
    pub const RG32F: GLenum = 33328;
    pub const RG32I: GLenum = 33339;
    pub const RG32UI: GLenum = 33340;
    pub const RG8: GLenum = 33323;
    pub const RG8I: GLenum = 33335;
    pub const RG8UI: GLenum = 33336;
    pub const RG8_SNORM: GLenum = 36757;
    pub const RGB: GLenum = 6407;
    pub const RGB10_A2: GLenum = 32857;
    pub const RGB10_A2UI: GLenum = 36975;
    pub const RGB16F: GLenum = 34843;
    pub const RGB16I: GLenum = 36233;
    pub const RGB16UI: GLenum = 36215;
    pub const RGB32F: GLenum = 34837;
    pub const RGB32I: GLenum = 36227;
    pub const RGB32UI: GLenum = 36209;
    pub const RGB565: GLenum = 36194;
    pub const RGB5_A1: GLenum = 32855;
    pub const RGB8: GLenum = 32849;
    pub const RGB8I: GLenum = 36239;
    pub const RGB8UI: GLenum = 36221;
    pub const RGB8_SNORM: GLenum = 36758;
    pub const RGB9_E5: GLenum = 35901;
    pub const RGBA: GLenum = 6408;
    pub const RGBA16F: GLenum = 34842;
    pub const RGBA16I: GLenum = 36232;
    pub const RGBA16UI: GLenum = 36214;
    pub const RGBA32F: GLenum = 34836;
    pub const RGBA32I: GLenum = 36226;
    pub const RGBA32UI: GLenum = 36208;
    pub const RGBA4: GLenum = 32854;
    pub const RGBA8: GLenum = 32856;
    pub const RGBA8I: GLenum = 36238;
    pub const RGBA8UI: GLenum = 36220;
    pub const RGBA8_SNORM: GLenum = 36759;
    pub const RGBA_INTEGER: GLenum = 36249;
    pub const RGB_INTEGER: GLenum = 36248;
    pub const RG_INTEGER: GLenum = 33320;
    pub const SAMPLER_2D: GLenum = 35678;
    pub const SAMPLER_2D_ARRAY: GLenum = 36289;
    pub const SAMPLER_2D_ARRAY_SHADOW: GLenum = 36292;
    pub const SAMPLER_2D_SHADOW: GLenum = 35682;
    pub const SAMPLER_3D: GLenum = 35679;
    pub const SAMPLER_BINDING: GLenum = 35097;
    pub const SAMPLER_CUBE: GLenum = 35680;
    pub const SAMPLER_CUBE_SHADOW: GLenum = 36293;
    pub const SAMPLES: GLenum = 32937;
    pub const SAMPLE_ALPHA_TO_COVERAGE: GLenum = 32926;
    pub const SAMPLE_BUFFERS: GLenum = 32936;
    pub const SAMPLE_COVERAGE: GLenum = 32928;
    pub const SAMPLE_COVERAGE_INVERT: GLenum = 32939;
    pub const SAMPLE_COVERAGE_VALUE: GLenum = 32938;
    pub const SCISSOR_BOX: GLenum = 3088;
    pub const SCISSOR_TEST: GLenum = 3089;
    pub const SEPARATE_ATTRIBS: GLenum = 35981;
    pub const SHADER_TYPE: GLenum = 35663;
    pub const SHADING_LANGUAGE_VERSION: GLenum = 35724;
    pub const SHORT: GLenum = 5122;
    pub const SIGNALED: GLenum = 37145;
    pub const SIGNED_NORMALIZED: GLenum = 36764;
    pub const SRC_ALPHA: GLenum = 770;
    pub const SRC_ALPHA_SATURATE: GLenum = 776;
    pub const SRC_COLOR: GLenum = 768;
    pub const SRGB: GLenum = 35904;
    pub const SRGB8: GLenum = 35905;
    pub const SRGB8_ALPHA8: GLenum = 35907;
    pub const STATIC_COPY: GLenum = 35046;
    pub const STATIC_DRAW: GLenum = 35044;
    pub const STATIC_READ: GLenum = 35045;
    pub const STENCIL: GLenum = 6146;
    pub const STENCIL_ATTACHMENT: GLenum = 36128;
    pub const STENCIL_BACK_FAIL: GLenum = 34817;
    pub const STENCIL_BACK_FUNC: GLenum = 34816;
    pub const STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = 34818;
    pub const STENCIL_BACK_PASS_DEPTH_PASS: GLenum = 34819;
    pub const STENCIL_BACK_REF: GLenum = 36003;
    pub const STENCIL_BACK_VALUE_MASK: GLenum = 36004;
    pub const STENCIL_BACK_WRITEMASK: GLenum = 36005;
    pub const STENCIL_BITS: GLenum = 3415;
    pub const STENCIL_BUFFER_BIT: GLenum = 1024;
    pub const STENCIL_CLEAR_VALUE: GLenum = 2961;
    pub const STENCIL_FAIL: GLenum = 2964;
    pub const STENCIL_FUNC: GLenum = 2962;
    pub const STENCIL_INDEX8: GLenum = 36168;
    pub const STENCIL_PASS_DEPTH_FAIL: GLenum = 2965;
    pub const STENCIL_PASS_DEPTH_PASS: GLenum = 2966;
    pub const STENCIL_REF: GLenum = 2967;
    pub const STENCIL_TEST: GLenum = 2960;
    pub const STENCIL_VALUE_MASK: GLenum = 2963;
    pub const STENCIL_WRITEMASK: GLenum = 2968;
    pub const STREAM_COPY: GLenum = 35042;
    pub const STREAM_DRAW: GLenum = 35040;
    pub const STREAM_READ: GLenum = 35041;
    pub const SUBPIXEL_BITS: GLenum = 3408;
    pub const SYNC_CONDITION: GLenum = 37139;
    pub const SYNC_FENCE: GLenum = 37142;
    pub const SYNC_FLAGS: GLenum = 37141;
    pub const SYNC_FLUSH_COMMANDS_BIT: GLenum = 1;
    pub const SYNC_GPU_COMMANDS_COMPLETE: GLenum = 37143;
    pub const SYNC_STATUS: GLenum = 37140;
    pub const TEXTURE: GLenum = 5890;
    pub const TEXTURE0: GLenum = 33984;
    pub const TEXTURE1: GLenum = 33985;
    pub const TEXTURE10: GLenum = 33994;
    pub const TEXTURE11: GLenum = 33995;
    pub const TEXTURE12: GLenum = 33996;
    pub const TEXTURE13: GLenum = 33997;
    pub const TEXTURE14: GLenum = 33998;
    pub const TEXTURE15: GLenum = 33999;
    pub const TEXTURE16: GLenum = 34000;
    pub const TEXTURE17: GLenum = 34001;
    pub const TEXTURE18: GLenum = 34002;
    pub const TEXTURE19: GLenum = 34003;
    pub const TEXTURE2: GLenum = 33986;
    pub const TEXTURE20: GLenum = 34004;
    pub const TEXTURE21: GLenum = 34005;
    pub const TEXTURE22: GLenum = 34006;
    pub const TEXTURE23: GLenum = 34007;
    pub const TEXTURE24: GLenum = 34008;
    pub const TEXTURE25: GLenum = 34009;
    pub const TEXTURE26: GLenum = 34010;
    pub const TEXTURE27: GLenum = 34011;
    pub const TEXTURE28: GLenum = 34012;
    pub const TEXTURE29: GLenum = 34013;
    pub const TEXTURE3: GLenum = 33987;
    pub const TEXTURE30: GLenum = 34014;
    pub const TEXTURE31: GLenum = 34015;
    pub const TEXTURE4: GLenum = 33988;
    pub const TEXTURE5: GLenum = 33989;
    pub const TEXTURE6: GLenum = 33990;
    pub const TEXTURE7: GLenum = 33991;
    pub const TEXTURE8: GLenum = 33992;
    pub const TEXTURE9: GLenum = 33993;
    pub const TEXTURE_2D: GLenum = 3553;
    pub const TEXTURE_2D_ARRAY: GLenum = 35866;
    pub const TEXTURE_3D: GLenum = 32879;
    pub const TEXTURE_BASE_LEVEL: GLenum = 33084;
    pub const TEXTURE_BINDING_2D: GLenum = 32873;
    pub const TEXTURE_BINDING_2D_ARRAY: GLenum = 35869;
    pub const TEXTURE_BINDING_3D: GLenum = 32874;
    pub const TEXTURE_BINDING_CUBE_MAP: GLenum = 34068;
    pub const TEXTURE_COMPARE_FUNC: GLenum = 34893;
    pub const TEXTURE_COMPARE_MODE: GLenum = 34892;
    pub const TEXTURE_CUBE_MAP: GLenum = 34067;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = 34070;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = 34072;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = 34074;
    pub const TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = 34069;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = 34071;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = 34073;
    pub const TEXTURE_IMMUTABLE_FORMAT: GLenum = 37167;
    pub const TEXTURE_IMMUTABLE_LEVELS: GLenum = 33503;
    pub const TEXTURE_MAG_FILTER: GLenum = 10240;
    pub const TEXTURE_MAX_LEVEL: GLenum = 33085;
    pub const TEXTURE_MAX_LOD: GLenum = 33083;
    pub const TEXTURE_MIN_FILTER: GLenum = 10241;
    pub const TEXTURE_MIN_LOD: GLenum = 33082;
    pub const TEXTURE_WRAP_R: GLenum = 32882;
    pub const TEXTURE_WRAP_S: GLenum = 10242;
    pub const TEXTURE_WRAP_T: GLenum = 10243;
    pub const TIMEOUT_EXPIRED: GLenum = 37147;
    pub const TIMEOUT_IGNORED: GLint64 = -1;
    pub const TRANSFORM_FEEDBACK: GLenum = 36386;
    pub const TRANSFORM_FEEDBACK_ACTIVE: GLenum = 36388;
    pub const TRANSFORM_FEEDBACK_BINDING: GLenum = 36389;
    pub const TRANSFORM_FEEDBACK_BUFFER: GLenum = 35982;
    pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: GLenum = 35983;
    pub const TRANSFORM_FEEDBACK_BUFFER_MODE: GLenum = 35967;
    pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: GLenum = 35973;
    pub const TRANSFORM_FEEDBACK_BUFFER_START: GLenum = 35972;
    pub const TRANSFORM_FEEDBACK_PAUSED: GLenum = 36387;
    pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: GLenum = 35976;
    pub const TRANSFORM_FEEDBACK_VARYINGS: GLenum = 35971;
    pub const TRIANGLES: GLenum = 4;
    pub const TRIANGLE_FAN: GLenum = 6;
    pub const TRIANGLE_STRIP: GLenum = 5;
    pub const UNIFORM_ARRAY_STRIDE: GLenum = 35388;
    pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: GLenum = 35394;
    pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: GLenum = 35395;
    pub const UNIFORM_BLOCK_BINDING: GLenum = 35391;
    pub const UNIFORM_BLOCK_DATA_SIZE: GLenum = 35392;
    pub const UNIFORM_BLOCK_INDEX: GLenum = 35386;
    pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 35398;
    pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: GLenum = 35396;
    pub const UNIFORM_BUFFER: GLenum = 35345;
    pub const UNIFORM_BUFFER_BINDING: GLenum = 35368;
    pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: GLenum = 35380;
    pub const UNIFORM_BUFFER_SIZE: GLenum = 35370;
    pub const UNIFORM_BUFFER_START: GLenum = 35369;
    pub const UNIFORM_IS_ROW_MAJOR: GLenum = 35390;
    pub const UNIFORM_MATRIX_STRIDE: GLenum = 35389;
    pub const UNIFORM_OFFSET: GLenum = 35387;
    pub const UNIFORM_SIZE: GLenum = 35384;
    pub const UNIFORM_TYPE: GLenum = 35383;
    pub const UNPACK_ALIGNMENT: GLenum = 3317;
    pub const UNPACK_COLORSPACE_CONVERSION_WEBGL: GLenum = 37443;
    pub const UNPACK_FLIP_Y_WEBGL: GLenum = 37440;
    pub const UNPACK_IMAGE_HEIGHT: GLenum = 32878;
    pub const UNPACK_PREMULTIPLY_ALPHA_WEBGL: GLenum = 37441;
    pub const UNPACK_ROW_LENGTH: GLenum = 3314;
    pub const UNPACK_SKIP_IMAGES: GLenum = 32877;
    pub const UNPACK_SKIP_PIXELS: GLenum = 3316;
    pub const UNPACK_SKIP_ROWS: GLenum = 3315;
    pub const UNSIGNALED: GLenum = 37144;
    pub const UNSIGNED_BYTE: GLenum = 5121;
    pub const UNSIGNED_INT: GLenum = 5125;
    pub const UNSIGNED_INT_10F_11F_11F_REV: GLenum = 35899;
    pub const UNSIGNED_INT_24_8: GLenum = 34042;
    pub const UNSIGNED_INT_2_10_10_10_REV: GLenum = 33640;
    pub const UNSIGNED_INT_5_9_9_9_REV: GLenum = 35902;
    pub const UNSIGNED_INT_SAMPLER_2D: GLenum = 36306;
    pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: GLenum = 36311;
    pub const UNSIGNED_INT_SAMPLER_3D: GLenum = 36307;
    pub const UNSIGNED_INT_SAMPLER_CUBE: GLenum = 36308;
    pub const UNSIGNED_INT_VEC2: GLenum = 36294;
    pub const UNSIGNED_INT_VEC3: GLenum = 36295;
    pub const UNSIGNED_INT_VEC4: GLenum = 36296;
    pub const UNSIGNED_NORMALIZED: GLenum = 35863;
    pub const UNSIGNED_SHORT: GLenum = 5123;
    pub const UNSIGNED_SHORT_4_4_4_4: GLenum = 32819;
    pub const UNSIGNED_SHORT_5_5_5_1: GLenum = 32820;
    pub const UNSIGNED_SHORT_5_6_5: GLenum = 33635;
    pub const VALIDATE_STATUS: GLenum = 35715;
    pub const VENDOR: GLenum = 7936;
    pub const VERSION: GLenum = 7938;
    pub const VERTEX_ARRAY_BINDING: GLenum = 34229;
    pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = 34975;
    pub const VERTEX_ATTRIB_ARRAY_DIVISOR: GLenum = 35070;
    pub const VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = 34338;
    pub const VERTEX_ATTRIB_ARRAY_INTEGER: GLenum = 35069;
    pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = 34922;
    pub const VERTEX_ATTRIB_ARRAY_POINTER: GLenum = 34373;
    pub const VERTEX_ATTRIB_ARRAY_SIZE: GLenum = 34339;
    pub const VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = 34340;
    pub const VERTEX_ATTRIB_ARRAY_TYPE: GLenum = 34341;
    pub const VERTEX_SHADER: GLenum = 35633;
    pub const VIEWPORT: GLenum = 2978;
    pub const WAIT_FAILED: GLenum = 37149;
    pub const ZERO: GLenum = 0;

    pub fn active_texture(&self, texture: GLenum) {
        js!( @(no_return) @{self}.activeTexture(@{texture}); );
    }

    pub fn attach_shader(&self, program: &WebGLProgram, shader: &WebGLShader) {
        js!( @(no_return) @{self}.attachShader(@{program}, @{shader}); );
    }

    pub fn begin_query(&self, target: GLenum, query: &WebGLQuery) {
        js!( @(no_return) @{self}.beginQuery(@{target}, @{query}); );
    }

    pub fn begin_transform_feedback(&self, primitive_mode: GLenum) {
        js!( @(no_return) @{self}.beginTransformFeedback(@{primitive_mode}); );
    }

    pub fn bind_attrib_location(&self, program: &WebGLProgram, index: GLuint, name: &str) {
        js!( @(no_return) @{self}.bindAttribLocation(@{program}, @{index}, @{name}); );
    }

    pub fn bind_buffer(&self, target: GLenum, buffer: Option<&WebGLBuffer>) {
        js!( @(no_return) @{self}.bindBuffer(@{target}, @{buffer}); );
    }

    pub fn bind_buffer_base(&self, target: GLenum, index: GLuint, buffer: Option<&WebGLBuffer>) {
        js!( @(no_return) @{self}.bindBufferBase(@{target}, @{index}, @{buffer}); );
    }

    pub fn bind_buffer_range(
        &self,
        target: GLenum,
        index: GLuint,
        buffer: Option<&WebGLBuffer>,
        offset: GLintptr,
        size: GLsizeiptr,
    ) {
        js!( @(no_return) @{self}.bindBufferRange(@{target}, @{index}, @{buffer}, @{(offset as f64)}, @{(size as f64)}); );
    }

    pub fn bind_framebuffer(&self, target: GLenum, framebuffer: Option<&WebGLFramebuffer>) {
        js!( @(no_return) @{self}.bindFramebuffer(@{target}, @{framebuffer}); );
    }

    pub fn bind_renderbuffer(&self, target: GLenum, renderbuffer: Option<&WebGLRenderbuffer>) {
        js!( @(no_return) @{self}.bindRenderbuffer(@{target}, @{renderbuffer}); );
    }

    pub fn bind_sampler(&self, unit: GLuint, sampler: Option<&WebGLSampler>) {
        js!( @(no_return) @{self}.bindSampler(@{unit}, @{sampler}); );
    }

    pub fn bind_texture(&self, target: GLenum, texture: Option<&WebGLTexture>) {
        js!( @(no_return) @{self}.bindTexture(@{target}, @{texture}); );
    }

    pub fn bind_transform_feedback(&self, target: GLenum, tf: Option<&WebGLTransformFeedback>) {
        js!( @(no_return) @{self}.bindTransformFeedback(@{target}, @{tf}); );
    }

    pub fn bind_vertex_array(&self, array: Option<&WebGLVertexArrayObject>) {
        js!( @(no_return) @{self}.bindVertexArray(@{array}); );
    }

    pub fn blend_color(&self, red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
        js!( @(no_return) @{self}.blendColor(@{red}, @{green}, @{blue}, @{alpha}); );
    }

    pub fn blend_equation(&self, mode: GLenum) {
        js!( @(no_return) @{self}.blendEquation(@{mode}); );
    }

    pub fn blend_equation_separate(&self, mode_rgb: GLenum, mode_alpha: GLenum) {
        js!( @(no_return) @{self}.blendEquationSeparate(@{mode_rgb}, @{mode_alpha}); );
    }

    pub fn blend_func(&self, sfactor: GLenum, dfactor: GLenum) {
        js!( @(no_return) @{self}.blendFunc(@{sfactor}, @{dfactor}); );
    }

    pub fn blend_func_separate(
        &self,
        src_rgb: GLenum,
        dst_rgb: GLenum,
        src_alpha: GLenum,
        dst_alpha: GLenum,
    ) {
        js!( @(no_return) @{self}.blendFuncSeparate(@{src_rgb}, @{dst_rgb}, @{src_alpha}, @{dst_alpha}); );
    }

    pub fn blit_framebuffer(
        &self,
        src_x0: GLint,
        src_y0: GLint,
        src_x1: GLint,
        src_y1: GLint,
        dst_x0: GLint,
        dst_y0: GLint,
        dst_x1: GLint,
        dst_y1: GLint,
        mask: GLbitfield,
        filter: GLenum,
    ) {
        js!( @(no_return) @{self}.blitFramebuffer(@{src_x0}, @{src_y0}, @{src_x1}, @{src_y1}, @{dst_x0}, @{dst_y0}, @{dst_x1}, @{dst_y1}, @{mask}, @{filter}); );
    }

    pub fn buffer_data(&self, target: GLenum, size: GLsizeiptr, usage: GLenum) {
        js!( @(no_return) @{self}.bufferData(@{target}, @{(size as f64)}, @{usage}); );
    }

    pub fn buffer_data_1(&self, target: GLenum, src_data: Option<&ArrayBuffer>, usage: GLenum) {
        js!( @(no_return) @{self}.bufferData(@{target}, @{src_data}, @{usage}); );
    }

    pub fn buffer_data_2<'a0, T0>(
        &self,
        target: GLenum,
        src_data: T0,
        usage: GLenum,
        src_offset: GLuint,
        length: GLuint,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.bufferData(@{target}, @{unsafe { src_data.as_array_buffer_view() }}, @{usage}, @{src_offset}, @{length}); );
    }

    pub fn buffer_sub_data(
        &self,
        target: GLenum,
        dst_byte_offset: GLintptr,
        src_data: &ArrayBuffer,
    ) {
        js!( @(no_return) @{self}.bufferSubData(@{target}, @{(dst_byte_offset as f64)}, @{src_data}); );
    }

    pub fn buffer_sub_data_1<'a0, T0>(
        &self,
        target: GLenum,
        dst_byte_offset: GLintptr,
        src_data: T0,
        src_offset: GLuint,
        length: GLuint,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.bufferSubData(@{target}, @{(dst_byte_offset as f64)}, @{unsafe { src_data.as_array_buffer_view() }}, @{src_offset}, @{length}); );
    }

    pub fn canvas(&self) -> CanvasElement {
        (js! { return @{self}.canvas; }).try_into().unwrap()
    }

    pub fn check_framebuffer_status(&self, target: GLenum) -> GLenum {
        (js! { return @{self}.checkFramebufferStatus(@{target}); })
            .try_into()
            .unwrap()
    }

    pub fn clear(&self, mask: GLbitfield) {
        js!( @(no_return) @{self}.clear(@{mask}); );
    }

    pub fn clear_bufferfi(
        &self,
        buffer: GLenum,
        drawbuffer: GLint,
        depth: GLfloat,
        stencil: GLint,
    ) {
        js!( @(no_return) @{self}.clearBufferfi(@{buffer}, @{drawbuffer}, @{depth}, @{stencil}); );
    }

    pub fn clear_bufferfv<'a0, T0>(
        &self,
        buffer: GLenum,
        drawbuffer: GLint,
        values: T0,
        src_offset: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.clearBufferfv(@{buffer}, @{drawbuffer}, @{unsafe { values.as_typed_array() }}, @{src_offset}); );
    }

    pub fn clear_bufferiv<'a0, T0>(
        &self,
        buffer: GLenum,
        drawbuffer: GLint,
        values: T0,
        src_offset: GLuint,
    ) where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @(no_return) @{self}.clearBufferiv(@{buffer}, @{drawbuffer}, @{unsafe { values.as_typed_array() }}, @{src_offset}); );
    }

    pub fn clear_bufferuiv<'a0, T0>(
        &self,
        buffer: GLenum,
        drawbuffer: GLint,
        values: T0,
        src_offset: GLuint,
    ) where
        T0: AsTypedArray<'a0, u32>,
    {
        js!( @(no_return) @{self}.clearBufferuiv(@{buffer}, @{drawbuffer}, @{unsafe { values.as_typed_array() }}, @{src_offset}); );
    }

    pub fn clear_color(&self, red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
        js!( @(no_return) @{self}.clearColor(@{red}, @{green}, @{blue}, @{alpha}); );
    }

    pub fn clear_depth(&self, depth: GLclampf) {
        js!( @(no_return) @{self}.clearDepth(@{depth}); );
    }

    pub fn clear_stencil(&self, s: GLint) {
        js!( @(no_return) @{self}.clearStencil(@{s}); );
    }

    pub fn client_wait_sync(
        &self,
        sync: &WebGLSync,
        flags: GLbitfield,
        timeout: GLuint64,
    ) -> GLenum {
        (js! { return @{self}.clientWaitSync(@{sync}, @{flags}, @{(timeout as f64)}); })
            .try_into()
            .unwrap()
    }

    pub fn color_mask(&self, red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) {
        js!( @(no_return) @{self}.colorMask(@{red}, @{green}, @{blue}, @{alpha}); );
    }

    pub fn compile_shader(&self, shader: &WebGLShader) {
        js!( @(no_return) @{self}.compileShader(@{shader}); );
    }

    pub fn compressed_tex_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        image_size: GLsizei,
        offset: GLintptr,
    ) {
        js!( @(no_return) @{self}.compressedTexImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{image_size}, @{(offset as f64)}); );
    }

    pub fn compressed_tex_image2_d_1<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        src_data: T0,
        src_offset: GLuint,
        src_length_override: GLuint,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.compressedTexImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{unsafe { src_data.as_array_buffer_view() }}, @{src_offset}, @{src_length_override}); );
    }

    pub fn compressed_tex_image2_d_2<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        data: T0,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.compressedTexImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{unsafe { data.as_array_buffer_view() }}); );
    }

    pub fn compressed_tex_image3_d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        image_size: GLsizei,
        offset: GLintptr,
    ) {
        js!( @(no_return) @{self}.compressedTexImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{image_size}, @{(offset as f64)}); );
    }

    pub fn compressed_tex_image3_d_1<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        src_data: T0,
        src_offset: GLuint,
        src_length_override: GLuint,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.compressedTexImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{unsafe { src_data.as_array_buffer_view() }}, @{src_offset}, @{src_length_override}); );
    }

    pub fn compressed_tex_sub_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        image_size: GLsizei,
        offset: GLintptr,
    ) {
        js!( @(no_return) @{self}.compressedTexSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{image_size}, @{(offset as f64)}); );
    }

    pub fn compressed_tex_sub_image2_d_1<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        src_data: T0,
        src_offset: GLuint,
        src_length_override: GLuint,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.compressedTexSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{unsafe { src_data.as_array_buffer_view() }}, @{src_offset}, @{src_length_override}); );
    }

    pub fn compressed_tex_sub_image2_d_2<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        data: T0,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.compressedTexSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{unsafe { data.as_array_buffer_view() }}); );
    }

    pub fn compressed_tex_sub_image3_d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        image_size: GLsizei,
        offset: GLintptr,
    ) {
        js!( @(no_return) @{self}.compressedTexSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{width}, @{height}, @{depth}, @{format}, @{image_size}, @{(offset as f64)}); );
    }

    pub fn compressed_tex_sub_image3_d_1<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        src_data: T0,
        src_offset: GLuint,
        src_length_override: GLuint,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.compressedTexSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{width}, @{height}, @{depth}, @{format}, @{unsafe { src_data.as_array_buffer_view() }}, @{src_offset}, @{src_length_override}); );
    }

    pub fn copy_buffer_sub_data(
        &self,
        read_target: GLenum,
        write_target: GLenum,
        read_offset: GLintptr,
        write_offset: GLintptr,
        size: GLsizeiptr,
    ) {
        js!( @(no_return) @{self}.copyBufferSubData(@{read_target}, @{write_target}, @{(read_offset as f64)}, @{(write_offset as f64)}, @{(size as f64)}); );
    }

    pub fn copy_tex_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
    ) {
        js!( @(no_return) @{self}.copyTexImage2D(@{target}, @{level}, @{internalformat}, @{x}, @{y}, @{width}, @{height}, @{border}); );
    }

    pub fn copy_tex_sub_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @(no_return) @{self}.copyTexSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{x}, @{y}, @{width}, @{height}); );
    }

    pub fn copy_tex_sub_image3_d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @(no_return) @{self}.copyTexSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{x}, @{y}, @{width}, @{height}); );
    }

    pub fn create_buffer(&self) -> Option<WebGLBuffer> {
        (js! { return @{self}.createBuffer(); }).try_into().ok()
    }

    pub fn create_framebuffer(&self) -> Option<WebGLFramebuffer> {
        (js! { return @{self}.createFramebuffer(); })
            .try_into()
            .ok()
    }

    pub fn create_program(&self) -> Option<WebGLProgram> {
        (js! { return @{self}.createProgram(); }).try_into().ok()
    }

    pub fn create_query(&self) -> Option<WebGLQuery> {
        (js! { return @{self}.createQuery(); }).try_into().ok()
    }

    pub fn create_renderbuffer(&self) -> Option<WebGLRenderbuffer> {
        (js! { return @{self}.createRenderbuffer(); })
            .try_into()
            .ok()
    }

    pub fn create_sampler(&self) -> Option<WebGLSampler> {
        (js! { return @{self}.createSampler(); }).try_into().ok()
    }

    pub fn create_shader(&self, type_: GLenum) -> Option<WebGLShader> {
        (js! { return @{self}.createShader(@{type_}); })
            .try_into()
            .ok()
    }

    pub fn create_texture(&self) -> Option<WebGLTexture> {
        (js! { return @{self}.createTexture(); }).try_into().ok()
    }

    pub fn create_transform_feedback(&self) -> Option<WebGLTransformFeedback> {
        (js! { return @{self}.createTransformFeedback(); })
            .try_into()
            .ok()
    }

    pub fn create_vertex_array(&self) -> Option<WebGLVertexArrayObject> {
        (js! { return @{self}.createVertexArray(); })
            .try_into()
            .ok()
    }

    pub fn cull_face(&self, mode: GLenum) {
        js!( @(no_return) @{self}.cullFace(@{mode}); );
    }

    pub fn delete_buffer(&self, buffer: Option<&WebGLBuffer>) {
        js!( @(no_return) @{self}.deleteBuffer(@{buffer}); );
    }

    pub fn delete_framebuffer(&self, framebuffer: Option<&WebGLFramebuffer>) {
        js!( @(no_return) @{self}.deleteFramebuffer(@{framebuffer}); );
    }

    pub fn delete_program(&self, program: Option<&WebGLProgram>) {
        js!( @(no_return) @{self}.deleteProgram(@{program}); );
    }

    pub fn delete_query(&self, query: Option<&WebGLQuery>) {
        js!( @(no_return) @{self}.deleteQuery(@{query}); );
    }

    pub fn delete_renderbuffer(&self, renderbuffer: Option<&WebGLRenderbuffer>) {
        js!( @(no_return) @{self}.deleteRenderbuffer(@{renderbuffer}); );
    }

    pub fn delete_sampler(&self, sampler: Option<&WebGLSampler>) {
        js!( @(no_return) @{self}.deleteSampler(@{sampler}); );
    }

    pub fn delete_shader(&self, shader: Option<&WebGLShader>) {
        js!( @(no_return) @{self}.deleteShader(@{shader}); );
    }

    pub fn delete_sync(&self, sync: Option<&WebGLSync>) {
        js!( @(no_return) @{self}.deleteSync(@{sync}); );
    }

    pub fn delete_texture(&self, texture: Option<&WebGLTexture>) {
        js!( @(no_return) @{self}.deleteTexture(@{texture}); );
    }

    pub fn delete_transform_feedback(&self, tf: Option<&WebGLTransformFeedback>) {
        js!( @(no_return) @{self}.deleteTransformFeedback(@{tf}); );
    }

    pub fn delete_vertex_array(&self, vertex_array: Option<&WebGLVertexArrayObject>) {
        js!( @(no_return) @{self}.deleteVertexArray(@{vertex_array}); );
    }

    pub fn depth_func(&self, func: GLenum) {
        js!( @(no_return) @{self}.depthFunc(@{func}); );
    }

    pub fn depth_mask(&self, flag: GLboolean) {
        js!( @(no_return) @{self}.depthMask(@{flag}); );
    }

    pub fn depth_range(&self, z_near: GLclampf, z_far: GLclampf) {
        js!( @(no_return) @{self}.depthRange(@{z_near}, @{z_far}); );
    }

    pub fn detach_shader(&self, program: &WebGLProgram, shader: &WebGLShader) {
        js!( @(no_return) @{self}.detachShader(@{program}, @{shader}); );
    }

    pub fn disable(&self, cap: GLenum) {
        js!( @(no_return) @{self}.disable(@{cap}); );
    }

    pub fn disable_vertex_attrib_array(&self, index: GLuint) {
        js!( @(no_return) @{self}.disableVertexAttribArray(@{index}); );
    }

    pub fn draw_arrays(&self, mode: GLenum, first: GLint, count: GLsizei) {
        js!( @(no_return) @{self}.drawArrays(@{mode}, @{first}, @{count}); );
    }

    pub fn draw_arrays_instanced(
        &self,
        mode: GLenum,
        first: GLint,
        count: GLsizei,
        instance_count: GLsizei,
    ) {
        js!( @(no_return) @{self}.drawArraysInstanced(@{mode}, @{first}, @{count}, @{instance_count}); );
    }

    pub fn draw_buffers(&self, buffers: &[GLenum]) {
        js!( @(no_return) @{self}.drawBuffers(@{buffers}); );
    }

    pub fn draw_elements(&self, mode: GLenum, count: GLsizei, type_: GLenum, offset: GLintptr) {
        js!( @(no_return) @{self}.drawElements(@{mode}, @{count}, @{type_}, @{(offset as f64)}); );
    }

    pub fn draw_elements_instanced(
        &self,
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        offset: GLintptr,
        instance_count: GLsizei,
    ) {
        js!( @(no_return) @{self}.drawElementsInstanced(@{mode}, @{count}, @{type_}, @{(offset as f64)}, @{instance_count}); );
    }

    pub fn draw_range_elements(
        &self,
        mode: GLenum,
        start: GLuint,
        end: GLuint,
        count: GLsizei,
        type_: GLenum,
        offset: GLintptr,
    ) {
        js!( @(no_return) @{self}.drawRangeElements(@{mode}, @{start}, @{end}, @{count}, @{type_}, @{(offset as f64)}); );
    }

    pub fn drawing_buffer_height(&self) -> GLsizei {
        (js! { return @{self}.drawingBufferHeight; })
            .try_into()
            .unwrap()
    }

    pub fn drawing_buffer_width(&self) -> GLsizei {
        (js! { return @{self}.drawingBufferWidth; })
            .try_into()
            .unwrap()
    }

    pub fn enable(&self, cap: GLenum) {
        js!( @(no_return) @{self}.enable(@{cap}); );
    }

    pub fn enable_vertex_attrib_array(&self, index: GLuint) {
        js!( @(no_return) @{self}.enableVertexAttribArray(@{index}); );
    }

    pub fn end_query(&self, target: GLenum) {
        js!( @(no_return) @{self}.endQuery(@{target}); );
    }

    pub fn end_transform_feedback(&self) {
        js!( @(no_return) @{self}.endTransformFeedback(); );
    }

    pub fn fence_sync(&self, condition: GLenum, flags: GLbitfield) -> Option<WebGLSync> {
        (js! { return @{self}.fenceSync(@{condition}, @{flags}); })
            .try_into()
            .ok()
    }

    pub fn finish(&self) {
        js!( @(no_return) @{self}.finish(); );
    }

    pub fn flush(&self) {
        js!( @(no_return) @{self}.flush(); );
    }

    pub fn framebuffer_renderbuffer(
        &self,
        target: GLenum,
        attachment: GLenum,
        renderbuffertarget: GLenum,
        renderbuffer: Option<&WebGLRenderbuffer>,
    ) {
        js!( @(no_return) @{self}.framebufferRenderbuffer(@{target}, @{attachment}, @{renderbuffertarget}, @{renderbuffer}); );
    }

    pub fn framebuffer_texture2_d(
        &self,
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: Option<&WebGLTexture>,
        level: GLint,
    ) {
        js!( @(no_return) @{self}.framebufferTexture2D(@{target}, @{attachment}, @{textarget}, @{texture}, @{level}); );
    }

    pub fn framebuffer_texture_layer(
        &self,
        target: GLenum,
        attachment: GLenum,
        texture: Option<&WebGLTexture>,
        level: GLint,
        layer: GLint,
    ) {
        js!( @(no_return) @{self}.framebufferTextureLayer(@{target}, @{attachment}, @{texture}, @{level}, @{layer}); );
    }

    pub fn front_face(&self, mode: GLenum) {
        js!( @(no_return) @{self}.frontFace(@{mode}); );
    }

    pub fn generate_mipmap(&self, target: GLenum) {
        js!( @(no_return) @{self}.generateMipmap(@{target}); );
    }

    pub fn get_active_attrib(
        &self,
        program: &WebGLProgram,
        index: GLuint,
    ) -> Option<WebGLActiveInfo> {
        (js! { return @{self}.getActiveAttrib(@{program}, @{index}); })
            .try_into()
            .ok()
    }

    pub fn get_active_uniform(
        &self,
        program: &WebGLProgram,
        index: GLuint,
    ) -> Option<WebGLActiveInfo> {
        (js! { return @{self}.getActiveUniform(@{program}, @{index}); })
            .try_into()
            .ok()
    }

    pub fn get_active_uniform_block_name(
        &self,
        program: &WebGLProgram,
        uniform_block_index: GLuint,
    ) -> Option<String> {
        (js! { return @{self}.getActiveUniformBlockName(@{program}, @{uniform_block_index}); })
            .try_into()
            .ok()
    }

    pub fn get_active_uniform_block_parameter(
        &self,
        program: &WebGLProgram,
        uniform_block_index: GLuint,
        pname: GLenum,
    ) -> Value {
        (js! { return @{self}.getActiveUniformBlockParameter(@{program}, @{uniform_block_index}, @{pname}); } ).try_into().unwrap()
    }

    pub fn get_active_uniforms(
        &self,
        program: &WebGLProgram,
        uniform_indices: &[GLuint],
        pname: GLenum,
    ) -> Value {
        (js! { return @{self}.getActiveUniforms(@{program}, @{uniform_indices}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_attached_shaders(&self, program: &WebGLProgram) -> Option<Vec<WebGLShader>> {
        (js! { return @{self}.getAttachedShaders(@{program}); })
            .try_into()
            .ok()
    }

    pub fn get_attrib_location(&self, program: &WebGLProgram, name: &str) -> GLint {
        (js! { return @{self}.getAttribLocation(@{program}, @{name}); })
            .try_into()
            .unwrap()
    }

    pub fn get_buffer_parameter(&self, target: GLenum, pname: GLenum) -> Value {
        (js! { return @{self}.getBufferParameter(@{target}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_buffer_sub_data<'a0, T0>(
        &self,
        target: GLenum,
        src_byte_offset: GLintptr,
        dst_buffer: T0,
        dst_offset: GLuint,
        length: GLuint,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.getBufferSubData(@{target}, @{(src_byte_offset as f64)}, @{unsafe { dst_buffer.as_array_buffer_view() }}, @{dst_offset}, @{length}); );
    }

    pub fn get_context_attributes(&self) -> Option<WebGLContextAttributes> {
        (js! { return @{self}.getContextAttributes(); })
            .try_into()
            .ok()
    }

    pub fn get_error(&self) -> GLenum {
        (js! { return @{self}.getError(); }).try_into().unwrap()
    }

    pub fn get_extension<E: Extension>(&self) -> Option<E> {
        (js! { return @{self}.getExtension(@{E::NAME}); })
            .try_into()
            .ok()
    }

    pub fn get_frag_data_location(&self, program: &WebGLProgram, name: &str) -> GLint {
        (js! { return @{self}.getFragDataLocation(@{program}, @{name}); })
            .try_into()
            .unwrap()
    }

    pub fn get_framebuffer_attachment_parameter(
        &self,
        target: GLenum,
        attachment: GLenum,
        pname: GLenum,
    ) -> Value {
        (js! { return @{self}.getFramebufferAttachmentParameter(@{target}, @{attachment}, @{pname}); } ).try_into().unwrap()
    }

    pub fn get_indexed_parameter(&self, target: GLenum, index: GLuint) -> Value {
        (js! { return @{self}.getIndexedParameter(@{target}, @{index}); })
            .try_into()
            .unwrap()
    }

    pub fn get_internalformat_parameter(
        &self,
        target: GLenum,
        internalformat: GLenum,
        pname: GLenum,
    ) -> Value {
        (js! { return @{self}.getInternalformatParameter(@{target}, @{internalformat}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_parameter(&self, pname: GLenum) -> Value {
        (js! { return @{self}.getParameter(@{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_program_info_log(&self, program: &WebGLProgram) -> Option<String> {
        (js! { return @{self}.getProgramInfoLog(@{program}); })
            .try_into()
            .ok()
    }

    pub fn get_program_parameter(&self, program: &WebGLProgram, pname: GLenum) -> Value {
        (js! { return @{self}.getProgramParameter(@{program}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_query(&self, target: GLenum, pname: GLenum) -> Option<WebGLQuery> {
        (js! { return @{self}.getQuery(@{target}, @{pname}); })
            .try_into()
            .ok()
    }

    pub fn get_query_parameter(&self, query: &WebGLQuery, pname: GLenum) -> Value {
        (js! { return @{self}.getQueryParameter(@{query}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_renderbuffer_parameter(&self, target: GLenum, pname: GLenum) -> Value {
        (js! { return @{self}.getRenderbufferParameter(@{target}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_sampler_parameter(&self, sampler: &WebGLSampler, pname: GLenum) -> Value {
        (js! { return @{self}.getSamplerParameter(@{sampler}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_shader_info_log(&self, shader: &WebGLShader) -> Option<String> {
        (js! { return @{self}.getShaderInfoLog(@{shader}); })
            .try_into()
            .ok()
    }

    pub fn get_shader_parameter(&self, shader: &WebGLShader, pname: GLenum) -> Value {
        (js! { return @{self}.getShaderParameter(@{shader}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_shader_precision_format(
        &self,
        shadertype: GLenum,
        precisiontype: GLenum,
    ) -> Option<WebGLShaderPrecisionFormat> {
        (js! { return @{self}.getShaderPrecisionFormat(@{shadertype}, @{precisiontype}); })
            .try_into()
            .ok()
    }

    pub fn get_shader_source(&self, shader: &WebGLShader) -> Option<String> {
        (js! { return @{self}.getShaderSource(@{shader}); })
            .try_into()
            .ok()
    }

    pub fn get_supported_extensions(&self) -> Option<Vec<String>> {
        (js! { return @{self}.getSupportedExtensions(); })
            .try_into()
            .ok()
    }

    pub fn get_sync_parameter(&self, sync: &WebGLSync, pname: GLenum) -> Value {
        (js! { return @{self}.getSyncParameter(@{sync}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_tex_parameter(&self, target: GLenum, pname: GLenum) -> Value {
        (js! { return @{self}.getTexParameter(@{target}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_transform_feedback_varying(
        &self,
        program: &WebGLProgram,
        index: GLuint,
    ) -> Option<WebGLActiveInfo> {
        (js! { return @{self}.getTransformFeedbackVarying(@{program}, @{index}); })
            .try_into()
            .ok()
    }

    pub fn get_uniform(&self, program: &WebGLProgram, location: &WebGLUniformLocation) -> Value {
        (js! { return @{self}.getUniform(@{program}, @{location}); })
            .try_into()
            .unwrap()
    }

    pub fn get_uniform_block_index(
        &self,
        program: &WebGLProgram,
        uniform_block_name: &str,
    ) -> GLuint {
        (js! { return @{self}.getUniformBlockIndex(@{program}, @{uniform_block_name}); })
            .try_into()
            .unwrap()
    }

    pub fn get_uniform_indices(
        &self,
        program: &WebGLProgram,
        uniform_names: &[&str],
    ) -> Option<Vec<GLuint>> {
        (js! { return @{self}.getUniformIndices(@{program}, @{uniform_names}); })
            .try_into()
            .ok()
    }

    pub fn get_uniform_location(
        &self,
        program: &WebGLProgram,
        name: &str,
    ) -> Option<WebGLUniformLocation> {
        (js! { return @{self}.getUniformLocation(@{program}, @{name}); })
            .try_into()
            .ok()
    }

    pub fn get_vertex_attrib(&self, index: GLuint, pname: GLenum) -> Value {
        (js! { return @{self}.getVertexAttrib(@{index}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_vertex_attrib_offset(&self, index: GLuint, pname: GLenum) -> GLintptr {
        (js! { return @{self}.getVertexAttribOffset(@{index}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn hint(&self, target: GLenum, mode: GLenum) {
        js!( @(no_return) @{self}.hint(@{target}, @{mode}); );
    }

    pub fn invalidate_framebuffer(&self, target: GLenum, attachments: &[GLenum]) {
        js!( @(no_return) @{self}.invalidateFramebuffer(@{target}, @{attachments}); );
    }

    pub fn invalidate_sub_framebuffer(
        &self,
        target: GLenum,
        attachments: &[GLenum],
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @(no_return) @{self}.invalidateSubFramebuffer(@{target}, @{attachments}, @{x}, @{y}, @{width}, @{height}); );
    }

    pub fn is_buffer(&self, buffer: Option<&WebGLBuffer>) -> GLboolean {
        (js! { return @{self}.isBuffer(@{buffer}); })
            .try_into()
            .unwrap()
    }

    pub fn is_context_lost(&self) -> bool {
        (js! { return @{self}.isContextLost(); })
            .try_into()
            .unwrap()
    }

    pub fn is_enabled(&self, cap: GLenum) -> GLboolean {
        (js! { return @{self}.isEnabled(@{cap}); })
            .try_into()
            .unwrap()
    }

    pub fn is_framebuffer(&self, framebuffer: Option<&WebGLFramebuffer>) -> GLboolean {
        (js! { return @{self}.isFramebuffer(@{framebuffer}); })
            .try_into()
            .unwrap()
    }

    pub fn is_program(&self, program: Option<&WebGLProgram>) -> GLboolean {
        (js! { return @{self}.isProgram(@{program}); })
            .try_into()
            .unwrap()
    }

    pub fn is_query(&self, query: Option<&WebGLQuery>) -> GLboolean {
        (js! { return @{self}.isQuery(@{query}); })
            .try_into()
            .unwrap()
    }

    pub fn is_renderbuffer(&self, renderbuffer: Option<&WebGLRenderbuffer>) -> GLboolean {
        (js! { return @{self}.isRenderbuffer(@{renderbuffer}); })
            .try_into()
            .unwrap()
    }

    pub fn is_sampler(&self, sampler: Option<&WebGLSampler>) -> GLboolean {
        (js! { return @{self}.isSampler(@{sampler}); })
            .try_into()
            .unwrap()
    }

    pub fn is_shader(&self, shader: Option<&WebGLShader>) -> GLboolean {
        (js! { return @{self}.isShader(@{shader}); })
            .try_into()
            .unwrap()
    }

    pub fn is_sync(&self, sync: Option<&WebGLSync>) -> GLboolean {
        (js! { return @{self}.isSync(@{sync}); })
            .try_into()
            .unwrap()
    }

    pub fn is_texture(&self, texture: Option<&WebGLTexture>) -> GLboolean {
        (js! { return @{self}.isTexture(@{texture}); })
            .try_into()
            .unwrap()
    }

    pub fn is_transform_feedback(&self, tf: Option<&WebGLTransformFeedback>) -> GLboolean {
        (js! { return @{self}.isTransformFeedback(@{tf}); })
            .try_into()
            .unwrap()
    }

    pub fn is_vertex_array(&self, vertex_array: Option<&WebGLVertexArrayObject>) -> GLboolean {
        (js! { return @{self}.isVertexArray(@{vertex_array}); })
            .try_into()
            .unwrap()
    }

    pub fn line_width(&self, width: GLfloat) {
        js!( @(no_return) @{self}.lineWidth(@{width}); );
    }

    pub fn link_program(&self, program: &WebGLProgram) {
        js!( @(no_return) @{self}.linkProgram(@{program}); );
    }

    pub fn pause_transform_feedback(&self) {
        js!( @(no_return) @{self}.pauseTransformFeedback(); );
    }

    pub fn pixel_storei(&self, pname: GLenum, param: GLint) {
        js!( @(no_return) @{self}.pixelStorei(@{pname}, @{param}); );
    }

    pub fn polygon_offset(&self, factor: GLfloat, units: GLfloat) {
        js!( @(no_return) @{self}.polygonOffset(@{factor}, @{units}); );
    }

    pub fn read_buffer(&self, src: GLenum) {
        js!( @(no_return) @{self}.readBuffer(@{src}); );
    }

    pub fn read_pixels<'a0, T0>(
        &self,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        dst_data: Option<T0>,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.readPixels(@{x}, @{y}, @{width}, @{height}, @{format}, @{type_}, @{dst_data.map(|inner| unsafe { inner.as_array_buffer_view() })}); );
    }

    pub fn read_pixels_1(
        &self,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        offset: GLintptr,
    ) {
        js!( @(no_return) @{self}.readPixels(@{x}, @{y}, @{width}, @{height}, @{format}, @{type_}, @{(offset as f64)}); );
    }

    pub fn read_pixels_2<'a0, T0>(
        &self,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        dst_data: T0,
        dst_offset: GLuint,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.readPixels(@{x}, @{y}, @{width}, @{height}, @{format}, @{type_}, @{unsafe { dst_data.as_array_buffer_view() }}, @{dst_offset}); );
    }

    pub fn renderbuffer_storage(
        &self,
        target: GLenum,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @(no_return) @{self}.renderbufferStorage(@{target}, @{internalformat}, @{width}, @{height}); );
    }

    pub fn renderbuffer_storage_multisample(
        &self,
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @(no_return) @{self}.renderbufferStorageMultisample(@{target}, @{samples}, @{internalformat}, @{width}, @{height}); );
    }

    pub fn resume_transform_feedback(&self) {
        js!( @(no_return) @{self}.resumeTransformFeedback(); );
    }

    pub fn sample_coverage(&self, value: GLclampf, invert: GLboolean) {
        js!( @(no_return) @{self}.sampleCoverage(@{value}, @{invert}); );
    }

    pub fn sampler_parameterf(&self, sampler: &WebGLSampler, pname: GLenum, param: GLfloat) {
        js!( @(no_return) @{self}.samplerParameterf(@{sampler}, @{pname}, @{param}); );
    }

    pub fn sampler_parameteri(&self, sampler: &WebGLSampler, pname: GLenum, param: GLint) {
        js!( @(no_return) @{self}.samplerParameteri(@{sampler}, @{pname}, @{param}); );
    }

    pub fn scissor(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        js!( @(no_return) @{self}.scissor(@{x}, @{y}, @{width}, @{height}); );
    }

    pub fn shader_source(&self, shader: &WebGLShader, source: &str) {
        js!( @(no_return) @{self}.shaderSource(@{shader}, @{source}); );
    }

    pub fn stencil_func(&self, func: GLenum, ref_: GLint, mask: GLuint) {
        js!( @(no_return) @{self}.stencilFunc(@{func}, @{ref_}, @{mask}); );
    }

    pub fn stencil_func_separate(&self, face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) {
        js!( @(no_return) @{self}.stencilFuncSeparate(@{face}, @{func}, @{ref_}, @{mask}); );
    }

    pub fn stencil_mask(&self, mask: GLuint) {
        js!( @(no_return) @{self}.stencilMask(@{mask}); );
    }

    pub fn stencil_mask_separate(&self, face: GLenum, mask: GLuint) {
        js!( @(no_return) @{self}.stencilMaskSeparate(@{face}, @{mask}); );
    }

    pub fn stencil_op(&self, fail: GLenum, zfail: GLenum, zpass: GLenum) {
        js!( @(no_return) @{self}.stencilOp(@{fail}, @{zfail}, @{zpass}); );
    }

    pub fn stencil_op_separate(&self, face: GLenum, fail: GLenum, zfail: GLenum, zpass: GLenum) {
        js!( @(no_return) @{self}.stencilOpSeparate(@{face}, @{fail}, @{zfail}, @{zpass}); );
    }

    pub fn tex_image2_d<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: Option<T0>,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{format}, @{type_}, @{pixels.map(|inner| unsafe { inner.as_array_buffer_view() })}); );
    }

    pub fn tex_image2_d_1<T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @(no_return) @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_image2_d_2(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pbo_offset: GLintptr,
    ) {
        js!( @(no_return) @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{format}, @{type_}, @{(pbo_offset as f64)}); );
    }

    pub fn tex_image2_d_3<T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @(no_return) @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_image2_d_4<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        src_data: T0,
        src_offset: GLuint,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{format}, @{type_}, @{unsafe { src_data.as_array_buffer_view() }}, @{src_offset}); );
    }

    pub fn tex_image3_d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pbo_offset: GLintptr,
    ) {
        js!( @(no_return) @{self}.texImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{format}, @{type_}, @{(pbo_offset as f64)}); );
    }

    pub fn tex_image3_d_1<T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @(no_return) @{self}.texImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_image3_d_2<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        src_data: Option<T0>,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.texImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{format}, @{type_}, @{src_data.map(|inner| unsafe { inner.as_array_buffer_view() })}); );
    }

    pub fn tex_image3_d_3<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        src_data: T0,
        src_offset: GLuint,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.texImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{format}, @{type_}, @{unsafe { src_data.as_array_buffer_view() }}, @{src_offset}); );
    }

    pub fn tex_parameterf(&self, target: GLenum, pname: GLenum, param: GLfloat) {
        js!( @(no_return) @{self}.texParameterf(@{target}, @{pname}, @{param}); );
    }

    pub fn tex_parameteri(&self, target: GLenum, pname: GLenum, param: GLint) {
        js!( @(no_return) @{self}.texParameteri(@{target}, @{pname}, @{param}); );
    }

    pub fn tex_storage2_d(
        &self,
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @(no_return) @{self}.texStorage2D(@{target}, @{levels}, @{internalformat}, @{width}, @{height}); );
    }

    pub fn tex_storage3_d(
        &self,
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
    ) {
        js!( @(no_return) @{self}.texStorage3D(@{target}, @{levels}, @{internalformat}, @{width}, @{height}, @{depth}); );
    }

    pub fn tex_sub_image2_d<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: Option<T0>,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{type_}, @{pixels.map(|inner| unsafe { inner.as_array_buffer_view() })}); );
    }

    pub fn tex_sub_image2_d_1<T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @(no_return) @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_sub_image2_d_2(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pbo_offset: GLintptr,
    ) {
        js!( @(no_return) @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{type_}, @{(pbo_offset as f64)}); );
    }

    pub fn tex_sub_image2_d_3<T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @(no_return) @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_sub_image2_d_4<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        src_data: T0,
        src_offset: GLuint,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{type_}, @{unsafe { src_data.as_array_buffer_view() }}, @{src_offset}); );
    }

    pub fn tex_sub_image3_d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        type_: GLenum,
        pbo_offset: GLintptr,
    ) {
        js!( @(no_return) @{self}.texSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{width}, @{height}, @{depth}, @{format}, @{type_}, @{(pbo_offset as f64)}); );
    }

    pub fn tex_sub_image3_d_1<T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @(no_return) @{self}.texSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{width}, @{height}, @{depth}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_sub_image3_d_2<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        type_: GLenum,
        src_data: Option<T0>,
        src_offset: GLuint,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.texSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{width}, @{height}, @{depth}, @{format}, @{type_}, @{src_data.map(|inner| unsafe { inner.as_array_buffer_view() })}, @{src_offset}); );
    }

    pub fn transform_feedback_varyings(
        &self,
        program: &WebGLProgram,
        varyings: &[&str],
        buffer_mode: GLenum,
    ) {
        js!( @(no_return) @{self}.transformFeedbackVaryings(@{program}, @{varyings}, @{buffer_mode}); );
    }

    pub fn uniform1f(&self, location: Option<&WebGLUniformLocation>, x: GLfloat) {
        js!( @(no_return) @{self}.uniform1f(@{location}, @{x}); );
    }

    pub fn uniform1fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniform1fv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform1fv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniform1fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform1i(&self, location: Option<&WebGLUniformLocation>, x: GLint) {
        js!( @(no_return) @{self}.uniform1i(@{location}, @{x}); );
    }

    pub fn uniform1iv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @(no_return) @{self}.uniform1iv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform1iv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @(no_return) @{self}.uniform1iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform1ui(&self, location: Option<&WebGLUniformLocation>, v0: GLuint) {
        js!( @(no_return) @{self}.uniform1ui(@{location}, @{v0}); );
    }

    pub fn uniform1uiv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, u32>,
    {
        js!( @(no_return) @{self}.uniform1uiv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform2f(&self, location: Option<&WebGLUniformLocation>, x: GLfloat, y: GLfloat) {
        js!( @(no_return) @{self}.uniform2f(@{location}, @{x}, @{y}); );
    }

    pub fn uniform2fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniform2fv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform2fv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniform2fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform2i(&self, location: Option<&WebGLUniformLocation>, x: GLint, y: GLint) {
        js!( @(no_return) @{self}.uniform2i(@{location}, @{x}, @{y}); );
    }

    pub fn uniform2iv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @(no_return) @{self}.uniform2iv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform2iv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @(no_return) @{self}.uniform2iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform2ui(&self, location: Option<&WebGLUniformLocation>, v0: GLuint, v1: GLuint) {
        js!( @(no_return) @{self}.uniform2ui(@{location}, @{v0}, @{v1}); );
    }

    pub fn uniform2uiv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, u32>,
    {
        js!( @(no_return) @{self}.uniform2uiv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform3f(
        &self,
        location: Option<&WebGLUniformLocation>,
        x: GLfloat,
        y: GLfloat,
        z: GLfloat,
    ) {
        js!( @(no_return) @{self}.uniform3f(@{location}, @{x}, @{y}, @{z}); );
    }

    pub fn uniform3fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniform3fv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform3fv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniform3fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform3i(&self, location: Option<&WebGLUniformLocation>, x: GLint, y: GLint, z: GLint) {
        js!( @(no_return) @{self}.uniform3i(@{location}, @{x}, @{y}, @{z}); );
    }

    pub fn uniform3iv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @(no_return) @{self}.uniform3iv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform3iv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @(no_return) @{self}.uniform3iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform3ui(
        &self,
        location: Option<&WebGLUniformLocation>,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
    ) {
        js!( @(no_return) @{self}.uniform3ui(@{location}, @{v0}, @{v1}, @{v2}); );
    }

    pub fn uniform3uiv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, u32>,
    {
        js!( @(no_return) @{self}.uniform3uiv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform4f(
        &self,
        location: Option<&WebGLUniformLocation>,
        x: GLfloat,
        y: GLfloat,
        z: GLfloat,
        w: GLfloat,
    ) {
        js!( @(no_return) @{self}.uniform4f(@{location}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn uniform4fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniform4fv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform4fv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniform4fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform4i(
        &self,
        location: Option<&WebGLUniformLocation>,
        x: GLint,
        y: GLint,
        z: GLint,
        w: GLint,
    ) {
        js!( @(no_return) @{self}.uniform4i(@{location}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn uniform4iv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @(no_return) @{self}.uniform4iv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform4iv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @(no_return) @{self}.uniform4iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform4ui(
        &self,
        location: Option<&WebGLUniformLocation>,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
        v3: GLuint,
    ) {
        js!( @(no_return) @{self}.uniform4ui(@{location}, @{v0}, @{v1}, @{v2}, @{v3}); );
    }

    pub fn uniform4uiv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, u32>,
    {
        js!( @(no_return) @{self}.uniform4uiv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_block_binding(
        &self,
        program: &WebGLProgram,
        uniform_block_index: GLuint,
        uniform_block_binding: GLuint,
    ) {
        js!( @(no_return) @{self}.uniformBlockBinding(@{program}, @{uniform_block_index}, @{uniform_block_binding}); );
    }

    pub fn uniform_matrix2fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniformMatrix2fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix2fv_1<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        value: T0,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniformMatrix2fv(@{location}, @{transpose}, @{unsafe { value.as_typed_array() }}); );
    }

    pub fn uniform_matrix2x3fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniformMatrix2x3fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix2x4fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniformMatrix2x4fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix3fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniformMatrix3fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix3fv_1<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        value: T0,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniformMatrix3fv(@{location}, @{transpose}, @{unsafe { value.as_typed_array() }}); );
    }

    pub fn uniform_matrix3x2fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniformMatrix3x2fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix3x4fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniformMatrix3x4fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix4fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniformMatrix4fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix4fv_1<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        value: T0,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniformMatrix4fv(@{location}, @{transpose}, @{unsafe { value.as_typed_array() }}); );
    }

    pub fn uniform_matrix4x2fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniformMatrix4x2fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix4x3fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        data: T0,
        src_offset: GLuint,
        src_length: GLuint,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniformMatrix4x3fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn use_program(&self, program: Option<&WebGLProgram>) {
        js!( @(no_return) @{self}.useProgram(@{program}); );
    }

    pub fn validate_program(&self, program: &WebGLProgram) {
        js!( @(no_return) @{self}.validateProgram(@{program}); );
    }

    pub fn vertex_attrib1f(&self, index: GLuint, x: GLfloat) {
        js!( @(no_return) @{self}.vertexAttrib1f(@{index}, @{x}); );
    }

    pub fn vertex_attrib1fv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.vertexAttrib1fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib2f(&self, index: GLuint, x: GLfloat, y: GLfloat) {
        js!( @(no_return) @{self}.vertexAttrib2f(@{index}, @{x}, @{y}); );
    }

    pub fn vertex_attrib2fv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.vertexAttrib2fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib3f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
        js!( @(no_return) @{self}.vertexAttrib3f(@{index}, @{x}, @{y}, @{z}); );
    }

    pub fn vertex_attrib3fv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.vertexAttrib3fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib4f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
        js!( @(no_return) @{self}.vertexAttrib4f(@{index}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn vertex_attrib4fv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.vertexAttrib4fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib_divisor(&self, index: GLuint, divisor: GLuint) {
        js!( @(no_return) @{self}.vertexAttribDivisor(@{index}, @{divisor}); );
    }

    pub fn vertex_attrib_i4i(&self, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) {
        js!( @(no_return) @{self}.vertexAttribI4i(@{index}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn vertex_attrib_i4iv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @(no_return) @{self}.vertexAttribI4iv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib_i4ui(&self, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) {
        js!( @(no_return) @{self}.vertexAttribI4ui(@{index}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn vertex_attrib_i4uiv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, u32>,
    {
        js!( @(no_return) @{self}.vertexAttribI4uiv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib_i_pointer(
        &self,
        index: GLuint,
        size: GLint,
        type_: GLenum,
        stride: GLsizei,
        offset: GLintptr,
    ) {
        js!( @(no_return) @{self}.vertexAttribIPointer(@{index}, @{size}, @{type_}, @{stride}, @{(offset as f64)}); );
    }

    pub fn vertex_attrib_pointer(
        &self,
        index: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        offset: GLintptr,
    ) {
        js!( @(no_return) @{self}.vertexAttribPointer(@{index}, @{size}, @{type_}, @{normalized}, @{stride}, @{(offset as f64)}); );
    }

    pub fn viewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        js!( @(no_return) @{self}.viewport(@{x}, @{y}, @{width}, @{height}); );
    }

    pub fn wait_sync(&self, sync: &WebGLSync, flags: GLbitfield, timeout: GLint64) {
        js!( @(no_return) @{self}.waitSync(@{sync}, @{flags}, @{(timeout as f64)}); );
    }
}

impl RenderingContext for WebGL2RenderingContext {
    type Error = ConversionError;
    fn from_canvas(canvas: &CanvasElement) -> Result<Self, ConversionError> {
        js!(
            return @{canvas}.getContext("webgl2");
        )
        .try_into()
    }
}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLActiveInfo")]
pub struct WebGLActiveInfo(Reference);

impl WebGLActiveInfo {
    pub fn name(&self) -> String {
        (js! { return @{self}.name; }).try_into().unwrap()
    }

    pub fn size(&self) -> GLint {
        (js! { return @{self}.size; }).try_into().unwrap()
    }

    pub fn type_(&self) -> GLenum {
        (js! { return @{self}.type; }).try_into().unwrap()
    }
}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLBuffer")]
pub struct WebGLBuffer(Reference);

impl WebGLBuffer {}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLContextEvent")]
pub struct WebGLContextEvent(Reference);

impl WebGLContextEvent {
    pub fn status_message(&self) -> String {
        (js! { return @{self}.statusMessage; }).try_into().unwrap()
    }
}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLFramebuffer")]
pub struct WebGLFramebuffer(Reference);

impl WebGLFramebuffer {}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLProgram")]
pub struct WebGLProgram(Reference);

impl WebGLProgram {}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLQuery")]
pub struct WebGLQuery(Reference);

impl WebGLQuery {}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLRenderbuffer")]
pub struct WebGLRenderbuffer(Reference);

impl WebGLRenderbuffer {}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLRenderingContext")]
pub struct WebGLRenderingContext(Reference);

impl WebGLRenderingContext {
    pub const ACTIVE_ATTRIBUTES: GLenum = 35721;
    pub const ACTIVE_TEXTURE: GLenum = 34016;
    pub const ACTIVE_UNIFORMS: GLenum = 35718;
    pub const ALIASED_LINE_WIDTH_RANGE: GLenum = 33902;
    pub const ALIASED_POINT_SIZE_RANGE: GLenum = 33901;
    pub const ALPHA: GLenum = 6406;
    pub const ALPHA_BITS: GLenum = 3413;
    pub const ALWAYS: GLenum = 519;
    pub const ARRAY_BUFFER: GLenum = 34962;
    pub const ARRAY_BUFFER_BINDING: GLenum = 34964;
    pub const ATTACHED_SHADERS: GLenum = 35717;
    pub const BACK: GLenum = 1029;
    pub const BLEND: GLenum = 3042;
    pub const BLEND_COLOR: GLenum = 32773;
    pub const BLEND_DST_ALPHA: GLenum = 32970;
    pub const BLEND_DST_RGB: GLenum = 32968;
    pub const BLEND_EQUATION: GLenum = 32777;
    pub const BLEND_EQUATION_ALPHA: GLenum = 34877;
    pub const BLEND_EQUATION_RGB: GLenum = 32777;
    pub const BLEND_SRC_ALPHA: GLenum = 32971;
    pub const BLEND_SRC_RGB: GLenum = 32969;
    pub const BLUE_BITS: GLenum = 3412;
    pub const BOOL: GLenum = 35670;
    pub const BOOL_VEC2: GLenum = 35671;
    pub const BOOL_VEC3: GLenum = 35672;
    pub const BOOL_VEC4: GLenum = 35673;
    pub const BROWSER_DEFAULT_WEBGL: GLenum = 37444;
    pub const BUFFER_SIZE: GLenum = 34660;
    pub const BUFFER_USAGE: GLenum = 34661;
    pub const BYTE: GLenum = 5120;
    pub const CCW: GLenum = 2305;
    pub const CLAMP_TO_EDGE: GLenum = 33071;
    pub const COLOR_ATTACHMENT0: GLenum = 36064;
    pub const COLOR_BUFFER_BIT: GLenum = 16384;
    pub const COLOR_CLEAR_VALUE: GLenum = 3106;
    pub const COLOR_WRITEMASK: GLenum = 3107;
    pub const COMPILE_STATUS: GLenum = 35713;
    pub const COMPRESSED_TEXTURE_FORMATS: GLenum = 34467;
    pub const CONSTANT_ALPHA: GLenum = 32771;
    pub const CONSTANT_COLOR: GLenum = 32769;
    pub const CONTEXT_LOST_WEBGL: GLenum = 37442;
    pub const CULL_FACE: GLenum = 2884;
    pub const CULL_FACE_MODE: GLenum = 2885;
    pub const CURRENT_PROGRAM: GLenum = 35725;
    pub const CURRENT_VERTEX_ATTRIB: GLenum = 34342;
    pub const CW: GLenum = 2304;
    pub const DECR: GLenum = 7683;
    pub const DECR_WRAP: GLenum = 34056;
    pub const DELETE_STATUS: GLenum = 35712;
    pub const DEPTH_ATTACHMENT: GLenum = 36096;
    pub const DEPTH_BITS: GLenum = 3414;
    pub const DEPTH_BUFFER_BIT: GLenum = 256;
    pub const DEPTH_CLEAR_VALUE: GLenum = 2931;
    pub const DEPTH_COMPONENT: GLenum = 6402;
    pub const DEPTH_COMPONENT16: GLenum = 33189;
    pub const DEPTH_FUNC: GLenum = 2932;
    pub const DEPTH_RANGE: GLenum = 2928;
    pub const DEPTH_STENCIL: GLenum = 34041;
    pub const DEPTH_STENCIL_ATTACHMENT: GLenum = 33306;
    pub const DEPTH_TEST: GLenum = 2929;
    pub const DEPTH_WRITEMASK: GLenum = 2930;
    pub const DITHER: GLenum = 3024;
    pub const DONT_CARE: GLenum = 4352;
    pub const DST_ALPHA: GLenum = 772;
    pub const DST_COLOR: GLenum = 774;
    pub const DYNAMIC_DRAW: GLenum = 35048;
    pub const ELEMENT_ARRAY_BUFFER: GLenum = 34963;
    pub const ELEMENT_ARRAY_BUFFER_BINDING: GLenum = 34965;
    pub const EQUAL: GLenum = 514;
    pub const FASTEST: GLenum = 4353;
    pub const FLOAT: GLenum = 5126;
    pub const FLOAT_MAT2: GLenum = 35674;
    pub const FLOAT_MAT3: GLenum = 35675;
    pub const FLOAT_MAT4: GLenum = 35676;
    pub const FLOAT_VEC2: GLenum = 35664;
    pub const FLOAT_VEC3: GLenum = 35665;
    pub const FLOAT_VEC4: GLenum = 35666;
    pub const FRAGMENT_SHADER: GLenum = 35632;
    pub const FRAMEBUFFER: GLenum = 36160;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLenum = 36049;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLenum = 36048;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLenum = 36051;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLenum = 36050;
    pub const FRAMEBUFFER_BINDING: GLenum = 36006;
    pub const FRAMEBUFFER_COMPLETE: GLenum = 36053;
    pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = 36054;
    pub const FRAMEBUFFER_INCOMPLETE_DIMENSIONS: GLenum = 36057;
    pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = 36055;
    pub const FRAMEBUFFER_UNSUPPORTED: GLenum = 36061;
    pub const FRONT: GLenum = 1028;
    pub const FRONT_AND_BACK: GLenum = 1032;
    pub const FRONT_FACE: GLenum = 2886;
    pub const FUNC_ADD: GLenum = 32774;
    pub const FUNC_REVERSE_SUBTRACT: GLenum = 32779;
    pub const FUNC_SUBTRACT: GLenum = 32778;
    pub const GENERATE_MIPMAP_HINT: GLenum = 33170;
    pub const GEQUAL: GLenum = 518;
    pub const GREATER: GLenum = 516;
    pub const GREEN_BITS: GLenum = 3411;
    pub const HIGH_FLOAT: GLenum = 36338;
    pub const HIGH_INT: GLenum = 36341;
    pub const IMPLEMENTATION_COLOR_READ_FORMAT: GLenum = 35739;
    pub const IMPLEMENTATION_COLOR_READ_TYPE: GLenum = 35738;
    pub const INCR: GLenum = 7682;
    pub const INCR_WRAP: GLenum = 34055;
    pub const INT: GLenum = 5124;
    pub const INT_VEC2: GLenum = 35667;
    pub const INT_VEC3: GLenum = 35668;
    pub const INT_VEC4: GLenum = 35669;
    pub const INVALID_ENUM: GLenum = 1280;
    pub const INVALID_FRAMEBUFFER_OPERATION: GLenum = 1286;
    pub const INVALID_OPERATION: GLenum = 1282;
    pub const INVALID_VALUE: GLenum = 1281;
    pub const INVERT: GLenum = 5386;
    pub const KEEP: GLenum = 7680;
    pub const LEQUAL: GLenum = 515;
    pub const LESS: GLenum = 513;
    pub const LINEAR: GLenum = 9729;
    pub const LINEAR_MIPMAP_LINEAR: GLenum = 9987;
    pub const LINEAR_MIPMAP_NEAREST: GLenum = 9985;
    pub const LINES: GLenum = 1;
    pub const LINE_LOOP: GLenum = 2;
    pub const LINE_STRIP: GLenum = 3;
    pub const LINE_WIDTH: GLenum = 2849;
    pub const LINK_STATUS: GLenum = 35714;
    pub const LOW_FLOAT: GLenum = 36336;
    pub const LOW_INT: GLenum = 36339;
    pub const LUMINANCE: GLenum = 6409;
    pub const LUMINANCE_ALPHA: GLenum = 6410;
    pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = 35661;
    pub const MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = 34076;
    pub const MAX_FRAGMENT_UNIFORM_VECTORS: GLenum = 36349;
    pub const MAX_RENDERBUFFER_SIZE: GLenum = 34024;
    pub const MAX_TEXTURE_IMAGE_UNITS: GLenum = 34930;
    pub const MAX_TEXTURE_SIZE: GLenum = 3379;
    pub const MAX_VARYING_VECTORS: GLenum = 36348;
    pub const MAX_VERTEX_ATTRIBS: GLenum = 34921;
    pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = 35660;
    pub const MAX_VERTEX_UNIFORM_VECTORS: GLenum = 36347;
    pub const MAX_VIEWPORT_DIMS: GLenum = 3386;
    pub const MEDIUM_FLOAT: GLenum = 36337;
    pub const MEDIUM_INT: GLenum = 36340;
    pub const MIRRORED_REPEAT: GLenum = 33648;
    pub const NEAREST: GLenum = 9728;
    pub const NEAREST_MIPMAP_LINEAR: GLenum = 9986;
    pub const NEAREST_MIPMAP_NEAREST: GLenum = 9984;
    pub const NEVER: GLenum = 512;
    pub const NICEST: GLenum = 4354;
    pub const NONE: GLenum = 0;
    pub const NOTEQUAL: GLenum = 517;
    pub const NO_ERROR: GLenum = 0;
    pub const ONE: GLenum = 1;
    pub const ONE_MINUS_CONSTANT_ALPHA: GLenum = 32772;
    pub const ONE_MINUS_CONSTANT_COLOR: GLenum = 32770;
    pub const ONE_MINUS_DST_ALPHA: GLenum = 773;
    pub const ONE_MINUS_DST_COLOR: GLenum = 775;
    pub const ONE_MINUS_SRC_ALPHA: GLenum = 771;
    pub const ONE_MINUS_SRC_COLOR: GLenum = 769;
    pub const OUT_OF_MEMORY: GLenum = 1285;
    pub const PACK_ALIGNMENT: GLenum = 3333;
    pub const POINTS: GLenum = 0;
    pub const POLYGON_OFFSET_FACTOR: GLenum = 32824;
    pub const POLYGON_OFFSET_FILL: GLenum = 32823;
    pub const POLYGON_OFFSET_UNITS: GLenum = 10752;
    pub const RED_BITS: GLenum = 3410;
    pub const RENDERBUFFER: GLenum = 36161;
    pub const RENDERBUFFER_ALPHA_SIZE: GLenum = 36179;
    pub const RENDERBUFFER_BINDING: GLenum = 36007;
    pub const RENDERBUFFER_BLUE_SIZE: GLenum = 36178;
    pub const RENDERBUFFER_DEPTH_SIZE: GLenum = 36180;
    pub const RENDERBUFFER_GREEN_SIZE: GLenum = 36177;
    pub const RENDERBUFFER_HEIGHT: GLenum = 36163;
    pub const RENDERBUFFER_INTERNAL_FORMAT: GLenum = 36164;
    pub const RENDERBUFFER_RED_SIZE: GLenum = 36176;
    pub const RENDERBUFFER_STENCIL_SIZE: GLenum = 36181;
    pub const RENDERBUFFER_WIDTH: GLenum = 36162;
    pub const RENDERER: GLenum = 7937;
    pub const REPEAT: GLenum = 10497;
    pub const REPLACE: GLenum = 7681;
    pub const RGB: GLenum = 6407;
    pub const RGB565: GLenum = 36194;
    pub const RGB5_A1: GLenum = 32855;
    pub const RGBA: GLenum = 6408;
    pub const RGBA4: GLenum = 32854;
    pub const SAMPLER_2D: GLenum = 35678;
    pub const SAMPLER_CUBE: GLenum = 35680;
    pub const SAMPLES: GLenum = 32937;
    pub const SAMPLE_ALPHA_TO_COVERAGE: GLenum = 32926;
    pub const SAMPLE_BUFFERS: GLenum = 32936;
    pub const SAMPLE_COVERAGE: GLenum = 32928;
    pub const SAMPLE_COVERAGE_INVERT: GLenum = 32939;
    pub const SAMPLE_COVERAGE_VALUE: GLenum = 32938;
    pub const SCISSOR_BOX: GLenum = 3088;
    pub const SCISSOR_TEST: GLenum = 3089;
    pub const SHADER_TYPE: GLenum = 35663;
    pub const SHADING_LANGUAGE_VERSION: GLenum = 35724;
    pub const SHORT: GLenum = 5122;
    pub const SRC_ALPHA: GLenum = 770;
    pub const SRC_ALPHA_SATURATE: GLenum = 776;
    pub const SRC_COLOR: GLenum = 768;
    pub const STATIC_DRAW: GLenum = 35044;
    pub const STENCIL_ATTACHMENT: GLenum = 36128;
    pub const STENCIL_BACK_FAIL: GLenum = 34817;
    pub const STENCIL_BACK_FUNC: GLenum = 34816;
    pub const STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = 34818;
    pub const STENCIL_BACK_PASS_DEPTH_PASS: GLenum = 34819;
    pub const STENCIL_BACK_REF: GLenum = 36003;
    pub const STENCIL_BACK_VALUE_MASK: GLenum = 36004;
    pub const STENCIL_BACK_WRITEMASK: GLenum = 36005;
    pub const STENCIL_BITS: GLenum = 3415;
    pub const STENCIL_BUFFER_BIT: GLenum = 1024;
    pub const STENCIL_CLEAR_VALUE: GLenum = 2961;
    pub const STENCIL_FAIL: GLenum = 2964;
    pub const STENCIL_FUNC: GLenum = 2962;
    pub const STENCIL_INDEX8: GLenum = 36168;
    pub const STENCIL_PASS_DEPTH_FAIL: GLenum = 2965;
    pub const STENCIL_PASS_DEPTH_PASS: GLenum = 2966;
    pub const STENCIL_REF: GLenum = 2967;
    pub const STENCIL_TEST: GLenum = 2960;
    pub const STENCIL_VALUE_MASK: GLenum = 2963;
    pub const STENCIL_WRITEMASK: GLenum = 2968;
    pub const STREAM_DRAW: GLenum = 35040;
    pub const SUBPIXEL_BITS: GLenum = 3408;
    pub const TEXTURE: GLenum = 5890;
    pub const TEXTURE0: GLenum = 33984;
    pub const TEXTURE1: GLenum = 33985;
    pub const TEXTURE10: GLenum = 33994;
    pub const TEXTURE11: GLenum = 33995;
    pub const TEXTURE12: GLenum = 33996;
    pub const TEXTURE13: GLenum = 33997;
    pub const TEXTURE14: GLenum = 33998;
    pub const TEXTURE15: GLenum = 33999;
    pub const TEXTURE16: GLenum = 34000;
    pub const TEXTURE17: GLenum = 34001;
    pub const TEXTURE18: GLenum = 34002;
    pub const TEXTURE19: GLenum = 34003;
    pub const TEXTURE2: GLenum = 33986;
    pub const TEXTURE20: GLenum = 34004;
    pub const TEXTURE21: GLenum = 34005;
    pub const TEXTURE22: GLenum = 34006;
    pub const TEXTURE23: GLenum = 34007;
    pub const TEXTURE24: GLenum = 34008;
    pub const TEXTURE25: GLenum = 34009;
    pub const TEXTURE26: GLenum = 34010;
    pub const TEXTURE27: GLenum = 34011;
    pub const TEXTURE28: GLenum = 34012;
    pub const TEXTURE29: GLenum = 34013;
    pub const TEXTURE3: GLenum = 33987;
    pub const TEXTURE30: GLenum = 34014;
    pub const TEXTURE31: GLenum = 34015;
    pub const TEXTURE4: GLenum = 33988;
    pub const TEXTURE5: GLenum = 33989;
    pub const TEXTURE6: GLenum = 33990;
    pub const TEXTURE7: GLenum = 33991;
    pub const TEXTURE8: GLenum = 33992;
    pub const TEXTURE9: GLenum = 33993;
    pub const TEXTURE_2D: GLenum = 3553;
    pub const TEXTURE_BINDING_2D: GLenum = 32873;
    pub const TEXTURE_BINDING_CUBE_MAP: GLenum = 34068;
    pub const TEXTURE_CUBE_MAP: GLenum = 34067;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = 34070;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = 34072;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = 34074;
    pub const TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = 34069;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = 34071;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = 34073;
    pub const TEXTURE_MAG_FILTER: GLenum = 10240;
    pub const TEXTURE_MIN_FILTER: GLenum = 10241;
    pub const TEXTURE_WRAP_S: GLenum = 10242;
    pub const TEXTURE_WRAP_T: GLenum = 10243;
    pub const TRIANGLES: GLenum = 4;
    pub const TRIANGLE_FAN: GLenum = 6;
    pub const TRIANGLE_STRIP: GLenum = 5;
    pub const UNPACK_ALIGNMENT: GLenum = 3317;
    pub const UNPACK_COLORSPACE_CONVERSION_WEBGL: GLenum = 37443;
    pub const UNPACK_FLIP_Y_WEBGL: GLenum = 37440;
    pub const UNPACK_PREMULTIPLY_ALPHA_WEBGL: GLenum = 37441;
    pub const UNSIGNED_BYTE: GLenum = 5121;
    pub const UNSIGNED_INT: GLenum = 5125;
    pub const UNSIGNED_SHORT: GLenum = 5123;
    pub const UNSIGNED_SHORT_4_4_4_4: GLenum = 32819;
    pub const UNSIGNED_SHORT_5_5_5_1: GLenum = 32820;
    pub const UNSIGNED_SHORT_5_6_5: GLenum = 33635;
    pub const VALIDATE_STATUS: GLenum = 35715;
    pub const VENDOR: GLenum = 7936;
    pub const VERSION: GLenum = 7938;
    pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = 34975;
    pub const VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = 34338;
    pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = 34922;
    pub const VERTEX_ATTRIB_ARRAY_POINTER: GLenum = 34373;
    pub const VERTEX_ATTRIB_ARRAY_SIZE: GLenum = 34339;
    pub const VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = 34340;
    pub const VERTEX_ATTRIB_ARRAY_TYPE: GLenum = 34341;
    pub const VERTEX_SHADER: GLenum = 35633;
    pub const VIEWPORT: GLenum = 2978;
    pub const ZERO: GLenum = 0;

    pub fn active_texture(&self, texture: GLenum) {
        js!( @(no_return) @{self}.activeTexture(@{texture}); );
    }

    pub fn attach_shader(&self, program: &WebGLProgram, shader: &WebGLShader) {
        js!( @(no_return) @{self}.attachShader(@{program}, @{shader}); );
    }

    pub fn bind_attrib_location(&self, program: &WebGLProgram, index: GLuint, name: &str) {
        js!( @(no_return) @{self}.bindAttribLocation(@{program}, @{index}, @{name}); );
    }

    pub fn bind_buffer(&self, target: GLenum, buffer: Option<&WebGLBuffer>) {
        js!( @(no_return) @{self}.bindBuffer(@{target}, @{buffer}); );
    }

    pub fn bind_framebuffer(&self, target: GLenum, framebuffer: Option<&WebGLFramebuffer>) {
        js!( @(no_return) @{self}.bindFramebuffer(@{target}, @{framebuffer}); );
    }

    pub fn bind_renderbuffer(&self, target: GLenum, renderbuffer: Option<&WebGLRenderbuffer>) {
        js!( @(no_return) @{self}.bindRenderbuffer(@{target}, @{renderbuffer}); );
    }

    pub fn bind_texture(&self, target: GLenum, texture: Option<&WebGLTexture>) {
        js!( @(no_return) @{self}.bindTexture(@{target}, @{texture}); );
    }

    pub fn blend_color(&self, red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
        js!( @(no_return) @{self}.blendColor(@{red}, @{green}, @{blue}, @{alpha}); );
    }

    pub fn blend_equation(&self, mode: GLenum) {
        js!( @(no_return) @{self}.blendEquation(@{mode}); );
    }

    pub fn blend_equation_separate(&self, mode_rgb: GLenum, mode_alpha: GLenum) {
        js!( @(no_return) @{self}.blendEquationSeparate(@{mode_rgb}, @{mode_alpha}); );
    }

    pub fn blend_func(&self, sfactor: GLenum, dfactor: GLenum) {
        js!( @(no_return) @{self}.blendFunc(@{sfactor}, @{dfactor}); );
    }

    pub fn blend_func_separate(
        &self,
        src_rgb: GLenum,
        dst_rgb: GLenum,
        src_alpha: GLenum,
        dst_alpha: GLenum,
    ) {
        js!( @(no_return) @{self}.blendFuncSeparate(@{src_rgb}, @{dst_rgb}, @{src_alpha}, @{dst_alpha}); );
    }

    pub fn buffer_data(&self, target: GLenum, size: GLsizeiptr, usage: GLenum) {
        js!( @(no_return) @{self}.bufferData(@{target}, @{(size as f64)}, @{usage}); );
    }

    pub fn buffer_data_1(&self, target: GLenum, data: Option<&ArrayBuffer>, usage: GLenum) {
        js!( @(no_return) @{self}.bufferData(@{target}, @{data}, @{usage}); );
    }

    pub fn buffer_sub_data(&self, target: GLenum, offset: GLintptr, data: &ArrayBuffer) {
        js!( @(no_return) @{self}.bufferSubData(@{target}, @{(offset as f64)}, @{data}); );
    }

    pub fn canvas(&self) -> CanvasElement {
        (js! { return @{self}.canvas; }).try_into().unwrap()
    }

    pub fn check_framebuffer_status(&self, target: GLenum) -> GLenum {
        (js! { return @{self}.checkFramebufferStatus(@{target}); })
            .try_into()
            .unwrap()
    }

    pub fn clear(&self, mask: GLbitfield) {
        js!( @(no_return) @{self}.clear(@{mask}); );
    }

    pub fn clear_color(&self, red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
        js!( @(no_return) @{self}.clearColor(@{red}, @{green}, @{blue}, @{alpha}); );
    }

    pub fn clear_depth(&self, depth: GLclampf) {
        js!( @(no_return) @{self}.clearDepth(@{depth}); );
    }

    pub fn clear_stencil(&self, s: GLint) {
        js!( @(no_return) @{self}.clearStencil(@{s}); );
    }

    pub fn color_mask(&self, red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) {
        js!( @(no_return) @{self}.colorMask(@{red}, @{green}, @{blue}, @{alpha}); );
    }

    pub fn compile_shader(&self, shader: &WebGLShader) {
        js!( @(no_return) @{self}.compileShader(@{shader}); );
    }

    pub fn compressed_tex_image2_d<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        data: T0,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.compressedTexImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{unsafe { data.as_array_buffer_view() }}); );
    }

    pub fn compressed_tex_sub_image2_d<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        data: T0,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.compressedTexSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{unsafe { data.as_array_buffer_view() }}); );
    }

    pub fn copy_tex_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
    ) {
        js!( @(no_return) @{self}.copyTexImage2D(@{target}, @{level}, @{internalformat}, @{x}, @{y}, @{width}, @{height}, @{border}); );
    }

    pub fn copy_tex_sub_image2_d(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @(no_return) @{self}.copyTexSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{x}, @{y}, @{width}, @{height}); );
    }

    pub fn create_buffer(&self) -> Option<WebGLBuffer> {
        (js! { return @{self}.createBuffer(); }).try_into().ok()
    }

    pub fn create_framebuffer(&self) -> Option<WebGLFramebuffer> {
        (js! { return @{self}.createFramebuffer(); })
            .try_into()
            .ok()
    }

    pub fn create_program(&self) -> Option<WebGLProgram> {
        (js! { return @{self}.createProgram(); }).try_into().ok()
    }

    pub fn create_renderbuffer(&self) -> Option<WebGLRenderbuffer> {
        (js! { return @{self}.createRenderbuffer(); })
            .try_into()
            .ok()
    }

    pub fn create_shader(&self, type_: GLenum) -> Option<WebGLShader> {
        (js! { return @{self}.createShader(@{type_}); })
            .try_into()
            .ok()
    }

    pub fn create_texture(&self) -> Option<WebGLTexture> {
        (js! { return @{self}.createTexture(); }).try_into().ok()
    }

    pub fn cull_face(&self, mode: GLenum) {
        js!( @(no_return) @{self}.cullFace(@{mode}); );
    }

    pub fn delete_buffer(&self, buffer: Option<&WebGLBuffer>) {
        js!( @(no_return) @{self}.deleteBuffer(@{buffer}); );
    }

    pub fn delete_framebuffer(&self, framebuffer: Option<&WebGLFramebuffer>) {
        js!( @(no_return) @{self}.deleteFramebuffer(@{framebuffer}); );
    }

    pub fn delete_program(&self, program: Option<&WebGLProgram>) {
        js!( @(no_return) @{self}.deleteProgram(@{program}); );
    }

    pub fn delete_renderbuffer(&self, renderbuffer: Option<&WebGLRenderbuffer>) {
        js!( @(no_return) @{self}.deleteRenderbuffer(@{renderbuffer}); );
    }

    pub fn delete_shader(&self, shader: Option<&WebGLShader>) {
        js!( @(no_return) @{self}.deleteShader(@{shader}); );
    }

    pub fn delete_texture(&self, texture: Option<&WebGLTexture>) {
        js!( @(no_return) @{self}.deleteTexture(@{texture}); );
    }

    pub fn depth_func(&self, func: GLenum) {
        js!( @(no_return) @{self}.depthFunc(@{func}); );
    }

    pub fn depth_mask(&self, flag: GLboolean) {
        js!( @(no_return) @{self}.depthMask(@{flag}); );
    }

    pub fn depth_range(&self, z_near: GLclampf, z_far: GLclampf) {
        js!( @(no_return) @{self}.depthRange(@{z_near}, @{z_far}); );
    }

    pub fn detach_shader(&self, program: &WebGLProgram, shader: &WebGLShader) {
        js!( @(no_return) @{self}.detachShader(@{program}, @{shader}); );
    }

    pub fn disable(&self, cap: GLenum) {
        js!( @(no_return) @{self}.disable(@{cap}); );
    }

    pub fn disable_vertex_attrib_array(&self, index: GLuint) {
        js!( @(no_return) @{self}.disableVertexAttribArray(@{index}); );
    }

    pub fn draw_arrays(&self, mode: GLenum, first: GLint, count: GLsizei) {
        js!( @(no_return) @{self}.drawArrays(@{mode}, @{first}, @{count}); );
    }

    pub fn draw_elements(&self, mode: GLenum, count: GLsizei, type_: GLenum, offset: GLintptr) {
        js!( @(no_return) @{self}.drawElements(@{mode}, @{count}, @{type_}, @{(offset as f64)}); );
    }

    pub fn drawing_buffer_height(&self) -> GLsizei {
        (js! { return @{self}.drawingBufferHeight; })
            .try_into()
            .unwrap()
    }

    pub fn drawing_buffer_width(&self) -> GLsizei {
        (js! { return @{self}.drawingBufferWidth; })
            .try_into()
            .unwrap()
    }

    pub fn enable(&self, cap: GLenum) {
        js!( @(no_return) @{self}.enable(@{cap}); );
    }

    pub fn enable_vertex_attrib_array(&self, index: GLuint) {
        js!( @(no_return) @{self}.enableVertexAttribArray(@{index}); );
    }

    pub fn finish(&self) {
        js!( @(no_return) @{self}.finish(); );
    }

    pub fn flush(&self) {
        js!( @(no_return) @{self}.flush(); );
    }

    pub fn framebuffer_renderbuffer(
        &self,
        target: GLenum,
        attachment: GLenum,
        renderbuffertarget: GLenum,
        renderbuffer: Option<&WebGLRenderbuffer>,
    ) {
        js!( @(no_return) @{self}.framebufferRenderbuffer(@{target}, @{attachment}, @{renderbuffertarget}, @{renderbuffer}); );
    }

    pub fn framebuffer_texture2_d(
        &self,
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: Option<&WebGLTexture>,
        level: GLint,
    ) {
        js!( @(no_return) @{self}.framebufferTexture2D(@{target}, @{attachment}, @{textarget}, @{texture}, @{level}); );
    }

    pub fn front_face(&self, mode: GLenum) {
        js!( @(no_return) @{self}.frontFace(@{mode}); );
    }

    pub fn generate_mipmap(&self, target: GLenum) {
        js!( @(no_return) @{self}.generateMipmap(@{target}); );
    }

    pub fn get_active_attrib(
        &self,
        program: &WebGLProgram,
        index: GLuint,
    ) -> Option<WebGLActiveInfo> {
        (js! { return @{self}.getActiveAttrib(@{program}, @{index}); })
            .try_into()
            .ok()
    }

    pub fn get_active_uniform(
        &self,
        program: &WebGLProgram,
        index: GLuint,
    ) -> Option<WebGLActiveInfo> {
        (js! { return @{self}.getActiveUniform(@{program}, @{index}); })
            .try_into()
            .ok()
    }

    pub fn get_attached_shaders(&self, program: &WebGLProgram) -> Option<Vec<WebGLShader>> {
        (js! { return @{self}.getAttachedShaders(@{program}); })
            .try_into()
            .ok()
    }

    pub fn get_attrib_location(&self, program: &WebGLProgram, name: &str) -> GLint {
        (js! { return @{self}.getAttribLocation(@{program}, @{name}); })
            .try_into()
            .unwrap()
    }

    pub fn get_buffer_parameter(&self, target: GLenum, pname: GLenum) -> Value {
        (js! { return @{self}.getBufferParameter(@{target}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_context_attributes(&self) -> Option<WebGLContextAttributes> {
        (js! { return @{self}.getContextAttributes(); })
            .try_into()
            .ok()
    }

    pub fn get_error(&self) -> GLenum {
        (js! { return @{self}.getError(); }).try_into().unwrap()
    }

    pub fn get_extension<E: Extension>(&self) -> Option<E> {
        (js! { return @{self}.getExtension(@{E::NAME}); })
            .try_into()
            .ok()
    }

    pub fn get_framebuffer_attachment_parameter(
        &self,
        target: GLenum,
        attachment: GLenum,
        pname: GLenum,
    ) -> Value {
        (js! { return @{self}.getFramebufferAttachmentParameter(@{target}, @{attachment}, @{pname}); } ).try_into().unwrap()
    }

    pub fn get_parameter(&self, pname: GLenum) -> Value {
        (js! { return @{self}.getParameter(@{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_program_info_log(&self, program: &WebGLProgram) -> Option<String> {
        (js! { return @{self}.getProgramInfoLog(@{program}); })
            .try_into()
            .ok()
    }

    pub fn get_program_parameter(&self, program: &WebGLProgram, pname: GLenum) -> Value {
        (js! { return @{self}.getProgramParameter(@{program}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_renderbuffer_parameter(&self, target: GLenum, pname: GLenum) -> Value {
        (js! { return @{self}.getRenderbufferParameter(@{target}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_shader_info_log(&self, shader: &WebGLShader) -> Option<String> {
        (js! { return @{self}.getShaderInfoLog(@{shader}); })
            .try_into()
            .ok()
    }

    pub fn get_shader_parameter(&self, shader: &WebGLShader, pname: GLenum) -> Value {
        (js! { return @{self}.getShaderParameter(@{shader}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_shader_precision_format(
        &self,
        shadertype: GLenum,
        precisiontype: GLenum,
    ) -> Option<WebGLShaderPrecisionFormat> {
        (js! { return @{self}.getShaderPrecisionFormat(@{shadertype}, @{precisiontype}); })
            .try_into()
            .ok()
    }

    pub fn get_shader_source(&self, shader: &WebGLShader) -> Option<String> {
        (js! { return @{self}.getShaderSource(@{shader}); })
            .try_into()
            .ok()
    }

    pub fn get_supported_extensions(&self) -> Option<Vec<String>> {
        (js! { return @{self}.getSupportedExtensions(); })
            .try_into()
            .ok()
    }

    pub fn get_tex_parameter(&self, target: GLenum, pname: GLenum) -> Value {
        (js! { return @{self}.getTexParameter(@{target}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_uniform(&self, program: &WebGLProgram, location: &WebGLUniformLocation) -> Value {
        (js! { return @{self}.getUniform(@{program}, @{location}); })
            .try_into()
            .unwrap()
    }

    pub fn get_uniform_location(
        &self,
        program: &WebGLProgram,
        name: &str,
    ) -> Option<WebGLUniformLocation> {
        (js! { return @{self}.getUniformLocation(@{program}, @{name}); })
            .try_into()
            .ok()
    }

    pub fn get_vertex_attrib(&self, index: GLuint, pname: GLenum) -> Value {
        (js! { return @{self}.getVertexAttrib(@{index}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn get_vertex_attrib_offset(&self, index: GLuint, pname: GLenum) -> GLintptr {
        (js! { return @{self}.getVertexAttribOffset(@{index}, @{pname}); })
            .try_into()
            .unwrap()
    }

    pub fn hint(&self, target: GLenum, mode: GLenum) {
        js!( @(no_return) @{self}.hint(@{target}, @{mode}); );
    }

    pub fn is_buffer(&self, buffer: Option<&WebGLBuffer>) -> GLboolean {
        (js! { return @{self}.isBuffer(@{buffer}); })
            .try_into()
            .unwrap()
    }

    pub fn is_context_lost(&self) -> bool {
        (js! { return @{self}.isContextLost(); })
            .try_into()
            .unwrap()
    }

    pub fn is_enabled(&self, cap: GLenum) -> GLboolean {
        (js! { return @{self}.isEnabled(@{cap}); })
            .try_into()
            .unwrap()
    }

    pub fn is_framebuffer(&self, framebuffer: Option<&WebGLFramebuffer>) -> GLboolean {
        (js! { return @{self}.isFramebuffer(@{framebuffer}); })
            .try_into()
            .unwrap()
    }

    pub fn is_program(&self, program: Option<&WebGLProgram>) -> GLboolean {
        (js! { return @{self}.isProgram(@{program}); })
            .try_into()
            .unwrap()
    }

    pub fn is_renderbuffer(&self, renderbuffer: Option<&WebGLRenderbuffer>) -> GLboolean {
        (js! { return @{self}.isRenderbuffer(@{renderbuffer}); })
            .try_into()
            .unwrap()
    }

    pub fn is_shader(&self, shader: Option<&WebGLShader>) -> GLboolean {
        (js! { return @{self}.isShader(@{shader}); })
            .try_into()
            .unwrap()
    }

    pub fn is_texture(&self, texture: Option<&WebGLTexture>) -> GLboolean {
        (js! { return @{self}.isTexture(@{texture}); })
            .try_into()
            .unwrap()
    }

    pub fn line_width(&self, width: GLfloat) {
        js!( @(no_return) @{self}.lineWidth(@{width}); );
    }

    pub fn link_program(&self, program: &WebGLProgram) {
        js!( @(no_return) @{self}.linkProgram(@{program}); );
    }

    pub fn pixel_storei(&self, pname: GLenum, param: GLint) {
        js!( @(no_return) @{self}.pixelStorei(@{pname}, @{param}); );
    }

    pub fn polygon_offset(&self, factor: GLfloat, units: GLfloat) {
        js!( @(no_return) @{self}.polygonOffset(@{factor}, @{units}); );
    }

    pub fn read_pixels<'a0, T0>(
        &self,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: Option<T0>,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.readPixels(@{x}, @{y}, @{width}, @{height}, @{format}, @{type_}, @{pixels.map(|inner| unsafe { inner.as_array_buffer_view() })}); );
    }

    pub fn renderbuffer_storage(
        &self,
        target: GLenum,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) {
        js!( @(no_return) @{self}.renderbufferStorage(@{target}, @{internalformat}, @{width}, @{height}); );
    }

    pub fn sample_coverage(&self, value: GLclampf, invert: GLboolean) {
        js!( @(no_return) @{self}.sampleCoverage(@{value}, @{invert}); );
    }

    pub fn scissor(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        js!( @(no_return) @{self}.scissor(@{x}, @{y}, @{width}, @{height}); );
    }

    pub fn shader_source(&self, shader: &WebGLShader, source: &str) {
        js!( @(no_return) @{self}.shaderSource(@{shader}, @{source}); );
    }

    pub fn stencil_func(&self, func: GLenum, ref_: GLint, mask: GLuint) {
        js!( @(no_return) @{self}.stencilFunc(@{func}, @{ref_}, @{mask}); );
    }

    pub fn stencil_func_separate(&self, face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) {
        js!( @(no_return) @{self}.stencilFuncSeparate(@{face}, @{func}, @{ref_}, @{mask}); );
    }

    pub fn stencil_mask(&self, mask: GLuint) {
        js!( @(no_return) @{self}.stencilMask(@{mask}); );
    }

    pub fn stencil_mask_separate(&self, face: GLenum, mask: GLuint) {
        js!( @(no_return) @{self}.stencilMaskSeparate(@{face}, @{mask}); );
    }

    pub fn stencil_op(&self, fail: GLenum, zfail: GLenum, zpass: GLenum) {
        js!( @(no_return) @{self}.stencilOp(@{fail}, @{zfail}, @{zpass}); );
    }

    pub fn stencil_op_separate(&self, face: GLenum, fail: GLenum, zfail: GLenum, zpass: GLenum) {
        js!( @(no_return) @{self}.stencilOpSeparate(@{face}, @{fail}, @{zfail}, @{zpass}); );
    }

    pub fn tex_image2_d<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: Option<T0>,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{format}, @{type_}, @{pixels.map(|inner| unsafe { inner.as_array_buffer_view() })}); );
    }

    pub fn tex_image2_d_1<T0>(
        &self,
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @(no_return) @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_parameterf(&self, target: GLenum, pname: GLenum, param: GLfloat) {
        js!( @(no_return) @{self}.texParameterf(@{target}, @{pname}, @{param}); );
    }

    pub fn tex_parameteri(&self, target: GLenum, pname: GLenum, param: GLint) {
        js!( @(no_return) @{self}.texParameteri(@{target}, @{pname}, @{param}); );
    }

    pub fn tex_sub_image2_d<'a0, T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: Option<T0>,
    ) where
        T0: AsArrayBufferView<'a0>,
    {
        js!( @(no_return) @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{type_}, @{pixels.map(|inner| unsafe { inner.as_array_buffer_view() })}); );
    }

    pub fn tex_sub_image2_d_1<T0>(
        &self,
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        format: GLenum,
        type_: GLenum,
        source: T0,
    ) where
        T0: JsSerialize,
    {
        js!( @(no_return) @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{format}, @{type_}, @{source}); );
    }

    pub fn uniform1f(&self, location: Option<&WebGLUniformLocation>, x: GLfloat) {
        js!( @(no_return) @{self}.uniform1f(@{location}, @{x}); );
    }

    pub fn uniform1fv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniform1fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform1i(&self, location: Option<&WebGLUniformLocation>, x: GLint) {
        js!( @(no_return) @{self}.uniform1i(@{location}, @{x}); );
    }

    pub fn uniform1iv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @(no_return) @{self}.uniform1iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform2f(&self, location: Option<&WebGLUniformLocation>, x: GLfloat, y: GLfloat) {
        js!( @(no_return) @{self}.uniform2f(@{location}, @{x}, @{y}); );
    }

    pub fn uniform2fv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniform2fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform2i(&self, location: Option<&WebGLUniformLocation>, x: GLint, y: GLint) {
        js!( @(no_return) @{self}.uniform2i(@{location}, @{x}, @{y}); );
    }

    pub fn uniform2iv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @(no_return) @{self}.uniform2iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform3f(
        &self,
        location: Option<&WebGLUniformLocation>,
        x: GLfloat,
        y: GLfloat,
        z: GLfloat,
    ) {
        js!( @(no_return) @{self}.uniform3f(@{location}, @{x}, @{y}, @{z}); );
    }

    pub fn uniform3fv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniform3fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform3i(&self, location: Option<&WebGLUniformLocation>, x: GLint, y: GLint, z: GLint) {
        js!( @(no_return) @{self}.uniform3i(@{location}, @{x}, @{y}, @{z}); );
    }

    pub fn uniform3iv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @(no_return) @{self}.uniform3iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform4f(
        &self,
        location: Option<&WebGLUniformLocation>,
        x: GLfloat,
        y: GLfloat,
        z: GLfloat,
        w: GLfloat,
    ) {
        js!( @(no_return) @{self}.uniform4f(@{location}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn uniform4fv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniform4fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform4i(
        &self,
        location: Option<&WebGLUniformLocation>,
        x: GLint,
        y: GLint,
        z: GLint,
        w: GLint,
    ) {
        js!( @(no_return) @{self}.uniform4i(@{location}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn uniform4iv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0)
    where
        T0: AsTypedArray<'a0, i32>,
    {
        js!( @(no_return) @{self}.uniform4iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform_matrix2fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        value: T0,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniformMatrix2fv(@{location}, @{transpose}, @{unsafe { value.as_typed_array() }}); );
    }

    pub fn uniform_matrix3fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        value: T0,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniformMatrix3fv(@{location}, @{transpose}, @{unsafe { value.as_typed_array() }}); );
    }

    pub fn uniform_matrix4fv<'a0, T0>(
        &self,
        location: Option<&WebGLUniformLocation>,
        transpose: GLboolean,
        value: T0,
    ) where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.uniformMatrix4fv(@{location}, @{transpose}, @{unsafe { value.as_typed_array() }}); );
    }

    pub fn use_program(&self, program: Option<&WebGLProgram>) {
        js!( @(no_return) @{self}.useProgram(@{program}); );
    }

    pub fn validate_program(&self, program: &WebGLProgram) {
        js!( @(no_return) @{self}.validateProgram(@{program}); );
    }

    pub fn vertex_attrib1f(&self, index: GLuint, x: GLfloat) {
        js!( @(no_return) @{self}.vertexAttrib1f(@{index}, @{x}); );
    }

    pub fn vertex_attrib1fv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.vertexAttrib1fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib2f(&self, index: GLuint, x: GLfloat, y: GLfloat) {
        js!( @(no_return) @{self}.vertexAttrib2f(@{index}, @{x}, @{y}); );
    }

    pub fn vertex_attrib2fv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.vertexAttrib2fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib3f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
        js!( @(no_return) @{self}.vertexAttrib3f(@{index}, @{x}, @{y}, @{z}); );
    }

    pub fn vertex_attrib3fv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.vertexAttrib3fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib4f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
        js!( @(no_return) @{self}.vertexAttrib4f(@{index}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn vertex_attrib4fv<'a0, T0>(&self, index: GLuint, values: T0)
    where
        T0: AsTypedArray<'a0, f32>,
    {
        js!( @(no_return) @{self}.vertexAttrib4fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib_pointer(
        &self,
        index: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        offset: GLintptr,
    ) {
        js!( @(no_return) @{self}.vertexAttribPointer(@{index}, @{size}, @{type_}, @{normalized}, @{stride}, @{(offset as f64)}); );
    }

    pub fn viewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        js!( @(no_return) @{self}.viewport(@{x}, @{y}, @{width}, @{height}); );
    }
}

impl RenderingContext for WebGLRenderingContext {
    type Error = ConversionError;
    fn from_canvas(canvas: &CanvasElement) -> Result<Self, ConversionError> {
        js!(
            return @{canvas}.getContext("webgl");
        )
        .try_into()
    }
}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLSampler")]
pub struct WebGLSampler(Reference);

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLShader")]
pub struct WebGLShader(Reference);

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLShaderPrecisionFormat")]
pub struct WebGLShaderPrecisionFormat(Reference);

impl WebGLShaderPrecisionFormat {
    pub fn precision(&self) -> GLint {
        (js! { return @{self}.precision; }).try_into().unwrap()
    }

    pub fn range_max(&self) -> GLint {
        (js! { return @{self}.rangeMax; }).try_into().unwrap()
    }

    pub fn range_min(&self) -> GLint {
        (js! { return @{self}.rangeMin; }).try_into().unwrap()
    }
}

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLSync")]
pub struct WebGLSync(Reference);

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLTexture")]
pub struct WebGLTexture(Reference);

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLTransformFeedback")]
pub struct WebGLTransformFeedback(Reference);

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLUniformLocation")]
pub struct WebGLUniformLocation(Reference);

#[derive(Debug, Clone, ReferenceType)]
#[reference(instance_of = "WebGLVertexArrayObject")]
pub struct WebGLVertexArrayObject(Reference);
