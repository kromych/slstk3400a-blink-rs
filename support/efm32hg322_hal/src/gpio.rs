//! GPIO (general purpose input/output), mapped to embedded_hal::digital
//!
//! This implements only what is minimally essential to make input or output pins out of the GPIO
//! register block. Other drive modes could be added with relative ease (eg. Wired-And, Wired-Or,
//! input with pull-up/-down), others (eg. the per-bank drive strength, EM4 wakeup) will need
//! additional mechanisms, and some (eg. clearing the configuration lock) might need changes to the
//! whole model if at all desired.

use super::pac;
use core::marker::PhantomData;
use pac::gpio::pa_ctrl::DRIVEMODE_A as DriveMode;

/// State type for pin with disabled input.
/// In disabled state output pin can be in either
/// floating state (dout 0), or pulled up (dout 1)
pub struct Disabled<TYPE> {
    _private: PhantomData<TYPE>,
}

/// In efr32 chip series, casting pin into Output will set DINDIS register to 1.
/// There is no Output to Input conversion in any direction.
/// To enable both Input and Output, cast to `Input<Output<_state_>>` type from Disabled.
/// See Input type below.
/// List of possible Output mode:
///     Output<PushPull<Normal>>
///     Output<PushPull<Alternate>>
///     Output<WiredOr<Floating>>
///     Output<WiredOr<PullDown>>
///     Output<WiredAnd<Normal, Floating>>
///     Output<WiredAnd<Normal, PullUp>>
///     Output<WithFilter<WiredAnd<Normal, Floating>>>
///     Output<WithFilter<WiredAnd<Normal, PullUp>>>
///     Output<WiredAnd<Alternate, Floating>>
///     Output<WiredAnd<Alternate, PullUp>>
///     Output<WithFilter<WiredAnd<Alternate, Floating>>>
///     Output<WithFilter<WiredAnd<Alternate, PullUp>>>
pub struct Output<TYPE> {
    _private: PhantomData<TYPE>,
}

/// List of possible input mode:
///     Input<Floating>
///     Input<PullUp>
///     Input<PullDown>
///     Input<WithFilter<Floating>>
///     Input<WithFilter<PullDown>>
///     Input<WithFilter<PullUp>>
///
/// This mode, will also has Output implemented, see Output type above:
///     Input<_all output modes_>
pub struct Input<TYPE> {
    _private: PhantomData<TYPE>,
}

/// Pin states.
pub struct Floating;
pub struct PullUp;
pub struct PullDown;

/// Output mode.
///
/// PushPull can have:
///     PushPull<Normal>    // for efr32, Normal config in ctrl also can be configured separately.
///     PushPull<Alternate> // for efm32, Alternate config only change its current drive mode.
pub struct PushPull<CFG> {
    _private: PhantomData<CFG>,
}

/// OpenSource (wired "or") pin is set to be driven high.
/// Its state can be Floating or PullDown.
pub struct OpenSource<STATE> {
    _private: PhantomData<STATE>,
}

/// OpenDrain (wired "and") pin is set to be driven low.
/// Its state can be Floating or PullUp.
pub struct OpenDrain<MODE, STATE> {
    _private0: PhantomData<MODE>,
    _private1: PhantomData<STATE>,
}

pub struct WithFilter<STATE> {
    _private: PhantomData<STATE>,
}

pub type WiredOr<STATE> = OpenSource<STATE>;
pub type WiredAnd<MODE, STATE> = OpenDrain<MODE, STATE>;

pub struct Normal;
pub struct Alternate;

pub enum ExtInterruptEdge {
    Fall,
    Rise,
    Both,
}

pub trait GpioInterruptExt {
    fn enable_interrupt(&mut self, edge: ExtInterruptEdge);
    fn disable_interrupt(&mut self, edge: ExtInterruptEdge);
    fn clear_interrupt(&mut self);
}

fn sneak_into_gpio() -> &'static pac::gpio::RegisterBlock {
    unsafe { &*pac::GPIO::ptr() }
}

