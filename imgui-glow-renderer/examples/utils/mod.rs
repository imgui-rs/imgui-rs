use glow::HasContext;
use glutin::{event_loop::EventLoop, GlRequest};
use imgui_winit_support::WinitPlatform;

pub type Window = glutin::WindowedContext<glutin::PossiblyCurrent>;

pub fn create_window(title: &str, gl_request: GlRequest) -> (EventLoop<()>, Window) {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new()
        .with_title(title)
        .with_inner_size(glutin::dpi::LogicalSize::new(1024, 768));
    let window = glutin::ContextBuilder::new()
        .with_gl(gl_request)
        .with_vsync(true)
        .build_windowed(window, &event_loop)
        .expect("could not create window");
    let window = unsafe {
        window
            .make_current()
            .expect("could not make window context current")
    };
    (event_loop, window)
}

pub fn glow_context(window: &Window) -> glow::Context {
    unsafe { glow::Context::from_loader_function(|s| window.get_proc_address(s).cast()) }
}

pub fn imgui_init(window: &Window) -> (WinitPlatform, imgui::Context) {
    let mut imgui_context = imgui::Context::create();
    imgui_context.set_ini_filename(None);

    let mut winit_platform = WinitPlatform::init(&mut imgui_context);
    winit_platform.attach_window(
        imgui_context.io_mut(),
        window.window(),
        imgui_winit_support::HiDpiMode::Rounded,
    );

    imgui_context
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    imgui_context.io_mut().font_global_scale = (1.0 / winit_platform.hidpi_factor()) as f32;

    (winit_platform, imgui_context)
}

pub struct Triangler {
    pub program: <glow::Context as HasContext>::Program,
    pub vertex_array: <glow::Context as HasContext>::VertexArray,
}

impl Triangler {
    pub fn new(gl: &glow::Context, shader_header: &str) -> Self {
        const VERTEX_SHADER_SOURCE: &str = r#"
const vec2 verts[3] = vec2[3](
    vec2(0.5f, 1.0f),
    vec2(0.0f, 0.0f),
    vec2(1.0f, 0.0f)
);

out vec2 vert;

void main() {
    vert = verts[gl_VertexID];
    gl_Position = vec4(vert - 0.5, 0.0, 1.0);
}
"#;
        const FRAGMENT_SHADER_SOURCE: &str = r#"
in vec2 vert;
out vec4 colour;

void main() {
    colour = vec4(vert, 0.5, 1.0);
}
"#;

        let mut shaders = [
            (glow::VERTEX_SHADER, VERTEX_SHADER_SOURCE, 0),
            (glow::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE, 0),
        ];

        unsafe {
            let vertex_array = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");

            let program = gl.create_program().expect("Cannot create program");

            for (kind, source, handle) in &mut shaders {
                let shader = gl.create_shader(*kind).expect("Cannot create shader");
                gl.shader_source(shader, &format!("{}\n{}", shader_header, *source));
                gl.compile_shader(shader);
                if !gl.get_shader_compile_status(shader) {
                    panic!("{}", gl.get_shader_info_log(shader));
                }
                gl.attach_shader(program, shader);
                *handle = shader;
            }

            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }

            for &(_, _, shader) in &shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }

            Self {
                program,
                vertex_array,
            }
        }
    }

    pub fn render(&self, gl: &glow::Context) {
        unsafe {
            gl.clear_color(0.05, 0.05, 0.1, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
            gl.use_program(Some(self.program));
            gl.bind_vertex_array(Some(self.vertex_array));
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }
    }

    pub fn destroy(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_program(self.program);
            gl.delete_vertex_array(self.vertex_array);
        }
    }
}
