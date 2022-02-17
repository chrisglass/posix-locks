use file_lock::{FileLock, FileOptions};
use std::{thread, time};

fn main() {
    let should_we_block  = false;
    let options = FileOptions::new()
                        .write(true)
                        .create(true)
                        .append(true);

    let _filelock = match FileLock::lock("test.lock", should_we_block, options) {
        Ok(lock) => lock,
        Err(err) => panic!("Error getting write lock: {}", err),
    };

    println!("Lock acquired, sleeping forever....");

    loop {
        thread::sleep(time::Duration::from_secs(2));
        
    }
}
