use gif::{Encoder, Frame, Repeat, SetParameter};
use rand::Rng;
use std::borrow::Cow;
use std::f64::consts::PI;
use std::fs::File;

const SIZE: u64 = 400;
const CYCLES: u16 = 5;
const RES: f64 = 0.001;
const FRAMES: u16 = 64;
const DELAY: u16 = 8;

fn main() {
    let colors = &vec![0, 0, 0, 0x00, 0xFF, 0x00];
    let file = File::create("test.gif").unwrap();
    let mut encoder = Encoder::new(file, SIZE as u16, SIZE as u16, colors).unwrap();
    encoder.set(Repeat::Infinite).unwrap();
    let mut rng = rand::thread_rng();
    let freq = rng.gen::<f64>() * 3.0;
    let mut phase = 0.0;
    for _i in 0..FRAMES {
        let mut buffer = vec![0; (SIZE * SIZE) as usize];
        let mut frame = Frame::default();
        frame.width = SIZE as u16;
        frame.height = SIZE as u16;
        frame.delay = DELAY;
        let mut t = 0.0;
        while t < (CYCLES as f64 * 2.0 * PI) {
            let x = t.sin() + 1.0;
            let y = (t * freq + phase).sin() + 1.0;
            let index = to_one_dimention(
                (x * (SIZE as f64 / 2.0 - 1.0)) as u64,
                (y * (SIZE as f64 / 2.0 - 1.0)) as u64,
            );
            buffer[index] = 1;
            t += RES;
        }
        phase += 0.1;
        frame.buffer = Cow::Borrowed(&buffer);
        encoder.write_frame(&frame).unwrap();
    }
}

fn to_one_dimention(x: u64, y: u64) -> usize {
    (x + y * SIZE as u64) as usize
}
