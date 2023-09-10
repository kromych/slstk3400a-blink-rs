#[doc = "Register `DCFG` reader"]
pub type R = crate::R<DCFG_SPEC>;
#[doc = "Register `DCFG` writer"]
pub type W = crate::W<DCFG_SPEC>;
#[doc = "Field `DEVSPD` reader - Device Speed"]
pub type DEVSPD_R = crate::FieldReader<DEVSPD_A>;
#[doc = "Device Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DEVSPD_A {
    #[doc = "2: Low speed (PHY clock is 6 MHz). If you select 6 MHz LS mode, you must do a soft reset."]
    LS = 2,
    #[doc = "3: Full speed (PHY clock is 48 MHz)."]
    FS = 3,
}
impl From<DEVSPD_A> for u8 {
    #[inline(always)]
    fn from(variant: DEVSPD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DEVSPD_A {
    type Ux = u8;
}
impl DEVSPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DEVSPD_A> {
        match self.bits {
            2 => Some(DEVSPD_A::LS),
            3 => Some(DEVSPD_A::FS),
            _ => None,
        }
    }
    #[doc = "Low speed (PHY clock is 6 MHz). If you select 6 MHz LS mode, you must do a soft reset."]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        *self == DEVSPD_A::LS
    }
    #[doc = "Full speed (PHY clock is 48 MHz)."]
    #[inline(always)]
    pub fn is_fs(&self) -> bool {
        *self == DEVSPD_A::FS
    }
}
#[doc = "Field `DEVSPD` writer - Device Speed"]
pub type DEVSPD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, DEVSPD_A>;
impl<'a, REG, const O: u8> DEVSPD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed (PHY clock is 6 MHz). If you select 6 MHz LS mode, you must do a soft reset."]
    #[inline(always)]
    pub fn ls(self) -> &'a mut crate::W<REG> {
        self.variant(DEVSPD_A::LS)
    }
    #[doc = "Full speed (PHY clock is 48 MHz)."]
    #[inline(always)]
    pub fn fs(self) -> &'a mut crate::W<REG> {
        self.variant(DEVSPD_A::FS)
    }
}
#[doc = "Field `NZSTSOUTHSHK` reader - Non-Zero-Length Status OUT Handshake"]
pub type NZSTSOUTHSHK_R = crate::BitReader;
#[doc = "Field `NZSTSOUTHSHK` writer - Non-Zero-Length Status OUT Handshake"]
pub type NZSTSOUTHSHK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENA32KHZSUSP` reader - Enable 32 KHz Suspend mode"]
pub type ENA32KHZSUSP_R = crate::BitReader;
#[doc = "Field `ENA32KHZSUSP` writer - Enable 32 KHz Suspend mode"]
pub type ENA32KHZSUSP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DEVADDR` reader - Device Address"]
pub type DEVADDR_R = crate::FieldReader;
#[doc = "Field `DEVADDR` writer - Device Address"]
pub type DEVADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `PERFRINT` reader - Periodic Frame Interval"]
pub type PERFRINT_R = crate::FieldReader<PERFRINT_A>;
#[doc = "Periodic Frame Interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PERFRINT_A {
    #[doc = "0: 80% of the frame interval."]
    _80PCNT = 0,
    #[doc = "1: 85% of the frame interval."]
    _85PCNT = 1,
    #[doc = "2: 90% of the frame interval."]
    _90PCNT = 2,
    #[doc = "3: 95% of the frame interval."]
    _95PCNT = 3,
}
impl From<PERFRINT_A> for u8 {
    #[inline(always)]
    fn from(variant: PERFRINT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PERFRINT_A {
    type Ux = u8;
}
impl PERFRINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERFRINT_A {
        match self.bits {
            0 => PERFRINT_A::_80PCNT,
            1 => PERFRINT_A::_85PCNT,
            2 => PERFRINT_A::_90PCNT,
            3 => PERFRINT_A::_95PCNT,
            _ => unreachable!(),
        }
    }
    #[doc = "80% of the frame interval."]
    #[inline(always)]
    pub fn is_80pcnt(&self) -> bool {
        *self == PERFRINT_A::_80PCNT
    }
    #[doc = "85% of the frame interval."]
    #[inline(always)]
    pub fn is_85pcnt(&self) -> bool {
        *self == PERFRINT_A::_85PCNT
    }
    #[doc = "90% of the frame interval."]
    #[inline(always)]
    pub fn is_90pcnt(&self) -> bool {
        *self == PERFRINT_A::_90PCNT
    }
    #[doc = "95% of the frame interval."]
    #[inline(always)]
    pub fn is_95pcnt(&self) -> bool {
        *self == PERFRINT_A::_95PCNT
    }
}
#[doc = "Field `PERFRINT` writer - Periodic Frame Interval"]
pub type PERFRINT_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PERFRINT_A>;
impl<'a, REG, const O: u8> PERFRINT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "80% of the frame interval."]
    #[inline(always)]
    pub fn _80pcnt(self) -> &'a mut crate::W<REG> {
        self.variant(PERFRINT_A::_80PCNT)
    }
    #[doc = "85% of the frame interval."]
    #[inline(always)]
    pub fn _85pcnt(self) -> &'a mut crate::W<REG> {
        self.variant(PERFRINT_A::_85PCNT)
    }
    #[doc = "90% of the frame interval."]
    #[inline(always)]
    pub fn _90pcnt(self) -> &'a mut crate::W<REG> {
        self.variant(PERFRINT_A::_90PCNT)
    }
    #[doc = "95% of the frame interval."]
    #[inline(always)]
    pub fn _95pcnt(self) -> &'a mut crate::W<REG> {
        self.variant(PERFRINT_A::_95PCNT)
    }
}
#[doc = "Field `ERRATICINTMSK` reader - "]
pub type ERRATICINTMSK_R = crate::BitReader;
#[doc = "Field `ERRATICINTMSK` writer - "]
pub type ERRATICINTMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESVALID` reader - Resume Validation Period"]
pub type RESVALID_R = crate::FieldReader;
#[doc = "Field `RESVALID` writer - Resume Validation Period"]
pub type RESVALID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:1 - Device Speed"]
    #[inline(always)]
    pub fn devspd(&self) -> DEVSPD_R {
        DEVSPD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Non-Zero-Length Status OUT Handshake"]
    #[inline(always)]
    pub fn nzstsouthshk(&self) -> NZSTSOUTHSHK_R {
        NZSTSOUTHSHK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable 32 KHz Suspend mode"]
    #[inline(always)]
    pub fn ena32khzsusp(&self) -> ENA32KHZSUSP_R {
        ENA32KHZSUSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:10 - Device Address"]
    #[inline(always)]
    pub fn devaddr(&self) -> DEVADDR_R {
        DEVADDR_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval"]
    #[inline(always)]
    pub fn perfrint(&self) -> PERFRINT_R {
        PERFRINT_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn erraticintmsk(&self) -> ERRATICINTMSK_R {
        ERRATICINTMSK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 26:31 - Resume Validation Period"]
    #[inline(always)]
    pub fn resvalid(&self) -> RESVALID_R {
        RESVALID_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Device Speed"]
    #[inline(always)]
    #[must_use]
    pub fn devspd(&mut self) -> DEVSPD_W<DCFG_SPEC, 0> {
        DEVSPD_W::new(self)
    }
    #[doc = "Bit 2 - Non-Zero-Length Status OUT Handshake"]
    #[inline(always)]
    #[must_use]
    pub fn nzstsouthshk(&mut self) -> NZSTSOUTHSHK_W<DCFG_SPEC, 2> {
        NZSTSOUTHSHK_W::new(self)
    }
    #[doc = "Bit 3 - Enable 32 KHz Suspend mode"]
    #[inline(always)]
    #[must_use]
    pub fn ena32khzsusp(&mut self) -> ENA32KHZSUSP_W<DCFG_SPEC, 3> {
        ENA32KHZSUSP_W::new(self)
    }
    #[doc = "Bits 4:10 - Device Address"]
    #[inline(always)]
    #[must_use]
    pub fn devaddr(&mut self) -> DEVADDR_W<DCFG_SPEC, 4> {
        DEVADDR_W::new(self)
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval"]
    #[inline(always)]
    #[must_use]
    pub fn perfrint(&mut self) -> PERFRINT_W<DCFG_SPEC, 11> {
        PERFRINT_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn erraticintmsk(&mut self) -> ERRATICINTMSK_W<DCFG_SPEC, 15> {
        ERRATICINTMSK_W::new(self)
    }
    #[doc = "Bits 26:31 - Resume Validation Period"]
    #[inline(always)]
    #[must_use]
    pub fn resvalid(&mut self) -> RESVALID_W<DCFG_SPEC, 26> {
        RESVALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCFG_SPEC;
impl crate::RegisterSpec for DCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcfg::R`](R) reader structure"]
impl crate::Readable for DCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcfg::W`](W) writer structure"]
impl crate::Writable for DCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCFG to value 0x0800_0000"]
impl crate::Resettable for DCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800_0000;
}
