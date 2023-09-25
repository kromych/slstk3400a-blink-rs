use efm32hg322_hal as hal;

use hal::gpio::{
    pins::{PC10, PC9},
    ExtInterruptEdge, GpioInterruptExt, Input, PullUp,
};

pub struct PushButton<T>(pub T)
where
    T: ?Sized;

pub trait PushButtonTrait {
    /// Enable interrupt.
    fn enable_interrupt(&mut self, edge: ExtInterruptEdge);

    /// Disable interrupt.
    fn disable_interrupt(&mut self, edge: ExtInterruptEdge);

    /// Clear interrupt.
    fn clear_interrupt(&mut self);
}

pub type BTN0 = PushButton<PC9<Input<PullUp>>>;
pub type BTN1 = PushButton<PC10<Input<PullUp>>>;

pub struct PushButtons {
    pub btn0: BTN0,
    pub btn1: BTN1,
}

impl PushButtons {
    pub fn new(btn0: PC9<Input<PullUp>>, btn1: PC10<Input<PullUp>>) -> Self {
        PushButtons {
            btn0: PushButton(btn0),
            btn1: PushButton(btn1),
        }
    }
}

impl<T: GpioInterruptExt> PushButtonTrait for PushButton<T> {
    fn enable_interrupt(&mut self, edge: ExtInterruptEdge) {
        self.0.enable_interrupt(edge);
    }

    fn disable_interrupt(&mut self, edge: ExtInterruptEdge) {
        self.0.disable_interrupt(edge);
    }

    fn clear_interrupt(&mut self) {
        self.0.clear_interrupt();
    }
}
