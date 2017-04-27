extern crate scroll_phat_hd;

use scroll_phat_hd::display::*;
use scroll_phat_hd::scroller::*;

fn main() {
    println!("start");

    // let mut projector = I2CProjector::new();
    let mut display = TermDisplay::new();
    let mut scroller = Scroller::new(&mut display);

    scroller.set_text("ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
    for _ in 0..3000 {
        scroller.show();
        std::thread::sleep(std::time::Duration::from_millis(100));
        scroller.scroll();
    }

    println!("end");
}
