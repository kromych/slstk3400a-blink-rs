#[doc = "Register `DOEP2_DMAADDR` reader"]
pub type R = crate::R<Doep2DmaaddrSpec>;
#[doc = "Register `DOEP2_DMAADDR` writer"]
pub type W = crate::W<Doep2DmaaddrSpec>;
#[doc = "Field `DMAADDR` reader - DMA Address"]
pub type DmaaddrR = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR` writer - DMA Address"]
pub type DmaaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DmaaddrR {
        DmaaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DmaaddrW<'_, Doep2DmaaddrSpec> {
        DmaaddrW::new(self, 0)
    }
}
#[doc = "Device OUT Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep2_dmaaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep2_dmaaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doep2DmaaddrSpec;
impl crate::RegisterSpec for Doep2DmaaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doep2_dmaaddr::R`](R) reader structure"]
impl crate::Readable for Doep2DmaaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`doep2_dmaaddr::W`](W) writer structure"]
impl crate::Writable for Doep2DmaaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEP2_DMAADDR to value 0"]
impl crate::Resettable for Doep2DmaaddrSpec {}
