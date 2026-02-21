use std::{thread::sleep, time::Duration};

use nix::{
    fcntl::{O_RDWR, open},
    sys::stat::S_IRWXU,
    unistd::{Fork, fork, read, write},
};

fn main() {
    let f = open(
        b"virtualization/process-api-5-1/src/bin/hw2_file.txt".as_slice(),
        O_RDWR,
        S_IRWXU,
    )
    .unwrap();
    let mut buf = [0; 5];

    // Note here that since the buf len is 5, both reads 5 bytes from the same file.
    // at different offsets.
    match fork().unwrap() {
        Fork::Parent(_) => {
            let n_read = read(f, &mut buf).unwrap();
            let data = core::str::from_utf8(&buf[..n_read]).unwrap();
            println!("parent read ({n_read} bytes): {data}");

            sleep(Duration::from_millis(200));
            write(f, b"parent is cool").unwrap();
        }

        Fork::Child => {
            let n_read = read(f, &mut buf).unwrap();
            let data = core::str::from_utf8(&buf[..n_read]).unwrap();
            println!("child read ({n_read} bytes): {data}");

            sleep(Duration::from_millis(200));

            write(f, b"child wrote some stuff").unwrap();
        }
    }
}
