use crate::libbb::getopt32::getopt32;
use libc;
use libc::getutxent;
use libc::localtime;
use libc::sysinfo;
use libc::time;

// When reading the utmp entries with getuxent, this identifies
// a user entry.
const UTMP_USER_PROCESS: libc::c_short = 7;

fn get_users() -> u32 {
  let mut users = 0;
  unsafe {
    while let Some(ut) = getutxent().as_mut() {
      if ut.ut_type == UTMP_USER_PROCESS && ut.ut_user[0] != 0 {
        users += 1
      }
    }
  }
  users
}

/* nr of bits of precision */
/* 1.0 as fixed-point */
#[no_mangle]
pub extern "C" fn uptime_main(mut _argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int {
  let opts = unsafe { getopt32(argv, b"s\x00" as *const u8 as *const libc::c_char) };

  let mut current_secs = unsafe { time(std::ptr::null_mut()) };

  let mut info = unsafe {
    let mut info: sysinfo = std::mem::zeroed();
    sysinfo(&mut info);
    info
  };

  if opts != 0 {
    // -s
    current_secs -= info.uptime
  }

  let current_time = unsafe { *localtime(&current_secs) };
  if opts != 0 {
    // -s
    println!(
      "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
      current_time.tm_year + 1900,
      current_time.tm_mon + 1,
      current_time.tm_mday,
      current_time.tm_hour,
      current_time.tm_min,
      current_time.tm_sec,
    );
    /* The above way of calculating boot time is wobbly,
     * info.uptime has only 1 second precision, which makes
     * "uptime -s" wander +- one second.
     * /proc/uptime may be better, it has 0.01s precision.
     */
    return 0;
  }

  print!(
    " {:02}:{:02}:{:02} up ",
    current_time.tm_hour, current_time.tm_min, current_time.tm_sec,
  );

  let updays = info.uptime / (60 * 60 * 24);
  if updays > 0 {
    print!("{} day{}, ", updays, if updays != 1 { "s" } else { "" },);
  }

  let upminutes_total = info.uptime / 60;
  let uphours = (upminutes_total / 60) % 24;
  let upminutes = upminutes_total % 60;
  if uphours > 0 {
    print!("{:2}:{:02}", uphours, upminutes);
  } else {
    print!("{} min", upminutes);
  }

  println!(
    ",  {} users,  load average: {}.{:02}, {}.{:02}, {}.{:02}",
    get_users(),
    info.loads[0] >> 16,
    ((info.loads[0] & ((1 << 16) - 1)) * 100) >> 16,
    info.loads[1] >> 16,
    ((info.loads[1] & ((1 << 16) - 1)) * 100) >> 16,
    info.loads[2] >> 16,
    ((info.loads[2] & ((1 << 16) - 1)) * 100) >> 16,
  );

  0
}
