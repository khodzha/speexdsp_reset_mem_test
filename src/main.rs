extern crate speexdsp;

use speexdsp::speex_resample::*;
use std::vec::Vec;

fn main() {
    // 128 samples of sine wave and 128 zero samples so we could get 256 samples after resampling
    let data: Vec<f32> = (0..256).map(|i| {
        let ii = i as f32;
        match i {
            0..=127 => (std::f32::consts::PI * ii / 16.).sin(),
            _ => 0.
        }
    }).collect();

    let mut output = vec![0.; 256];

    let mut in_len = 256;
    let mut out_len = 256;

    let mut u = SpeexResamplerState::new(1, 44100, 44100 * 2, 5);
    u.skip_zeros();

    // this reallocs self.mem in SpeexResamplerState and breaks the code
    // this runs okay if you comment this line out
    u.reset_mem();

    u.process_float(0, &data, &mut in_len, &mut output[0..256], &mut out_len);
    println!("(in_len, out_len): {:?}", (in_len, out_len));

    println!("data = {:?}", data);
    println!("output = {:?}", output);
}
