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

fn real_factory() -> Box<dyn Timer> {
    Box::new(RealTimer {
        start_time: Instant::now(),
    })
}

fn fake_factory() -> Box<dyn Timer> {
    Box::new(FakeTimer {
        time_taken: 0.444,
    })
}

struct Container {
    timer_factory: Box<dyn Fn() -> Box<dyn Timer>>,
}

fn print_container(container: &Container) {
    let f = &container.timer_factory;
    let timer = f();
    print_duration(timer);
}

fn main() {
    let real_timer = Box::new(RealTimer {
        start_time: Instant::now(),
    });
    print_duration(real_timer);

    let fake_timer = Box::new(FakeTimer {
        time_taken: 0.555,
    });
    print_duration(fake_timer);

    let real_container = Container {
        timer_factory: Box::new(real_factory),
    };
    print_container(&real_container);

    let fake_container = Container {
        timer_factory: Box::new(fake_factory),
    };
    print_container(&fake_container);
}
