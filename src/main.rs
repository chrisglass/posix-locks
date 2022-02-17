use file_lock::{FileLock, FileOptions};
use std::fs::OpenOptions;
use std::{thread, time};
use std::io::prelude::*;

fn main() {
    let should_we_block  = false;
    let options = FileOptions::new()
                        .write(true)
                        .create(true)
                        .append(true);

    let mut filelock = match FileLock::lock("test.lock", should_we_block, options) {
        Ok(lock) => lock,
        Err(err) => panic!("Error getting write lock: {}", err),
    };

    println!("Lock acquired, sleeping forever....");

    loop {
        thread::sleep(time::Duration::from_secs(2));
        
    }

    //filelock.file.write_all(b"Hello, World!").is_ok();

    // Manually unlocking is optional as we unlock on Drop
    //filelock.unlock();
}
