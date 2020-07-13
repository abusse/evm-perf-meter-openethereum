#[cfg(not(target_os = "linux"))]
compile_error!("perf counters relies on linux.");

extern crate perfcnt;

use crate::measurement::Measurement;
use std::cell::RefCell;

use perfcnt::linux::PerfCounter;
use perfcnt::linux::PerfCounterBuilderLinux;
use perfcnt::AbstractPerfCounter;

pub struct Perf {
    counter: RefCell<PerfCounter>,
    id: &'static str,
    name: &'static str,
    unit: &'static str,
}

impl Perf {
    pub fn new(
        mut builder: PerfCounterBuilderLinux,
        id: &'static str,
        name: &'static str,
        unit: &'static str,
    ) -> Perf {
        Perf {
            counter: RefCell::new(
                builder
                    .for_pid(std::process::id() as i32)
                    .disable()
                    .finish()
                    .expect("Could not create counter"),
            ),
            id: id,
            name: name,
            unit: unit,
        }
    }
}

impl Measurement for Perf {
    type Intermediate = u8;

    fn start(&self) -> Self::Intermediate {
        self.counter
            .borrow()
            .start()
            .expect("Could not read perf counter");
        0
    }

    fn end(&self, _i: Self::Intermediate) -> serde_json::Number {
        self.counter
            .borrow()
            .stop()
            .expect("Could not stop perf counter");
        let ret = self
            .counter
            .borrow_mut()
            .read()
            .expect("Could not read perf counter");
        self.counter
            .borrow_mut()
            .reset()
            .expect("Could not reset perf counter");
        ret.into()
    }

    fn id(&self) -> &'static str {
        self.id
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn unit(&self) -> &'static str {
        self.unit
    }
}
