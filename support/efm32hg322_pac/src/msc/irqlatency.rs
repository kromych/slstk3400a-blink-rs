#[doc = "Register `IRQLATENCY` reader"]
pub type R = crate::R<IrqlatencySpec>;
#[doc = "Register `IRQLATENCY` writer"]
pub type W = crate::W<IrqlatencySpec>;
#[doc = "Field `IRQLATENCY` reader - Irq Latency Register"]
pub type IrqlatencyR = crate::FieldReader;
#[doc = "Field `IRQLATENCY` writer - Irq Latency Register"]
pub type IrqlatencyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Irq Latency Register"]
    #[inline(always)]
    pub fn irqlatency(&self) -> IrqlatencyR {
        IrqlatencyR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Irq Latency Register"]
    #[inline(always)]
    pub fn irqlatency(&mut self) -> IrqlatencyW<'_, IrqlatencySpec> {
        IrqlatencyW::new(self, 0)
    }
}
#[doc = "Irq Latency Register\n\nYou can [`read`](crate::Reg::read) this register and get [`irqlatency::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqlatency::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqlatencySpec;
impl crate::RegisterSpec for IrqlatencySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqlatency::R`](R) reader structure"]
impl crate::Readable for IrqlatencySpec {}
#[doc = "`write(|w| ..)` method takes [`irqlatency::W`](W) writer structure"]
impl crate::Writable for IrqlatencySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQLATENCY to value 0"]
impl crate::Resettable for IrqlatencySpec {}
