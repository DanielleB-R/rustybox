use crate::libbb::getopt32::getopt32;
use libc;
use libc::free;
use libc::putchar_unlocked;
use libc::puts;
use libc::strcmp;
use libc::FILE;
extern "C" {

  #[no_mangle]
  static mut optind: libc::c_int;


  #[no_mangle]
  fn xmalloc_fgetline(file: *mut FILE) -> *mut libc::c_char;
  #[no_mangle]
  fn xfopen_stdin(filename: *const libc::c_char) -> *mut FILE;
  #[no_mangle]
  static mut option_mask32: u32;
}

/* writeline outputs the input given, appropriately aligned according to class */
unsafe extern "C" fn writeline(mut line: *mut libc::c_char, mut class: libc::c_int) {
  let mut flags: libc::c_int = option_mask32 as libc::c_int;
  if class == 0i32 {
    if flags & 1i32 << 0i32 != 0 {
      return;
    }
  } else if class == 1i32 {
    if flags & 1i32 << 1i32 != 0 {
      return;
    }
    if flags & 1i32 << 0i32 == 0 {
      putchar_unlocked('\t' as i32);
    }
  } else {
    /*if (class == 2)*/
    if flags & 1i32 << 2i32 != 0 {
      return;
    }
    if flags & 1i32 << 0i32 == 0 {
      putchar_unlocked('\t' as i32);
    }
    if flags & 1i32 << 1i32 == 0 {
      putchar_unlocked('\t' as i32);
    }
  }
  puts(line);
}
#[no_mangle]
pub unsafe extern "C" fn comm_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  let mut thisline: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
  let mut stream: [*mut FILE; 2] = [0 as *mut FILE; 2];
  let mut i: libc::c_int = 0;
  let mut order: libc::c_int = 0;
  getopt32(argv, b"^123\x00=2\x00" as *const u8 as *const libc::c_char);
  argv = argv.offset(optind as isize);
  i = 0i32;
  while i < 2i32 {
    stream[i as usize] = xfopen_stdin(*argv.offset(i as isize));
    i += 1
  }
  order = 0i32;
  thisline[0] = std::ptr::null_mut::<libc::c_char>();
  thisline[1] = thisline[0];
  loop {
    if order <= 0i32 {
      free(thisline[0] as *mut libc::c_void);
      thisline[0] = xmalloc_fgetline(stream[0])
    }
    if order >= 0i32 {
      free(thisline[1] as *mut libc::c_void);
      thisline[1] = xmalloc_fgetline(stream[1])
    }
    i = thisline[0].is_null() as libc::c_int + ((thisline[1].is_null() as libc::c_int) << 1i32);
    if i != 0 {
      break;
    }
    order = strcmp(thisline[0], thisline[1]);
    if order >= 0i32 {
      writeline(thisline[1], if order != 0 { 1i32 } else { 2i32 });
    } else {
      writeline(thisline[0], 0i32);
    }
  }
  /* EOF at least on one of the streams */
  i &= 1i32;
  if !thisline[i as usize].is_null() {
    /* stream[i] is not at EOF yet */
    /* we did not print thisline[i] yet */
    let mut p: *mut libc::c_char = thisline[i as usize];
    writeline(p, i);
    loop {
      free(p as *mut libc::c_void);
      p = xmalloc_fgetline(stream[i as usize]);
      if p.is_null() {
        break;
      }
      writeline(p, i);
    }
  }
  return 0i32;
}
