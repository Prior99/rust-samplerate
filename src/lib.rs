extern crate libsamplerate_sys;

pub mod converter_type;
pub mod error;
pub mod samplerate;

pub use crate::converter_type::*;
pub use crate::error::*;
pub use crate::samplerate::*;
use libsamplerate_sys::*;
use std::ffi::CStr;

/// Perform a simple samplerate conversion of a large chunk of audio.
/// This calls `src_simple` of libsamplerate which is not suitable for streamed audio. Use the
/// `Samplerate` struct instead for this.
///
/// # Example
///
/// ```
/// use samplerate::{convert, ConverterType};
///
/// // Generate a 880Hz sine wave for 1 second in 44100Hz with one channel.
/// let freq = std::f32::consts::PI * 880f32 / 44100f32;
/// let mut input: Vec<f32> = (0..44100).map(|i| (freq * i as f32).sin()).collect();
///
/// // Resample the input from 44100Hz to 48000Hz.
/// let resampled = convert(44100, 48000, 1, ConverterType::SincBestQuality, &input).unwrap();
/// assert_eq!(resampled.len(), 48000);
/// ```
pub fn convert(from_rate: usize, to_rate: usize, channels: usize, converter_type: ConverterType, input: &[f32]) -> Result<Vec<f32>, Error> {
    let ratio = to_rate as f64 / from_rate as f64;
    let output_len = (ratio * input.len() as f64) as usize;
    let mut output = vec![0f32;output_len];
    let mut src = SRC_DATA {
        data_in: input.as_ptr(),
        data_out: output.as_mut_ptr(),
        input_frames: input.len() as i32,
        output_frames: output_len as i32,
        src_ratio: ratio,
        end_of_input: 0,
        input_frames_used: 0,
        output_frames_gen: 0,
    };
    let error_int = unsafe { src_simple(&mut src as *mut SRC_DATA, converter_type as i32, channels as i32) };
    let error_code = ErrorCode::from_int(error_int);
    match error_code {
        ErrorCode::NoError => Ok(output),
        _ => Err(Error::from_code(error_code)),
    }
}

/// Returns the version of `libsamplerate` used by this crate as a string.
pub fn version() -> &'static str {
    unsafe { CStr::from_ptr(src_get_version()) }.to_str().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_version() {
        assert_eq!(version(), "libsamplerate-0.1.9 (c) 2002-2008 Erik de Castro Lopo");
    }

    #[test]
    fn simple_sample_rate_conversion() {
        // Setup sample data and storage.
        let freq = ::std::f32::consts::PI * 880f32 / 44100f32;
        let input: Vec<f32> = (0..44100).map(|i| (freq * i as f32).sin()).collect();
        // Convert from 44100Hz to 48000Hz.
        let resampled = convert(44100, 48000, 1, ConverterType::SincBestQuality, &input).unwrap();
        // Convert back to 44100Hz.
        let output = convert(48000, 44100, 1, ConverterType::SincBestQuality, &resampled).unwrap();
        // Expect the difference between all input frames and all output frames to be less than
        // an epsilon.
        let error = input.iter().zip(output).fold(0f32, |max, (input, output)| max.max((input - output).abs()));
        assert!(error < 0.002);
    }
}
