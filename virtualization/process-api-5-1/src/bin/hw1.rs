use nix::unistd::{Fork, fork};

static mut X: i32 = 100;

fn main() {
    match fork().unwrap() {
        Fork::Parent(_) => {
            println!("running the parent process");
            println!("X's memory address: {:?}", &raw const X);
            for i in 10..20 {
                unsafe {
                    X = i;
                }
                let x = unsafe { core::ptr::read_volatile(&raw const X) };
                println!("the value of x in parent is: {x}");
            }
        }
        Fork::Child => {
            println!("running the child process");
            println!("X's memory address: {:?}", &raw const X);

            // Note that the child does not get effected by parent changing `X` since
            // it's actually reading his own `X`
            for _ in 0..10 {
                let x = unsafe { core::ptr::read_volatile(&raw const X) };
                println!("the value of x in child is: {x}");
            }
        }
    }
}
