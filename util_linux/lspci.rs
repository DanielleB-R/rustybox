use crate::libbb::getopt32::getopt32;
use crate::librb::size_t;

use libc;
use libc::free;
use libc::printf;
use libc::stat;
use libc::strcmp;
use libc::FILE;
extern "C" {

  #[no_mangle]
  fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;

  #[no_mangle]
  fn recursive_action(
    fileName: *const libc::c_char,
    flags: libc::c_uint,
    fileAction_0: Option<
      unsafe extern "C" fn(
        _: *const libc::c_char,
        _: *mut stat,
        _: *mut libc::c_void,
        _: libc::c_int,
      ) -> libc::c_int,
    >,
    dirAction: Option<
      unsafe extern "C" fn(
        _: *const libc::c_char,
        _: *mut stat,
        _: *mut libc::c_void,
        _: libc::c_int,
      ) -> libc::c_int,
    >,
    userData: *mut libc::c_void,
    depth: libc::c_uint,
  ) -> libc::c_int;

  #[no_mangle]
  fn bb_putchar(ch: libc::c_int) -> libc::c_int;

  #[no_mangle]
  fn fopen_for_read(path: *const libc::c_char) -> *mut FILE;

  #[no_mangle]
  fn xstrtou(str: *const libc::c_char, b: libc::c_int) -> libc::c_uint;

  #[no_mangle]
  static mut option_mask32: u32;


  #[no_mangle]
  fn config_open2(
    filename: *const libc::c_char,
    fopen_func: Option<unsafe extern "C" fn(_: *const libc::c_char) -> *mut FILE>,
  ) -> *mut parser_t;

  #[no_mangle]
  fn config_read(
    parser: *mut parser_t,
    tokens: *mut *mut libc::c_char,
    flags: libc::c_uint,
    delims: *const libc::c_char,
  ) -> libc::c_int;

  #[no_mangle]
  fn config_close(parser: *mut parser_t);

  #[no_mangle]
  fn concat_path_file(
    path: *const libc::c_char,
    filename: *const libc::c_char,
  ) -> *mut libc::c_char;
}

pub type C2RustUnnamed = libc::c_uint;
// pub const ACTION_DANGLING_OK: C2RustUnnamed = 64;
// pub const ACTION_QUIET: C2RustUnnamed = 32;
// pub const ACTION_DEPTHFIRST: C2RustUnnamed = 8;
// pub const ACTION_FOLLOWLINKS_L0: C2RustUnnamed = 4;
// pub const ACTION_FOLLOWLINKS: C2RustUnnamed = 2;
pub const ACTION_RECURSE: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PARSE_NORMAL: C2RustUnnamed_0 = 4653056;
// pub const PARSE_WS_COMMENTS: C2RustUnnamed_0 = 16777216;
// pub const PARSE_ALT_COMMENTS: C2RustUnnamed_0 = 8388608;
// pub const PARSE_EOL_COMMENTS: C2RustUnnamed_0 = 4194304;
// pub const PARSE_KEEP_COPY: C2RustUnnamed_0 = 2097152;
// pub const PARSE_MIN_DIE: C2RustUnnamed_0 = 1048576;
// pub const PARSE_GREEDY: C2RustUnnamed_0 = 262144;
// pub const PARSE_TRIM: C2RustUnnamed_0 = 131072;
// pub const PARSE_COLLAPSE: C2RustUnnamed_0 = 65536;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct parser_t {
  pub fp: *mut FILE,
  pub data: *mut libc::c_char,
  pub line: *mut libc::c_char,
  pub nline: *mut libc::c_char,
  pub line_alloc: size_t,
  pub nline_alloc: size_t,
  pub lineno: libc::c_int,
}

/*
 * lspci implementation for busybox
 *
 * Copyright (C) 2009  Malek Degachi <malek-degachi@laposte.net>
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config LSPCI
//config:	bool "lspci (6.3 kb)"
//config:	default y
//config:	#select PLATFORM_LINUX
//config:	help
//config:	lspci is a utility for displaying information about PCI buses in the
//config:	system and devices connected to them.
//config:
//config:	This version uses sysfs (/sys/bus/pci/devices) only.
//applet:IF_LSPCI(APPLET_NOEXEC(lspci, lspci, BB_DIR_USR_BIN, SUID_DROP, lspci))
//kbuild:lib-$(CONFIG_LSPCI) += lspci.o
//usage:#define lspci_trivial_usage
//usage:       "[-mk]"
//usage:#define lspci_full_usage "\n\n"
//usage:       "List all PCI devices"
//usage:     "\n"
//usage:     "\n	-m	Parsable output"
//usage:     "\n	-k	Show driver"
pub type C2RustUnnamed_1 = libc::c_uint;
pub const OPT_k: C2RustUnnamed_1 = 2;
pub const OPT_m: C2RustUnnamed_1 = 1;
/*
 * PCI_SLOT_NAME PCI_CLASS: PCI_VID:PCI_DID [PCI_SUBSYS_VID:PCI_SUBSYS_DID] [DRIVER]
 */
