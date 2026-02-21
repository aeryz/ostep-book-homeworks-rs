use nix::unistd::{Fork, close, fork, pipe, read, write};

fn main() {
    let (pipe_read, pipe_write) = pipe().unwrap();

    match fork().unwrap() {
        Fork::Parent(_) => {
            // Can easily do with waiting the child process:
            // waitpid(child, None).unwrap();
            close(pipe_read).unwrap();
            write(pipe_write, b"goodbye").unwrap();
        }
        Fork::Child => {
            close(pipe_write).unwrap();

            println!("hello");

            let mut buf = [0; 100];

            let n_read = read(pipe_read, &mut buf).unwrap();
            println!("{}", core::str::from_utf8(&buf[..n_read]).unwrap());
        }
    }
}
