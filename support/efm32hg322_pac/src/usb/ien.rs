#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `VREGOSH` reader - VREGO Sense High Interrupt Enable"]
pub type VregoshR = crate::BitReader;
#[doc = "Field `VREGOSH` writer - VREGO Sense High Interrupt Enable"]
pub type VregoshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREGOSL` reader - VREGO Sense Low Interrupt Enable"]
pub type VregoslR = crate::BitReader;
#[doc = "Field `VREGOSL` writer - VREGO Sense Low Interrupt Enable"]
pub type VregoslW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - VREGO Sense High Interrupt Enable"]
    #[inline(always)]
    pub fn vregosh(&self) -> VregoshR {
        VregoshR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VREGO Sense Low Interrupt Enable"]
    #[inline(always)]
    pub fn vregosl(&self) -> VregoslR {
        VregoslR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VREGO Sense High Interrupt Enable"]
    #[inline(always)]
    pub fn vregosh(&mut self) -> VregoshW<'_, IenSpec> {
        VregoshW::new(self, 0)
    }
    #[doc = "Bit 1 - VREGO Sense Low Interrupt Enable"]
    #[inline(always)]
    pub fn vregosl(&mut self) -> VregoslW<'_, IenSpec> {
        VregoslW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
