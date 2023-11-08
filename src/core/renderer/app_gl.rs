use core::ffi::c_void;
use gl::types::GLint;
use itertools::Itertools;
use sfml::graphics::Image;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use gl::*;
extern crate nalgebra_glm as glm;

use std::convert::TryInto;

use crate::core::resource::TextLoadInfo;
use crate::util::clamp;

#[derive(Debug, Default, Clone, Copy)]
pub struct GLProgram {
    pub id: u32,
    pub mvp_loc: i32,
    pub color_loc: i32,
    pub uv_rect_loc: i32,
}

#[derive(Debug, Default)]
pub struct AppGL {
    has_init: bool,
    pub vao: u32,
    pub vbo: u32,
    pub ebo: u32,

    pub image_program: GLProgram,
    pub text_program: GLProgram,
}

#[repr(C)]
#[derive(Debug)]
struct Vertex {
    pos: [f32; 3],
    uv: [f32; 2],
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Texture {
    pub texture_id: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Default)]
struct TextTextureData {
    pub rows: HashMap<i32, Vec<u8>>,
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}

pub fn load_image_from_disk(path: &str) -> Result<Texture, String> {
    let mut f = File::open(path).unwrap();
    let mut img_bytes = Vec::new();
    f.read_to_end(&mut img_bytes).unwrap();

    unsafe {
        let mut id: u32 = 0;
        GenTextures(1, &mut id);
        if id != 0 {
            BindTexture(TEXTURE_2D, id);
            TexParameteri(
                TEXTURE_2D,
                TEXTURE_WRAP_S,
                CLAMP_TO_EDGE.try_into().unwrap(),
            );
            TexParameteri(
                TEXTURE_2D,
                TEXTURE_WRAP_T,
                CLAMP_TO_EDGE.try_into().unwrap(),
            );

            let img_data = Image::from_memory(&img_bytes);
            match img_data {
                Some(img_data) => {
                    let img_data_ptr = img_data.pixel_data().as_ptr() as *const c_void;
                    let size = img_data.size();

                    println!("img_data_size: {:?}", size);

                    // RGBA since pixel_data pads to 4 channels
                    TexImage2D(
                        TEXTURE_2D,
                        0,
                        RGBA.try_into().unwrap(),
                        size.x as GLint,
                        size.y as GLint,
                        0,
                        RGBA,
                        UNSIGNED_BYTE,
                        img_data_ptr,
                    );
                    GenerateMipmap(TEXTURE_2D);
                    BindTexture(TEXTURE_2D, 0);
                    return Ok(Texture {
                        texture_id: id,
                        width: size.x,
                        height: size.y,
                    });
                }
                None => {
                    DeleteTextures(1, &id);
                    println!("Bad Image for path: {:?}", path);
                    return Err("Bad Image".to_string());
                }
            }
        }

        Err("Failed to load disk image".to_string())
    }
}

pub fn load_image_from_url(
    client: &reqwest::blocking::Client,
    url: &str,
) -> Result<Texture, String> {
    match client.get(url).send() {
        Ok(response) => {
            let resp = response.bytes();
            let resp_bytes = resp.unwrap();
            unsafe {
                let mut id: u32 = 0;
                GenTextures(1, &mut id);
                if id != 0 {
                    BindTexture(TEXTURE_2D, id);
                    TexParameteri(
                        TEXTURE_2D,
                        TEXTURE_WRAP_S,
                        CLAMP_TO_EDGE.try_into().unwrap(),
                    );
                    TexParameteri(
                        TEXTURE_2D,
                        TEXTURE_WRAP_T,
                        CLAMP_TO_EDGE.try_into().unwrap(),
                    );
                    TexParameteri(
                        TEXTURE_2D,
                        TEXTURE_MIN_FILTER,
                        LINEAR_MIPMAP_LINEAR.try_into().unwrap(),
                    );
                    TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR.try_into().unwrap());
                    let img_data = Image::from_memory(&resp_bytes);
                    match img_data {
                        Some(img_data) => {
                            let img_data_ptr = img_data.pixel_data().as_ptr() as *const c_void;
                            let img_size = img_data.size();
                            // RGBA since pixel_data pads to 4 channels
                            // TODO: Need to factor in the size of the resulting image
                            TexImage2D(
                                TEXTURE_2D,
                                0,
                                RGBA.try_into().unwrap(),
                                img_size.x as i32,
                                img_size.y as i32,
                                0,
                                RGBA,
                                UNSIGNED_BYTE,
                                img_data_ptr,
                            );
                            GenerateMipmap(TEXTURE_2D);
                            BindTexture(TEXTURE_2D, 0);
                            return Ok(Texture {
                                texture_id: id,
                                width: img_size.x,
                                height: img_size.y,
                            });
                        }
                        None => {
                            DeleteTextures(1, &id);
                            println!("Bad Image for url: {:?}", url);
                            return Err("Bad Image".to_string());
                        }
                    }
                }

                Err("Bad Image".to_string())
            }
        }
        Err(_) => Err("Bad Image Url".to_string()),
    }
}

