//! Process management syscalls

use crate::{
    mm::{translated_byte_buffer, translated_ref, translated_refmut, MapPermission},
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next, get_trace_info,
        insert_framed_area_to_current_task, remove_framed_area_from_current_task,
        suspend_current_and_run_next,
    },
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let token = current_user_token();
    let buf = translated_byte_buffer(token, ts as *const u8, core::mem::size_of::<TimeVal>());

    let time_us = get_time_us();
    let tv = TimeVal {
        sec: time_us / 1_000_000,
        usec: time_us % 1_000_000,
    };
    let tv_bytes = unsafe {
        core::slice::from_raw_parts(
            &tv as *const TimeVal as *const u8,
            core::mem::size_of::<TimeVal>(),
        )
    };
    let mut len = 0;
    for piece in buf {
        let l = piece.len().min(tv_bytes.len() - len);
        piece[..l].copy_from_slice(&tv_bytes[len..len + l]);
        len += l;
        if len == tv_bytes.len() {
            break;
        }
    }
    if len == tv_bytes.len() {
        0
    } else {
        -1
    }
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match trace_request {
        0 => {
            let id = translated_ref(current_user_token(), id as *const u8, true);
            if let Some(id) = id {
                *id as isize
            } else {
                -1
            }
        }
        1 => {
            let id = translated_refmut(current_user_token(), id as *const u8, true);
            if let Some(id) = id {
                *id = data as u8;
                0
            } else {
                -1
            }
        }
        2 => get_trace_info(id) as isize,
        _ => -1,
    }
}

bitflags! {
    /// map permission corresponding to that in pte: `R W X U`
    pub struct PortPermission: u8 {
        ///Readable
        const R = 1 << 0;
        ///Writable
        const W = 1 << 1;
        ///Excutable
        const X = 1 << 2;
    }
}

impl From<PortPermission> for MapPermission {
    fn from(p: PortPermission) -> Self {
        let mut m = MapPermission::empty();
        if p.contains(PortPermission::R) {
            m |= MapPermission::R;
        }
        if p.contains(PortPermission::W) {
            m |= MapPermission::W;
        }
        if p.contains(PortPermission::X) {
            m |= MapPermission::X;
        }
        m
    }
}

impl TryFrom<usize> for PortPermission {
    type Error = ();
    fn try_from(p: usize) -> Result<Self, Self::Error> {
        PortPermission::from_bits(p as u8)
            .and_then(|p| if p.is_empty() { None } else { Some(p) })
            .ok_or(())
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    let port = match PortPermission::try_from(port) {
        Ok(p) => p,
        Err(_) => return -1,
    };

    if !insert_framed_area_to_current_task(start, len, port) {
        -1
    } else {
        0
    }
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    if remove_framed_area_from_current_task(start, len) {
        0
    } else {
        -1
    }
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
