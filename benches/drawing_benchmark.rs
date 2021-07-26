use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

const WIDTH: usize = 240;
const HEIGHT: usize = 240;

const PIXEL_COUNT: usize = WIDTH * HEIGHT;

struct Buffer {
    buffer: [u32; PIXEL_COUNT]
}

impl Buffer {
    fn new() -> Self {
        Buffer {
            buffer: [0; PIXEL_COUNT]
        }
    }

    fn xy(&mut self, point: Point2D) -> Option<&mut u32> {
        if point.is_contained() {
            let calc = (point.y - 1) * WIDTH + point.x - 1;

            Some(&mut self.buffer[calc])
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Point2D {
    x: usize,
    y: usize
}


impl Point2D {
    fn new(x: usize, y: usize) -> Self {
        Point2D {
            x,
            y
        }
    }

    fn is_nonzero(&self) -> bool {
        (self.x as u16) << 8 | self.y as u16 != 0
    }

    fn is_contained(&self) -> bool {
        if self.is_nonzero() && self.x <= WIDTH && self.y <= HEIGHT {
            true
        } else {
            false
        }
    }

    fn x_distance_to(&self, other: &Self) -> isize {
        self.x as isize - other.x as isize
    }

    fn y_distance_to(&self, other: &Self) -> isize {
        self.y as isize - other.y as isize
    }
}

fn line(point_1: &Point2D, point_2: &Point2D, buffer: &mut Buffer) {
    let mut p1 = *point_1;
    let mut p2 = *point_2;

    let mut steep: bool = false;

    let x_distance: isize = p2.x_distance_to(&p1); // p2.x - p1.x
    let y_distance: isize = p2.y_distance_to(&p1); // p2.y - p1.y

    // if the line is steep, we transpose the image
    if x_distance.abs() < y_distance.abs() {
        std::mem::swap(&mut p1.x, &mut p1.y);
        std::mem::swap(&mut p2.x, &mut p2.y);

        steep = true;
    }

    // make it left−to−right
    if x_distance.is_negative() {
        std::mem::swap(&mut p1.x, &mut p2.x);
        std::mem::swap(&mut p1.y, &mut p2.y);
    }

    let swapped_x_distance = p2.x_distance_to(&p1);
    let swapped_y_distance = p2.y_distance_to(&p1);

    let mut error_double_difference: isize = swapped_y_distance.abs() * 2;
    let mut error_double: isize = 0;

    let mut y = p1.y;

    for x in p1.x..=p2.x {
        if steep {
            *buffer.xy(Point2D{y, x}).unwrap() = 255 << 16 | 255 << 8 | 255;
        } else {
            *buffer.xy(Point2D{x, y}).unwrap() = 255 << 16 | 255 << 8 | 255;
        }

        error_double += error_double_difference;
        if error_double > swapped_x_distance {
            if swapped_y_distance.is_positive() {y += 1} else {y -= 1};
            error_double -= swapped_x_distance * 2;
        }
    }
}


fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    let x_1 = rng.gen_range(1..=240);
    let y_1 = rng.gen_range(1..=240);

    let x_2 = rng.gen_range(1..=240);
    let y_2 = rng.gen_range(1..=240);

    let p1: Point2D = Point2D::new(x_1, y_1);
    let p2: Point2D = Point2D::new(x_2, y_2);

    let mut buffer: Buffer = Buffer::new();

    c.bench_function("lines", |b| b.iter(|| line(black_box(&p1), black_box(&p2), black_box(&mut buffer))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
