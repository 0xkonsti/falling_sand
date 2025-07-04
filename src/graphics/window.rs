use glad_gl::gl;
use glam::UVec2;
use glfw::Context;
use log::{debug, error, info};

use crate::graphics::Color;

pub struct Window {
    glfw:   glfw::Glfw,
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
}

impl Window {
    pub fn new(config: &WindowConfig) -> Option<Self> {
        debug!("Creating window with config: {config:?}");
        debug!("Initializing GLFW");

        let result = glfw::init(glfw::fail_on_errors);
        if let Err(e) = result {
            error!("Failed to initialize GLFW: {e}");
            return None;
        }
        let mut glfw = result.unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::Resizable(config.resizable));
        glfw.window_hint(glfw::WindowHint::Samples(config.samples.to_glfw_samples()));

        debug!("Creating GLFW window");

        let Some((mut window, events)) = glfw.with_primary_monitor(|glfw, m| {
            glfw.create_window(
                config.width,
                config.height,
                config.title,
                m.map_or(glfw::WindowMode::Windowed, |m| match config.mode {
                    WindowMode::Windowed => glfw::WindowMode::Windowed,
                    WindowMode::Fullscreen => glfw::WindowMode::FullScreen(m),
                }),
            )
        }) else {
            error!("Failed to get primary monitor");
            return None;
        };

        // Polling
        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_enter_polling(true);

        debug!("Loading OpenGL function pointers");

        gl::load(|symbol| window.get_proc_address(symbol));

        unsafe {
            let data = [
                ("OpenGL version: ", gl::GetString(gl::VERSION)),
                ("Vendor: ", gl::GetString(gl::VENDOR)),
                ("Renderer: ", gl::GetString(gl::RENDERER)),
            ];
            data.iter().for_each(|(msg, data)| {
                info!("{:<20} {}", msg, std::ffi::CStr::from_ptr(*data as *const i8).to_str().unwrap());
            });
        };

        // Global GL configuration
        glfw.set_swap_interval(if config.vsync { glfw::SwapInterval::Sync(1) } else { glfw::SwapInterval::None });

        unsafe {
            // gl::Enable(gl::DEPTH_TEST);

            // gl::Enable(gl::CULL_FACE);
            // gl::CullFace(gl::BACK);

            gl::Enable(gl::MULTISAMPLE);

            // gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            // gl::Enable(gl::BLEND);

            if config.debug {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            }
        }

        info!("Window created successfully");
        Some(Self { glfw, window, events })
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn close(&mut self) {
        self.window.set_should_close(true);
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    pub fn poll_events(&mut self) {
        self.glfw.poll_events();
    }

    pub fn events(&mut self) -> Vec<glfw::WindowEvent> {
        glfw::flush_messages(&mut self.events).map(|(_, event)| event).collect()
    }

    pub fn set_clear_color(&self, color: Color) {
        unsafe {
            gl::ClearColor(color.r, color.g, color.b, color.a);
        }
    }

    pub fn clear(&self) {
        unsafe {
            // gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}

#[derive(Debug)]
pub enum WindowMode {
    Windowed,
    Fullscreen,
}

#[derive(Debug, Default)]
pub enum Samples {
    #[default]
    None,
    X2,
    X4,
    X8,
    X16,
}

impl Samples {
    pub fn to_glfw_samples(&self) -> Option<u32> {
        match self {
            Samples::None => None,
            Samples::X2 => Some(2),
            Samples::X4 => Some(4),
            Samples::X8 => Some(8),
            Samples::X16 => Some(16),
        }
    }

    pub fn raw_value(&self) -> f32 {
        match self {
            Samples::None => 0.0,
            Samples::X2 => 2.0,
            Samples::X4 => 4.0,
            Samples::X8 => 8.0,
            Samples::X16 => 16.0,
        }
    }
}

#[derive(Debug)]
pub struct WindowConfig<'a> {
    pub width:     u32,
    pub height:    u32,
    pub title:     &'a str,
    pub resizable: bool,
    pub vsync:     bool,
    pub samples:   Samples,
    pub mode:      WindowMode,
    pub debug:     bool,
}

impl<'a> WindowConfig<'a> {
    pub fn with_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn with_size(mut self, size: impl Into<(u32, u32)>) -> Self {
        let (width, height) = size.into();
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }

    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    pub fn with_vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
        self
    }

    pub fn with_samples(mut self, samples: Samples) -> Self {
        self.samples = samples;
        self
    }

    pub fn with_mode(mut self, mode: WindowMode) -> Self {
        self.mode = mode;
        self
    }
}

impl Default for WindowConfig<'_> {
    fn default() -> Self {
        Self {
            width:     800,
            height:    600,
            title:     "Paper Window",
            resizable: false,
            vsync:     true,
            samples:   Samples::None,
            mode:      WindowMode::Windowed,
            debug:     false,
        }
    }
}
