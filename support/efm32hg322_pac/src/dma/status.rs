#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `EN` reader - DMA Enable Status"]
pub type EnR = crate::BitReader;
#[doc = "Control Current State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum State {
    #[doc = "0: Idle"]
    Idle = 0,
    #[doc = "1: Reading channel controller data"]
    Rdchctrldata = 1,
    #[doc = "2: Reading source data end pointer"]
    Rdsrcendptr = 2,
    #[doc = "3: Reading destination data end pointer"]
    Rddstendptr = 3,
    #[doc = "4: Reading source data"]
    Rdsrcdata = 4,
    #[doc = "5: Writing destination data"]
    Wrdstdata = 5,
    #[doc = "6: Waiting for DMA request to clear"]
    Waitreqclr = 6,
    #[doc = "7: Writing channel controller data"]
    Wrchctrldata = 7,
    #[doc = "8: Stalled"]
    Stalled = 8,
    #[doc = "9: Done"]
    Done = 9,
    #[doc = "10: Peripheral scatter-gather transition"]
    Perscattrans = 10,
}
impl From<State> for u8 {
    #[inline(always)]
    fn from(variant: State) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for State {
    type Ux = u8;
}
impl crate::IsEnum for State {}
#[doc = "Field `STATE` reader - Control Current State"]
pub type StateR = crate::FieldReader<State>;
impl StateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<State> {
        match self.bits {
            0 => Some(State::Idle),
            1 => Some(State::Rdchctrldata),
            2 => Some(State::Rdsrcendptr),
            3 => Some(State::Rddstendptr),
            4 => Some(State::Rdsrcdata),
            5 => Some(State::Wrdstdata),
            6 => Some(State::Waitreqclr),
            7 => Some(State::Wrchctrldata),
            8 => Some(State::Stalled),
            9 => Some(State::Done),
            10 => Some(State::Perscattrans),
            _ => None,
        }
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == State::Idle
    }
    #[doc = "Reading channel controller data"]
    #[inline(always)]
    pub fn is_rdchctrldata(&self) -> bool {
        *self == State::Rdchctrldata
    }
    #[doc = "Reading source data end pointer"]
    #[inline(always)]
    pub fn is_rdsrcendptr(&self) -> bool {
        *self == State::Rdsrcendptr
    }
    #[doc = "Reading destination data end pointer"]
    #[inline(always)]
    pub fn is_rddstendptr(&self) -> bool {
        *self == State::Rddstendptr
    }
    #[doc = "Reading source data"]
    #[inline(always)]
    pub fn is_rdsrcdata(&self) -> bool {
        *self == State::Rdsrcdata
    }
    #[doc = "Writing destination data"]
    #[inline(always)]
    pub fn is_wrdstdata(&self) -> bool {
        *self == State::Wrdstdata
    }
    #[doc = "Waiting for DMA request to clear"]
    #[inline(always)]
    pub fn is_waitreqclr(&self) -> bool {
        *self == State::Waitreqclr
    }
    #[doc = "Writing channel controller data"]
    #[inline(always)]
    pub fn is_wrchctrldata(&self) -> bool {
        *self == State::Wrchctrldata
    }
    #[doc = "Stalled"]
    #[inline(always)]
    pub fn is_stalled(&self) -> bool {
        *self == State::Stalled
    }
    #[doc = "Done"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == State::Done
    }
    #[doc = "Peripheral scatter-gather transition"]
    #[inline(always)]
    pub fn is_perscattrans(&self) -> bool {
        *self == State::Perscattrans
    }
}
#[doc = "Field `CHNUM` reader - Channel Number"]
pub type ChnumR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - DMA Enable Status"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Control Current State"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Channel Number"]
    #[inline(always)]
    pub fn chnum(&self) -> ChnumR {
        ChnumR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[doc = "DMA Status Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0x1005_0000"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x1005_0000;
}
