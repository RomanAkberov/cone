use std::collections::HashSet;
use gl::types::*;
pub use glutin::event::VirtualKeyCode as KeyCode;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait App {
    fn draw(&self, frame: &mut Frame);
    fn update(&mut self, update: &Update);
    fn quit(&mut self) {}
}

#[derive(Default)]
pub struct Update {
    mouse_pos: (i32, i32),
    pressed: HashSet<KeyCode>,
}

impl Update {
    pub fn is_pressed(&self, key: KeyCode) -> bool {
        self.pressed.contains(&key)
    }

    pub fn mouse_pos(&self) -> (i32, i32) {
        self.mouse_pos
    }

    fn press(&mut self, key: KeyCode) {
        self.pressed.insert(key);
    }

    fn release(&mut self, key: KeyCode) {
        self.pressed.remove(&key);
    }

    fn clear(&mut self) {
        self.pressed.clear();
    }
}

pub struct Frame {
    width: i32,
    height: i32,
    mesh: Mesh,
}

impl Frame {
    pub fn clear(&mut self) {
        for vertex in self.mesh.vertices.iter_mut() {
            vertex.uv = [0.0, 0.0];
        }
    }

    pub fn put_char(&mut self, x: i32, y: i32, ch: char, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }
        let index = 4 * (x * self.height + y) as usize;
        let uvs = char_to_uvs(ch);
        for (vertex, &uv) in self.mesh.vertices[index .. index + 4].iter_mut().zip(&uvs) {
            vertex.uv = uv;
            vertex.color = [
                color.r as f32 / 255.0, 
                color.g as f32 / 255.0,
                color.b as f32 / 255.0,
                color.a as f32 / 255.0,
            ]; 
        }
    }

    pub fn put_str(&mut self, x: i32, y: i32, str: &str, color: Color) {
        for (i, ch) in str.chars().enumerate() {
            self.put_char(x + (i as i32), y, ch, color);
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const WHITE: Self = Color::rgb(255, 255, 255);
    pub const BLACK: Self = Color::rgb(0, 0, 0);

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
}

struct Font {
    bytes: Vec<u8>,
    width: u32,
    height: u32,
}

fn load_font(bytes: &[u8]) -> Result<Font> {
    let decoder = png::Decoder::new(bytes);
    let (info, mut reader) = decoder.read_info()?;
    let mut bytes = vec![0; info.buffer_size()];
    reader.next_frame(&mut bytes)?;
    Ok(Font {
        bytes,
        width: info.width,
        height: info.height,
    })
}

fn create_font_texture(font: &Font) -> GLuint {
    let mut texture = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexImage2D(
            gl::TEXTURE_2D, 0, gl::RGBA as GLint, 
            font.width as GLsizei, font.height as GLsizei, 
            0, gl::RGBA, gl::UNSIGNED_BYTE, font.bytes.as_ptr() as *const GLvoid,
        );
    }
    texture
}

macro_rules! offset_of {
    ($ty: ty, $field: ident) => {
        {
            let default = <$ty>::default();
            &default.$field as *const _ as usize - &default as *const _ as usize
        }
    };
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
struct Vertex {
    pub position: [f32; 2],
    pub uv: [f32; 2],
    pub color: [f32; 4],
}

type Index = u32;

#[derive(Debug)]
struct Mesh {
    vertex_buffer: GLuint,
    vertex_array: GLuint,
    index_buffer: GLuint,
    vertices: Vec<Vertex>,
    indices: Vec<Index>,
}

const U_STEP: f32 = 1.0 / 16.0;
const V_STEP: f32 = 1.0 / 16.0;
const FONT_ROWS: u32 = 16;
const FONT_COLUMNS: u32 = 16;

fn char_to_uvs(ch: char) -> [[f32; 2]; 4] {
    let mut idx = ch as u32;
    if idx >= FONT_ROWS * FONT_COLUMNS {
        idx = 1;
    }
    let i = idx % FONT_COLUMNS;
    let j = idx / FONT_COLUMNS;
    let u0 = i as f32 * U_STEP;
    let v0 = j as f32 * V_STEP;
    let u1 = u0 + U_STEP;
    let v1 = v0 + V_STEP;
    [
        [u0, v0],
        [u0, v1],
        [u1, v0],
        [u1, v1],
    ]
}

fn create_frame(width: i32, height: i32) -> Frame {
    assert!(width > 0);
    assert!(height > 0);
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let x_step = 2.0 / (width as f32);
    let y_step = 2.0 / (height as f32);
    for i in 0 .. width {
        for j in 0 .. height {
            let x = i as f32 / width as f32;
            let y = j as f32 / height as f32;
            let index = vertices.len() as Index;
            let x0 = 2.0 * x - 1.0;
            let y0 = 1.0 - 2.0 * y;
            let x1 = x0 + x_step;
            let y1 = y0 - y_step;
            indices.push(index);
            indices.push(index + 1);
            indices.push(index + 3);
            indices.push(index + 2);
            indices.push(index);
            indices.push(index + 3);
            vertices.push(Vertex {
                position: [x0, y0],
                uv: [0.0, 0.0],
                color: [0.0; 4],
            });
            vertices.push(Vertex {
                position: [x0, y1],
                uv: [0.0, 0.0],
                color: [0.0; 4],
            });
            vertices.push(Vertex {
                position: [x1, y0],
                uv: [0.0, 0.0],
                color: [0.0; 4],
            });
            vertices.push(Vertex {
                position: [x1, y1],
                uv: [0.0, 0.0],
                color: [0.0; 4],
            });
        }
    }
    let mut vertex_buffer = 0;
    let mut vertex_array = 0;
    let mut index_buffer = 0;
    unsafe {
        gl::GenBuffers(1, &mut vertex_buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
        gl::GenVertexArrays(1, &mut vertex_array);
        gl::BindVertexArray(vertex_array);
        gl::GenBuffers(1, &mut index_buffer);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (std::mem::size_of::<Index>() * indices.len()) as GLsizeiptr,
            indices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, std::mem::size_of::<Vertex>() as GLsizei, offset_of!(Vertex, position) as *const GLvoid);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, std::mem::size_of::<Vertex>() as GLsizei, offset_of!(Vertex, uv) as *const GLvoid);
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(2, 4, gl::FLOAT, gl::FALSE, std::mem::size_of::<Vertex>() as GLsizei, offset_of!(Vertex, color) as *const GLvoid);
        gl::EnableVertexAttribArray(2);
    }
    Frame {
        width,
        height,
        mesh: Mesh {
            vertex_buffer,
            vertex_array,
            index_buffer,
            vertices, 
            indices,
        },
    }
}

