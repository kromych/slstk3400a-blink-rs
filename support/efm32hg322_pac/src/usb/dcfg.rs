#[doc = "Register `DCFG` reader"]
pub type R = crate::R<DcfgSpec>;
#[doc = "Register `DCFG` writer"]
pub type W = crate::W<DcfgSpec>;
#[doc = "Device Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Devspd {
    #[doc = "2: Low speed (PHY clock is 6 MHz). If you select 6 MHz LS mode, you must do a soft reset."]
    Ls = 2,
    #[doc = "3: Full speed (PHY clock is 48 MHz)."]
    Fs = 3,
}
impl From<Devspd> for u8 {
    #[inline(always)]
    fn from(variant: Devspd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Devspd {
    type Ux = u8;
}
impl crate::IsEnum for Devspd {}
#[doc = "Field `DEVSPD` reader - Device Speed"]
pub type DevspdR = crate::FieldReader<Devspd>;
impl DevspdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Devspd> {
        match self.bits {
            2 => Some(Devspd::Ls),
            3 => Some(Devspd::Fs),
            _ => None,
        }
    }
    #[doc = "Low speed (PHY clock is 6 MHz). If you select 6 MHz LS mode, you must do a soft reset."]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        *self == Devspd::Ls
    }
    #[doc = "Full speed (PHY clock is 48 MHz)."]
    #[inline(always)]
    pub fn is_fs(&self) -> bool {
        *self == Devspd::Fs
    }
}
#[doc = "Field `DEVSPD` writer - Device Speed"]
pub type DevspdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Devspd>;
impl<'a, REG> DevspdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed (PHY clock is 6 MHz). If you select 6 MHz LS mode, you must do a soft reset."]
    #[inline(always)]
    pub fn ls(self) -> &'a mut crate::W<REG> {
        self.variant(Devspd::Ls)
    }
    #[doc = "Full speed (PHY clock is 48 MHz)."]
    #[inline(always)]
    pub fn fs(self) -> &'a mut crate::W<REG> {
        self.variant(Devspd::Fs)
    }
}
#[doc = "Field `NZSTSOUTHSHK` reader - Non-Zero-Length Status OUT Handshake"]
pub type NzstsouthshkR = crate::BitReader;
#[doc = "Field `NZSTSOUTHSHK` writer - Non-Zero-Length Status OUT Handshake"]
pub type NzstsouthshkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENA32KHZSUSP` reader - Enable 32 KHz Suspend mode"]
pub type Ena32khzsuspR = crate::BitReader;
#[doc = "Field `ENA32KHZSUSP` writer - Enable 32 KHz Suspend mode"]
pub type Ena32khzsuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVADDR` reader - Device Address"]
pub type DevaddrR = crate::FieldReader;
#[doc = "Field `DEVADDR` writer - Device Address"]
pub type DevaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Periodic Frame Interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Perfrint {
    #[doc = "0: 80% of the frame interval."]
    _80pcnt = 0,
    #[doc = "1: 85% of the frame interval."]
    _85pcnt = 1,
    #[doc = "2: 90% of the frame interval."]
    _90pcnt = 2,
    #[doc = "3: 95% of the frame interval."]
    _95pcnt = 3,
}
impl From<Perfrint> for u8 {
    #[inline(always)]
    fn from(variant: Perfrint) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Perfrint {
    type Ux = u8;
}
impl crate::IsEnum for Perfrint {}
#[doc = "Field `PERFRINT` reader - Periodic Frame Interval"]
pub type PerfrintR = crate::FieldReader<Perfrint>;
impl PerfrintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perfrint {
        match self.bits {
            0 => Perfrint::_80pcnt,
            1 => Perfrint::_85pcnt,
            2 => Perfrint::_90pcnt,
            3 => Perfrint::_95pcnt,
            _ => unreachable!(),
        }
    }
    #[doc = "80% of the frame interval."]
    #[inline(always)]
    pub fn is_80pcnt(&self) -> bool {
        *self == Perfrint::_80pcnt
    }
    #[doc = "85% of the frame interval."]
    #[inline(always)]
    pub fn is_85pcnt(&self) -> bool {
        *self == Perfrint::_85pcnt
    }
    #[doc = "90% of the frame interval."]
    #[inline(always)]
    pub fn is_90pcnt(&self) -> bool {
        *self == Perfrint::_90pcnt
    }
    #[doc = "95% of the frame interval."]
    #[inline(always)]
    pub fn is_95pcnt(&self) -> bool {
        *self == Perfrint::_95pcnt
    }
}
#[doc = "Field `PERFRINT` writer - Periodic Frame Interval"]
pub type PerfrintW<'a, REG> = crate::FieldWriter<'a, REG, 2, Perfrint, crate::Safe>;
impl<'a, REG> PerfrintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "80% of the frame interval."]
    #[inline(always)]
    pub fn _80pcnt(self) -> &'a mut crate::W<REG> {
        self.variant(Perfrint::_80pcnt)
    }
    #[doc = "85% of the frame interval."]
    #[inline(always)]
    pub fn _85pcnt(self) -> &'a mut crate::W<REG> {
        self.variant(Perfrint::_85pcnt)
    }
    #[doc = "90% of the frame interval."]
    #[inline(always)]
    pub fn _90pcnt(self) -> &'a mut crate::W<REG> {
        self.variant(Perfrint::_90pcnt)
    }
    #[doc = "95% of the frame interval."]
    #[inline(always)]
    pub fn _95pcnt(self) -> &'a mut crate::W<REG> {
        self.variant(Perfrint::_95pcnt)
    }
}
#[doc = "Field `ERRATICINTMSK` reader - "]
pub type ErraticintmskR = crate::BitReader;
#[doc = "Field `ERRATICINTMSK` writer - "]
pub type ErraticintmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESVALID` reader - Resume Validation Period"]
pub type ResvalidR = crate::FieldReader;
#[doc = "Field `RESVALID` writer - Resume Validation Period"]
pub type ResvalidW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - Device Speed"]
    #[inline(always)]
    pub fn devspd(&self) -> DevspdR {
        DevspdR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Non-Zero-Length Status OUT Handshake"]
    #[inline(always)]
    pub fn nzstsouthshk(&self) -> NzstsouthshkR {
        NzstsouthshkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable 32 KHz Suspend mode"]
    #[inline(always)]
    pub fn ena32khzsusp(&self) -> Ena32khzsuspR {
        Ena32khzsuspR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:10 - Device Address"]
    #[inline(always)]
    pub fn devaddr(&self) -> DevaddrR {
        DevaddrR::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval"]
    #[inline(always)]
    pub fn perfrint(&self) -> PerfrintR {
        PerfrintR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn erraticintmsk(&self) -> ErraticintmskR {
        ErraticintmskR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 26:31 - Resume Validation Period"]
    #[inline(always)]
    pub fn resvalid(&self) -> ResvalidR {
        ResvalidR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Device Speed"]
    #[inline(always)]
    pub fn devspd(&mut self) -> DevspdW<'_, DcfgSpec> {
        DevspdW::new(self, 0)
    }
    #[doc = "Bit 2 - Non-Zero-Length Status OUT Handshake"]
    #[inline(always)]
    pub fn nzstsouthshk(&mut self) -> NzstsouthshkW<'_, DcfgSpec> {
        NzstsouthshkW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable 32 KHz Suspend mode"]
    #[inline(always)]
    pub fn ena32khzsusp(&mut self) -> Ena32khzsuspW<'_, DcfgSpec> {
        Ena32khzsuspW::new(self, 3)
    }
    #[doc = "Bits 4:10 - Device Address"]
    #[inline(always)]
    pub fn devaddr(&mut self) -> DevaddrW<'_, DcfgSpec> {
        DevaddrW::new(self, 4)
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval"]
    #[inline(always)]
    pub fn perfrint(&mut self) -> PerfrintW<'_, DcfgSpec> {
        PerfrintW::new(self, 11)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn erraticintmsk(&mut self) -> ErraticintmskW<'_, DcfgSpec> {
        ErraticintmskW::new(self, 15)
    }
    #[doc = "Bits 26:31 - Resume Validation Period"]
    #[inline(always)]
    pub fn resvalid(&mut self) -> ResvalidW<'_, DcfgSpec> {
        ResvalidW::new(self, 26)
    }
}
#[doc = "Device Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcfgSpec;
impl crate::RegisterSpec for DcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcfg::R`](R) reader structure"]
impl crate::Readable for DcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dcfg::W`](W) writer structure"]
impl crate::Writable for DcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCFG to value 0x0800_0000"]
impl crate::Resettable for DcfgSpec {
    const RESET_VALUE: u32 = 0x0800_0000;
}
