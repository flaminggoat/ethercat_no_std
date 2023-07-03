use embassy_time::driver::{Driver, AlarmHandle};
use core::task::Waker;

use embassy_time::Instant;
use embassy_time::queue::{TimerQueue};

struct MyDriver{} // not public!
embassy_time::time_driver_impl!(static DRIVER: MyDriver = MyDriver{});

impl Driver for MyDriver {
    fn now(&self) -> u64 {
        10
    }
    unsafe fn allocate_alarm(&self) -> Option<AlarmHandle> {
        todo!()
    }
    fn set_alarm_callback(&self, alarm: AlarmHandle, callback: fn(*mut ()), ctx: *mut ()) {
        todo!()
    }
    fn set_alarm(&self, alarm: AlarmHandle, timestamp: u64) -> bool {
        todo!()
    }
}

struct MyTimerQueue{} // not public!
embassy_time::timer_queue_impl!(static QUEUE: MyTimerQueue = MyTimerQueue{});

impl TimerQueue for MyTimerQueue {
    fn schedule_wake(&'static self, at: Instant, waker: &Waker) {
        // todo!()
    }
}