fn create_shader(shader_type: GLenum, source: &[u8]) -> Result<GLuint> {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        let source_len = source.len() as GLint;
        let source_ptr = source.as_ptr() as *const GLchar;
        gl::ShaderSource(shader, 1, &source_ptr, &source_len);
        gl::CompileShader(shader);
        let mut success = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == gl::TRUE as GLint {
            return Ok(shader);
        }
        let mut error_len = 0;
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut error_len);
        let mut error_bytes = vec![0u8; error_len as usize];
        gl::GetShaderInfoLog(shader, error_len, &mut error_len, error_bytes.as_mut_ptr() as *mut GLchar);
        let error = String::from_utf8(error_bytes).unwrap_or_default();
        Err(error.into())
    }
}

fn create_program(vertex_shader: GLuint, fragment_shader: GLuint) -> Result<GLuint> {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);
        let mut success = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == gl::TRUE as GLint {
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            return Ok(program);
        }
        let mut error_len = 0;
        gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut error_len);
        let mut error_bytes = vec![0u8; error_len as usize];
        gl::GetProgramInfoLog(program, error_len, &mut error_len, error_bytes.as_mut_ptr() as *mut GLchar);
        let error = String::from_utf8(error_bytes).unwrap_or_default();
        Err(error.into())
    }
}

fn draw_mesh(mesh: &Mesh, program: GLuint) {
    unsafe {
        gl::BufferData(
            gl::ARRAY_BUFFER, 
            (std::mem::size_of::<Vertex>() * mesh.vertices.len()) as GLsizeiptr, 
            mesh.vertices.as_ptr() as *const GLvoid, 
            gl::DYNAMIC_DRAW,
        );
        gl::ActiveTexture(gl::TEXTURE0);
        gl::UseProgram(program);
        gl::BindVertexArray(mesh.vertex_array);
        gl::DrawElements(gl::TRIANGLES, mesh.indices.len() as _, gl::UNSIGNED_INT, std::ptr::null());
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }
}

pub struct Config<'a> {
    pub title: &'a str,
    pub width: i32,
    pub height: i32, 
    pub font: &'a [u8],
}

pub fn run<A: App + 'static>(config: Config, mut app: A) -> Result<()> {
    let font = load_font(config.font)?;
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new()
        .with_title(config.title)
        .with_inner_size(glutin::dpi::PhysicalSize::new(
            config.width as u32 * font.width / FONT_COLUMNS, 
            config.height as u32 * font.height / FONT_ROWS,
        ))
        .with_resizable(false);
    let context = glutin::ContextBuilder::new()
        .with_vsync(true)
        .build_windowed(window_builder, &event_loop)?;
    let context = unsafe {
        context.make_current().map_err(|(_, err)| err)?
    };
    let glyph_width = (font.width / FONT_COLUMNS) as i32;
    let glyph_height = (font.height / FONT_ROWS) as i32;
    gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);
    let vertex_shader = create_shader(gl::VERTEX_SHADER, include_bytes!("../shaders/cone.vert"))?;
    let fragment_shader = create_shader(gl::FRAGMENT_SHADER, include_bytes!("../shaders/cone.frag"))?;
    let program = create_program(vertex_shader, fragment_shader)?;
    create_font_texture(&font);
    let mut frame = create_frame(config.width, config.height);
    let mut update = Update::default();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = glutin::event_loop::ControlFlow::Poll;
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    app.quit();
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }
                glutin::event::WindowEvent::Resized(physical_size) => {
                    unsafe {
                        gl::Viewport(0, 0, physical_size.width as GLsizei, physical_size.height as GLsizei);
                    }
                }
                glutin::event::WindowEvent::CursorMoved {
                    position,
                    .. 
                } => {
                    update.mouse_pos = (position.x as i32 / glyph_width, position.y as i32 / glyph_height);
                }
                glutin::event::WindowEvent::KeyboardInput {
                    input: glutin::event::KeyboardInput {
                        state,
                        virtual_keycode: Some(key),
                        ..
                    },
                    ..
                } => {
                    match state {
                        glutin::event::ElementState::Pressed => {
                            update.press(key);
                        }
                        glutin::event::ElementState::Released => {
                            update.release(key);
                        }
                    }
                }
                _ => ()
            }
            glutin::event::Event::MainEventsCleared => {
                app.update(&update);
                update.clear();
                context.window().request_redraw();
            }
            glutin::event::Event::RedrawRequested(_) => {
                unsafe {
                    gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
                app.draw(&mut frame);
                draw_mesh(&frame.mesh, program);
                context.swap_buffers().unwrap();
            }
            _ => ()
        }
    });
}
