extern crate samplerate;
extern crate hound;

use samplerate::{Samplerate, ConverterType};
use hound::{WavSpec, WavWriter, SampleFormat};

fn main() {
    // Generate a 880Hz sine wave for 1 second in 44100Hz with one channel.
    let freq = std::f32::consts::PI * 880f32 / 44100f32;
    let input: Vec<f32> = (0..44100 * 10).map(|i| (freq * i as f32).sin()).collect();

    // Create a new converter.
    let converter = Samplerate::new(ConverterType::SincBestQuality, 44100, 48000, 1).unwrap();

    // Create a writer for writing the resampled data to disk.
    let mut writer_48000 = WavWriter::create("sine-48000.wav", WavSpec {
        channels: 1,
        sample_rate: 48000,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    }).unwrap();

    // Write the audio to the converter a loop, or if you may, as a stream.
    let chunk_size = 4410; // 100ms
    let mut resampled = vec![0.;chunk_size*480/441];
    let mut frame_ptr = 0;
    while frame_ptr < input.len() {
        let (input_processed, output_generated) = converter.process_into(&input[frame_ptr..frame_ptr+chunk_size], &mut resampled).unwrap();
        resampled[..output_generated].iter().for_each(|i| writer_48000.write_sample(*i).unwrap());
        frame_ptr += input_processed;
    }
    // Drain the last bits
    loop {
        let (_, output_generated) = converter.process_into_last(&[0.;0], &mut resampled).unwrap();
        if output_generated == 0 {
            // No more data was available, we can assume it's drained
            break;
        }
        resampled[..output_generated].iter().for_each(|i| writer_48000.write_sample(*i).unwrap());
    }
}