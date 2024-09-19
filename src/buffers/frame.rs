use crate::common::space::{ScreenPoint, ScreenScalar};
use glamour::Vector2;
use palette::{rgb, Srgb};
use std::ops::DerefMut;

pub struct FrameBuffer<'a, D: DerefMut<Target = [u32]>> {
    data: &'a mut D,
    width: u32,
    height: u32,
}

impl<'a, D: DerefMut<Target = [u32]>> FrameBuffer<'a, D> {
    const INSIDE: u8 = 0b0000;
    const LEFT: u8 = 0b0001;
    const RIGHT: u8 = 0b0010;
    const BOTTOM: u8 = 0b0100;
    const TOP: u8 = 0b1000;

    pub fn new(buffer: &'a mut D, dimensions: Vector2<u32>) -> Self {
        Self {
            data: buffer,
            width: dimensions.x,
            height: dimensions.y,
        }
    }

    // Cohen-Sutherland algorithm
    fn compute_outcode(&self, x: ScreenScalar, y: ScreenScalar) -> u8 {
        let mut code = Self::INSIDE;
        if x < 0.0 {
            code |= Self::LEFT;
        } else if x > self.width as ScreenScalar {
            code |= Self::RIGHT;
        }
        if y < 0.0 {
            code |= Self::BOTTOM;
        } else if y > self.height as ScreenScalar {
            code |= Self::TOP;
        }

        code
    }

    // Cohen-Sutherland algorithm
    fn clip_line(
        &self,
        p1: &mut ScreenPoint,
        p2: &mut ScreenPoint,
        mut outcode1: u8,
        mut outcode2: u8,
    ) -> bool {
        loop {
            if (outcode1 | outcode2) == 0 {
                // Both points are inside the screen
                return true;
            } else if (outcode1 & outcode2) != 0 {
                // Both points are outside the screen on the same side
                return false;
            } else {
                // At least one point is outside, find an outside point
                let outcode_out = if outcode1 != 0 { outcode1 } else { outcode2 };

                // Find intersection point
                let x: ScreenScalar;
                let y: ScreenScalar;

                if (outcode_out & Self::TOP) != 0 {
                    x = p1.x + (p2.x - p1.x) * (self.height as ScreenScalar - p1.y) / (p2.y - p1.y);
                    y = self.height as f32;
                } else if (outcode_out & Self::BOTTOM) != 0 {
                    x = p1.x + (p2.x - p1.x) * (0.0 - p1.y) / (p2.y - p1.y);
                    y = 0.0;
                } else if (outcode_out & Self::RIGHT) != 0 {
                    y = p1.y + (p2.y - p1.y) * (self.width as ScreenScalar - p1.x) / (p2.x - p1.x);
                    x = self.width as f32;
                } else {
                    y = p1.y + (p2.y - p1.y) * (0.0 - p1.x) / (p2.x - p1.x);
                    x = 0.0;
                }

                // Replace the outside point with the intersection point
                if outcode_out == outcode1 {
                    p1.x = x;
                    p1.y = y;
                    outcode1 = self.compute_outcode(x, y);
                } else {
                    p2.x = x;
                    p2.y = y;
                    outcode2 = self.compute_outcode(x, y);
                }
            }
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Srgb<u8>) {
        let rgb_color: u32 = color.into_u32::<rgb::channels::Rgba>() >> 8;

        let index = y * self.width + x;
        if x < self.width && y < self.height {
            self.data[index as usize] = rgb_color;
        }
    }

    fn draw_line_inside(&mut self, p1: &ScreenPoint, p2: &ScreenPoint, color: Srgb<u8>) {
        let mut x0 = p1.x.round() as i32;
        let mut y0 = p1.y.round() as i32;
        let x1 = p2.x.round() as i32;
        let y1 = p2.y.round() as i32;

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut error = dx + dy;

        loop {
            self.set_pixel(x0 as u32, y0 as u32, color);
            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * error;
            if e2 >= dy {
                error += dy;
                x0 += sx;
            }
            if e2 <= dx {
                error += dx;
                y0 += sy;
            }
        }
    }

    pub fn draw_line(&mut self, p1: &ScreenPoint, p2: &ScreenPoint, color: Srgb<u8>) {
        let outcode1 = self.compute_outcode(p1.x, p1.y);
        let outcode2 = self.compute_outcode(p2.x, p2.y);

        if (outcode1 | outcode2) == Self::INSIDE {
            self.draw_line_inside(p1, p2, color);
        } else {
            let mut cloned_p1 = p1.clone();
            let mut cloned_p2 = p2.clone();

            self.clip_line(&mut cloned_p1, &mut cloned_p2, outcode1, outcode2);

            self.draw_line_inside(&cloned_p1, &cloned_p2, color);
        }
    }

    pub fn clear(&mut self) {
        self.data.fill(0);
    }
}