#[macro_export]
macro_rules! gpio {
    ({
         port_configs: [$(($pX_drive:ident, $pX_ctrl:ident),)+],
         pin_configs: [$($PXi:ident: ($pxi:ident, $i:expr, $px_din:ident, $px_dout:ident, $modei:ident, $px_modehl:ident),)+],
         switches: [$($PXj:ident: ($outclr:ident, $outset:ident, $outtgl:ident),)+],
         interrupts: [$($PXk:ident: ($pinsel:ident, $portsel:ident, $lhsel:ident),)+],
     }) => {

        pub mod pins {
            use embedded_hal::digital::v2 as digital;
            use core::convert::Infallible;
            use core::marker::PhantomData;
            use super::*;

            $(
                pub struct $PXi<MODE> {
                    pub(super) _mode: PhantomData<MODE>,
                }

                impl<MODE> $PXi<MODE> {
                    pub fn into_disabled(self) -> $PXi<Disabled<Floating>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().disabled());
                        unsafe { gpio.$outclr().write(|w| w.bits(1 << $i)); }

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_disabled_pulled_up(self) -> $PXi<Disabled<PullUp>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().disabled());
                        unsafe { gpio.$outset().write(|w| w.bits(1 << $i)); }

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_input(self) -> $PXi<Input<Floating>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().input());
                        unsafe { gpio.$outclr().write(|w| w.bits(1 << $i)); }

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_input_with_filter(self) -> $PXi<Input<WithFilter<Floating>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().input());
                        unsafe { gpio.$outset().write(|w| w.bits(1 << $i)); }

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_input_pulled_down(self) -> $PXi<Input<PullDown>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().inputpull());
                        unsafe { gpio.$outclr().write(|w| w.bits(1 << $i)); }

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_input_pulled_up(self) -> $PXi<Input<PullUp>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().inputpull());
                        unsafe { gpio.$outset().write(|w| w.bits(1 << $i)); }

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_input_pulled_down_with_filter(self) -> $PXi<Input<WithFilter<PullDown>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().inputpullfilter());
                        unsafe { gpio.$outclr().write(|w| w.bits(1 << $i)); }

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_input_pulled_up_with_filter(self) -> $PXi<Input<WithFilter<PullUp>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().inputpullfilter());
                        unsafe { gpio.$outset().write(|w| w.bits(1 << $i)); }

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_pushpull(self) -> $PXi<Output<PushPull<Normal>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().pushpull());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_pushpull_alt_drive(self) -> $PXi<Output<PushPull<Alternate>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().pushpulldrive());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_wiredor(self) -> $PXi<Output<WiredOr<Floating>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().wiredor());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_wiredor_pulled_down(self) -> $PXi<Output<WiredOr<PullDown>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().wiredorpulldown());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_wiredand(self) -> $PXi<Output<WiredAnd<Normal, Floating>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().wiredand());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_wiredand_pulled_up(self) -> $PXi<Output<WiredAnd<Normal, PullUp>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().wiredandpullup());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_wiredand_with_filter(self) -> $PXi<Output<WithFilter<WiredAnd<Normal, Floating>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().wiredandfilter());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_wiredand_with_filter_pulled_up(self) -> $PXi<Output<WithFilter<WiredAnd<Normal, PullUp>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().wiredandpullupfilter());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_wiredand_alt_drive(self) -> $PXi<Output<WiredAnd<Alternate, Floating>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().wiredanddrive());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_wiredand_alt_drive_with_filter(self) -> $PXi<Output<WithFilter<WiredAnd<Alternate, Floating>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().wiredanddrivefilter());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_wiredand_alt_drive_pulled_up(self) -> $PXi<Output<WiredAnd<Alternate, PullUp>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().wiredanddrivepullup());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_wiredand_alt_drive_with_filter_pulled_up(self) -> $PXi<Output<WithFilter<WiredAnd<Alternate, PullUp>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().wiredanddrivepullupfilter());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_io_pushpull(self) -> $PXi<Input<Output<PushPull<Normal>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().pushpull());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_io_pushpull_alt_drive(self) -> $PXi<Input<Output<PushPull<Alternate>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().pushpulldrive());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_io_wiredor(self) -> $PXi<Input<Output<WiredOr<Floating>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().wiredor());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_io_wiredor_pulled_down(self) -> $PXi<Input<Output<WiredOr<PullDown>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().wiredorpulldown());

                        $PXi { _mode: PhantomData }
                    }


                    pub fn into_io_wiredand(self) -> $PXi<Input<Output<WiredAnd<Normal, Floating>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().wiredand());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_io_wiredand_pulled_up(self) -> $PXi<Input<Output<WiredAnd<Normal, PullUp>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().wiredandpullup());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_io_wiredand_with_filter(self) -> $PXi<Input<Output<WithFilter<WiredAnd<Normal, Floating>>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().wiredandfilter());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_io_wiredand_with_filter_pulled_up(self) -> $PXi<Input<Output<WithFilter<WiredAnd<Normal, PullUp>>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().wiredandpullupfilter());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_io_wiredand_alt_drive(self) -> $PXi<Input<Output<WiredAnd<Alternate, Floating>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().wiredanddrive());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_io_wiredand_alt_drive_with_filter(self) -> $PXi<Input<Output<WithFilter<WiredAnd<Alternate, Floating>>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().wiredanddrivefilter());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_io_wiredand_alt_drive_pulled_up(self) -> $PXi<Input<Output<WiredAnd<Alternate, PullUp>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().wiredanddrivepullup());

                        $PXi { _mode: PhantomData }
                    }

                    pub fn into_io_wiredand_alt_drive_with_filter_pulled_up(self) -> $PXi<Input<Output<WithFilter<WiredAnd<Alternate, PullUp>>>>> {
                        let gpio = sneak_into_gpio();
                        gpio.$px_modehl().modify(|_, w| w.$modei().wiredanddrivepullupfilter());

                        $PXi { _mode: PhantomData }
                    }
                }

                impl<MODE> GpioInterruptExt for $PXi<MODE> {
                    fn enable_interrupt(&mut self, edge: ExtInterruptEdge) {
                        let gpio = sneak_into_gpio();
                        gpio.$lhsel().modify(|_, w| w.$pinsel().$portsel());

                        let mask = (1 << $i) as u16;
                        match edge {
                            ExtInterruptEdge::Fall => {
                                gpio.extifall().modify(|r, w| unsafe {
                                    let current = r.bits() as u16;
                                    w.extifall().bits(current | mask)
                                });
                            },
                            ExtInterruptEdge::Rise => {
                                gpio.extirise().modify(|r, w| unsafe {
                                    let current = r.bits() as u16;
                                    w.extirise().bits(current | mask)
                                });
                            },
                            ExtInterruptEdge::Both => {
                                gpio.extifall().modify(|r, w| unsafe {
                                    let current = r.bits() as u16;
                                    w.extifall().bits(current | mask)
                                });
                                gpio.extirise().modify(|r, w| unsafe {
                                    let current = r.bits() as u16;
                                    w.extirise().bits(current | mask)
                                });
                            }
                        }
                        gpio.ien().modify(|r, w| unsafe {
                            let current = r.bits() as u16;
                            w.ext().bits(current | mask)
                        });
                    }

                    fn disable_interrupt(&mut self, edge: ExtInterruptEdge) {
                        let gpio = sneak_into_gpio();
                        let mask = !(1 << $i) as u16;

                        gpio.ien().modify(|r, w| unsafe {
                            let current = r.bits() as u16;
                            w.ext().bits(current & mask)
                        });
                        match edge {
                            ExtInterruptEdge::Fall => {
                                gpio.extifall().modify(|r, w| unsafe {
                                    let current = r.bits() as u16;
                                    w.extifall().bits(current & mask)
                                });
                            },
                            ExtInterruptEdge::Rise => {
                                gpio.extirise().modify(|r, w| unsafe {
                                    let current = r.bits() as u16;
                                    w.extirise().bits(current & mask)
                                });
                            },
                            ExtInterruptEdge::Both => {
                                gpio.extifall().modify(|r, w| unsafe {
                                    let current = r.bits() as u16;
                                    w.extifall().bits(current & mask)
                                });
                                gpio.extirise().modify(|r, w| unsafe {
                                    let current = r.bits() as u16;
                                    w.extirise().bits(current & mask)
                                });
                            },
                        }
                    }

                    fn clear_interrupt(&mut self) {
                        let gpio = sneak_into_gpio();
                        gpio.$lhsel().modify(|_, w| w.$pinsel().$portsel());

                        // Clears interrupts for the entire port if needed:
                        // let pending = gpio.if_.read().ext().bits();

                        let pending = 1 << $i;
                        gpio.ifc().write(|w| unsafe {
                            w.ext().bits(pending)
                        });
                    }
                }

                gpio_impl_from_trait! {
                    pin:  $PXi,
                    from: Disabled<Floating>,
                    into: [
                        (Disabled<PullUp>, into_disabled_pulled_up),

                        (Input<Floating>,             into_input),
                        (Input<WithFilter<Floating>>, into_input_with_filter),
                        (Input<PullDown>,             into_input_pulled_down),
                        (Input<PullUp>,               into_input_pulled_up),
                        (Input<WithFilter<PullDown>>, into_input_pulled_down_with_filter),
                        (Input<WithFilter<PullUp>>,   into_input_pulled_up_with_filter),

                        (Output<PushPull<Normal>>,    into_pushpull),
                        (Output<PushPull<Alternate>>, into_pushpull_alt_drive),

                        (Output<OpenSource<Floating>>, into_wiredor),
                        (Output<OpenSource<PullDown>>, into_wiredor_pulled_down),

                        (Output<OpenDrain<Normal, Floating>>,                into_wiredand),
                        (Output<OpenDrain<Normal, PullUp>>,                  into_wiredand_pulled_up),
                        (Output<WithFilter<OpenDrain<Normal, Floating>>>,    into_wiredand_with_filter),
                        (Output<WithFilter<OpenDrain<Normal, PullUp>>>,      into_wiredand_with_filter_pulled_up),
                        (Output<OpenDrain<Alternate, Floating>>,             into_wiredand_alt_drive),
                        (Output<OpenDrain<Alternate, PullUp>>,               into_wiredand_alt_drive_pulled_up),
                        (Output<WithFilter<OpenDrain<Alternate, Floating>>>, into_wiredand_alt_drive_with_filter),
                        (Output<WithFilter<OpenDrain<Alternate, PullUp>>>,   into_wiredand_alt_drive_with_filter_pulled_up),

                        (Input<Output<PushPull<Normal>>>,    into_io_pushpull),
                        (Input<Output<PushPull<Alternate>>>, into_io_pushpull_alt_drive),

                        (Input<Output<OpenSource<Floating>>>, into_io_wiredor),
                        (Input<Output<OpenSource<PullDown>>>, into_io_wiredor_pulled_down),

                        (Input<Output<OpenDrain<Normal, Floating>>>,                into_io_wiredand),
                        (Input<Output<OpenDrain<Normal, PullUp>>>,                  into_io_wiredand_pulled_up),
                        (Input<Output<WithFilter<OpenDrain<Normal, Floating>>>>,    into_io_wiredand_with_filter),
                        (Input<Output<WithFilter<OpenDrain<Normal, PullUp>>>>,      into_io_wiredand_with_filter_pulled_up),
                        (Input<Output<OpenDrain<Alternate, Floating>>>,             into_io_wiredand_alt_drive),
                        (Input<Output<OpenDrain<Alternate, PullUp>>>,               into_io_wiredand_alt_drive_pulled_up),
                        (Input<Output<WithFilter<OpenDrain<Alternate, Floating>>>>, into_io_wiredand_alt_drive_with_filter),
                        (Input<Output<WithFilter<OpenDrain<Alternate, PullUp>>>>,   into_io_wiredand_alt_drive_with_filter_pulled_up),
                    ],
                }

                impl<P> digital::OutputPin for $PXi<Output<P>> {
                    type Error = Infallible;

                    fn set_low(self: &mut Self) -> Result<(), Self::Error> {
                        let gpio = sneak_into_gpio();
                        unsafe { gpio.$outclr().write(|w| w.bits(1 << $i)); }
                        Ok(())
                    }

                    fn set_high(self: &mut Self) -> Result<(), Self::Error> {
                        let gpio = sneak_into_gpio();
                        unsafe { gpio.$outset().write(|w| w.bits(1 << $i)); }
                        Ok(())
                    }
                }

                #[cfg(feature = "unproven")]
                impl<P> digital::StatefulOutputPin for $PXi<Output<P>> {
                    fn is_set_low(self: &Self) -> Result<bool, Self::Error> {
                        let gpio = sneak_into_gpio();
                        Ok(gpio.$px_dout.read().bits() & (1 << $i) == 0)
                    }

                    fn is_set_high(self: &Self) -> Result<bool, Self::Error> {
                        self.is_set_low().map(|state| !state )
                    }
                }

                #[cfg(feature = "unproven")]
                impl<P> digital::ToggleableOutputPin for $PXi<Output<P>> {
                    type Error = Infallible;

                    fn toggle(self: &mut Self) -> Result<(), Self::Error> {
                        let gpio = sneak_into_gpio();
                        unsafe { gpio.$outtgl.write(|w| w.bits(1 << $i)); }
                        Ok(())
                    }
                }

                #[cfg(feature = "unproven")]
                impl<P> digital::InputPin for $PXi<Input<P>> {
                    type Error = Infallible;

                    fn is_low(self: &Self) -> Result<bool, Self::Error> {
                        let gpio = sneak_into_gpio();
                        Ok(gpio.$px_din.read().bits() & (1 << $i) == 0)
                    }

                    fn is_high(self: &Self) -> Result<bool, Self::Error> {
                        self.is_low().map(|state| !state )
                    }
                }

                impl<P> digital::OutputPin for $PXi<Input<Output<P>>> {
                    type Error = Infallible;

                    fn set_low(self: &mut Self) -> Result<(), Self::Error> {
                        let gpio = sneak_into_gpio();
                        unsafe { gpio.$outclr().write(|w| w.bits(1 << $i)); }
                        Ok(())
                    }

                    fn set_high(self: &mut Self) -> Result<(), Self::Error> {
                        let gpio = sneak_into_gpio();
                        unsafe { gpio.$outset().write(|w| w.bits(1 << $i)); }
                        Ok(())
                    }
                }

                #[cfg(feature = "unproven")]
                impl<P> digital::StatefulOutputPin for $PXi<Input<Output<P>>> {
                    fn is_set_low(self: &Self) -> Result<bool, Self::Error> {
                        let gpio = sneak_into_gpio();
                        Ok(gpio.$px_dout.read().bits() & (1 << $i) == 0)
                    }

                    fn is_set_high(self: &Self) -> Result<bool, Self::Error> {
                        self.is_set_low().map(|state| !state )
                    }
                }

                )+
        }

        /// Ports, unlike Pins, doesn't need to be abstractized as a set of bank/port
        /// we're only using this for extra configuration at bank/port level.
        /// Ports is consumed after another split call.
        pub struct Ports;

        pub struct Pins {
            $(
                pub $pxi: pins::$PXi<Disabled<Floating>>,
                )+
        }

        impl Ports {
            /// when Ports splitted, give Pins back.
            /// Ports then will be consumed implicitly.
            pub fn split(self) -> Pins {
                Pins {
                    $(
                        $pxi: pins::$PXi { _mode: PhantomData },
                        )+
                }
            }

            $(
                pub fn $pX_drive(self, mode: DriveMode) -> Self {
                    let gpio = sneak_into_gpio();
                    gpio.$pX_ctrl().write(|w| w.drivemode().bits(mode.into()));
                    self
                }
            )+
        }

        /// Parts has both pins and ports, but designed so only one or the other
        /// can be used. Its fields are private, and one field will be not available
        /// when the other is used/moved.
        pub struct Parts {
            ports: Ports,
            pins: Pins,
        }

        impl Parts {
            /// Move Ports, consume Parts and pins implicitly.
            pub fn ports(self) -> Ports {
                self.ports
            }

            /// Move Pins, consume Parts and ports implicitly.
            pub fn pins(self) -> Pins {
                self.pins
            }

            pub fn split(self) -> Pins {
                self.pins
            }
        }

        pub trait GPIOExt {
            fn constrain(self) -> Parts;
        }

        impl GPIOExt for pac::GPIO {
            fn constrain(self) -> Parts {
                let _consumed = self;

                Parts {
                    ports: Ports,
                    pins: Pins {
                        $(
                            $pxi: pins::$PXi { _mode: PhantomData },
                            )+
                    }
                }
            }
        }
    }
}

