
use std::{fs::File, io::BufReader, time::Instant};

use clap::{App, Arg};
use pixel_canvas::Canvas;
use ray::{engine::Engine, image::Image, world::World};

fn main() {
    println!("Running raytracer...");
    let matches = App::new("Ray tracer")
        .version("0.1.0")
        .author("Manuel Deval <manuel.deval@gmail.com>")
        .about("Ray tracing")
        .arg(Arg::with_name("file")
                 .short("f")
                 .long("file")
                 .takes_value(true)
                 .help("world to render (yaml format)"))
        .get_matches();
    
    let yaml_file = matches.value_of("file").unwrap();
    let file = File::open(yaml_file).unwrap();
    let reader = BufReader::new(file);

    let world: World = serde_yaml::from_reader(reader).unwrap();
    let engine = Engine::new(world);
    let start = Instant::now();
    let image_to_display = engine.generate();
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    display_image(image_to_display);
}

pub fn display_image(image_to_display: Image) {
    let canvas = Canvas::new(
        image_to_display.width() as usize,
        image_to_display.height() as usize,
    )
    .title("Ray");
    canvas.render(move |_mouse, image| {
        let width = image.width() as usize;
        let height = image.height() as usize;

        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                let image_color = image_to_display
                    .get_color(x as u16, ((height - 1) - y) as u16)
                    .to_drawing_color();
                *pixel = pixel_canvas::Color::rgb(
                    image_color.r as u8,
                    image_color.g as u8,
                    image_color.b as u8,
                );
            }
        }
    });
}
