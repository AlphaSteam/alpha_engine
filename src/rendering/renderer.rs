extern crate glium;
pub use crate::rendering::vertex::Vertex;
use crate::sys::system::System;
pub use crate::window::Window;
use glium::{uniform, BackfaceCullingMode, Blend, Display, Surface};

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {}
    }
    pub fn start(&self, display: &Display, system: &mut System) {
        let game_objects = system.game_objects_mut().clone();
        for game_object in game_objects {
            let shape = game_object.mesh().vertices();
            let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
            system.add_vertex_buffer(vertex_buffer);

            let indices = game_object.mesh().indices();
            let index_buffer = glium::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                &indices,
            )
            .unwrap();
            system.add_index_buffer(index_buffer);
        }
    }
    pub fn render(&self, display: &Display, system: &mut System) {
        let mut target = display.draw();
        let vertex_shader_src = r#"
        #version 140

        in vec3 position;
    
        void main() {
            gl_Position = vec4(position, 1.0);
        }
"#;
        let fragment_shader_src = r#"
        #version 140

        out vec4 color;
    
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
"#;
        let program =
            glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        let params = glium::DrawParameters {
            // GO BACK TO THIS
            /*   depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            }, */
            blend: Blend::alpha_blending(),
            backface_culling: BackfaceCullingMode::CullCounterClockwise,
            ..Default::default()
        };
        for n in 0..system.game_objects().len() {
            let vertex_buffer = &system.vertex_buffers()[n];
            let index_buffer = &system.index_buffers()[n];
            target
                .draw(
                    vertex_buffer,
                    index_buffer,
                    &program,
                    &glium::uniforms::EmptyUniforms,
                    &params,
                )
                .unwrap();
        }

        target.finish().unwrap();
    }
    pub fn stop(&self) {}
}
