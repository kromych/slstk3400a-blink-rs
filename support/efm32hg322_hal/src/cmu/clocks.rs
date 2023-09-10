use crate::registers;
use core::marker::PhantomData;

macro_rules! peripheral_clocks {
    ($name: ident, $section: ident, $enabled: ident) => {
        pub struct $name {
            _private: (),
        }

        impl $name {
            pub(crate) fn constrain() -> Self {
                Self { _private: () }
            }

            pub fn enable(self) -> $enabled {
                let cmu = unsafe { &*registers::CMU::ptr() };
                cmu.hfperclken0.modify(|_, w| w.$section().set_bit());

                $enabled {
                    _private: PhantomData,
                }
            }
        }

        pub struct $enabled {
            _private: PhantomData<$name>,
        }

        impl $enabled {
            pub fn disable(self) -> $name {
                let cmu = unsafe { &*registers::CMU::ptr() };
                cmu.hfperclken0.modify(|_, w| w.$section().clear_bit());

                $name { _private: () }
            }
        }
    };
}
peripheral_clocks!(ACMP0, acmp0, ACMP0Enabled);
peripheral_clocks!(ADC0, adc0, ADC0Enabled);
peripheral_clocks!(GPIO, gpio, GPIOEnabled);
peripheral_clocks!(I2C0, i2c0, I2C0Enabled);
peripheral_clocks!(IDAC0, idac0, IDAC0Enabled);
peripheral_clocks!(PRS, prs, PRSEnabled);
peripheral_clocks!(TIMER0, timer0, TIMER0Enabled);
peripheral_clocks!(TIMER1, timer1, TIMER1Enabled);

#[cfg(feature = "_has_timer2")]
peripheral_clocks!(TIMER2, timer2, TIMER2Enabled);

peripheral_clocks!(USART0, usart0, USART0Enabled);
peripheral_clocks!(USART1, usart1, USART1Enabled);
peripheral_clocks!(VCMP, vcmp, VCMPEnabled);

#[cfg(feature = "use_le_peripherals")]
use crate::cmu::Clocks;

#[rustfmt::skip]
#[cfg(feature = "use_le_peripherals")]
pub fn reset_le_clocks(clocks: &Clocks) {
    clocks.cmu.hfcoreclken0.modify(|_, w| w.le().set_bit());
    clocks.cmu.lfclksel.modify(|_, w|
        w.lfa().disabled()
         .lfb().disabled()
         .lfc().disabled()
    );
}

macro_rules! le_peripheral_clocks {
    ($name: ident, $lfreg: ident, $clkreg: ident, $section: ident, $enabled: ident) => {
        #[cfg(feature = "use_le_peripherals")]
        pub struct $name {
            _private: (),
        }

        #[cfg(feature = "use_le_peripherals")]
        impl $name {
            pub(crate) fn constrain() -> Self {
                Self { _private: () }
            }

            pub fn enable(self) -> $enabled {
                let cmu = unsafe { &*registers::CMU::ptr() };
                cmu.lfclksel.modify(|_, w| w.$lfreg().lfrco());
                cmu.$clkreg.modify(|_, w| w.$section().set_bit());

                $enabled {
                    _private: PhantomData,
                }
            }
        }

        #[cfg(feature = "use_le_peripherals")]
        pub struct $enabled {
            _private: PhantomData<$name>,
        }

        #[cfg(feature = "use_le_peripherals")]
        impl $enabled {
            pub fn disable(self) -> $name {
                let cmu = unsafe { &*registers::CMU::ptr() };
                cmu.$clkreg.modify(|_, w| w.$section().clear_bit());
                cmu.lfclksel.modify(|_, w| w.$lfreg().disabled());

                $name { _private: () }
            }
        }
    };
}

le_peripheral_clocks!(RTC, lfa, lfaclken0, rtc, RTCEnabled);
le_peripheral_clocks!(LEUART0, lfb, lfbclken0, leuart0, LEUART0Enabled);
le_peripheral_clocks!(USBLE, lfc, lfcclken0, usble, USBLEEnabled);
