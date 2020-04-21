use std::cmp::PartialOrd;

macro_rules! test_template {
    // `()` indicates that the macro takes no argument.
    ($func_name:ident, $data_len:expr, $sig_freq: expr, $ch_count:expr, $from_rate:expr, $to_rate:expr, $bleed_size:expr, $in_bleed_eps:expr, $out_bleed_eps:expr) => {
        #[test]
        fn $func_name() {
            const N: usize = $data_len;
            const N_CH: usize = $ch_count;
            let data = (0..N * N_CH).map(|i| (2.0 * std::f64::consts::PI * $sig_freq * ((i / N_CH) as f64 / N as f64)).sin()).map(|x| x as f32).collect::<Vec<f32>>();
            let down_data = crate::convert($from_rate, $to_rate, N_CH, crate::ConverterType::SincBestQuality, &data).unwrap();
            let up_data = crate::convert($to_rate, $from_rate, N_CH, crate::ConverterType::SincBestQuality, &down_data).unwrap();
            assert_eq!(up_data.len(), ((N * $to_rate + ($from_rate - 1)) / $from_rate * $from_rate + ($to_rate - 1)) / $to_rate * N_CH);
            let max_diff = data.iter().enumerate().zip(up_data.iter()).map(|((i, a), b)| (i, (a - b).abs(),)).max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap();
            let max_diff_bleed = data.iter().take((N - $bleed_size) * N_CH).enumerate().skip($bleed_size * N_CH).zip(up_data.iter().skip($bleed_size * N_CH)).map(|((i, a), b)| (i, (a - b).abs(),)).max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap();
            dbg!(max_diff, max_diff_bleed);
            assert!(max_diff_bleed.1 < $in_bleed_eps);
            assert!(max_diff.1 < $out_bleed_eps);
        }
    };
}

test_template!(mono_intdiv, 16384, 128.0, 1, 2, 1, 512, 1e-6, 0.02);
test_template!(multich_2_intdiv, 16384, 128.0, 2, 2, 1, 512, 1e-6, 0.02);
test_template!(multich_7_intdiv, 16384, 128.0, 7, 2, 1, 512, 1e-6, 0.02);
test_template!(mono_nonintdiv, 44100, 128.0, 1, 44100, 48000, 512, 1e-6, 0.001);
test_template!(multich_2_nonintdiv, 44100, 128.0, 2, 44100, 48000, 512, 1e-6, 0.001);
test_template!(multich_7_nonintdiv, 44100, 128.0, 7, 44100, 48000, 512, 1e-6, 0.001);