#[doc = "Register `STATE` reader"]
pub type R = crate::R<STATE_SPEC>;
#[doc = "Field `BUSY` reader - Bus Busy"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `MASTER` reader - Master"]
pub type MASTER_R = crate::BitReader;
#[doc = "Field `TRANSMITTER` reader - Transmitter"]
pub type TRANSMITTER_R = crate::BitReader;
#[doc = "Field `NACKED` reader - Nack Received"]
pub type NACKED_R = crate::BitReader;
#[doc = "Field `BUSHOLD` reader - Bus Held"]
pub type BUSHOLD_R = crate::BitReader;
#[doc = "Field `STATE` reader - Transmission State"]
pub type STATE_R = crate::FieldReader<STATE_A>;
#[doc = "Transmission State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STATE_A {
    #[doc = "0: No transmission is being performed."]
    IDLE = 0,
    #[doc = "1: Waiting for idle. Will send a start condition as soon as the bus is idle."]
    WAIT = 1,
    #[doc = "2: Start transmitted or received"]
    START = 2,
    #[doc = "3: Address transmitted or received"]
    ADDR = 3,
    #[doc = "4: Address ack/nack transmitted or received"]
    ADDRACK = 4,
    #[doc = "5: Data transmitted or received"]
    DATA = 5,
    #[doc = "6: Data ack/nack transmitted or received"]
    DATAACK = 6,
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
            1 => Some(STATE_A::WAIT),
            2 => Some(STATE_A::START),
            3 => Some(STATE_A::ADDR),
            4 => Some(STATE_A::ADDRACK),
            5 => Some(STATE_A::DATA),
            6 => Some(STATE_A::DATAACK),
            _ => None,
        }
    }
    #[doc = "No transmission is being performed."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == STATE_A::IDLE
    }
    #[doc = "Waiting for idle. Will send a start condition as soon as the bus is idle."]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == STATE_A::WAIT
    }
    #[doc = "Start transmitted or received"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == STATE_A::START
    }
    #[doc = "Address transmitted or received"]
    #[inline(always)]
    pub fn is_addr(&self) -> bool {
        *self == STATE_A::ADDR
    }
    #[doc = "Address ack/nack transmitted or received"]
    #[inline(always)]
    pub fn is_addrack(&self) -> bool {
        *self == STATE_A::ADDRACK
    }
    #[doc = "Data transmitted or received"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == STATE_A::DATA
    }
    #[doc = "Data ack/nack transmitted or received"]
    #[inline(always)]
    pub fn is_dataack(&self) -> bool {
        *self == STATE_A::DATAACK
    }
}
impl R {
    #[doc = "Bit 0 - Bus Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitter"]
    #[inline(always)]
    pub fn transmitter(&self) -> TRANSMITTER_R {
        TRANSMITTER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Nack Received"]
    #[inline(always)]
    pub fn nacked(&self) -> NACKED_R {
        NACKED_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bus Held"]
    #[inline(always)]
    pub fn bushold(&self) -> BUSHOLD_R {
        BUSHOLD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Transmission State"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 5) & 7) as u8)
    }
}
#[doc = "State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATE_SPEC;
impl crate::RegisterSpec for STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state::R`](R) reader structure"]
impl crate::Readable for STATE_SPEC {}
#[doc = "`reset()` method sets STATE to value 0x01"]
impl crate::Resettable for STATE_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
