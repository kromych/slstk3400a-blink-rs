#[doc = "Register `GAHBCFG` reader"]
pub type R = crate::R<GahbcfgSpec>;
#[doc = "Register `GAHBCFG` writer"]
pub type W = crate::W<GahbcfgSpec>;
#[doc = "Field `GLBLINTRMSK` reader - Global Interrupt Mask"]
pub type GlblintrmskR = crate::BitReader;
#[doc = "Field `GLBLINTRMSK` writer - Global Interrupt Mask"]
pub type GlblintrmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Burst Length/Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hbstlen {
    #[doc = "0: Single transfer."]
    Single = 0,
    #[doc = "1: Incrementing burst of unspecified length."]
    Incr = 1,
    #[doc = "3: 4-beat incrementing burst."]
    Incr4 = 3,
    #[doc = "5: 8-beat incrementing burst."]
    Incr8 = 5,
    #[doc = "7: 16-beat incrementing burst."]
    Incr16 = 7,
}
impl From<Hbstlen> for u8 {
    #[inline(always)]
    fn from(variant: Hbstlen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hbstlen {
    type Ux = u8;
}
impl crate::IsEnum for Hbstlen {}
#[doc = "Field `HBSTLEN` reader - Burst Length/Type"]
pub type HbstlenR = crate::FieldReader<Hbstlen>;
impl HbstlenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hbstlen> {
        match self.bits {
            0 => Some(Hbstlen::Single),
            1 => Some(Hbstlen::Incr),
            3 => Some(Hbstlen::Incr4),
            5 => Some(Hbstlen::Incr8),
            7 => Some(Hbstlen::Incr16),
            _ => None,
        }
    }
    #[doc = "Single transfer."]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Hbstlen::Single
    }
    #[doc = "Incrementing burst of unspecified length."]
    #[inline(always)]
    pub fn is_incr(&self) -> bool {
        *self == Hbstlen::Incr
    }
    #[doc = "4-beat incrementing burst."]
    #[inline(always)]
    pub fn is_incr4(&self) -> bool {
        *self == Hbstlen::Incr4
    }
    #[doc = "8-beat incrementing burst."]
    #[inline(always)]
    pub fn is_incr8(&self) -> bool {
        *self == Hbstlen::Incr8
    }
    #[doc = "16-beat incrementing burst."]
    #[inline(always)]
    pub fn is_incr16(&self) -> bool {
        *self == Hbstlen::Incr16
    }
}
#[doc = "Field `HBSTLEN` writer - Burst Length/Type"]
pub type HbstlenW<'a, REG> = crate::FieldWriter<'a, REG, 4, Hbstlen>;
impl<'a, REG> HbstlenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single transfer."]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Hbstlen::Single)
    }
    #[doc = "Incrementing burst of unspecified length."]
    #[inline(always)]
    pub fn incr(self) -> &'a mut crate::W<REG> {
        self.variant(Hbstlen::Incr)
    }
    #[doc = "4-beat incrementing burst."]
    #[inline(always)]
    pub fn incr4(self) -> &'a mut crate::W<REG> {
        self.variant(Hbstlen::Incr4)
    }
    #[doc = "8-beat incrementing burst."]
    #[inline(always)]
    pub fn incr8(self) -> &'a mut crate::W<REG> {
        self.variant(Hbstlen::Incr8)
    }
    #[doc = "16-beat incrementing burst."]
    #[inline(always)]
    pub fn incr16(self) -> &'a mut crate::W<REG> {
        self.variant(Hbstlen::Incr16)
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NPTXFEMPLVL` reader - Non-Periodic TxFIFO Empty Level"]
pub type NptxfemplvlR = crate::BitReader;
#[doc = "Field `NPTXFEMPLVL` writer - Non-Periodic TxFIFO Empty Level"]
pub type NptxfemplvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REMMEMSUPP` reader - Remote Memory Support"]
pub type RemmemsuppR = crate::BitReader;
#[doc = "Field `REMMEMSUPP` writer - Remote Memory Support"]
pub type RemmemsuppW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOTIALLDMAWRIT` reader - Notify All DMA Writes"]
pub type NotialldmawritR = crate::BitReader;
#[doc = "Field `NOTIALLDMAWRIT` writer - Notify All DMA Writes"]
pub type NotialldmawritW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBSINGLE` reader - AHB Single Support"]
pub type AhbsingleR = crate::BitReader;
#[doc = "Field `AHBSINGLE` writer - AHB Single Support"]
pub type AhbsingleW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Global Interrupt Mask"]
    #[inline(always)]
    pub fn glblintrmsk(&self) -> GlblintrmskR {
        GlblintrmskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Burst Length/Type"]
    #[inline(always)]
    pub fn hbstlen(&self) -> HbstlenR {
        HbstlenR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Non-Periodic TxFIFO Empty Level"]
    #[inline(always)]
    pub fn nptxfemplvl(&self) -> NptxfemplvlR {
        NptxfemplvlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 21 - Remote Memory Support"]
    #[inline(always)]
    pub fn remmemsupp(&self) -> RemmemsuppR {
        RemmemsuppR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Notify All DMA Writes"]
    #[inline(always)]
    pub fn notialldmawrit(&self) -> NotialldmawritR {
        NotialldmawritR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - AHB Single Support"]
    #[inline(always)]
    pub fn ahbsingle(&self) -> AhbsingleR {
        AhbsingleR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global Interrupt Mask"]
    #[inline(always)]
    pub fn glblintrmsk(&mut self) -> GlblintrmskW<'_, GahbcfgSpec> {
        GlblintrmskW::new(self, 0)
    }
    #[doc = "Bits 1:4 - Burst Length/Type"]
    #[inline(always)]
    pub fn hbstlen(&mut self) -> HbstlenW<'_, GahbcfgSpec> {
        HbstlenW::new(self, 1)
    }
    #[doc = "Bit 5 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, GahbcfgSpec> {
        DmaenW::new(self, 5)
    }
    #[doc = "Bit 7 - Non-Periodic TxFIFO Empty Level"]
    #[inline(always)]
    pub fn nptxfemplvl(&mut self) -> NptxfemplvlW<'_, GahbcfgSpec> {
        NptxfemplvlW::new(self, 7)
    }
    #[doc = "Bit 21 - Remote Memory Support"]
    #[inline(always)]
    pub fn remmemsupp(&mut self) -> RemmemsuppW<'_, GahbcfgSpec> {
        RemmemsuppW::new(self, 21)
    }
    #[doc = "Bit 22 - Notify All DMA Writes"]
    #[inline(always)]
    pub fn notialldmawrit(&mut self) -> NotialldmawritW<'_, GahbcfgSpec> {
        NotialldmawritW::new(self, 22)
    }
    #[doc = "Bit 23 - AHB Single Support"]
    #[inline(always)]
    pub fn ahbsingle(&mut self) -> AhbsingleW<'_, GahbcfgSpec> {
        AhbsingleW::new(self, 23)
    }
}
#[doc = "AHB Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gahbcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gahbcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GahbcfgSpec;
impl crate::RegisterSpec for GahbcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gahbcfg::R`](R) reader structure"]
impl crate::Readable for GahbcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`gahbcfg::W`](W) writer structure"]
impl crate::Writable for GahbcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GAHBCFG to value 0"]
impl crate::Resettable for GahbcfgSpec {}
