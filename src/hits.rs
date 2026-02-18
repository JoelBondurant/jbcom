use std::{
	arch::asm,
	ffi::CString,
	sync::atomic::{AtomicU64, Ordering},
};

/// A high-performance, dependency-free hit counter.
///
/// This uses raw Linux syscalls to memory-map an 8-byte file
/// directly to an `AtomicU64`.
pub struct HitCounter {
	ptr: *mut AtomicU64,
}

unsafe impl Send for HitCounter {}
unsafe impl Sync for HitCounter {}

impl HitCounter {
	pub fn new(file_name: &str) -> Self {
		unsafe {
			let path = CString::new(file_name).unwrap();
			// Syscall 2: OPEN, O_RDWR (2) | O_CREAT (64) = 66
			let fd: i64;
			asm!(
				"syscall",
				in("rax") 2, in("rdi") path.as_ptr(), in("rsi") 66, in("rdx") 0o666,
				lateout("rax") fd,
				out("rcx") _, out("r11") _,
			);
			if fd < 0 {
				panic!("Open failed: {}", fd);
			}
			// Syscall 8: LSEEK: Check size to avoid zeroing out existing data
			let size: i64;
			asm!(
				"syscall",
				in("rax") 8, in("rdi") fd, in("rsi") 0, in("rdx") 2, // SEEK_END
				lateout("rax") size,
				out("rcx") _, out("r11") _,
			);
			if size < 8 {
				// Syscall 77: FTRUNCATE: Size empty to 8 bytes.
				let res: i64;
				asm!(
					"syscall",
					in("rax") 77, in("rdi") fd, in("rsi") 8,
					lateout("rax") res,
					out("rcx") _, out("r11") _,
				);
				if res < 0 {
					panic!("Truncate failed: {}", res);
				}
			}
			// Syscall 9: MMAP: PROT_READ(1) | PROT_WRITE(2) = 3, MAP_SHARED(1)
			let addr_raw: i64;
			asm!(
				"syscall",
				in("rax") 9, in("rdi") 0, in("rsi") 8, in("rdx") 3, in("r10") 1, in("r8") fd, in("r9") 0,
				lateout("rax") addr_raw,
				out("rcx") _, out("r11") _,
			);
			if addr_raw < 0 && addr_raw > -4096 {
				panic!("mmap failed with error code: {}", addr_raw);
			}
			Self {
				ptr: addr_raw as *mut AtomicU64,
			}
		}
	}

	/// Increments the counter and returns the new value.
	///
	/// # Safety
	/// This operation is thread-safe via CPU-level atomic instructions.
	/// ```rust
	/// let counter = HitCounter::new("test.u64");
	/// assert!(counter.increment() > 0);
	/// ```
	pub fn increment(&self) -> u64 {
		unsafe { (*self.ptr).fetch_add(1, Ordering::SeqCst) + 1 }
	}

	#[allow(dead_code)]
	pub fn get(&self) -> u64 {
		unsafe { (*self.ptr).load(Ordering::SeqCst) }
	}
}

impl Drop for HitCounter {
	fn drop(&mut self) {
		unsafe {
			// Syscall 11: munmap(addr, len)
			asm!(
				"syscall",
				in("rax") 11, in("rdi") self.ptr, in("rsi") 8,
				out("rcx") _, out("r11") _,
				lateout("rax") _,
			);
		}
	}
}
