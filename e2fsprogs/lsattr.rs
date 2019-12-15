use crate::libbb::getopt32::getopt32;
use libc;
use libc::free;
use libc::lstat;
use libc::printf;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;
  #[no_mangle]
  static mut stdout: *mut FILE;

  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;
  #[no_mangle]
  static mut option_mask32: u32;
  #[no_mangle]
  fn bb_perror_msg(s: *const libc::c_char, _: ...);
  #[no_mangle]
  fn concat_path_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;

  /*
   * See README for additional information
   *
   * This file can be redistributed under the terms of the GNU Library General
   * Public License
   */
  /* Constants and structures */
  /* Iterate a function on each entry of a directory */
  #[no_mangle]
  fn iterate_on_dir(
    dir_name: *const libc::c_char,
    func: Option<
      unsafe extern "C" fn(
        _: *const libc::c_char,
        _: *mut dirent,
        _: *mut libc::c_void,
      ) -> libc::c_int,
    >,
    private: *mut libc::c_void,
  ) -> libc::c_int;
  /* Get/set a file version on an ext2 file system */
  #[no_mangle]
  fn fgetsetversion(
    name: *const libc::c_char,
    get_version: *mut libc::c_ulong,
    set_version: libc::c_ulong,
  ) -> libc::c_int;
  #[no_mangle]
  fn fgetsetflags(
    name: *const libc::c_char,
    get_flags: *mut libc::c_ulong,
    set_flags: libc::c_ulong,
  ) -> libc::c_int;
  /* Print file attributes on an ext2 file system */
  #[no_mangle]
  fn print_e2flags(f: *mut FILE, flags: libc::c_ulong, options: libc::c_uint);
}

use libc::dirent;
use libc::stat;
use libc::FILE;
/*
 * lsattr.c		- List file attributes on an ext2 file system
 *
 * Copyright (C) 1993, 1994  Remy Card <card@masi.ibp.fr>
 *                           Laboratoire MASI, Institut Blaise Pascal
 *                           Universite Pierre et Marie Curie (Paris VI)
 *
 * This file can be redistributed under the terms of the GNU General
 * Public License
 */
