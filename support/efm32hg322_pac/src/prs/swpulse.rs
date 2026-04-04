#[doc = "Register `SWPULSE` writer"]
pub type W = crate::W<SwpulseSpec>;
#[doc = "Field `CH0PULSE` writer - Channel 0 Pulse Generation"]
pub type Ch0pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1PULSE` writer - Channel 1 Pulse Generation"]
pub type Ch1pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2PULSE` writer - Channel 2 Pulse Generation"]
pub type Ch2pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3PULSE` writer - Channel 3 Pulse Generation"]
pub type Ch3pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4PULSE` writer - Channel 4 Pulse Generation"]
pub type Ch4pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5PULSE` writer - Channel 5 Pulse Generation"]
pub type Ch5pulseW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 Pulse Generation"]
    #[inline(always)]
    pub fn ch0pulse(&mut self) -> Ch0pulseW<'_, SwpulseSpec> {
        Ch0pulseW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Pulse Generation"]
    #[inline(always)]
    pub fn ch1pulse(&mut self) -> Ch1pulseW<'_, SwpulseSpec> {
        Ch1pulseW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Pulse Generation"]
    #[inline(always)]
    pub fn ch2pulse(&mut self) -> Ch2pulseW<'_, SwpulseSpec> {
        Ch2pulseW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Pulse Generation"]
    #[inline(always)]
    pub fn ch3pulse(&mut self) -> Ch3pulseW<'_, SwpulseSpec> {
        Ch3pulseW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Pulse Generation"]
    #[inline(always)]
    pub fn ch4pulse(&mut self) -> Ch4pulseW<'_, SwpulseSpec> {
        Ch4pulseW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Pulse Generation"]
    #[inline(always)]
    pub fn ch5pulse(&mut self) -> Ch5pulseW<'_, SwpulseSpec> {
        Ch5pulseW::new(self, 5)
    }
}
#[doc = "Software Pulse Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swpulse::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwpulseSpec;
impl crate::RegisterSpec for SwpulseSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swpulse::W`](W) writer structure"]
impl crate::Writable for SwpulseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWPULSE to value 0"]
impl crate::Resettable for SwpulseSpec {}
