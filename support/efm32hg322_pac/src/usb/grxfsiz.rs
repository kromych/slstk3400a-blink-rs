#[doc = "Register `GRXFSIZ` reader"]
pub type R = crate::R<GrxfsizSpec>;
#[doc = "Register `GRXFSIZ` writer"]
pub type W = crate::W<GrxfsizSpec>;
#[doc = "Field `RXFDEP` reader - RxFIFO Depth"]
pub type RxfdepR = crate::FieldReader<u16>;
#[doc = "Field `RXFDEP` writer - RxFIFO Depth"]
pub type RxfdepW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - RxFIFO Depth"]
    #[inline(always)]
    pub fn rxfdep(&self) -> RxfdepR {
        RxfdepR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - RxFIFO Depth"]
    #[inline(always)]
    pub fn rxfdep(&mut self) -> RxfdepW<'_, GrxfsizSpec> {
        RxfdepW::new(self, 0)
    }
}
#[doc = "Receive FIFO Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`grxfsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grxfsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrxfsizSpec;
impl crate::RegisterSpec for GrxfsizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grxfsiz::R`](R) reader structure"]
impl crate::Readable for GrxfsizSpec {}
#[doc = "`write(|w| ..)` method takes [`grxfsiz::W`](W) writer structure"]
impl crate::Writable for GrxfsizSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRXFSIZ to value 0x0200"]
impl crate::Resettable for GrxfsizSpec {
    const RESET_VALUE: u32 = 0x0200;
}
