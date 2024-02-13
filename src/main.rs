use KernelHack::{helper, kernel};



fn main() {
    let mut kn = kernel::Kernel::new();
    // kn.get_pid("com.tencent.tmgp.pubgmhd");
    println!("{}",helper::kernel_version().unwrap());
}
