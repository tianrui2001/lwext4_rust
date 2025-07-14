#![no_std]
#![feature(linkage)]
#![feature(c_variadic, c_size_t)]
#![feature(associated_type_defaults)]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate alloc;

#[macro_use]
extern crate log;

mod ulibc;

// include!("bindings.rs");
pub mod bindings;
pub mod blockdev;
pub mod file;

pub use blockdev::*;
pub use file::{Ext4File, InodeTypes};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// Creates a symbolic link.
pub fn file_symlink(&mut self, target: &str, link_path: &str) -> Result<(), i32> {
    let c_target = CString::new(target).unwrap();
    let c_link_path = CString::new(link_path).unwrap();
    
    // FFI 函数不需要 fs 指针，可以直接调用
    let r = unsafe {
        ext4_fsymlink(c_target.as_ptr(), c_link_path.as_ptr())
    };
    
    if r != (EOK as i32) { Err(r) } else { Ok(()) }
}

/// Reads a symbolic link's target.
pub fn file_readlink(&mut self, path: &str, buf: &mut [u8]) -> Result<usize, i32> {
    let c_path = CString::new(path).unwrap();
    let mut read_len: usize = 0;
    
    let r = unsafe {
        ext4_readlink(
            c_path.as_ptr(),
            buf.as_mut_ptr() as *mut i8,
            buf.len(),
            &mut read_len, // 将 read_len 的可变引用传给 rcnt
        )
    };

    if r != (EOK as i32) { Err(r) } else { Ok(read_len) }
}

/// Changes file mode. (这个函数你已经有了，我们确认一下实现)
// pub fn file_mode_set(&mut self, mode: u32) -> Result<usize, i32> {
//     ...
// }
// 你的 file_mode_set 就是我们要找的 chmod 实现！

/// Changes file owner and group.
pub fn file_chown(&mut self, path: &str, uid: u32, gid: u32) -> Result<(), i32> {
    let c_path = CString::new(path).unwrap();
    
    let r = unsafe {
        ext4_owner_set(c_path.as_ptr(), uid, gid)
    };
    
    if r != (EOK as i32) { Err(r) } else { Ok(()) }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
