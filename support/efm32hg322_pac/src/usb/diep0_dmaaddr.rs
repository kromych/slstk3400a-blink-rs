#[doc = "Register `DIEP0_DMAADDR` reader"]
pub type R = crate::R<Diep0DmaaddrSpec>;
#[doc = "Register `DIEP0_DMAADDR` writer"]
pub type W = crate::W<Diep0DmaaddrSpec>;
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
    pub fn dmaaddr(&mut self) -> DmaaddrW<'_, Diep0DmaaddrSpec> {
        DmaaddrW::new(self, 0)
    }
}
#[doc = "Device IN Endpoint x+1 DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0_dmaaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep0_dmaaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diep0DmaaddrSpec;
impl crate::RegisterSpec for Diep0DmaaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep0_dmaaddr::R`](R) reader structure"]
impl crate::Readable for Diep0DmaaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`diep0_dmaaddr::W`](W) writer structure"]
impl crate::Writable for Diep0DmaaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEP0_DMAADDR to value 0"]
impl crate::Resettable for Diep0DmaaddrSpec {}
