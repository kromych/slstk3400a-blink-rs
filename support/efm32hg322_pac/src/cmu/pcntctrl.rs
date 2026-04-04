#[doc = "Register `PCNTCTRL` reader"]
pub type R = crate::R<PcntctrlSpec>;
#[doc = "Register `PCNTCTRL` writer"]
pub type W = crate::W<PcntctrlSpec>;
#[doc = "Field `PCNT0CLKEN` reader - PCNT0 Clock Enable"]
pub type Pcnt0clkenR = crate::BitReader;
#[doc = "Field `PCNT0CLKEN` writer - PCNT0 Clock Enable"]
pub type Pcnt0clkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT0CLKSEL` reader - PCNT0 Clock Select"]
pub type Pcnt0clkselR = crate::BitReader;
#[doc = "Field `PCNT0CLKSEL` writer - PCNT0 Clock Select"]
pub type Pcnt0clkselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PCNT0 Clock Enable"]
    #[inline(always)]
    pub fn pcnt0clken(&self) -> Pcnt0clkenR {
        Pcnt0clkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCNT0 Clock Select"]
    #[inline(always)]
    pub fn pcnt0clksel(&self) -> Pcnt0clkselR {
        Pcnt0clkselR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PCNT0 Clock Enable"]
    #[inline(always)]
    pub fn pcnt0clken(&mut self) -> Pcnt0clkenW<'_, PcntctrlSpec> {
        Pcnt0clkenW::new(self, 0)
    }
    #[doc = "Bit 1 - PCNT0 Clock Select"]
    #[inline(always)]
    pub fn pcnt0clksel(&mut self) -> Pcnt0clkselW<'_, PcntctrlSpec> {
        Pcnt0clkselW::new(self, 1)
    }
}
#[doc = "PCNT Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcntctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcntctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcntctrlSpec;
impl crate::RegisterSpec for PcntctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcntctrl::R`](R) reader structure"]
impl crate::Readable for PcntctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pcntctrl::W`](W) writer structure"]
impl crate::Writable for PcntctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCNTCTRL to value 0"]
impl crate::Resettable for PcntctrlSpec {}
