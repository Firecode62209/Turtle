use glam::{vec3, Mat4};
use turtle::*;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() -> AnyResult<()> {
    pretty_env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = TurtleApp::default();
    
    app.set_init_function(init);

    event_loop.run_app(&mut app).unwrap();
    Ok(())
}

fn init(app_data: &mut AppData) {
        let mesh = app_data.renderer.context.create_mesh_from_cube().unwrap();
        let mut instance_group = InstanceGroup::from(mesh);
        instance_group.create_instance_buffer(&app_data.renderer.context).unwrap();
        let count = 10000;        // how many cubes you want
        let radius = 50.0;      // radius of sphere
        let spacing = 1.0;      // optional multiplier for cube separation

        for i in 0..count {
            // Compute a normalized position on the sphere
            let phi = std::f32::consts::PI * (3.0 - 5.0_f32.sqrt()); // golden angle
            let y = 1.0 - (i as f32 / (count - 1) as f32) * 2.0;     // y from 1 to -1
            let r = (1.0 - y * y).sqrt();                            // radius at that y
            let theta = phi * i as f32;

            let x = theta.cos() * r;
            let z = theta.sin() * r;

            // Position on the sphere scaled by radius
            let position = vec3(x, y, z) * radius * spacing;

            let forward = position.normalize();
            let up = glam::Vec3::Y;
            let right = up.cross(forward).normalize();
            let adjusted_up = forward.cross(right);

            let rotation = Mat4::from_cols(
                right.extend(0.0),
                adjusted_up.extend(0.0),
                forward.extend(0.0),
                glam::Vec4::W,
            );

            let transform = Mat4::from_translation(position) * rotation;

            let instance_data = tvk::InstanceData {
                model: transform,
                color: vec3(x, y, z)
            };

            instance_group.add_instance(
                instance_data,
                true,
            );
        }
        instance_group.update_gpu_buffer().unwrap();
        app_data.instance_groups.push(instance_group);
}