/*  An binary to help with trying strace on a
 *      repeating process that uses the filesystem.
 *  Adam R Bell (github.com/arossbell; https://keybase.io/iscsi);
 *      Released under MIT License.
 */


use std::{thread, time, process, fs};

fn main() {
    println!("Run `sudo strace -p {}`.\n", process::id());
    loop {
        // This section will show as "stat" things in strace.
        println!("Stating some things...");
        fs::metadata("/etc/passwd");
        fs::metadata("/root");
        fs::metadata("/usr/bin/nc");
        fs::metadata("/usr/lib/doesnt-exist");
        // Sleep for 5 seconds.
        thread::sleep(time::Duration::from_millis(5000));

        // This section will show as "open" things in strace.
        println!("Opening some things...");
        fs::read("/usr/bin/ssh");
        fs::read("/etc/resolv.conf");
        fs::read("/should-be-nothing");
        fs::read(".bashrc");
        // Sleep for 5 seconds.
        thread::sleep(time::Duration::from_millis(5000));
    }
}
