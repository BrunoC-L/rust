use piston_window::{ButtonEvent, ButtonState, Button, Key};
use rand::Rng;
extern crate image;

fn main() {
    let windowdim = [640, 480];
    let wallwidth = 70;
    let gamewidth = windowdim[0] - 2 * wallwidth;
    let firstfloory = 45.0;
    let floorheight = 10;
    let playerwidth = 30;
    
    let original_pos: [f64; 2] = [windowdim[0] as f64/ 2.0, firstfloory];
    let mut player = original_pos;

    let original_floors: [[u32; 2]; 6] = [[70, 570], [90, 550], [110, 530], [130, 510], [150, 490], [170, 470]];
    let mut floors = original_floors;

    let mut window: piston_window::PistonWindow = piston_window::WindowSettings::new("Icy Tower", [windowdim[0], windowdim[1]]).resizable(false).build().unwrap();

    let mut texture_context = piston_window::TextureContext{
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into()
    };

    let canvas = image::load(image::io::Reader::open("player.png").ok().unwrap().into_inner(), image::ImageFormat::Png).ok().unwrap().into_rgba8();

	let texture: piston_window::G2dTexture = piston_window::Texture::from_image(
        &mut texture_context,
        &canvas,
        &piston_window::TextureSettings::new()
    ).unwrap();

    let playerscale = [0.0032f64, 0.004166f64];

    let input_ratio_to_pixels_x = (windowdim[0] / 2) as f64;
    let pixels_to_input_ratio_x = 1.0 / input_ratio_to_pixels_x;

    let input_ratio_to_pixels_y = 0.75 * input_ratio_to_pixels_x;
    let pixels_to_input_ratio_y = 1.0 / input_ratio_to_pixels_y;

    let mut left = false;
    let mut right = false;
    let mut space = false;
    let mut scrolling = false;

    let mut acceleration_y = 0.0;
    let mut speed_y = 0.0;
    let mut speed_x = 0.0;
    let max_speed_x = 2.0;

    let g = -0.03;

    let mut current_height_of_first_floor = firstfloory;
    let floorgap = 80.0;
    
    let mut rng = rand::thread_rng();

    while let Some(e) = window.next() {

        if player[1] < 0.0 {
            player = original_pos;
            floors = original_floors;
            current_height_of_first_floor = firstfloory;
            scrolling = false;
            speed_y = 0.0;
            acceleration_y = 0.0;
            speed_x = 0.0;
        }
        
        if acceleration_y != 0.0 {
            player[1] += speed_y;
            if speed_y > -1.5 {
                speed_y += acceleration_y;
            }
        }

        acceleration_y = g;

        if left && speed_x > - 2.0 {
            let acceleration_x = if speed_x > 0.0 {
                -0.042
            } else {
                -0.01
            };
            speed_x += acceleration_x;
        }
        if right && speed_x < 2.0 {
            let acceleration_x = if speed_x < 0.0 {
                0.042
            } else {
                0.01
            };
            speed_x += acceleration_x;
        }
        if !left && !right {
            if speed_x > -0.1 && speed_x < 0.1 {
                speed_x = 0.0;
            }
            speed_x *= 0.9;
        }
        player[0] += speed_x;
        
        let relative_position_to_floors = (player[1] - current_height_of_first_floor + 2.0) % floorgap;
        let whichfloor = (player[1] - current_height_of_first_floor + 2.0) / floorgap;
        if relative_position_to_floors <= 2.0 && speed_y <= 0.0 && whichfloor < 6.0 && whichfloor >= 0.0 {
            let floor = floors[whichfloor as usize];
            if (player[0] as u32 + (playerwidth + 6) / 2) >= floor[0] && (player[0] as u32 - (playerwidth + 6) / 2) <= floor[1] {
                if space {
                    speed_y = 2.5 + ((speed_x.abs() - 0.6).abs() as f64).powi(4) / max_speed_x;
                } else {
                    speed_y = 0.0;
                    acceleration_y = 0.0;
                }
            }
        }
        
        let powerscroll = player[1] - current_height_of_first_floor - 4.0 * floorgap;
        if powerscroll > 0.0 {
            let actual = powerscroll.sqrt() / 5.0;
            current_height_of_first_floor -= actual;
            player[1] -= actual;
        } else if scrolling {
            current_height_of_first_floor -= 0.1;
        }
        if current_height_of_first_floor < 0.0 {
            scrolling = true;
            current_height_of_first_floor += floorgap;
            floors.rotate_left(1);
            loop {
                let x: u32 = rng.gen::<u32>() % gamewidth;
                if x < (gamewidth / 3) {
                    let left = x + wallwidth;
                    let width: u32 = rng.gen::<u32>() % (gamewidth / 2);
                    if width > 60 {
                        floors[floors.len() - 1] = [left, left + width];
                        break;
                    }
                } else if x > (gamewidth / 3 * 2) {
                    let right = x;
                    let width: u32 = rng.gen::<u32>() % (gamewidth / 2);
                    if width > 60 {
                        floors[floors.len() - 1] = [right - width + wallwidth, right + wallwidth];
                        break;
                    }
                } else {
                    let x2: u32 = rng.gen::<u32>() % gamewidth;
                    let left  = if x2 > x { x  } else { x2 };
                    let right = if x2 > x { x2 } else { x  };
                    let width = right - left;
                    if width > 60 {
                        floors[floors.len() - 1] = [left, right];
                        break;
                    }
                }
            }
        }

        window.draw_2d(&e, |c, g, _| {
            piston_window::clear([0.5, 0.5, 0.5, 1.0], g);

            for (i, floor) in floors.iter().enumerate() {
                piston_window::rectangle(
                    [1.0, 0.0, 0.0, 1.0],
                    [   floor[0] as f64,
                        windowdim[1] as f64 - current_height_of_first_floor + floorheight as f64- (floorgap * i as f64),
                        (floor[1] - floor[0])  as f64,
                        floorheight as f64],
                    c.transform,
                    g);
            }

            let tx = player[0] - (playerwidth / 2) as f64;
            let ty = player[1];
            piston_window::image(&texture, [
                [playerscale[0], 0.0            , - 1.0 + tx * pixels_to_input_ratio_x],
                [0.0           , -playerscale[1], - 0.83333 + ty * pixels_to_input_ratio_y]
            ], g);
        });

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                match k.button {
                    Button::Keyboard(Key::Space) => space = true,
                    Button::Keyboard(Key::Left) => left = true,
                    Button::Keyboard(Key::Right) => right = true,
                    _ => (),
                }
            } else if k.state == ButtonState::Release {
                match k.button {
                    Button::Keyboard(Key::Space) => space = false,
                    Button::Keyboard(Key::Left) => left = false,
                    Button::Keyboard(Key::Right) => right = false,
                    _ => (),
                }
            }
        }
    }
}