fn sw_blit_to_buffer(
    offset: (i32, i32),
    size: (u32, u32),
    top: i32,
    dst: &mut TextTextureData,
    src: &[u8],
) {
    let y_offset = -top + offset.1;
    for x in 0..size.0 {
        let x_pos = (x + offset.0 as u32) as usize;
        for y in 0..size.1 {
            let y_dst_pos = y as i32 + y_offset;
            dst.rows.entry(y_dst_pos).or_default();

            while dst.rows[&y_dst_pos].len() <= x_pos {
                dst.rows.get_mut(&y_dst_pos).unwrap().push(0);
            }

            let val = src[x as usize + ((y * size.0) as usize)] as i32;
            let existing = dst.rows.get_mut(&y_dst_pos).unwrap()[x_pos] as i32;
            dst.rows.get_mut(&y_dst_pos).unwrap()[x_pos] = clamp(val + existing, 0, 255) as u8;
        }
    }
}

fn sw_render_text_to_buffer(text_info: &TextLoadInfo, data: &mut TextTextureData) {
    // TODO: This should be externalized
    let lib = freetype::Library::init().unwrap();
    let face = lib.new_face(&text_info.font_path, 0).unwrap();
    face.set_char_size(80 * text_info.font_size, 0, 100, 0)
        .map_err(|err| println!("{:?}", err))
        .ok();
    let mut offset = (0i32, 0i32);
    for c in text_info.text.chars() {
        face.load_char(c as usize, freetype::face::LoadFlag::RENDER)
            .map_err(|err| println!("{:?}", err))
            .ok();
        let glyph = face.glyph();
        let glyph_bitmap = glyph.bitmap();
        let bitmap_data = glyph_bitmap.buffer();
        sw_blit_to_buffer(
            offset,
            (glyph_bitmap.width() as u32, glyph_bitmap.rows() as u32),
            glyph.bitmap_top(),
            data,
            bitmap_data,
        );
        offset.0 += glyph.advance().x / 64;
        offset.1 += glyph.advance().y / 64;
    }

    data.height = data.rows.len();

    let mut max_width = 0;
    for v in data.rows.values() {
        if v.len() > max_width {
            max_width = v.len();
        }
    }
    data.width = max_width;

    for k in data.rows.keys().sorted() {
        let row = data.rows.get(k).unwrap();
        for j in 0..data.width {
            if j >= row.len() {
                data.data.push(0);
            } else {
                data.data.push(row[j]);
            }
        }
    }
    data.rows.clear();

    assert!(
        data.data.len() == data.width * data.height,
        "data should be width * height"
    );
}

pub fn render_text_to_texture(text_info: &TextLoadInfo) -> Result<Texture, &str> {
    unsafe {
        let mut id: u32 = 0;
        GenTextures(1, &mut id);

        if id == 0 {
            return Err("Failed to generate texture for text");
        }

        BindTexture(TEXTURE_2D, id);
        TexParameteri(
            TEXTURE_2D,
            TEXTURE_WRAP_S,
            CLAMP_TO_EDGE.try_into().unwrap(),
        );
        TexParameteri(
            TEXTURE_2D,
            TEXTURE_WRAP_T,
            CLAMP_TO_EDGE.try_into().unwrap(),
        );
        TexParameteri(
            TEXTURE_2D,
            TEXTURE_MIN_FILTER,
            LINEAR_MIPMAP_LINEAR.try_into().unwrap(),
        );
        TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR.try_into().unwrap());
        PixelStorei(UNPACK_ALIGNMENT, 1);

        let mut texture_data = TextTextureData::default();
        sw_render_text_to_buffer(text_info, &mut texture_data);
        let texture_data_ptr = texture_data.data.as_ptr() as *const c_void;
        TexImage2D(
            TEXTURE_2D,
            0,
            RED.try_into().unwrap(),
            texture_data.width as i32,
            texture_data.height as i32,
            0,
            RED,
            UNSIGNED_BYTE,
            texture_data_ptr,
        );
        GenerateMipmap(TEXTURE_2D);
        BindTexture(TEXTURE_2D, 0);

        Ok(Texture {
            texture_id: id,
            width: texture_data.width as u32,
            height: texture_data.height as u32,
        })
    }
}