unsafe extern "C" fn fileAction(
  mut fileName: *const libc::c_char,
  mut _statbuf: *mut stat,
  mut _userData: *mut libc::c_void,
  mut _depth: libc::c_int,
) -> libc::c_int {
  let mut parser: *mut parser_t = 0 as *mut parser_t;
  let mut tokens: [*mut libc::c_char; 3] = [0 as *mut libc::c_char; 3];
  let mut pci_slot_name: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut driver: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
  let mut pci_class: libc::c_int = 0i32;
  let mut pci_vid: libc::c_int = 0i32;
  let mut pci_did: libc::c_int = 0i32;
  let mut pci_subsys_vid: libc::c_int = 0i32;
  let mut pci_subsys_did: libc::c_int = 0i32;
  let mut uevent_filename: *mut libc::c_char =
    concat_path_file(fileName, b"/uevent\x00" as *const u8 as *const libc::c_char);
  parser = config_open2(
    uevent_filename,
    Some(fopen_for_read as unsafe extern "C" fn(_: *const libc::c_char) -> *mut FILE),
  );
  free(uevent_filename as *mut libc::c_void);
  while config_read(
    parser,
    tokens.as_mut_ptr(),
    (PARSE_NORMAL as libc::c_int | (2i32 & 0xffi32) << 8i32 | 3i32 & 0xffi32) as libc::c_uint,
    b"\x00:=\x00" as *const u8 as *const libc::c_char,
  ) != 0
  {
    if strcmp(tokens[0], b"DRIVER\x00" as *const u8 as *const libc::c_char) == 0i32 {
      driver = xstrdup(tokens[1])
    } else if strcmp(
      tokens[0],
      b"PCI_CLASS\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
      pci_class = (xstrtou(tokens[1], 16i32) >> 8i32) as libc::c_int
    } else if strcmp(tokens[0], b"PCI_ID\x00" as *const u8 as *const libc::c_char) == 0i32 {
      pci_vid = xstrtou(tokens[1], 16i32) as libc::c_int;
      pci_did = xstrtou(tokens[2], 16i32) as libc::c_int
    } else if strcmp(
      tokens[0],
      b"PCI_SUBSYS_ID\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
      pci_subsys_vid = xstrtou(tokens[1], 16i32) as libc::c_int;
      pci_subsys_did = xstrtou(tokens[2], 16i32) as libc::c_int
    } else {
      if !(strcmp(
        tokens[0],
        b"PCI_SLOT_NAME\x00" as *const u8 as *const libc::c_char,
      ) == 0i32)
      {
        continue;
      }
      pci_slot_name = xstrdup(tokens[2])
    }
  }
  config_close(parser);
  if option_mask32 & OPT_m as libc::c_int as libc::c_uint != 0 {
    printf(
      b"%s \"Class %04x\" \"%04x\" \"%04x\" \"%04x\" \"%04x\"\x00" as *const u8
        as *const libc::c_char,
      pci_slot_name,
      pci_class,
      pci_vid,
      pci_did,
      pci_subsys_vid,
      pci_subsys_did,
    );
  } else {
    printf(
      b"%s Class %04x: %04x:%04x\x00" as *const u8 as *const libc::c_char,
      pci_slot_name,
      pci_class,
      pci_vid,
      pci_did,
    );
  }
  if option_mask32 & OPT_k as libc::c_int as libc::c_uint != 0 && !driver.is_null() {
    if option_mask32 & OPT_m as libc::c_int as libc::c_uint != 0 {
      printf(b" \"%s\"\x00" as *const u8 as *const libc::c_char, driver);
    } else {
      printf(b" %s\x00" as *const u8 as *const libc::c_char, driver);
    }
  }
  bb_putchar('\n' as i32);
  free(driver as *mut libc::c_void);
  free(pci_slot_name as *mut libc::c_void);
  return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn lspci_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  getopt32(argv, b"mknv\x00" as *const u8 as *const libc::c_char);
  recursive_action(
    b"/sys/bus/pci/devices\x00" as *const u8 as *const libc::c_char,
    ACTION_RECURSE as libc::c_int as libc::c_uint,
    Some(
      fileAction
        as unsafe extern "C" fn(
          _: *const libc::c_char,
          _: *mut stat,
          _: *mut libc::c_void,
          _: libc::c_int,
        ) -> libc::c_int,
    ),
    None,
    0 as *mut libc::c_void,
    0i32 as libc::c_uint,
  );
  return 0i32;
}
