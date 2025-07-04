mod graphics;
mod sandbox;
mod utils;

use glam::{Vec2, Vec3};
use graphics::*;
use log::error;
use sandbox::*;

use crate::utils::flatten;

#[rustfmt::skip]
const QUAD_VERTICES: [[f32; 3]; 4] = [
    [-0.5, -0.5, 0.0],
    [0.5, -0.5, 0.0],
    [0.5,  0.5, 0.0],
    [-0.5,  0.5, 0.0],
];
const QUAD_INDICES: [u32; 6] = [0, 1, 3, 1, 2, 3];

fn main() {
    env_logger::Builder::from_default_env().filter_level(log::LevelFilter::Debug).init();

    let w_config = WindowConfig::default().with_title("Falling Sand").with_vsync(false);
    // w_config.debug = true;

    let Some(mut window) = Window::new(&w_config) else {
        error!("Failed to create window");
        return;
    };

    let mut camera =
        Camera2D { viewport: Vec2::new(w_config.width as f32, w_config.height as f32), ..Default::default() };

    let instance = Mesh::new()
        .with_attribute(AttributeType::Position, flatten(QUAD_VERTICES))
        .with_attribute(AttributeType::Color, flatten([Color::WHITE; 4]))
        .with_indices(QUAD_INDICES.to_vec())
        .create_instance(100);

    let material = Material::new(Shader::instance());

    let mut sandbox = Sandbox::new(instance);

    let mut current_kind = CellKind::Sand;

    let mut dt;
    let mut last_frame = std::time::Instant::now();
    let mut cursor_pos = Vec2::ZERO;
    let mut mouse_pressed = [false; 3]; // [left, right, middle]

    window.set_clear_color(Color::DEEP_DARK_BLUE);
    while !window.should_close() {
        let now = std::time::Instant::now();
        dt = (now - last_frame).as_secs_f64();
        last_frame = now;

        window.poll_events();

        for event in window.events() {
            match event {
                glfw::WindowEvent::CursorPos(x, y) => {
                    if mouse_pressed[2] {
                        let delta_x = x as f32 - cursor_pos.x;
                        let delta_y = y as f32 - cursor_pos.y;

                        let zoom = camera.zoom;

                        camera.transform.translate(Vec3::new(-delta_x / zoom, delta_y / zoom, 0.0));
                    }

                    cursor_pos.x = x as f32;
                    cursor_pos.y = y as f32;
                }
                glfw::WindowEvent::MouseButton(button, action, _modifiers) => {
                    if action == glfw::Action::Press {
                        match button {
                            glfw::MouseButton::Button1 => {
                                mouse_pressed[0] = true;
                            }
                            glfw::MouseButton::Button2 => {
                                mouse_pressed[1] = true;
                            }
                            glfw::MouseButton::Button3 => {
                                mouse_pressed[2] = true;
                            }
                            _ => {}
                        }
                    } else if action == glfw::Action::Release {
                        match button {
                            glfw::MouseButton::Button1 => {
                                mouse_pressed[0] = false;
                            }
                            glfw::MouseButton::Button2 => {
                                mouse_pressed[1] = false;
                            }
                            glfw::MouseButton::Button3 => {
                                mouse_pressed[2] = false;
                            }
                            _ => {}
                        }
                    }
                }
                glfw::WindowEvent::Scroll(_x_offset, y_offset) => {
                    camera.zoom *= (1.0 + y_offset * 0.1) as f32;
                }
                glfw::WindowEvent::Key(key, _scancode, action, _modifiers) => {
                    if action == glfw::Action::Press {
                        match key {
                            glfw::Key::Escape => window.close(),
                            glfw::Key::Num1 => {
                                current_kind = CellKind::Sand;
                            }
                            glfw::Key::Num2 => {
                                current_kind = CellKind::Stone;
                            }
                            glfw::Key::Num3 => {
                                current_kind = CellKind::Water;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        if mouse_pressed[0] {
            let world_pos = get_world_position(&camera, cursor_pos);
            sandbox.insert_cell(Sandbox::grid_pos_from_world_pos(world_pos), current_kind);
        }

        if mouse_pressed[1] {
            let world_pos = get_world_position(&camera, cursor_pos);
            sandbox.remove_cell(Sandbox::grid_pos_from_world_pos(world_pos));
        }

        window.clear();

        sandbox.update(dt);

        material.apply(&[(PROJECTION_UNIFORM, ShaderUniform::Mat4(camera.projection_matrix().to_cols_array()))]);
        sandbox.draw();

        window.swap_buffers();
    }
}

fn get_world_position(camera: &Camera2D, screen_pos: Vec2) -> Vec3 {
    let ndc_x = (screen_pos.x / camera.viewport.x) * 2.0 - 1.0;
    let ndc_y = 1.0 - (screen_pos.y / camera.viewport.y) * 2.0;

    camera.unproject(Vec2::new(ndc_x, ndc_y))
}
