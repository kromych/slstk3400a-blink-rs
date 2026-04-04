#[doc = "Register `PCGCCTL` reader"]
pub type R = crate::R<PcgcctlSpec>;
#[doc = "Register `PCGCCTL` writer"]
pub type W = crate::W<PcgcctlSpec>;
#[doc = "Field `STOPPCLK` reader - Stop PHY clock"]
pub type StoppclkR = crate::BitReader;
#[doc = "Field `STOPPCLK` writer - Stop PHY clock"]
pub type StoppclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GATEHCLK` reader - Gate HCLK"]
pub type GatehclkR = crate::BitReader;
#[doc = "Field `GATEHCLK` writer - Gate HCLK"]
pub type GatehclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRCLMP` reader - Power Clamp"]
pub type PwrclmpR = crate::BitReader;
#[doc = "Field `PWRCLMP` writer - Power Clamp"]
pub type PwrclmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTPDWNMODULE` reader - Reset Power-Down Modules"]
pub type RstpdwnmoduleR = crate::BitReader;
#[doc = "Field `RSTPDWNMODULE` writer - Reset Power-Down Modules"]
pub type RstpdwnmoduleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHYSLEEP` reader - PHY In Sleep"]
pub type PhysleepR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Stop PHY clock"]
    #[inline(always)]
    pub fn stoppclk(&self) -> StoppclkR {
        StoppclkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gate HCLK"]
    #[inline(always)]
    pub fn gatehclk(&self) -> GatehclkR {
        GatehclkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power Clamp"]
    #[inline(always)]
    pub fn pwrclmp(&self) -> PwrclmpR {
        PwrclmpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset Power-Down Modules"]
    #[inline(always)]
    pub fn rstpdwnmodule(&self) -> RstpdwnmoduleR {
        RstpdwnmoduleR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - PHY In Sleep"]
    #[inline(always)]
    pub fn physleep(&self) -> PhysleepR {
        PhysleepR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop PHY clock"]
    #[inline(always)]
    pub fn stoppclk(&mut self) -> StoppclkW<'_, PcgcctlSpec> {
        StoppclkW::new(self, 0)
    }
    #[doc = "Bit 1 - Gate HCLK"]
    #[inline(always)]
    pub fn gatehclk(&mut self) -> GatehclkW<'_, PcgcctlSpec> {
        GatehclkW::new(self, 1)
    }
    #[doc = "Bit 2 - Power Clamp"]
    #[inline(always)]
    pub fn pwrclmp(&mut self) -> PwrclmpW<'_, PcgcctlSpec> {
        PwrclmpW::new(self, 2)
    }
    #[doc = "Bit 3 - Reset Power-Down Modules"]
    #[inline(always)]
    pub fn rstpdwnmodule(&mut self) -> RstpdwnmoduleW<'_, PcgcctlSpec> {
        RstpdwnmoduleW::new(self, 3)
    }
}
#[doc = "Power and Clock Gating Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcgcctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcgcctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcgcctlSpec;
impl crate::RegisterSpec for PcgcctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcgcctl::R`](R) reader structure"]
impl crate::Readable for PcgcctlSpec {}
#[doc = "`write(|w| ..)` method takes [`pcgcctl::W`](W) writer structure"]
impl crate::Writable for PcgcctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCGCCTL to value 0"]
impl crate::Resettable for PcgcctlSpec {}
