use crate::helper;
use std::{
    ffi::{c_char, c_int, c_uint, CString},
    fs::{File, OpenOptions},
    io::{BufRead, BufReader},
    os::{fd::AsRawFd},
    path::Path,
    process::Command,
};

// Opertaion
static OP_INIT_KEY: i32 = 0x800;
static OP_READ_MEM: i32 = 0x801;
static OP_WRITE_MEM: i32 = 0x802;
static OP_MODULE_BASE: i32 = 0x803;
//C FFI

#[repr(C)]
struct MODULE_BASE {
    pid: i32,
    name: [c_char; 0x100],
    base: usize,
}
extern "C" {
    fn ioctl(fd: c_int, request: c_int, argp: *mut MODULE_BASE) -> c_int;
}

pub struct Kernel {
    pub pid: i32,
    pub kernel_name: String,
    fd: c_int,
}

impl Kernel {
    pub fn new() -> Self {
        let dev_name = helper::get_dev_name();
        let binding = dev_name.clone();
        let dev_name_splite: Option<&str> = binding.split("/").last();
        let kernel_name = if let Some(kn) = dev_name_splite {
            println!("驱动名:{}",kn);
            kn
        } else {
            ""
        };
        let mut raw_fd: c_int = 0;
        if kernel_name == "" {
            panic!("未找到驱动文件");
        } else {
            let fd = OpenOptions::new()
                .read(true)
                .write(true)
                .open(dev_name)
                .unwrap();
            raw_fd = fd.as_raw_fd() as c_int;
            println!("驱动Raw_FD : {}",raw_fd);
        }
        Kernel {
            pid: 0,
            kernel_name: kernel_name.to_owned(),
            fd: raw_fd,
        }
    }

    pub fn get_pid(&mut self, package_name: &str) -> i32 {
        let mut cmd = Command::new("pidof");
        cmd.arg(package_name);
        let output = cmd.output().unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let pid_str = stdout.trim();
        match pid_str.parse::<i32>() {
            Ok(pid) => {
                self.pid = pid;
                pid
            }
            Err(_) => 0,
        }
    }
    pub fn get_module_base_addr_v4(&mut self, name: &[u8]) -> Option<usize> {
        let c_name = CString::new(name).expect("无效的模块名（非零终止字符串）");
        let mut mb = MODULE_BASE {
            pid: self.pid,
            name: [0 as c_char; 0x100],
            base: 0,
        };
        unsafe {
            let c_name_bytes = c_name.as_bytes_with_nul();
            let c_name_i8: &[i8] = std::mem::transmute(c_name_bytes);
            let name_len = std::cmp::min(c_name_bytes.len(), mb.name.len());
            mb.name[..name_len].copy_from_slice(&c_name_i8[..name_len]);
            mb.pid = self.pid;
            let res = ioctl(self.fd, OP_MODULE_BASE, &mut mb);
            if res == 0 {
                Some(mb.base)
            } else {
                None
            }
        }
    }
}
