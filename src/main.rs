use minifb::Window;
use nalgebra::Vector3;

mod camera;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const POINT_WIDTH: usize = 4;

fn main() {
    let mut buffer = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("Test", WIDTH, HEIGHT, minifb::WindowOptions::default()).unwrap();

    let ps = vec![
        Vector3::new(-10.0, 10.0, 50.0),
        Vector3::new(10.0, 10.0, 50.0),
        Vector3::new(10.0, -10.0, 50.0),
        Vector3::new(-10.0, -10.0, 50.0),
        Vector3::new(-10.0, 10.0, 70.0),
        Vector3::new(10.0, 10.0, 70.0),
        Vector3::new(10.0, -10.0, 70.0),
        Vector3::new(-10.0, -10.0, 70.0),
        Vector3::new(0.0, 0.0, 60.0),
    ];

    let mut camera = camera::Camera::new(0.5, WIDTH as f32, HEIGHT as f32);
    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        let start = std::time::Instant::now();
        // set buffer to black
        for i in buffer.iter_mut() {
            *i = 0;
        }

        for p in &ps {
            let v = camera.project(p);
            let x = v.x as usize;
            let y = v.y as usize;
            if x < POINT_WIDTH
                || y < POINT_WIDTH
                || x >= WIDTH - POINT_WIDTH
                || y >= HEIGHT - POINT_WIDTH
            {
                continue;
            }
            for i in x - POINT_WIDTH..x + POINT_WIDTH {
                for j in y - POINT_WIDTH..y + POINT_WIDTH {
                    if i < WIDTH && j < HEIGHT {
                        buffer[i + j * WIDTH] = 0xFFFFFF;
                    }
                }
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        let elapsed = start.elapsed();
        if elapsed < std::time::Duration::from_millis(16) {
            std::thread::sleep(std::time::Duration::from_millis(16) - elapsed);
        }
        if window.is_key_down(minifb::Key::W) {
            camera.translate(Vector3::new(0.0, 0.0, -0.1));
        } else if window.is_key_down(minifb::Key::S) {
            camera.translate(Vector3::new(0.0, 0.0, 0.1));
        }
        if window.is_key_down(minifb::Key::A) {
            camera.translate(Vector3::new(0.1, 0.0, 0.0));
        } else if window.is_key_down(minifb::Key::D) {
            camera.translate(Vector3::new(-0.1, 0.0, 0.0));
        }
        if window.is_key_down(minifb::Key::LeftCtrl) {
            camera.translate(Vector3::new(0.0, -0.1, 0.0));
        } else if window.is_key_down(minifb::Key::Space) {
            camera.translate(Vector3::new(0.0, 0.1, 0.0));
        }

        if window.is_key_down(minifb::Key::Left) {
            camera.rotate(0.2, 0.0, 0.0);
        } else if window.is_key_down(minifb::Key::Right) {
            camera.rotate(-0.2, 0.0, 0.0);
        }
        if window.is_key_down(minifb::Key::Up) {
            camera.rotate(0.0, 0.0, -0.2);
        } else if window.is_key_down(minifb::Key::Down) {
            camera.rotate(0.0, 0.0, 0.2);
        }

        if window.is_key_down(minifb::Key::Q) {
            camera.reset();
        }
    }
}
