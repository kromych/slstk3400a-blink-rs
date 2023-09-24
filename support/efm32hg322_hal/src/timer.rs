//! TIMER (Timer/Counter peripheral)
//!
//! This module exposes some features of the EFM32 timer/counter peripheral; most notably, it
//! allows easy configuration of PWM pins.
#![allow(dead_code)]

use crate::registers;

pub trait TimerExt<Timer> {
    fn with_clock(self) -> Timer;
}

macro_rules! timer {
    ($TIMERn: ident, $TIMERnClk: ident, $TimerN: ident, $timerN: ident) => {
        impl TimerExt<$TimerN> for registers::$TIMERn {
            fn with_clock(self) -> $TimerN {
                $TimerN { register: self }
            }
        }

        pub struct $TimerN {
            register: registers::$TIMERn,
        }

        impl $TimerN {
            pub fn set_top(&mut self, top: u16) {
                self.register
                    .top
                    .modify(|_, w| unsafe { w.top().bits(top) });
            }

            pub fn interrupt_enable(&mut self, interrupt: InterruptFlag) {
                self.register
                    .ien
                    .modify(|r, w| unsafe { w.bits(interrupt.bits() | r.bits()) });
            }

            pub fn interrupt_is_pending(interrupt: InterruptFlag) -> bool {
                let reg = unsafe { &*registers::$TIMERn::ptr() };
                reg.if_.read().bits() & interrupt.bits() != 0
            }

            pub fn interrupt_unpend(interrupt: InterruptFlag) {
                unsafe {
                    let reg = &*registers::$TIMERn::ptr();
                    reg.ifc.write(|w| w.bits(interrupt.bits()));
                }
            }

            pub fn start(&mut self) {
                self.register.cmd.write(|w| w.start().bit(true));
            }
        }
    };
}

/// Timer interrupt flags
///
/// Each value represents a particular interrupt flag that is available for enabling, setting and
/// clearing in all timers.
///
/// These are implemented explicitly rather than re-using the register block's individual types, as
/// not only those are duplicate across the timers (a common occurrence in svd2rust crates), but
/// even over all interrupt registers of a timer. Implementing them as one bakes in the assumption
/// that the same flags that can be enabled can also be set or cleared.
#[derive(Copy, Clone)]
pub enum InterruptFlag {
    /// Overflow
    OF = 1,
    /// Underflow
    UF = 2,
    /// CC Channel 0
    CC0 = 16,
    /// CC Channel 1
    CC1 = 32,
    /// CC Channel 2
    CC2 = 64,
    /// CC Channel 0 Input Capture Buffer Overflow
    ICBOF0 = 256,
    /// CC Channel 1 Input Capture Buffer Overflow
    ICBOF1 = 512,
    /// CC Channel 2 Input Capture Buffer Overflow
    ICBOF2 = 1024,
}

impl InterruptFlag {
    const fn bits(&self) -> u32 {
        *self as u32
    }
}

timer!(TIMER0, TIMER0Enabled, Timer0, timer0);
timer!(TIMER1, TIMER1Enabled, Timer1, timer1);
timer!(TIMER2, TIMER2Enabled, Timer2, timer2);
