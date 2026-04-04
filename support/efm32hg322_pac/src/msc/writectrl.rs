#[doc = "Register `WRITECTRL` reader"]
pub type R = crate::R<WritectrlSpec>;
#[doc = "Register `WRITECTRL` writer"]
pub type W = crate::W<WritectrlSpec>;
#[doc = "Field `WREN` reader - Enable Write/Erase Controller"]
pub type WrenR = crate::BitReader;
#[doc = "Field `WREN` writer - Enable Write/Erase Controller"]
pub type WrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQERASEABORT` reader - Abort Page Erase on Interrupt"]
pub type IrqeraseabortR = crate::BitReader;
#[doc = "Field `IRQERASEABORT` writer - Abort Page Erase on Interrupt"]
pub type IrqeraseabortW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write/Erase Controller"]
    #[inline(always)]
    pub fn wren(&self) -> WrenR {
        WrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Abort Page Erase on Interrupt"]
    #[inline(always)]
    pub fn irqeraseabort(&self) -> IrqeraseabortR {
        IrqeraseabortR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write/Erase Controller"]
    #[inline(always)]
    pub fn wren(&mut self) -> WrenW<'_, WritectrlSpec> {
        WrenW::new(self, 0)
    }
    #[doc = "Bit 1 - Abort Page Erase on Interrupt"]
    #[inline(always)]
    pub fn irqeraseabort(&mut self) -> IrqeraseabortW<'_, WritectrlSpec> {
        IrqeraseabortW::new(self, 1)
    }
}
#[doc = "Write Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`writectrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`writectrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WritectrlSpec;
impl crate::RegisterSpec for WritectrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`writectrl::R`](R) reader structure"]
impl crate::Readable for WritectrlSpec {}
#[doc = "`write(|w| ..)` method takes [`writectrl::W`](W) writer structure"]
impl crate::Writable for WritectrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WRITECTRL to value 0"]
impl crate::Resettable for WritectrlSpec {}
