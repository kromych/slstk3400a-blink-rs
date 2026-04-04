#[doc = "Register `GRSTCTL` reader"]
pub type R = crate::R<GrstctlSpec>;
#[doc = "Register `GRSTCTL` writer"]
pub type W = crate::W<GrstctlSpec>;
#[doc = "Field `CSFTRST` reader - Core Soft Reset"]
pub type CsftrstR = crate::BitReader;
#[doc = "Field `CSFTRST` writer - Core Soft Reset"]
pub type CsftrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIUFSSFTRST` reader - PIU FS Dedicated Controller Soft Reset"]
pub type PiufssftrstR = crate::BitReader;
#[doc = "Field `PIUFSSFTRST` writer - PIU FS Dedicated Controller Soft Reset"]
pub type PiufssftrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFFLSH` reader - RxFIFO Flush"]
pub type RxfflshR = crate::BitReader;
#[doc = "Field `RXFFLSH` writer - RxFIFO Flush"]
pub type RxfflshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFFLSH` reader - TxFIFO Flush"]
pub type TxfflshR = crate::BitReader;
#[doc = "Field `TXFFLSH` writer - TxFIFO Flush"]
pub type TxfflshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "TxFIFO Number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txfnum {
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
    Fall = 16,
}
impl From<Txfnum> for u8 {
    #[inline(always)]
    fn from(variant: Txfnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txfnum {
    type Ux = u8;
}
impl crate::IsEnum for Txfnum {}
#[doc = "Field `TXFNUM` reader - TxFIFO Number"]
pub type TxfnumR = crate::FieldReader<Txfnum>;
impl TxfnumR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txfnum> {
        match self.bits {
            0 => Some(Txfnum::F0),
            1 => Some(Txfnum::F1),
            2 => Some(Txfnum::F2),
            3 => Some(Txfnum::F3),
            4 => Some(Txfnum::F4),
            5 => Some(Txfnum::F5),
            6 => Some(Txfnum::F6),
            16 => Some(Txfnum::Fall),
            _ => None,
        }
    }
    #[doc = "Host mode: Non-periodic TxFIFO flush. Device: Tx FIFO 0 flush"]
    #[inline(always)]
    pub fn is_f0(&self) -> bool {
        *self == Txfnum::F0
    }
    #[doc = "Host mode: Periodic TxFIFO flush. Device: TXFIFO 1 flush."]
    #[inline(always)]
    pub fn is_f1(&self) -> bool {
        *self == Txfnum::F1
    }
    #[doc = "Device mode: TXFIFO 2 flush."]
    #[inline(always)]
    pub fn is_f2(&self) -> bool {
        *self == Txfnum::F2
    }
    #[doc = "Device mode: TXFIFO 3 flush."]
    #[inline(always)]
    pub fn is_f3(&self) -> bool {
        *self == Txfnum::F3
    }
    #[doc = "Device mode: TXFIFO 4 flush."]
    #[inline(always)]
    pub fn is_f4(&self) -> bool {
        *self == Txfnum::F4
    }
    #[doc = "Device mode: TXFIFO 5 flush."]
    #[inline(always)]
    pub fn is_f5(&self) -> bool {
        *self == Txfnum::F5
    }
    #[doc = "Device mode: TXFIFO 6 flush."]
    #[inline(always)]
    pub fn is_f6(&self) -> bool {
        *self == Txfnum::F6
    }
    #[doc = "Flush all the transmit FIFOs in device or host mode."]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Txfnum::Fall
    }
}
#[doc = "Field `TXFNUM` writer - TxFIFO Number"]
pub type TxfnumW<'a, REG> = crate::FieldWriter<'a, REG, 5, Txfnum>;
impl<'a, REG> TxfnumW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Host mode: Non-periodic TxFIFO flush. Device: Tx FIFO 0 flush"]
    #[inline(always)]
    pub fn f0(self) -> &'a mut crate::W<REG> {
        self.variant(Txfnum::F0)
    }
    #[doc = "Host mode: Periodic TxFIFO flush. Device: TXFIFO 1 flush."]
    #[inline(always)]
    pub fn f1(self) -> &'a mut crate::W<REG> {
        self.variant(Txfnum::F1)
    }
    #[doc = "Device mode: TXFIFO 2 flush."]
    #[inline(always)]
    pub fn f2(self) -> &'a mut crate::W<REG> {
        self.variant(Txfnum::F2)
    }
    #[doc = "Device mode: TXFIFO 3 flush."]
    #[inline(always)]
    pub fn f3(self) -> &'a mut crate::W<REG> {
        self.variant(Txfnum::F3)
    }
    #[doc = "Device mode: TXFIFO 4 flush."]
    #[inline(always)]
    pub fn f4(self) -> &'a mut crate::W<REG> {
        self.variant(Txfnum::F4)
    }
    #[doc = "Device mode: TXFIFO 5 flush."]
    #[inline(always)]
    pub fn f5(self) -> &'a mut crate::W<REG> {
        self.variant(Txfnum::F5)
    }
    #[doc = "Device mode: TXFIFO 6 flush."]
    #[inline(always)]
    pub fn f6(self) -> &'a mut crate::W<REG> {
        self.variant(Txfnum::F6)
    }
    #[doc = "Flush all the transmit FIFOs in device or host mode."]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Txfnum::Fall)
    }
}
#[doc = "Field `DMAREQ` reader - DMA Request Signal"]
pub type DmareqR = crate::BitReader;
#[doc = "Field `AHBIDLE` reader - AHB Master Idle"]
pub type AhbidleR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Core Soft Reset"]
    #[inline(always)]
    pub fn csftrst(&self) -> CsftrstR {
        CsftrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PIU FS Dedicated Controller Soft Reset"]
    #[inline(always)]
    pub fn piufssftrst(&self) -> PiufssftrstR {
        PiufssftrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - RxFIFO Flush"]
    #[inline(always)]
    pub fn rxfflsh(&self) -> RxfflshR {
        RxfflshR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TxFIFO Flush"]
    #[inline(always)]
    pub fn txfflsh(&self) -> TxfflshR {
        TxfflshR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - TxFIFO Number"]
    #[inline(always)]
    pub fn txfnum(&self) -> TxfnumR {
        TxfnumR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - DMA Request Signal"]
    #[inline(always)]
    pub fn dmareq(&self) -> DmareqR {
        DmareqR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AHB Master Idle"]
    #[inline(always)]
    pub fn ahbidle(&self) -> AhbidleR {
        AhbidleR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core Soft Reset"]
    #[inline(always)]
    pub fn csftrst(&mut self) -> CsftrstW<'_, GrstctlSpec> {
        CsftrstW::new(self, 0)
    }
    #[doc = "Bit 1 - PIU FS Dedicated Controller Soft Reset"]
    #[inline(always)]
    pub fn piufssftrst(&mut self) -> PiufssftrstW<'_, GrstctlSpec> {
        PiufssftrstW::new(self, 1)
    }
    #[doc = "Bit 4 - RxFIFO Flush"]
    #[inline(always)]
    pub fn rxfflsh(&mut self) -> RxfflshW<'_, GrstctlSpec> {
        RxfflshW::new(self, 4)
    }
    #[doc = "Bit 5 - TxFIFO Flush"]
    #[inline(always)]
    pub fn txfflsh(&mut self) -> TxfflshW<'_, GrstctlSpec> {
        TxfflshW::new(self, 5)
    }
    #[doc = "Bits 6:10 - TxFIFO Number"]
    #[inline(always)]
    pub fn txfnum(&mut self) -> TxfnumW<'_, GrstctlSpec> {
        TxfnumW::new(self, 6)
    }
}
#[doc = "Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`grstctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grstctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrstctlSpec;
impl crate::RegisterSpec for GrstctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grstctl::R`](R) reader structure"]
impl crate::Readable for GrstctlSpec {}
#[doc = "`write(|w| ..)` method takes [`grstctl::W`](W) writer structure"]
impl crate::Writable for GrstctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRSTCTL to value 0x8000_0000"]
impl crate::Resettable for GrstctlSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
