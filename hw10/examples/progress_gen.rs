use hw10::progress_gen::{BaseProgress, BracketsDecorator, Progress, ProgressBarDecorator};
use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

fn main() {
    println!("Example simple progress");
    let p = BaseProgress {};
    for i in 1..101 {
        p.draw(i as f32);
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(25));
    }

    println!("\nExample progress with brackets");
    let p1 = BracketsDecorator::new(BaseProgress {});
    for i in 1..101 {
        p1.draw(i as f32);
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(25));
    }

    println!("\nExample progressbar");
    let p2 = ProgressBarDecorator::new(BaseProgress {});
    for i in 1..101 {
        p2.draw(i as f32);
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(50));
    }

    println!("\nExample progressbar with brackets");
    let p3 = ProgressBarDecorator::new(BaseProgress {});
    let p4 = BracketsDecorator::new(p3);
    for i in 1..101 {
        p4.draw(i as f32);
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(50));
    }
    println!("");
}
