#[doc = "Register `GRSTCTL` reader"]
pub type R = crate::R<GRSTCTL_SPEC>;
#[doc = "Register `GRSTCTL` writer"]
pub type W = crate::W<GRSTCTL_SPEC>;
#[doc = "Field `CSFTRST` reader - Core Soft Reset"]
pub type CSFTRST_R = crate::BitReader;
#[doc = "Field `CSFTRST` writer - Core Soft Reset"]
pub type CSFTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIUFSSFTRST` reader - PIU FS Dedicated Controller Soft Reset"]
pub type PIUFSSFTRST_R = crate::BitReader;
#[doc = "Field `PIUFSSFTRST` writer - PIU FS Dedicated Controller Soft Reset"]
pub type PIUFSSFTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFFLSH` reader - RxFIFO Flush"]
pub type RXFFLSH_R = crate::BitReader;
#[doc = "Field `RXFFLSH` writer - RxFIFO Flush"]
pub type RXFFLSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFFLSH` reader - TxFIFO Flush"]
pub type TXFFLSH_R = crate::BitReader;
#[doc = "Field `TXFFLSH` writer - TxFIFO Flush"]
pub type TXFFLSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFNUM` reader - TxFIFO Number"]
pub type TXFNUM_R = crate::FieldReader<TXFNUM_A>;
#[doc = "TxFIFO Number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXFNUM_A {
    #[doc = "0: Host mode: Non-periodic TxFIFO flush. Device: Tx FIFO 0 flush"]
    F0 = 0,
    #[doc = "1: Host mode: Periodic TxFIFO flush. Device: TXFIFO 1 flush."]
    F1 = 1,
    #[doc = "2: Device mode: TXFIFO 2 flush."]
    F2 = 2,
    #[doc = "3: Device mode: TXFIFO 3 flush."]
    F3 = 3,
    #[doc = "4: Device mode: TXFIFO 4 flush."]
    F4 = 4,
    #[doc = "5: Device mode: TXFIFO 5 flush."]
    F5 = 5,
    #[doc = "6: Device mode: TXFIFO 6 flush."]
    F6 = 6,
    #[doc = "16: Flush all the transmit FIFOs in device or host mode."]
    FALL = 16,
}
impl From<TXFNUM_A> for u8 {
    #[inline(always)]
    fn from(variant: TXFNUM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TXFNUM_A {
    type Ux = u8;
}
impl TXFNUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TXFNUM_A> {
        match self.bits {
            0 => Some(TXFNUM_A::F0),
            1 => Some(TXFNUM_A::F1),
            2 => Some(TXFNUM_A::F2),
            3 => Some(TXFNUM_A::F3),
            4 => Some(TXFNUM_A::F4),
            5 => Some(TXFNUM_A::F5),
            6 => Some(TXFNUM_A::F6),
            16 => Some(TXFNUM_A::FALL),
            _ => None,
        }
    }
    #[doc = "Host mode: Non-periodic TxFIFO flush. Device: Tx FIFO 0 flush"]
    #[inline(always)]
    pub fn is_f0(&self) -> bool {
        *self == TXFNUM_A::F0
    }
    #[doc = "Host mode: Periodic TxFIFO flush. Device: TXFIFO 1 flush."]
    #[inline(always)]
    pub fn is_f1(&self) -> bool {
        *self == TXFNUM_A::F1
    }
    #[doc = "Device mode: TXFIFO 2 flush."]
    #[inline(always)]
    pub fn is_f2(&self) -> bool {
        *self == TXFNUM_A::F2
    }
    #[doc = "Device mode: TXFIFO 3 flush."]
    #[inline(always)]
    pub fn is_f3(&self) -> bool {
        *self == TXFNUM_A::F3
    }
    #[doc = "Device mode: TXFIFO 4 flush."]
    #[inline(always)]
    pub fn is_f4(&self) -> bool {
        *self == TXFNUM_A::F4
    }
    #[doc = "Device mode: TXFIFO 5 flush."]
    #[inline(always)]
    pub fn is_f5(&self) -> bool {
        *self == TXFNUM_A::F5
    }
    #[doc = "Device mode: TXFIFO 6 flush."]
    #[inline(always)]
    pub fn is_f6(&self) -> bool {
        *self == TXFNUM_A::F6
    }
    #[doc = "Flush all the transmit FIFOs in device or host mode."]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == TXFNUM_A::FALL
    }
}
#[doc = "Field `TXFNUM` writer - TxFIFO Number"]
pub type TXFNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 5, TXFNUM_A>;
impl<'a, REG> TXFNUM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Host mode: Non-periodic TxFIFO flush. Device: Tx FIFO 0 flush"]
    #[inline(always)]
    pub fn f0(self) -> &'a mut crate::W<REG> {
        self.variant(TXFNUM_A::F0)
    }
    #[doc = "Host mode: Periodic TxFIFO flush. Device: TXFIFO 1 flush."]
    #[inline(always)]
    pub fn f1(self) -> &'a mut crate::W<REG> {
        self.variant(TXFNUM_A::F1)
    }
    #[doc = "Device mode: TXFIFO 2 flush."]
    #[inline(always)]
    pub fn f2(self) -> &'a mut crate::W<REG> {
        self.variant(TXFNUM_A::F2)
    }
    #[doc = "Device mode: TXFIFO 3 flush."]
    #[inline(always)]
    pub fn f3(self) -> &'a mut crate::W<REG> {
        self.variant(TXFNUM_A::F3)
    }
    #[doc = "Device mode: TXFIFO 4 flush."]
    #[inline(always)]
    pub fn f4(self) -> &'a mut crate::W<REG> {
        self.variant(TXFNUM_A::F4)
    }
    #[doc = "Device mode: TXFIFO 5 flush."]
    #[inline(always)]
    pub fn f5(self) -> &'a mut crate::W<REG> {
        self.variant(TXFNUM_A::F5)
    }
    #[doc = "Device mode: TXFIFO 6 flush."]
    #[inline(always)]
    pub fn f6(self) -> &'a mut crate::W<REG> {
        self.variant(TXFNUM_A::F6)
    }
    #[doc = "Flush all the transmit FIFOs in device or host mode."]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(TXFNUM_A::FALL)
    }
}
#[doc = "Field `DMAREQ` reader - DMA Request Signal"]
pub type DMAREQ_R = crate::BitReader;
#[doc = "Field `AHBIDLE` reader - AHB Master Idle"]
pub type AHBIDLE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Core Soft Reset"]
    #[inline(always)]
    pub fn csftrst(&self) -> CSFTRST_R {
        CSFTRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PIU FS Dedicated Controller Soft Reset"]
    #[inline(always)]
    pub fn piufssftrst(&self) -> PIUFSSFTRST_R {
        PIUFSSFTRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - RxFIFO Flush"]
    #[inline(always)]
    pub fn rxfflsh(&self) -> RXFFLSH_R {
        RXFFLSH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TxFIFO Flush"]
    #[inline(always)]
    pub fn txfflsh(&self) -> TXFFLSH_R {
        TXFFLSH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - TxFIFO Number"]
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - DMA Request Signal"]
    #[inline(always)]
    pub fn dmareq(&self) -> DMAREQ_R {
        DMAREQ_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AHB Master Idle"]
    #[inline(always)]
    pub fn ahbidle(&self) -> AHBIDLE_R {
        AHBIDLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core Soft Reset"]
    #[inline(always)]
    #[must_use]
    pub fn csftrst(&mut self) -> CSFTRST_W<GRSTCTL_SPEC> {
        CSFTRST_W::new(self, 0)
    }
    #[doc = "Bit 1 - PIU FS Dedicated Controller Soft Reset"]
    #[inline(always)]
    #[must_use]
    pub fn piufssftrst(&mut self) -> PIUFSSFTRST_W<GRSTCTL_SPEC> {
        PIUFSSFTRST_W::new(self, 1)
    }
    #[doc = "Bit 4 - RxFIFO Flush"]
    #[inline(always)]
    #[must_use]
    pub fn rxfflsh(&mut self) -> RXFFLSH_W<GRSTCTL_SPEC> {
        RXFFLSH_W::new(self, 4)
    }
    #[doc = "Bit 5 - TxFIFO Flush"]
    #[inline(always)]
    #[must_use]
    pub fn txfflsh(&mut self) -> TXFFLSH_W<GRSTCTL_SPEC> {
        TXFFLSH_W::new(self, 5)
    }
    #[doc = "Bits 6:10 - TxFIFO Number"]
    #[inline(always)]
    #[must_use]
    pub fn txfnum(&mut self) -> TXFNUM_W<GRSTCTL_SPEC> {
        TXFNUM_W::new(self, 6)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grstctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grstctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GRSTCTL_SPEC;
impl crate::RegisterSpec for GRSTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grstctl::R`](R) reader structure"]
impl crate::Readable for GRSTCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`grstctl::W`](W) writer structure"]
impl crate::Writable for GRSTCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRSTCTL to value 0x8000_0000"]
impl crate::Resettable for GRSTCTL_SPEC {
    const RESET_VALUE: u32 = 0x8000_0000;
}
