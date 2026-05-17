//! Must be a separate test because it modifies the _global_ rayon pool.
use std::{fs::File, path::Path, thread};
use stegano_f5_jpeg_decoder::Decoder;

// Progressive JPEG decoding has deep stack frames. Windows' default thread
// stack (~1 MiB) is too small; Linux/macOS default to ~8 MiB. Pin 8 MiB
// explicitly so the test behaves identically across platforms.
const STACK_SIZE: usize = 8 * 1024 * 1024;

#[test]
fn decoding_in_global_pool() {
    let path = Path::new("tests/reftest/images/progressive3.jpg");

    rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .stack_size(STACK_SIZE)
        .build_global()
        .unwrap();

    let _: Vec<_> = (0..1024)
        .map(|_| {
            thread::Builder::new()
                .stack_size(STACK_SIZE)
                .spawn(move || {
                    let mut decoder = Decoder::new(File::open(path).unwrap());
                    let _ = decoder.decode().unwrap();
                })
                .unwrap();
        })
        .collect();
}