macro_rules! gpio_impl_from_trait {
    (pin: $pX:ident, from: $from:ty, into: [$(($targetTy:ty, $fn:ident),)+],) => {
        $(
            impl From<$pX<$from>> for $pX<$targetTy> {
                fn from(p: $pX<$from>) -> $pX<$targetTy> {
                    p.$fn()
                }
            }
        )+
    }
}

// efm32hg322 comes in QFP48
gpio!({
    port_configs: [
        (pa_alt_drive, pa_ctrl),
        (pb_alt_drive, pb_ctrl),
        (pc_alt_drive, pc_ctrl),
        (pd_alt_drive, pd_ctrl),
        (pe_alt_drive, pe_ctrl),
        (pf_alt_drive, pf_ctrl),
    ],
    pin_configs: [
        PA0:  (pa0,   0, pa_din, pa_dout, mode0,  pa_model),
        PA1:  (pa1,   1, pa_din, pa_dout, mode1,  pa_model),
        PA2:  (pa2,   2, pa_din, pa_dout, mode2,  pa_model),
        PA8:  (pa8,   8, pa_din, pa_dout, mode8,  pa_modeh),
        PA9:  (pa9,   9, pa_din, pa_dout, mode9,  pa_modeh),
        PA10: (pa10, 10, pa_din, pa_dout, mode10, pa_modeh),
        PB7:  (pb7,   7, pb_din, pb_dout, mode7,  pb_model),
        PB8:  (pb8,   8, pb_din, pb_dout, mode8,  pb_modeh),
        PB11: (pb11, 11, pb_din, pb_dout, mode11, pb_modeh),
        PB13: (pb13, 13, pb_din, pb_dout, mode13, pb_modeh),
        PB14: (pb14, 14, pb_din, pb_dout, mode14, pb_modeh),
        PC0:  (pc0,   0, pc_din, pc_dout, mode0,  pc_model),
        PC1:  (pc1,   1, pc_din, pc_dout, mode1,  pc_model),
        PC2:  (pc2,   2, pc_din, pc_dout, mode2,  pc_model),
        PC3:  (pc3,   3, pc_din, pc_dout, mode3,  pc_model),
        PC4:  (pc4,   4, pc_din, pc_dout, mode4,  pc_model),
        PC8:  (pc8,   8, pc_din, pc_dout, mode8,  pc_modeh),
        PC9:  (pc9,   9, pc_din, pc_dout, mode9,  pc_modeh),
        PC10: (pc10, 10, pc_din, pc_dout, mode10, pc_modeh),
        PC14: (pc14, 14, pc_din, pc_dout, mode14, pc_modeh),
        PC15: (pc15, 15, pc_din, pc_dout, mode15, pc_modeh),
        PD4:  (pd4,   4, pd_din, pd_dout, mode4,  pd_model),
        PD5:  (pd5,   5, pd_din, pd_dout, mode5,  pd_model),
        PD6:  (pd6,   6, pd_din, pd_dout, mode6,  pd_model),
        PD7:  (pd7,   7, pd_din, pd_dout, mode7,  pd_model),
        PE10: (pe10, 10, pe_din, pe_dout, mode10, pe_modeh),
        PE11: (pe11, 11, pe_din, pe_dout, mode11, pe_modeh),
        PE12: (pe12, 12, pe_din, pe_dout, mode12, pe_modeh),
        PE13: (pe13, 13, pe_din, pe_dout, mode13, pe_modeh),
        PF0:  (pf0,   0, pf_din, pf_dout, mode0,  pf_model),
        PF1:  (pf1,   1, pf_din, pf_dout, mode1,  pf_model),
        PF2:  (pf2,   2, pf_din, pf_dout, mode2,  pf_model),
        PF3:  (pf3,   3, pf_din, pf_dout, mode3,  pf_model),
        PF4:  (pf4,   4, pf_din, pf_dout, mode4,  pf_model),
        PF5:  (pf5,   5, pf_din, pf_dout, mode5,  pf_model),
    ],
    switches: [
        PA0:    (pa_doutclr, pa_doutset, pa_douttgl),
        PA1:    (pa_doutclr, pa_doutset, pa_douttgl),
        PA2:    (pa_doutclr, pa_doutset, pa_douttgl),
        PA8:    (pa_doutclr, pa_doutset, pa_douttgl),
        PA9:    (pa_doutclr, pa_doutset, pa_douttgl),
        PA10:   (pa_doutclr, pa_doutset, pa_douttgl),
        PB7:    (pb_doutclr, pb_doutset, pb_douttgl),
        PB8:    (pb_doutclr, pb_doutset, pb_douttgl),
        PB11:   (pb_doutclr, pb_doutset, pb_douttgl),
        PB13:   (pb_doutclr, pb_doutset, pb_douttgl),
        PB14:   (pb_doutclr, pb_doutset, pb_douttgl),
        PC0:    (pc_doutclr, pc_doutset, pc_douttgl),
        PC1:    (pc_doutclr, pc_doutset, pc_douttgl),
        PC2:    (pc_doutclr, pc_doutset, pc_douttgl),
        PC3:    (pc_doutclr, pc_doutset, pc_douttgl),
        PC4:    (pc_doutclr, pc_doutset, pc_douttgl),
        PC8:    (pc_doutclr, pc_doutset, pc_douttgl),
        PC9:    (pc_doutclr, pc_doutset, pc_douttgl),
        PC10:   (pc_doutclr, pc_doutset, pc_douttgl),
        PC14:   (pc_doutclr, pc_doutset, pc_douttgl),
        PC15:   (pc_doutclr, pc_doutset, pc_douttgl),
        PD4:    (pd_doutclr, pd_doutset, pd_douttgl),
        PD5:    (pd_doutclr, pd_doutset, pd_douttgl),
        PD6:    (pd_doutclr, pd_doutset, pd_douttgl),
        PD7:    (pd_doutclr, pd_doutset, pd_douttgl),
        PE10:   (pe_doutclr, pe_doutset, pe_douttgl),
        PE11:   (pe_doutclr, pe_doutset, pe_douttgl),
        PE12:   (pe_doutclr, pe_doutset, pe_douttgl),
        PE13:   (pe_doutclr, pe_doutset, pe_douttgl),
        PF0:    (pf_doutclr, pf_doutset, pf_douttgl),
        PF1:    (pf_doutclr, pf_doutset, pf_douttgl),
        PF2:    (pf_doutclr, pf_doutset, pf_douttgl),
        PF3:    (pf_doutclr, pf_doutset, pf_douttgl),
        PF4:    (pf_doutclr, pf_doutset, pf_douttgl),
        PF5:    (pf_doutclr, pf_doutset, pf_douttgl),
    ],
    interrupts: [
        PA0:    (extipsel0,  porta, extipsell),
        PA1:    (extipsel1,  porta, extipsell),
        PA2:    (extipsel2,  porta, extipsell),
        PA8:    (extipsel8,  porta, extipselh),
        PA9:    (extipsel9,  porta, extipselh),
        PA10:   (extipsel10, porta, extipselh),
        PB7:    (extipsel7,  portb, extipsell),
        PB8:    (extipsel8,  portb, extipselh),
        PB11:   (extipsel11, portb, extipselh),
        PB13:   (extipsel13, portb, extipselh),
        PB14:   (extipsel14, portb, extipselh),
        PC0:    (extipsel0,  portc, extipsell),
        PC1:    (extipsel1,  portc, extipsell),
        PC2:    (extipsel2,  portc, extipsell),
        PC3:    (extipsel3,  portc, extipsell),
        PC4:    (extipsel4,  portc, extipsell),
        PC8:    (extipsel8,  portc, extipselh),
        PC9:    (extipsel9,  portc, extipselh),
        PC10:   (extipsel10, portc, extipselh),
        PC14:   (extipsel14, portc, extipselh),
        PC15:   (extipsel15, portc, extipselh),
        PD4:    (extipsel4,  portd, extipsell),
        PD5:    (extipsel5,  portd, extipsell),
        PD6:    (extipsel6,  portd, extipsell),
        PD7:    (extipsel7,  portd, extipsell),
        PE10:   (extipsel10, porte, extipselh),
        PE11:   (extipsel11, porte, extipselh),
        PE12:   (extipsel12, porte, extipselh),
        PE13:   (extipsel13, porte, extipselh),
        PF0:    (extipsel0,  portf, extipsell),
        PF1:    (extipsel1,  portf, extipsell),
        PF2:    (extipsel2,  portf, extipsell),
        PF3:    (extipsel3,  portf, extipsell),
        PF4:    (extipsel4,  portf, extipsell),
        PF5:    (extipsel5,  portf, extipsell),
    ],
});
