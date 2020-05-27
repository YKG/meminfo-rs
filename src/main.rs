fn main() {
    use sysinfo::{SystemExt};
    use std::{thread, time};

    loop {
        let mut system = sysinfo::System::new_all();

        // First we update all information of our system struct.
        system.refresh_all();

        // And finally the RAM and SWAP information:
        println!("total memory: {} KiB", system.get_total_memory());
        println!("used memory : {} KiB", system.get_used_memory());
        println!("total swap  : {} KiB", system.get_total_swap());
        println!("used swap   : {} KiB\n", system.get_used_swap());

        thread::sleep(time::Duration::from_secs(1))
    }
}
