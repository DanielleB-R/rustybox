use libc;
extern "C" {
  pub type __dirstream;
  #[no_mangle]
  fn sched_getaffinity(
    __pid: __pid_t,
    __cpusetsize: size_t,
    __cpuset: *mut cpu_set_t,
  ) -> libc::c_int;
  #[no_mangle]
  fn opendir(__name: *const libc::c_char) -> *mut DIR;
  #[no_mangle]
  fn closedir(__dirp: *mut DIR) -> libc::c_int;
  #[no_mangle]
  fn readdir(__dirp: *mut DIR) -> *mut dirent;
  #[no_mangle]
  fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
  #[no_mangle]
  fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn getopt32long(
    argv: *mut *mut libc::c_char,
    optstring: *const libc::c_char,
    longopts: *const libc::c_char,
    _: ...
  ) -> uint32_t;
}
pub type __uint32_t = libc::c_uint;
pub type __ino64_t = libc::c_ulong;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type size_t = libc::c_ulong;
pub type __cpu_mask = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpu_set_t {
  pub __bits: [__cpu_mask; 16],
}
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
  pub d_ino: __ino64_t,
  pub d_off: __off64_t,
  pub d_reclen: libc::c_ushort,
  pub d_type: libc::c_uchar,
  pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
/*
 * Copyright (C) 2017 Denys Vlasenko <vda.linux@googlemail.com>
 *
 * Licensed under GPLv2, see LICENSE in this source tree
 */
//config:config NPROC
//config:	bool "nproc (3.7 kb)"
//config:	default y
//config:	help
//config:	Print number of CPUs
//applet:IF_NPROC(APPLET_NOFORK(nproc, nproc, BB_DIR_USR_BIN, BB_SUID_DROP, nproc))
//kbuild:lib-$(CONFIG_NPROC) += nproc.o
//usage:#define nproc_trivial_usage
//usage:	""IF_LONG_OPTS("--all --ignore=N")
//usage:#define nproc_full_usage "\n\n"
//usage:	"Print number of available CPUs"
//usage:	IF_LONG_OPTS(
//usage:     "\n"
//usage:     "\n	--all		Number of installed CPUs"
//usage:     "\n	--ignore=N	Exclude N CPUs"
//usage:	)
#[no_mangle]
pub unsafe extern "C" fn nproc_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut mask: [libc::c_ulong; 1024] = [0; 1024];
  let mut count: libc::c_int = 0i32;
  let mut ignore: libc::c_int = 0i32;
  let mut opts: libc::c_int = getopt32long(
    argv,
    b"\xfe:+\x00" as *const u8 as *const libc::c_char,
    b"ignore\x00\x01\xfeall\x00\x00\xff\x00" as *const u8 as *const libc::c_char,
    &mut ignore as *mut libc::c_int,
  ) as libc::c_int;
  if opts & 1i32 << 1i32 != 0 {
    let mut cpusd: *mut DIR =
      opendir(b"/sys/devices/system/cpu\x00" as *const u8 as *const libc::c_char);
    if !cpusd.is_null() {
      let mut de: *mut dirent = 0 as *mut dirent;
      loop {
        de = readdir(cpusd);
        if de.is_null() {
          break;
        }
        let mut cpuid: *mut libc::c_char = strstr(
          (*de).d_name.as_mut_ptr(),
          b"cpu\x00" as *const u8 as *const libc::c_char,
        );
        if !cpuid.is_null()
          && (*cpuid.offset(strlen(cpuid).wrapping_sub(1i32 as libc::c_ulong) as isize)
            as libc::c_int
            - '0' as i32) as libc::c_uchar as libc::c_int
            <= 9i32
        {
          count += 1
        }
      }
      closedir(cpusd);
    }
  } else if sched_getaffinity(
    0i32,
    ::std::mem::size_of::<[libc::c_ulong; 1024]>() as libc::c_ulong,
    mask.as_mut_ptr() as *mut libc::c_void as *mut cpu_set_t,
  ) == 0i32
  {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while (i as libc::c_uint)
      < (::std::mem::size_of::<[libc::c_ulong; 1024]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
        as libc::c_uint
    {
      let mut m: libc::c_ulong = mask[i as usize];
      while m != 0 {
        if m & 1i32 as libc::c_ulong != 0 {
          count += 1
        }
        m >>= 1i32
      }
      i += 1
    }
  }
  count -= ignore;
  if count <= 0i32 {
    count = 1i32
  }
  printf(b"%u\n\x00" as *const u8 as *const libc::c_char, count);
  return 0i32;
}