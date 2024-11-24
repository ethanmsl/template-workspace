//! Tracing - with Jon Gj.
//!
//! clear; RUST_LOG=trace carrbn samples_tracing  bb ccc dddd

use std::{io::Read, thread};

use tracing::{Level, debug, error, info, info_span, span, trace, warn};
fn main() {
        tracing_subscriber::fmt::init();
        let mut handles = vec![];
        let span = span!(Level::INFO, "main");
        let _guard = span.enter();
        for file in std::env::args().skip(1) {
                let handle = thread::spawn(move || {
                        let span = info_span!("file", fname = %file);
                        let _guard = span.enter();
                        info!("opening the file");
                        // let mut file = std::fs::File::open(file).unwrap();
                        info!("reading file contents");
                        // let mut bytes = Vec::new();
                        // file.read_exact(&mut bytes).unwrap();
                        info!(bytes = 0, "parsing");
                        // ..
                        info!("done with file");
                });
                handles.push(handle);
        }
        let span = span!(Level::INFO, "joining");
        let _guard = span.enter();
        for handle in handles {
                let span = span!(Level::INFO, "indiv_join");
                let _guard = span.enter();
                debug!("joining thread");
                handle.join().expect("thread join error");
                debug!("thread joined");
        }
}
