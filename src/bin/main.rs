use glamour::{Angle, Transform3, Vector2, Vector3};
use itertools::Itertools;
use palette::Srgb;
use std::fs::File;
use std::io::BufReader;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::Instant;
use sw_render::buffers::frame::FrameBuffer;
use sw_render::common::camera::PerspectiveCamera;
use sw_render::common::space::{ScreenPoint, WorldPoint, WorldSpace, WorldVector};
use sw_render::objects::mesh::Mesh;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes};

const WIDTH: usize = 480;
const HEIGHT: usize = 480;

const FPS_TARGET: usize = 60;

const DISPLAY_DIMENSIONS: Vector2<u32> = Vector2::new(WIDTH as u32, HEIGHT as u32);

const PIXEL_COUNT: usize = WIDTH * HEIGHT;

fn transform_vertex(
    vertex: &WorldPoint,
    camera: &PerspectiveCamera,
    width: usize,
    height: usize,
) -> ScreenPoint {
    // Apply view matrix
    let view_point = camera.view_matrix.map_point(*vertex);

    // Store the view space z-coordinate
    let view_z = view_point.z;

    // Apply perspective matrix
    let clip_point = camera.perspective_matrix.map_point(view_point);

    // Manually calculate w
    let w = view_z;

    // Perform perspective divide
    let ndc = Vector2::<f32>::new(clip_point.x / w, clip_point.y / w);

    ScreenPoint::new(
        (ndc.x + 1.0) * width as f32 / 2.0,
        (ndc.y + 1.0) * height as f32 / 2.0,
    )
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let start = Instant::now();

    let input = BufReader::new(File::open("african_head.obj").unwrap());
    let face_mesh = Mesh::from_obj(input).unwrap();

    let wireframe_color = Srgb::<u8>::new(255, 255, 255);

    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let mut camera = PerspectiveCamera::new(
        WorldPoint::new(0.0, 0.0, 2.25),  // Position
        WorldVector::new(0.0, 0.0, -1.0), // Direction (looking towards negative z)
        // Up vector
        0.1,  // Near plane
        10.0, // Far plane
        37.5, // Field of view in degrees
        aspect_ratio,
    );

    let mut rotate_around_y_axis: Transform3<WorldSpace, WorldSpace> =
        Transform3::from_scale_rotation_translation(
            Vector3::ONE,
            Vector3::Y,
            Angle::from_degrees(1.0),
            Vector3::ZERO,
        );

    let window_attributes = Window::default_attributes()
        .with_title("Software Renderer")
        .with_inner_size(PhysicalSize::new(WIDTH as u32, HEIGHT as u32))
        .with_resizable(false);

    let mut app = sw_render::utils::winit_app::WinitAppBuilder::with_init(|elwt| {
        let window = {
            let window = elwt.create_window(window_attributes.clone());
            Rc::new(window.unwrap())
        };
        let context = softbuffer::Context::new(window.clone()).unwrap();
        let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

        let old_size = (0, 0);

        (window, surface, old_size)
    })
    .with_event_handler(|state, event, elwt| {
        let (window, surface, old_size) = state;
        elwt.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent {
                window_id,
                event: WindowEvent::RedrawRequested,
            } if window_id == window.id() => {
                let size = window.inner_size();
                if let (Some(width), Some(height)) =
                    (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                {
                    surface
                        .resize(
                            NonZeroU32::new(WIDTH as u32).unwrap(),
                            NonZeroU32::new(HEIGHT as u32).unwrap(),
                        )
                        .unwrap();

                    if (width.get(), height.get()) != *old_size {
                        *old_size = (width.get(), height.get());
                    };

                    let mut window_buffer = surface.buffer_mut().unwrap();

                    let mut smart_buffer = FrameBuffer::new(&mut window_buffer, DISPLAY_DIMENSIONS);
                    smart_buffer.clear();

                    let elapsed_frames =
                        (start.elapsed().as_millis() % (1_000 / FPS_TARGET) as u128) as f64
                            / FPS_TARGET as f64;

                    rotate_around_y_axis = Transform3::from_scale_rotation_translation(
                        Vector3::ONE,
                        Vector3::Y,
                        Angle::from_degrees(1.0) * (elapsed_frames as f32),
                        Vector3::ZERO,
                    );
                    camera.position = rotate_around_y_axis.map_point(camera.position);
                    camera.look_at_point(&WorldPoint::ZERO);
                    face_mesh.tris_faces().for_each(|face| {
                        face.iter()
                            .map(|point| transform_vertex(&point.as_(), &camera, WIDTH, HEIGHT))
                            .circular_tuple_windows::<(_, _)>()
                            .for_each(|(p1, p2)| smart_buffer.draw_line(&p1, &p2, wireframe_color));
                    });

                    window_buffer.present().unwrap();
                }
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                elwt.exit();
            }
            _ => {}
        }
    });

    event_loop.run_app(&mut app).unwrap();
}
