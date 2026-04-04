#[doc = "Register `MASTER` reader"]
pub type R = crate::R<MasterSpec>;
#[doc = "Register `MASTER` writer"]
pub type W = crate::W<MasterSpec>;
#[doc = "Field `MASK` reader - This value determines the maximum size of the trace buffer in SRAM."]
pub type MaskR = crate::FieldReader;
#[doc = "Field `MASK` writer - This value determines the maximum size of the trace buffer in SRAM."]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TSTARTEN` reader - Trace start input enable."]
pub type TstartenR = crate::BitReader;
#[doc = "Field `TSTARTEN` writer - Trace start input enable."]
pub type TstartenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTOPEN` reader - Trace stop input enable."]
pub type TstopenR = crate::BitReader;
#[doc = "Field `TSTOPEN` writer - Trace stop input enable."]
pub type TstopenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALTREQ` reader - Halt request bit."]
pub type HaltreqR = crate::BitReader;
#[doc = "Field `HALTREQ` writer - Halt request bit."]
pub type HaltreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - Main trace enable bit."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Main trace enable bit."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - This value determines the maximum size of the trace buffer in SRAM."]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Trace start input enable."]
    #[inline(always)]
    pub fn tstarten(&self) -> TstartenR {
        TstartenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trace stop input enable."]
    #[inline(always)]
    pub fn tstopen(&self) -> TstopenR {
        TstopenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Halt request bit."]
    #[inline(always)]
    pub fn haltreq(&self) -> HaltreqR {
        HaltreqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - Main trace enable bit."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - This value determines the maximum size of the trace buffer in SRAM."]
    #[inline(always)]
    pub fn mask(&mut self) -> MaskW<'_, MasterSpec> {
        MaskW::new(self, 0)
    }
    #[doc = "Bit 5 - Trace start input enable."]
    #[inline(always)]
    pub fn tstarten(&mut self) -> TstartenW<'_, MasterSpec> {
        TstartenW::new(self, 5)
    }
    #[doc = "Bit 6 - Trace stop input enable."]
    #[inline(always)]
    pub fn tstopen(&mut self) -> TstopenW<'_, MasterSpec> {
        TstopenW::new(self, 6)
    }
    #[doc = "Bit 9 - Halt request bit."]
    #[inline(always)]
    pub fn haltreq(&mut self) -> HaltreqW<'_, MasterSpec> {
        HaltreqW::new(self, 9)
    }
    #[doc = "Bit 31 - Main trace enable bit."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, MasterSpec> {
        EnW::new(self, 31)
    }
}
#[doc = "MTB Trace Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`master::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`master::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MasterSpec;
impl crate::RegisterSpec for MasterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`master::R`](R) reader structure"]
impl crate::Readable for MasterSpec {}
#[doc = "`write(|w| ..)` method takes [`master::W`](W) writer structure"]
impl crate::Writable for MasterSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MASTER to value 0"]
impl crate::Resettable for MasterSpec {}