//config:config LSATTR
//config:	bool "lsattr (5.5 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	lsattr lists the file attributes on a second extended file system.
//applet:IF_LSATTR(APPLET_NOEXEC(lsattr, lsattr, BB_DIR_BIN, SUID_DROP, lsattr))
/* ls is NOEXEC, so we should be too! ;) */
//kbuild:lib-$(CONFIG_LSATTR) += lsattr.o e2fs_lib.o
//usage:#define lsattr_trivial_usage
//usage:       "[-Radlv] [FILE]..."
//usage:#define lsattr_full_usage "\n\n"
//usage:       "List ext2 file attributes\n"
//usage:     "\n	-R	Recurse"
//usage:     "\n	-a	Don't hide entries starting with ."
//usage:     "\n	-d	List directory entries instead of contents"
//usage:     "\n	-l	List long flag names"
//usage:     "\n	-v	List version/generation number"
pub type C2RustUnnamed = libc::c_uint;
pub const OPT_GENERATION: C2RustUnnamed = 16;
pub const OPT_PF_LONG: C2RustUnnamed = 8;
pub const OPT_DIRS_OPT: C2RustUnnamed = 4;
pub const OPT_ALL: C2RustUnnamed = 2;
pub const OPT_RECUR: C2RustUnnamed = 1;
unsafe extern "C" fn list_attributes(mut name: *const libc::c_char) {
  let mut current_block: u64;
  let mut fsflags: libc::c_ulong = 0;
  let mut generation: libc::c_ulong = 0;
  if !(fgetsetflags(name, &mut fsflags, 0i32 as libc::c_ulong) != 0i32) {
    if option_mask32 & OPT_GENERATION as libc::c_int as libc::c_uint != 0 {
      if fgetsetversion(name, &mut generation, 0i32 as libc::c_ulong) != 0i32 {
        current_block = 3114758340063453716;
      } else {
        printf(b"%5lu \x00" as *const u8 as *const libc::c_char, generation);
        current_block = 6873731126896040597;
      }
    } else {
      current_block = 6873731126896040597;
    }
    match current_block {
      3114758340063453716 => {}
      _ => {
        if option_mask32 & OPT_PF_LONG as libc::c_int as libc::c_uint != 0 {
          printf(b"%-28s \x00" as *const u8 as *const libc::c_char, name);
          print_e2flags(stdout, fsflags, 1i32 as libc::c_uint);
          bb_putchar('\n' as i32);
        } else {
          print_e2flags(stdout, fsflags, 0i32 as libc::c_uint);
          printf(b" %s\n\x00" as *const u8 as *const libc::c_char, name);
        }
        return;
      }
    }
  }
  bb_perror_msg(b"reading %s\x00" as *const u8 as *const libc::c_char, name);
}
unsafe extern "C" fn lsattr_dir_proc(
  mut dir_name: *const libc::c_char,
  mut de: *mut dirent,
  mut _private: *mut libc::c_void,
) -> libc::c_int {
  let mut st: stat = std::mem::zeroed();
  let mut path: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  path = concat_path_file(dir_name, (*de).d_name.as_mut_ptr());
  if lstat(path, &mut st) != 0i32 {
    bb_perror_msg(b"stat %s\x00" as *const u8 as *const libc::c_char, path);
  } else if (*de).d_name[0] as libc::c_int != '.' as i32
    || option_mask32 & OPT_ALL as libc::c_int as libc::c_uint != 0
  {
    list_attributes(path);
    if st.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint
      && option_mask32 & OPT_RECUR as libc::c_int as libc::c_uint != 0
      && !((*de).d_name[0] as libc::c_int == '.' as i32
        && ((*de).d_name[1] == 0
          || (*de).d_name[1] as libc::c_int == '.' as i32 && (*de).d_name[2] == 0))
    {
      printf(b"\n%s:\n\x00" as *const u8 as *const libc::c_char, path);
      iterate_on_dir(
        path,
        Some(
          lsattr_dir_proc
            as unsafe extern "C" fn(
              _: *const libc::c_char,
              _: *mut dirent,
              _: *mut libc::c_void,
            ) -> libc::c_int,
        ),
        0 as *mut libc::c_void,
      );
      bb_putchar('\n' as i32);
    }
  }
  free(path as *mut libc::c_void);
  return 0i32;
}
unsafe extern "C" fn lsattr_args(mut name: *const libc::c_char) {
  let mut st: stat = std::mem::zeroed();
  if lstat(name, &mut st) == -1i32 {
    bb_perror_msg(b"stat %s\x00" as *const u8 as *const libc::c_char, name);
  } else if st.st_mode & 0o170000i32 as libc::c_uint == 0o40000i32 as libc::c_uint
    && option_mask32 & OPT_DIRS_OPT as libc::c_int as libc::c_uint == 0
  {
    iterate_on_dir(
      name,
      Some(
        lsattr_dir_proc
          as unsafe extern "C" fn(
            _: *const libc::c_char,
            _: *mut dirent,
            _: *mut libc::c_void,
          ) -> libc::c_int,
      ),
      0 as *mut libc::c_void,
    );
  } else {
    list_attributes(name);
  };
}
#[no_mangle]
pub unsafe extern "C" fn lsattr_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  getopt32(argv, b"Radlv\x00" as *const u8 as *const libc::c_char);
  argv = argv.offset(optind as isize);
  if (*argv).is_null() {
    argv = argv.offset(-1);
    *argv = b".\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
  }
  loop {
    let fresh0 = argv;
    argv = argv.offset(1);
    lsattr_args(*fresh0);
    if (*argv).is_null() {
      break;
    }
  }
  return 0i32;
}
