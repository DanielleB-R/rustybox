// Things that used to live in include/bb_archive.h.

use crate::libbb::llist::llist_t;
use crate::librb::bb_uidgid_t;
use crate::librb::fd_pair;
use crate::librb::size_t;
use crate::librb::smallint;
use crate::librb::uoff_t;
use libc;
use libc::gid_t;
use libc::mode_t;
use libc::off_t;
use libc::pid_t;
use libc::stat;
use libc::uid_t;
use libc::FILE;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_header_t {
  pub name: *mut libc::c_char,
  pub link_target: *mut libc::c_char,
  pub tar__uname: *mut libc::c_char,
  pub tar__gname: *mut libc::c_char,
  pub size: libc::off_t,
  pub uid: libc::uid_t,
  pub gid: libc::gid_t,
  pub mode: libc::mode_t,
  pub mtime: libc::time_t,
  pub device: libc::dev_t,
}

// Declared in bb_archive.h but defined in get_header_cpio.c.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hardlinks_t {
  pub next: *mut hardlinks_t,
  pub inode: libc::c_int,
  pub mode: libc::c_int,
  pub mtime: libc::c_int,
  pub uid: libc::c_int,
  pub gid: libc::c_int,
  pub name: [libc::c_char; 1],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_handle_t {
  pub ah_flags: libc::c_uint,
  pub src_fd: libc::c_int,
  pub filter: Option<unsafe extern "C" fn(_: *mut archive_handle_t) -> libc::c_char>,
  pub accept: *mut llist_t,
  pub reject: *mut llist_t,
  pub passed: *mut llist_t,
  pub file_header: *mut file_header_t,
  pub link_placeholders: *mut llist_t,
  pub action_header: Option<unsafe extern "C" fn(_: *const file_header_t) -> ()>,
  pub action_data: Option<unsafe extern "C" fn(_: *mut archive_handle_t) -> ()>,
  pub seek: Option<unsafe extern "C" fn(_: libc::c_int, _: off_t) -> ()>,
  pub offset: off_t,
  pub tar__strip_components: libc::c_uint,
  pub tar__end: smallint,
  pub tar__longname: *mut libc::c_char,
  pub tar__linkname: *mut libc::c_char,
  pub tar__to_command: *mut libc::c_char,
  pub tar__to_command_shell: *const libc::c_char,
  pub cpio__blocks: uoff_t,
  pub cpio__owner: bb_uidgid_t,
  pub cpio__hardlinks_to_create: *mut hardlinks_t,
  pub cpio__created_hardlinks: *mut hardlinks_t,
  pub dpkg__buffer: *mut libc::c_char,
  pub dpkg__action_data_subarchive:
    Option<unsafe extern "C" fn(_: *mut archive_handle_t) -> libc::c_char>,
  pub dpkg__sub_archive: *mut archive_handle_t,
}