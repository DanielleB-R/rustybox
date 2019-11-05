use libc;
extern "C" {
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
  //TODO: supply a pointer to char[11] buffer (avoid statics)?
  #[no_mangle]
  fn bb_mode_string(mode: mode_t) -> *const libc::c_char;
  /* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
   * But potentially slow, don't use in one-billion-times loops */
  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn utoa(n: libc::c_uint) -> *mut libc::c_char;
}

use crate::librb::__dev_t;
use crate::librb::__gid_t;
use crate::librb::__mode_t;
use crate::librb::__off64_t;
use crate::librb::__time_t;
use crate::librb::__uid_t;
use crate::librb::dev_t;
use crate::librb::gid_t;
use crate::librb::mode_t;
use crate::librb::off_t;
use crate::librb::time_t;
use crate::librb::uid_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
  pub tm_sec: libc::c_int,
  pub tm_min: libc::c_int,
  pub tm_hour: libc::c_int,
  pub tm_mday: libc::c_int,
  pub tm_mon: libc::c_int,
  pub tm_year: libc::c_int,
  pub tm_wday: libc::c_int,
  pub tm_yday: libc::c_int,
  pub tm_isdst: libc::c_int,
  pub tm_gmtoff: libc::c_long,
  pub tm_zone: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_header_t {
  pub name: *mut libc::c_char,
  pub link_target: *mut libc::c_char,
  pub tar__uname: *mut libc::c_char,
  pub tar__gname: *mut libc::c_char,
  pub size: off_t,
  pub uid: uid_t,
  pub gid: gid_t,
  pub mode: mode_t,
  pub mtime: time_t,
  pub device: dev_t,
}

/*
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn header_verbose_list(mut file_header: *const file_header_t) {
  let mut tm_time: tm = tm {
    tm_sec: 0,
    tm_min: 0,
    tm_hour: 0,
    tm_mday: 0,
    tm_mon: 0,
    tm_year: 0,
    tm_wday: 0,
    tm_yday: 0,
    tm_isdst: 0,
    tm_gmtoff: 0,
    tm_zone: 0 as *const libc::c_char,
  }; //localtime(&file_header->mtime);
  let mut ptm: *mut tm = &mut tm_time;
  let mut uid: [libc::c_char; 14] = [0; 14];
  /*char gid[sizeof(int)*3 + 2];*/
  let mut user: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut group: *mut libc::c_char = 0 as *mut libc::c_char;
  localtime_r(&(*file_header).mtime, ptm);
  user = (*file_header).tar__uname;
  if user.is_null() {
    sprintf(
      uid.as_mut_ptr(),
      b"%u\x00" as *const u8 as *const libc::c_char,
      (*file_header).uid,
    );
    user = uid.as_mut_ptr()
  }
  group = (*file_header).tar__gname;
  if group.is_null() {
    /*sprintf(gid, "%u", (unsigned)file_header->gid);*/
    group = utoa((*file_header).gid)
  }
  printf(
    b"%s %s/%s %9lu %4u-%02u-%02u %02u:%02u:%02u %s\x00" as *const u8 as *const libc::c_char,
    bb_mode_string((*file_header).mode),
    user,
    group,
    (*file_header).size,
    1900i32 + (*ptm).tm_year,
    1i32 + (*ptm).tm_mon,
    (*ptm).tm_mday,
    (*ptm).tm_hour,
    (*ptm).tm_min,
    (*ptm).tm_sec,
    (*file_header).name,
  );
  /* !FEATURE_TAR_UNAME_GNAME */
  /* FEATURE_TAR_UNAME_GNAME */
  /* NB: GNU tar shows "->" for symlinks and "link to" for hardlinks */
  if !(*file_header).link_target.is_null() {
    printf(
      b" -> %s\x00" as *const u8 as *const libc::c_char,
      (*file_header).link_target,
    );
  }
  bb_putchar('\n' as i32);
}
