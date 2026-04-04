#[doc = "Register `DIEP0DMAADDR` reader"]
pub type R = crate::R<Diep0dmaaddrSpec>;
#[doc = "Register `DIEP0DMAADDR` writer"]
pub type W = crate::W<Diep0dmaaddrSpec>;
#[doc = "Field `DIEP0DMAADDR` reader - DMA Address"]
pub type Diep0dmaaddrR = crate::FieldReader<u32>;
#[doc = "Field `DIEP0DMAADDR` writer - DMA Address"]
pub type Diep0dmaaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn diep0dmaaddr(&self) -> Diep0dmaaddrR {
        Diep0dmaaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn diep0dmaaddr(&mut self) -> Diep0dmaaddrW<'_, Diep0dmaaddrSpec> {
        Diep0dmaaddrW::new(self, 0)
    }
}
#[doc = "Device IN Endpoint 0 DMA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0dmaaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep0dmaaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diep0dmaaddrSpec;
impl crate::RegisterSpec for Diep0dmaaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep0dmaaddr::R`](R) reader structure"]
impl crate::Readable for Diep0dmaaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`diep0dmaaddr::W`](W) writer structure"]
impl crate::Writable for Diep0dmaaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEP0DMAADDR to value 0"]
impl crate::Resettable for Diep0dmaaddrSpec {}
