#[doc = "Register `SWPULSE` writer"]
pub type W = crate::W<SWPULSE_SPEC>;
#[doc = "Field `CH0PULSE` writer - Channel 0 Pulse Generation"]
pub type CH0PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1PULSE` writer - Channel 1 Pulse Generation"]
pub type CH1PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2PULSE` writer - Channel 2 Pulse Generation"]
pub type CH2PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3PULSE` writer - Channel 3 Pulse Generation"]
pub type CH3PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4PULSE` writer - Channel 4 Pulse Generation"]
pub type CH4PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5PULSE` writer - Channel 5 Pulse Generation"]
pub type CH5PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch0pulse(&mut self) -> CH0PULSE_W<SWPULSE_SPEC> {
        CH0PULSE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch1pulse(&mut self) -> CH1PULSE_W<SWPULSE_SPEC> {
        CH1PULSE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch2pulse(&mut self) -> CH2PULSE_W<SWPULSE_SPEC> {
        CH2PULSE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch3pulse(&mut self) -> CH3PULSE_W<SWPULSE_SPEC> {
        CH3PULSE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch4pulse(&mut self) -> CH4PULSE_W<SWPULSE_SPEC> {
        CH4PULSE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Pulse Generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch5pulse(&mut self) -> CH5PULSE_W<SWPULSE_SPEC> {
        CH5PULSE_W::new(self, 5)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Software Pulse Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swpulse::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWPULSE_SPEC;
impl crate::RegisterSpec for SWPULSE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swpulse::W`](W) writer structure"]
impl crate::Writable for SWPULSE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWPULSE to value 0"]
impl crate::Resettable for SWPULSE_SPEC {
    const RESET_VALUE: u32 = 0;
}
