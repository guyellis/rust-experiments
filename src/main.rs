use std::time::Instant;

trait Timer {
    fn duration(&self) -> f64;
}

struct RealTimer {
    start_time: Instant,
}

struct FakeTimer {
    time_taken: f64,
}

impl Timer for RealTimer {
    fn duration(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64()
    }
}

impl Timer for FakeTimer {
    fn duration(&self) -> f64 {
        self.time_taken
    }
}

fn print_duration(timer: Box<dyn Timer>) {
    println!("{}", timer.duration());
}

fn main() {
    let real_timer = Box::new(RealTimer {
        start_time: Instant::now(),
    });
    print_duration(real_timer);

    let fake_timer = Box::new(FakeTimer {
        time_taken: 0.5,
    });
    print_duration(fake_timer);
}
