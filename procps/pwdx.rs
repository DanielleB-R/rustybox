use crate::libbb::getopt32::getopt32;
use crate::libbb::ptr_to_globals::bb_errno;

use libc;
use libc::free;
use libc::printf;
use libc::sprintf;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;

  #[no_mangle]
  fn strerror(_: libc::c_int) -> *mut libc::c_char;

  #[no_mangle]
  fn is_prefixed_with(string: *const libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn xmalloc_readlink(path: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_strtou(
    arg: *const libc::c_char,
    endp: *mut *mut libc::c_char,
    base: libc::c_int,
  ) -> libc::c_uint;
  #[no_mangle]
  fn bb_error_msg_and_die(s: *const libc::c_char, _: ...) -> !;
}

/*
 * pwdx implementation for busybox
 *
 * Copyright (c) 2004 Nicholas Miell
 * ported from procps by Pere Orga <gotrunks@gmail.com> 2011
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config PWDX
//config:	bool "pwdx (3.7 kb)"
//config:	default y
//config:	help
//config:	Report current working directory of a process
//applet:IF_PWDX(APPLET_NOFORK(pwdx, pwdx, BB_DIR_USR_BIN, SUID_DROP, pwdx))
//kbuild:lib-$(CONFIG_PWDX) += pwdx.o
//usage:#define pwdx_trivial_usage
//usage:       "PID..."
//usage:#define pwdx_full_usage "\n\n"
//usage:       "Show current directory for PIDs"
#[no_mangle]
pub unsafe extern "C" fn pwdx_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  getopt32(argv, b"^\x00-1\x00" as *const u8 as *const libc::c_char);
  argv = argv.offset(optind as isize);
  loop {
    let mut buf: [libc::c_char; 25] = [0; 25];
    let mut pid: libc::c_uint = 0;
    let mut s: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut arg: *mut libc::c_char = *argv;
    // Allowed on the command line:
    // /proc/NUM
    // NUM
    if !is_prefixed_with(arg, b"/proc/\x00" as *const u8 as *const libc::c_char).is_null() {
      arg = arg.offset(6)
    }
    pid = bb_strtou(arg, 0 as *mut *mut libc::c_char, 10i32);
    if *bb_errno != 0 {
      bb_error_msg_and_die(
        b"invalid process id: \'%s\'\x00" as *const u8 as *const libc::c_char,
        arg,
      );
    }
    sprintf(
      buf.as_mut_ptr(),
      b"/proc/%u/cwd\x00" as *const u8 as *const libc::c_char,
      pid,
    );
    /* NOFORK: only one alloc is allowed; must free */
    s = xmalloc_readlink(buf.as_mut_ptr());
    // "pwdx /proc/1" says "/proc/1: DIR", not "1: DIR"
    printf(
      b"%s: %s\n\x00" as *const u8 as *const libc::c_char,
      *argv,
      if !s.is_null() {
        s
      } else {
        strerror(if *bb_errno == 2i32 { 3i32 } else { *bb_errno })
      },
    );
    free(s as *mut libc::c_void);
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
  }
  return 0i32;
}
