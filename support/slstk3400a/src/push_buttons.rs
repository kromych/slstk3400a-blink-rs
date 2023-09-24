use efm32hg322_hal as hal;

use hal::gpio::{
    pins::{PC10, PC9},
    Input, PullUp, WithFilter,
};

pub struct PushButton<T>(pub T)
where
    T: ?Sized;

pub type PB0 = PushButton<PC9<Input<WithFilter<PullUp>>>>;
pub type PB1 = PushButton<PC10<Input<WithFilter<PullUp>>>>;

pub struct PushButtons {
    pub btn0: PB0,
    pub btn1: PB1,
}

impl PushButtons {
    pub fn new(
        btn0: PC9<Input<WithFilter<PullUp>>>,
        btn1: PC10<Input<WithFilter<PullUp>>>,
    ) -> Self {
        PushButtons {
            btn0: PushButton(btn0),
            btn1: PushButton(btn1),
        }
    }
}
