
use window::{event::*, Window};
use dust::*;
use dust::objects::MeshShader;

fn main() {
    let mut window = Window::new_default("Hello, world!").unwrap();
    let (width, height) = window.framebuffer_size();
    let gl = window.gl();

    // Renderer
    let mut renderer = DeferredPipeline::new(&gl, width, height, vec4(0.8, 0.8, 0.8, 1.0)).unwrap();

    let monkey = Mesh::new_from_obj_source(&gl, include_str!("../assets/models/suzanne.obj").to_string()).unwrap();
    let plane = Mesh::new_plane(&gl).unwrap();
    let mesh_shader = MeshShader::new(&gl).unwrap();

    renderer.ambient_light().set_intensity(0.1);

    let mut directional_light = renderer.directional_light(0).unwrap();
    directional_light.set_direction(&vec3(1.0, -1.0, -1.0));
    directional_light.set_intensity(0.3);
    directional_light.enable_shadows();

    directional_light = renderer.directional_light(1).unwrap();
    directional_light.set_direction(&vec3(-1.0, -1.0, 1.0));
    directional_light.set_intensity(0.3);
    directional_light.enable_shadows();

    let mut point_light = renderer.point_light(0).unwrap();
    point_light.set_position(&vec3(5.0, 5.0, 5.0));
    point_light.set_intensity(0.5);
    point_light.set_color(&vec3(0.0, 1.0, 0.0));

    point_light = renderer.point_light(1).unwrap();
    point_light.set_position(&vec3(-5.0, 5.0, -5.0));
    point_light.set_intensity(0.5);
    point_light.set_color(&vec3(1.0, 0.0, 0.0));

    let spot_light = renderer.spot_light(0).unwrap();
    spot_light.set_intensity(0.5);
    spot_light.set_color(&vec3(0.0, 0.0, 1.0));
    spot_light.set_position(&vec3(5.0, 5.0, 5.0));
    spot_light.set_direction(&vec3(-1.0, -1.0, -1.0));
    spot_light.set_cutoff(0.05*std::f32::consts::PI);
    spot_light.enable_shadows();

    let mut camera_handler = camerahandler::CameraHandler::new(camerahandler::CameraState::SPHERICAL);

    // main loop
    window.render_loop(move |events, _elapsed_time|
    {
        for event in events {
            handle_camera_events(event, &mut camera_handler, &mut renderer.camera);
            match event {
                Event::Key { state, kind } => {
                    if kind == "R" && *state == State::Pressed
                    {
                        let l = renderer.directional_light(0).unwrap();
                        if l.is_shadows_enabled() {
                            l.disable_shadows();
                        } else {
                            l.enable_shadows();
                        }
                    }
                },
                _ => {}
            }
            //handle_ambient_light_parameters(event, &mut ambient_light);
            //handle_directional_light_parameters(event, &mut directional_light);
            //handle_surface_parameters(event, &mut monkey);
        }

        // Draw
        let render_scene = |camera: &Camera| {
            mesh_shader.render(&monkey, &Mat4::identity(), camera);
        };

        // Shadow pass
        renderer.shadow_pass(&render_scene);

        // Geometry pass
        renderer.geometry_pass(&|camera|
            {
                render_scene(camera);
                mesh_shader.render(&plane, &(Mat4::from_translation(vec3(0.0, -1.0, 0.0))
                    * Mat4::from_scale(10.0)), camera);
            }).unwrap();

        // Light pass
        renderer.light_pass().unwrap();

    }).unwrap();
}

pub fn handle_camera_events(event: &Event, camera_handler: &mut dust::camerahandler::CameraHandler, camera: &mut Camera)
{
    match event {
        Event::Key {state, kind} => {
            if kind == "Tab" && *state == State::Pressed
            {
                camera_handler.next_state();
            }
        },
        Event::MouseClick {state, button, ..} => {
            if *button == MouseButton::Left
            {
                if *state == State::Pressed { camera_handler.start_rotation(); }
                else { camera_handler.end_rotation() }
            }
        },
        Event::MouseMotion {delta} => {
            camera_handler.rotate(camera, delta.0 as f32, delta.1 as f32);
        },
        Event::MouseWheel {delta} => {
            camera_handler.zoom(camera, *delta as f32);
        }
    }
}

/*fn handle_ambient_light_parameters(event: &Event, light: &mut light::AmbientLight)
{
    match event {
        Event::WindowEvent{ event, .. } => match event {
            WindowEvent::KeyboardInput {input, ..} => {
                if let Some(keycode) = input.virtual_keycode
                {
                    match keycode {
                        VirtualKeyCode::X => {
                            light.base.intensity = (light.base.intensity + 0.1).min(1.0);
                            println!("Ambient light intensity: {}", light.base.intensity);
                        },
                        VirtualKeyCode::Z => {
                            light.base.intensity = (light.base.intensity - 0.1).max(0.0);
                            println!("Ambient light intensity: {}", light.base.intensity);
                        },
                        _ => {}
                    }
                }
            },
            _ => {}
        },
        _ => {}
    }
}

fn handle_directional_light_parameters(event: &Event, light: &mut light::DirectionalLight)
{
    match event {
        Event::WindowEvent{ event, .. } => match event {
            WindowEvent::KeyboardInput {input, ..} => {
                if let Some(keycode) = input.virtual_keycode
                {
                    match keycode {
                        VirtualKeyCode::V => {
                            light.base.intensity = (light.base.intensity + 0.1).min(1.0);
                            println!("Directional light intensity: {}", light.base.intensity);
                        },
                        VirtualKeyCode::C => {
                            light.base.intensity = (light.base.intensity - 0.1).max(0.0);
                            println!("Directional light intensity: {}", light.base.intensity);
                        },
                        _ => {}
                    }
                }
            },
            _ => {}
        },
        _ => {}
    }
}

fn handle_surface_parameters(event: &Event, surface: &mut crate::objects::ShadedMesh)
{
    match event {
        Event::WindowEvent{ event, .. } => match event {
            WindowEvent::KeyboardInput {input, ..} => {
                if let Some(keycode) = input.virtual_keycode
                {
                    match keycode {
                        VirtualKeyCode::S => {
                            surface.diffuse_intensity = (surface.diffuse_intensity + 0.1).min(1.0);
                            println!("Diffuse intensity: {}", surface.diffuse_intensity);
                        },
                        VirtualKeyCode::A => {
                            surface.diffuse_intensity = (surface.diffuse_intensity - 0.1).max(0.0);
                            println!("Diffuse intensity: {}", surface.diffuse_intensity);
                        },
                        VirtualKeyCode::F => {
                            surface.specular_intensity = (surface.specular_intensity + 0.1).min(1.0);
                            println!("Specular intensity: {}", surface.specular_intensity);
                        },
                        VirtualKeyCode::D => {
                            surface.specular_intensity = (surface.specular_intensity - 0.1).max(0.0);
                            println!("Specular intensity: {}", surface.specular_intensity);
                        },
                        VirtualKeyCode::H => {
                            surface.specular_power = surface.specular_power + 1.0;
                            println!("Specular power: {}", surface.specular_power);
                        },
                        VirtualKeyCode::G => {
                            surface.specular_power = (surface.specular_power - 1.0).max(0.0);
                            println!("Specular power: {}", surface.specular_power);
                        },
                        _ => {}
                    }
                }
            },
            _ => {}
        },
        _ => {}
    }
}*/