pub fn release_texture(texture_id: u32) {
    unsafe {
        DeleteTextures(1, &texture_id);
    }
}

fn gen_buffer() -> u32 {
    unsafe {
        let mut id: u32 = 0;
        GenBuffers(1, &mut id);
        id
    }
}

fn gen_vertex_buffer() -> u32 {
    unsafe {
        let mut id: u32 = 0;
        GenVertexArrays(1, &mut id);
        id
    }
}

fn create_shader(shader_type: u32, shader_source_location: &str) -> Result<u32, &str> {
    unsafe {
        let id = CreateShader(shader_type);

        if id != 0 {
            let mut source = File::open(shader_source_location).unwrap();
            let mut contents = Vec::new();
            source
                .read_to_end(&mut contents)
                .map_err(|err| println!("{:?}", err))
                .ok();
            let content_length = contents.len() as i32;
            let contents_ptr = contents.as_ptr();
            let contents_i8_ptr = contents_ptr as *const i8;
            ShaderSource(id, 1, &contents_i8_ptr, &content_length);
            CompileShader(id);

            let mut compile_status: i32 = 0;
            GetShaderiv(id, COMPILE_STATUS, &mut compile_status);

            if compile_status == 0 {
                let mut num_written = 0;
                let mut info_log_buffer: [i8; 512] = [0; 512];
                GetShaderInfoLog(id, 512, &mut num_written, info_log_buffer.as_mut_ptr());
                let mut str_data = Vec::new();
                for x in info_log_buffer {
                    if x != 0 {
                        str_data.push(x as u8);
                    }
                }
                let error_string = std::str::from_utf8(&str_data);
                println!(
                    "Failed to compile shader: {}, error_status: {}, log: {:?}",
                    shader_source_location, compile_status, error_string
                );
            } else {
                return Ok(id);
            }
        }
        Err("Failed to compile shader")
    }
}

fn create_and_link_program(vertex_shader_source: &str, fragment_shader_source: &str) -> u32 {
    let vertex_shader = create_shader(VERTEX_SHADER, vertex_shader_source).unwrap();
    let fragment_shader = create_shader(FRAGMENT_SHADER, fragment_shader_source).unwrap();

    unsafe {
        let id = CreateProgram();
        AttachShader(id, vertex_shader);
        AttachShader(id, fragment_shader);
        LinkProgram(id);

        let mut link_status: i32 = 0;
        GetProgramiv(id, LINK_STATUS, &mut link_status);

        if link_status == 0 {
            let mut num_written = 0;
            let mut info_log_buffer: [i8; 512] = [0; 512];
            GetProgramInfoLog(id, 512, &mut num_written, info_log_buffer.as_mut_ptr());
            let mut str_data = Vec::new();
            for x in info_log_buffer {
                if x != 0 {
                    str_data.push(x as u8);
                }
            }
            let error_string = std::str::from_utf8(&str_data);
            println!(
                "Failed to link program with error_status: {}, log: {:?}",
                link_status, error_string
            );
        }

        DeleteShader(vertex_shader);
        DeleteShader(fragment_shader);

        id
    }
}

fn f32_size_mult(len: usize) -> isize {
    static F32_SIZE: usize = std::mem::size_of::<f32>();
    (F32_SIZE * len).try_into().unwrap()
}

