/// Read target's memory trait.
pub trait ReadTargetMem {
    /// Read target physical memory.
    fn read_phys(&mut self, paddr: usize, buf: &mut [u8]);
    /// Read target virtual memory.
    fn read_virt(&mut self, vaddr: usize, buf: &mut [u8]);
}

/// Write target's memory trait.
pub trait WriteTargetMem {
    /// Write target physical memory.
    fn write_phys(&mut self, paddr: usize, buf: &[u8]);
    /// Write target virtual memory.
    fn write_virt(&mut self, vaddr: usize, buf: &[u8]);
}

#[cfg(feature = "qemu")]
pub use qemu::QemuMem;

#[cfg(feature = "qemu")]
mod qemu {
    use super::{ReadTargetMem, WriteTargetMem};

    /// LibAFL QEMU memory access.
    pub struct QemuMem;

    impl ReadTargetMem for QemuMem {
        fn read_phys(&mut self, paddr: usize, buf: &mut [u8]) {
            unsafe {
                libafl_qemu::sys::cpu_physical_memory_rw(
                    paddr as u64,
                    buf.as_mut_ptr() as *mut _,
                    buf.len() as u64,
                    false,
                );
            }
        }
        fn read_virt(&mut self, vaddr: usize, buf: &mut [u8]) {
            let cpu_ptr = unsafe { libafl_qemu::sys::libafl_qemu_get_cpu(0) };
            unsafe {
                libafl_qemu::sys::cpu_memory_rw_debug(
                    cpu_ptr,
                    vaddr as u64,
                    buf.as_mut_ptr() as *mut _,
                    buf.len(),
                    false,
                );
            }
        }
    }

    impl WriteTargetMem for QemuMem {
        fn write_phys(&mut self, paddr: usize, buf: &[u8]) {
            unsafe {
                libafl_qemu::sys::cpu_physical_memory_rw(
                    paddr as u64,
                    buf.as_ptr() as *mut _,
                    buf.len() as u64,
                    true,
                );
            }
        }
        fn write_virt(&mut self, vaddr: usize, buf: &[u8]) {
            let cpu_ptr = unsafe { libafl_qemu::sys::libafl_qemu_get_cpu(0) };
            unsafe {
                libafl_qemu::sys::cpu_memory_rw_debug(
                    cpu_ptr,
                    vaddr as u64,
                    buf.as_ptr() as *mut _,
                    buf.len(),
                    true,
                );
            }
        }
    }
}
