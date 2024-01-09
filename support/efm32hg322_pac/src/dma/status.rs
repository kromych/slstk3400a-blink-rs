#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `EN` reader - DMA Enable Status"]
pub type EN_R = crate::BitReader;
#[doc = "Field `STATE` reader - Control Current State"]
pub type STATE_R = crate::FieldReader<STATE_A>;
#[doc = "Control Current State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STATE_A {
    #[doc = "0: Idle"]
    IDLE = 0,
    #[doc = "1: Reading channel controller data"]
    RDCHCTRLDATA = 1,
    #[doc = "2: Reading source data end pointer"]
    RDSRCENDPTR = 2,
    #[doc = "3: Reading destination data end pointer"]
    RDDSTENDPTR = 3,
    #[doc = "4: Reading source data"]
    RDSRCDATA = 4,
    #[doc = "5: Writing destination data"]
    WRDSTDATA = 5,
    #[doc = "6: Waiting for DMA request to clear"]
    WAITREQCLR = 6,
    #[doc = "7: Writing channel controller data"]
    WRCHCTRLDATA = 7,
    #[doc = "8: Stalled"]
    STALLED = 8,
    #[doc = "9: Done"]
    DONE = 9,
    #[doc = "10: Peripheral scatter-gather transition"]
    PERSCATTRANS = 10,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STATE_A {
    type Ux = u8;
}
impl STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<STATE_A> {
        match self.bits {
            0 => Some(STATE_A::IDLE),
            1 => Some(STATE_A::RDCHCTRLDATA),
            2 => Some(STATE_A::RDSRCENDPTR),
            3 => Some(STATE_A::RDDSTENDPTR),
            4 => Some(STATE_A::RDSRCDATA),
            5 => Some(STATE_A::WRDSTDATA),
            6 => Some(STATE_A::WAITREQCLR),
            7 => Some(STATE_A::WRCHCTRLDATA),
            8 => Some(STATE_A::STALLED),
            9 => Some(STATE_A::DONE),
            10 => Some(STATE_A::PERSCATTRANS),
            _ => None,
        }
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == STATE_A::IDLE
    }
    #[doc = "Reading channel controller data"]
    #[inline(always)]
    pub fn is_rdchctrldata(&self) -> bool {
        *self == STATE_A::RDCHCTRLDATA
    }
    #[doc = "Reading source data end pointer"]
    #[inline(always)]
    pub fn is_rdsrcendptr(&self) -> bool {
        *self == STATE_A::RDSRCENDPTR
    }
    #[doc = "Reading destination data end pointer"]
    #[inline(always)]
    pub fn is_rddstendptr(&self) -> bool {
        *self == STATE_A::RDDSTENDPTR
    }
    #[doc = "Reading source data"]
    #[inline(always)]
    pub fn is_rdsrcdata(&self) -> bool {
        *self == STATE_A::RDSRCDATA
    }
    #[doc = "Writing destination data"]
    #[inline(always)]
    pub fn is_wrdstdata(&self) -> bool {
        *self == STATE_A::WRDSTDATA
    }
    #[doc = "Waiting for DMA request to clear"]
    #[inline(always)]
    pub fn is_waitreqclr(&self) -> bool {
        *self == STATE_A::WAITREQCLR
    }
    #[doc = "Writing channel controller data"]
    #[inline(always)]
    pub fn is_wrchctrldata(&self) -> bool {
        *self == STATE_A::WRCHCTRLDATA
    }
    #[doc = "Stalled"]
    #[inline(always)]
    pub fn is_stalled(&self) -> bool {
        *self == STATE_A::STALLED
    }
    #[doc = "Done"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == STATE_A::DONE
    }
    #[doc = "Peripheral scatter-gather transition"]
    #[inline(always)]
    pub fn is_perscattrans(&self) -> bool {
        *self == STATE_A::PERSCATTRANS
    }
}
#[doc = "Field `CHNUM` reader - Channel Number"]
pub type CHNUM_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - DMA Enable Status"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Control Current State"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Channel Number"]
    #[inline(always)]
    pub fn chnum(&self) -> CHNUM_R {
        CHNUM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[doc = "DMA Status Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0x1005_0000"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0x1005_0000;
}