fn upload_buffer_data(vao: u32, vbo: u32, ebo: u32) {
    let vertex_data: [Vertex; 4] = [
        Vertex {
            pos: [0.5, 0.5, 0.],
            uv: [1., 0.],
        },
        Vertex {
            pos: [0.5, -0.5, 0.],
            uv: [1., 1.],
        },
        Vertex {
            pos: [-0.5, -0.5, 0.],
            uv: [0., 1.],
        },
        Vertex {
            pos: [-0.5, 0.5, 0.],
            uv: [0., 0.],
        },
    ];
    let size_of_vertex = std::mem::size_of_val(&vertex_data[0]).try_into().unwrap();
    let size_of_vertex_pos = std::mem::size_of_val(&vertex_data[0].pos);
    let _size_of_vertex_uv = std::mem::size_of_val(&vertex_data[0].uv);

    let index_data = [0, 1, 3, 1, 2, 3];

    unsafe {
        BindVertexArray(vao);
        BindBuffer(ARRAY_BUFFER, vbo);
        BufferData(
            ARRAY_BUFFER,
            f32_size_mult(size_of_vertex as usize * vertex_data.len()),
            vertex_data.as_ptr() as *const c_void,
            STATIC_DRAW,
        );
        VertexAttribPointer(
            0,
            3,
            FLOAT,
            FALSE,
            size_of_vertex,
            std::ptr::null::<c_void>(),
        );
        EnableVertexAttribArray(0);
        VertexAttribPointer(
            1,
            2,
            FLOAT,
            FALSE,
            size_of_vertex,
            size_of_vertex_pos as *const c_void,
        );
        EnableVertexAttribArray(1);
        BindBuffer(ELEMENT_ARRAY_BUFFER, ebo);
        BufferData(
            ELEMENT_ARRAY_BUFFER,
            (std::mem::size_of::<i32>() * index_data.len())
                .try_into()
                .unwrap(),
            index_data.as_ptr() as *const c_void,
            STATIC_DRAW,
        );
    }
}

pub unsafe fn report_error(prefix: &str) {
    let mut error = GetError();

    while error != 0 {
        println!("app_gl ERROR[{}]: {}", prefix, GetError());
        error = GetError();
    }
}

impl AppGL {
    pub fn init(&mut self) {
        if self.has_init {
            return;
        }

        // Init GL after GL context has been created
        gl_loader::init_gl();
        load_with(|s| gl_loader::get_proc_address(s) as *const _);

        unsafe {
            report_error("gl-init");

            self.vao = gen_vertex_buffer();
            report_error("gen vao");

            self.vbo = gen_buffer();
            report_error("gen vbo");

            self.ebo = gen_buffer();
            report_error("gen ebo");

            self.image_program.id =
                create_and_link_program("res/glsl/imagev.glsl", "res/glsl/image.glsl");
            report_error("create_and_link_program image");

            self.text_program.id =
                create_and_link_program("res/glsl/textv.glsl", "res/glsl/text.glsl");
            report_error("text");

            upload_buffer_data(self.vao, self.vbo, self.ebo);
            report_error("upload buffer data");

            let mvp_name = "mvp\0".as_bytes();
            let color_name = "color\0".as_bytes();
            let uv_rect = "uv_rect\0".as_bytes();

            self.image_program.mvp_loc =
                GetUniformLocation(self.image_program.id, mvp_name.as_ptr() as *const i8);
            report_error("image mvp");

            self.image_program.color_loc =
                GetUniformLocation(self.image_program.id, color_name.as_ptr() as *const i8);
            report_error("image color");

            self.image_program.uv_rect_loc =
                GetUniformLocation(self.image_program.id, uv_rect.as_ptr() as *const i8);
            report_error("image uv_rect");

            self.text_program.mvp_loc =
                GetUniformLocation(self.text_program.id, mvp_name.as_ptr() as *const i8);
            report_error("text mvp");

            self.text_program.color_loc =
                GetUniformLocation(self.text_program.id, color_name.as_ptr() as *const i8);
            report_error("text color");
        }
    }
}

impl Drop for AppGL {
    fn drop(&mut self) {
        unsafe {
            DeleteBuffers(1, &self.ebo);
            DeleteBuffers(1, &self.vbo);
            DeleteVertexArrays(1, &self.vao);
            DeleteProgram(self.image_program.id);
            DeleteProgram(self.text_program.id);
            gl_loader::end_gl();
        }
    }
}
