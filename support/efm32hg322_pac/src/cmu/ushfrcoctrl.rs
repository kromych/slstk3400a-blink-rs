#[doc = "Register `USHFRCOCTRL` reader"]
pub type R = crate::R<UshfrcoctrlSpec>;
#[doc = "Register `USHFRCOCTRL` writer"]
pub type W = crate::W<UshfrcoctrlSpec>;
#[doc = "Field `TUNING` reader - USHFRCO frequency adjust"]
pub type TuningR = crate::FieldReader;
#[doc = "Field `TUNING` writer - USHFRCO frequency adjust"]
pub type TuningW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DITHEN` reader - USHFRCO dither enable"]
pub type DithenR = crate::BitReader;
#[doc = "Field `DITHEN` writer - USHFRCO dither enable"]
pub type DithenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPEND` reader - USHFRCO suspend"]
pub type SuspendR = crate::BitReader;
#[doc = "Field `SUSPEND` writer - USHFRCO suspend"]
pub type SuspendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` reader - USHFRCO Timeout"]
pub type TimeoutR = crate::FieldReader;
#[doc = "Field `TIMEOUT` writer - USHFRCO Timeout"]
pub type TimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:6 - USHFRCO frequency adjust"]
    #[inline(always)]
    pub fn tuning(&self) -> TuningR {
        TuningR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - USHFRCO dither enable"]
    #[inline(always)]
    pub fn dithen(&self) -> DithenR {
        DithenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USHFRCO suspend"]
    #[inline(always)]
    pub fn suspend(&self) -> SuspendR {
        SuspendR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:19 - USHFRCO Timeout"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - USHFRCO frequency adjust"]
    #[inline(always)]
    pub fn tuning(&mut self) -> TuningW<'_, UshfrcoctrlSpec> {
        TuningW::new(self, 0)
    }
    #[doc = "Bit 8 - USHFRCO dither enable"]
    #[inline(always)]
    pub fn dithen(&mut self) -> DithenW<'_, UshfrcoctrlSpec> {
        DithenW::new(self, 8)
    }
    #[doc = "Bit 9 - USHFRCO suspend"]
    #[inline(always)]
    pub fn suspend(&mut self) -> SuspendW<'_, UshfrcoctrlSpec> {
        SuspendW::new(self, 9)
    }
    #[doc = "Bits 12:19 - USHFRCO Timeout"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TimeoutW<'_, UshfrcoctrlSpec> {
        TimeoutW::new(self, 12)
    }
}
#[doc = "USHFRCO Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ushfrcoctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ushfrcoctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UshfrcoctrlSpec;
impl crate::RegisterSpec for UshfrcoctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ushfrcoctrl::R`](R) reader structure"]
impl crate::Readable for UshfrcoctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ushfrcoctrl::W`](W) writer structure"]
impl crate::Writable for UshfrcoctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USHFRCOCTRL to value 0x000f_f040"]
impl crate::Resettable for UshfrcoctrlSpec {
    const RESET_VALUE: u32 = 0x000f_f040;
}
