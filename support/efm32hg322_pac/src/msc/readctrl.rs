#[doc = "Register `READCTRL` reader"]
pub type R = crate::R<ReadctrlSpec>;
#[doc = "Register `READCTRL` writer"]
pub type W = crate::W<ReadctrlSpec>;
#[doc = "Read Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Zero wait-states inserted in fetch or read transfers."]
    Ws0 = 0,
    #[doc = "1: One wait-state inserted for each fetch or read transfer. This mode is required for a core frequency above 16 MHz."]
    Ws1 = 1,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Read Mode"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::Ws0),
            1 => Some(Mode::Ws1),
            _ => None,
        }
    }
    #[doc = "Zero wait-states inserted in fetch or read transfers."]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == Mode::Ws0
    }
    #[doc = "One wait-state inserted for each fetch or read transfer. This mode is required for a core frequency above 16 MHz."]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == Mode::Ws1
    }
}
#[doc = "Field `MODE` writer - Read Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Zero wait-states inserted in fetch or read transfers."]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ws0)
    }
    #[doc = "One wait-state inserted for each fetch or read transfer. This mode is required for a core frequency above 16 MHz."]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ws1)
    }
}
#[doc = "Field `IFCDIS` reader - Internal Flash Cache Disable"]
pub type IfcdisR = crate::BitReader;
#[doc = "Field `IFCDIS` writer - Internal Flash Cache Disable"]
pub type IfcdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIDIS` reader - Automatic Invalidate Disable"]
pub type AidisR = crate::BitReader;
#[doc = "Field `AIDIS` writer - Automatic Invalidate Disable"]
pub type AidisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMCEN` reader - RAM Cache Enable"]
pub type RamcenR = crate::BitReader;
#[doc = "Field `RAMCEN` writer - RAM Cache Enable"]
pub type RamcenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Read Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Internal Flash Cache Disable"]
    #[inline(always)]
    pub fn ifcdis(&self) -> IfcdisR {
        IfcdisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Automatic Invalidate Disable"]
    #[inline(always)]
    pub fn aidis(&self) -> AidisR {
        AidisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - RAM Cache Enable"]
    #[inline(always)]
    pub fn ramcen(&self) -> RamcenR {
        RamcenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Read Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, ReadctrlSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 3 - Internal Flash Cache Disable"]
    #[inline(always)]
    pub fn ifcdis(&mut self) -> IfcdisW<'_, ReadctrlSpec> {
        IfcdisW::new(self, 3)
    }
    #[doc = "Bit 4 - Automatic Invalidate Disable"]
    #[inline(always)]
    pub fn aidis(&mut self) -> AidisW<'_, ReadctrlSpec> {
        AidisW::new(self, 4)
    }
    #[doc = "Bit 7 - RAM Cache Enable"]
    #[inline(always)]
    pub fn ramcen(&mut self) -> RamcenW<'_, ReadctrlSpec> {
        RamcenW::new(self, 7)
    }
}
#[doc = "Read Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`readctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`readctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReadctrlSpec;
impl crate::RegisterSpec for ReadctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`readctrl::R`](R) reader structure"]
impl crate::Readable for ReadctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`readctrl::W`](W) writer structure"]
impl crate::Writable for ReadctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets READCTRL to value 0x01"]
impl crate::Resettable for ReadctrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
