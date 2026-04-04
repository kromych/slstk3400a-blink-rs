#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `EN` reader - I2C Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - I2C Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAVE` reader - Addressable as Slave"]
pub type SlaveR = crate::BitReader;
#[doc = "Field `SLAVE` writer - Addressable as Slave"]
pub type SlaveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOACK` reader - Automatic Acknowledge"]
pub type AutoackR = crate::BitReader;
#[doc = "Field `AUTOACK` writer - Automatic Acknowledge"]
pub type AutoackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOSE` reader - Automatic STOP when Empty"]
pub type AutoseR = crate::BitReader;
#[doc = "Field `AUTOSE` writer - Automatic STOP when Empty"]
pub type AutoseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOSN` reader - Automatic STOP on NACK"]
pub type AutosnR = crate::BitReader;
#[doc = "Field `AUTOSN` writer - Automatic STOP on NACK"]
pub type AutosnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBDIS` reader - Arbitration Disable"]
pub type ArbdisR = crate::BitReader;
#[doc = "Field `ARBDIS` writer - Arbitration Disable"]
pub type ArbdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCAMEN` reader - General Call Address Match Enable"]
pub type GcamenR = crate::BitReader;
#[doc = "Field `GCAMEN` writer - General Call Address Match Enable"]
pub type GcamenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Clock Low High Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clhr {
    #[doc = "0: The ratio between low period and high period counters (Nlow:Nhigh) is 4:4"]
    Standard = 0,
    #[doc = "1: The ratio between low period and high period counters (Nlow:Nhigh) is 6:3"]
    Asymmetric = 1,
    #[doc = "2: The ratio between low period and high period counters (Nlow:Nhigh) is 11:6"]
    Fast = 2,
}
impl From<Clhr> for u8 {
    #[inline(always)]
    fn from(variant: Clhr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clhr {
    type Ux = u8;
}
impl crate::IsEnum for Clhr {}
#[doc = "Field `CLHR` reader - Clock Low High Ratio"]
pub type ClhrR = crate::FieldReader<Clhr>;
impl ClhrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clhr> {
        match self.bits {
            0 => Some(Clhr::Standard),
            1 => Some(Clhr::Asymmetric),
            2 => Some(Clhr::Fast),
            _ => None,
        }
    }
    #[doc = "The ratio between low period and high period counters (Nlow:Nhigh) is 4:4"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == Clhr::Standard
    }
    #[doc = "The ratio between low period and high period counters (Nlow:Nhigh) is 6:3"]
    #[inline(always)]
    pub fn is_asymmetric(&self) -> bool {
        *self == Clhr::Asymmetric
    }
    #[doc = "The ratio between low period and high period counters (Nlow:Nhigh) is 11:6"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == Clhr::Fast
    }
}
#[doc = "Field `CLHR` writer - Clock Low High Ratio"]
pub type ClhrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clhr>;
impl<'a, REG> ClhrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The ratio between low period and high period counters (Nlow:Nhigh) is 4:4"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(Clhr::Standard)
    }
    #[doc = "The ratio between low period and high period counters (Nlow:Nhigh) is 6:3"]
    #[inline(always)]
    pub fn asymmetric(self) -> &'a mut crate::W<REG> {
        self.variant(Clhr::Asymmetric)
    }
    #[doc = "The ratio between low period and high period counters (Nlow:Nhigh) is 11:6"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut crate::W<REG> {
        self.variant(Clhr::Fast)
    }
}
#[doc = "Bus Idle Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bito {
    #[doc = "0: Timeout disabled"]
    Off = 0,
    #[doc = "1: Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    _40pcc = 1,
    #[doc = "2: Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    _80pcc = 2,
    #[doc = "3: Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    _160pcc = 3,
}
impl From<Bito> for u8 {
    #[inline(always)]
    fn from(variant: Bito) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bito {
    type Ux = u8;
}
impl crate::IsEnum for Bito {}
#[doc = "Field `BITO` reader - Bus Idle Timeout"]
pub type BitoR = crate::FieldReader<Bito>;
impl BitoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bito {
        match self.bits {
            0 => Bito::Off,
            1 => Bito::_40pcc,
            2 => Bito::_80pcc,
            3 => Bito::_160pcc,
            _ => unreachable!(),
        }
    }
    #[doc = "Timeout disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Bito::Off
    }
    #[doc = "Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    #[inline(always)]
    pub fn is_40pcc(&self) -> bool {
        *self == Bito::_40pcc
    }
    #[doc = "Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    #[inline(always)]
    pub fn is_80pcc(&self) -> bool {
        *self == Bito::_80pcc
    }
    #[doc = "Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    #[inline(always)]
    pub fn is_160pcc(&self) -> bool {
        *self == Bito::_160pcc
    }
}
#[doc = "Field `BITO` writer - Bus Idle Timeout"]
pub type BitoW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bito, crate::Safe>;
impl<'a, REG> BitoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timeout disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Bito::Off)
    }
    #[doc = "Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    #[inline(always)]
    pub fn _40pcc(self) -> &'a mut crate::W<REG> {
        self.variant(Bito::_40pcc)
    }
    #[doc = "Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    #[inline(always)]
    pub fn _80pcc(self) -> &'a mut crate::W<REG> {
        self.variant(Bito::_80pcc)
    }
    #[doc = "Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    #[inline(always)]
    pub fn _160pcc(self) -> &'a mut crate::W<REG> {
        self.variant(Bito::_160pcc)
    }
}
#[doc = "Field `GIBITO` reader - Go Idle on Bus Idle Timeout"]
pub type GibitoR = crate::BitReader;
#[doc = "Field `GIBITO` writer - Go Idle on Bus Idle Timeout"]
pub type GibitoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Clock Low Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clto {
    #[doc = "0: Timeout disabled"]
    Off = 0,
    #[doc = "1: Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    _40pcc = 1,
    #[doc = "2: Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    _80pcc = 2,
    #[doc = "3: Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    _160pcc = 3,
    #[doc = "4: Timeout after 320 prescaled clock cycles. In standard mode at 100 kHz, this results in a 400us timeout."]
    _320ppc = 4,
    #[doc = "5: Timeout after 1024 prescaled clock cycles. In standard mode at 100 kHz, this results in a 1280us timeout."]
    _1024ppc = 5,
}
impl From<Clto> for u8 {
    #[inline(always)]
    fn from(variant: Clto) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clto {
    type Ux = u8;
}
impl crate::IsEnum for Clto {}
#[doc = "Field `CLTO` reader - Clock Low Timeout"]
pub type CltoR = crate::FieldReader<Clto>;
impl CltoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clto> {
        match self.bits {
            0 => Some(Clto::Off),
            1 => Some(Clto::_40pcc),
            2 => Some(Clto::_80pcc),
            3 => Some(Clto::_160pcc),
            4 => Some(Clto::_320ppc),
            5 => Some(Clto::_1024ppc),
            _ => None,
        }
    }
    #[doc = "Timeout disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Clto::Off
    }
    #[doc = "Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    #[inline(always)]
    pub fn is_40pcc(&self) -> bool {
        *self == Clto::_40pcc
    }
    #[doc = "Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    #[inline(always)]
    pub fn is_80pcc(&self) -> bool {
        *self == Clto::_80pcc
    }
    #[doc = "Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    #[inline(always)]
    pub fn is_160pcc(&self) -> bool {
        *self == Clto::_160pcc
    }
    #[doc = "Timeout after 320 prescaled clock cycles. In standard mode at 100 kHz, this results in a 400us timeout."]
    #[inline(always)]
    pub fn is_320ppc(&self) -> bool {
        *self == Clto::_320ppc
    }
    #[doc = "Timeout after 1024 prescaled clock cycles. In standard mode at 100 kHz, this results in a 1280us timeout."]
    #[inline(always)]
    pub fn is_1024ppc(&self) -> bool {
        *self == Clto::_1024ppc
    }
}
#[doc = "Field `CLTO` writer - Clock Low Timeout"]
pub type CltoW<'a, REG> = crate::FieldWriter<'a, REG, 3, Clto>;
impl<'a, REG> CltoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timeout disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Clto::Off)
    }
    #[doc = "Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    #[inline(always)]
    pub fn _40pcc(self) -> &'a mut crate::W<REG> {
        self.variant(Clto::_40pcc)
    }
    #[doc = "Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    #[inline(always)]
    pub fn _80pcc(self) -> &'a mut crate::W<REG> {
        self.variant(Clto::_80pcc)
    }
    #[doc = "Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    #[inline(always)]
    pub fn _160pcc(self) -> &'a mut crate::W<REG> {
        self.variant(Clto::_160pcc)
    }
    #[doc = "Timeout after 320 prescaled clock cycles. In standard mode at 100 kHz, this results in a 400us timeout."]
    #[inline(always)]
    pub fn _320ppc(self) -> &'a mut crate::W<REG> {
        self.variant(Clto::_320ppc)
    }
    #[doc = "Timeout after 1024 prescaled clock cycles. In standard mode at 100 kHz, this results in a 1280us timeout."]
    #[inline(always)]
    pub fn _1024ppc(self) -> &'a mut crate::W<REG> {
        self.variant(Clto::_1024ppc)
    }
}
impl R {
    #[doc = "Bit 0 - I2C Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Addressable as Slave"]
    #[inline(always)]
    pub fn slave(&self) -> SlaveR {
        SlaveR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Automatic Acknowledge"]
    #[inline(always)]
    pub fn autoack(&self) -> AutoackR {
        AutoackR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Automatic STOP when Empty"]
    #[inline(always)]
    pub fn autose(&self) -> AutoseR {
        AutoseR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Automatic STOP on NACK"]
    #[inline(always)]
    pub fn autosn(&self) -> AutosnR {
        AutosnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Arbitration Disable"]
    #[inline(always)]
    pub fn arbdis(&self) -> ArbdisR {
        ArbdisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - General Call Address Match Enable"]
    #[inline(always)]
    pub fn gcamen(&self) -> GcamenR {
        GcamenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clock Low High Ratio"]
    #[inline(always)]
    pub fn clhr(&self) -> ClhrR {
        ClhrR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Bus Idle Timeout"]
    #[inline(always)]
    pub fn bito(&self) -> BitoR {
        BitoR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Go Idle on Bus Idle Timeout"]
    #[inline(always)]
    pub fn gibito(&self) -> GibitoR {
        GibitoR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Clock Low Timeout"]
    #[inline(always)]
    pub fn clto(&self) -> CltoR {
        CltoR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CtrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Addressable as Slave"]
    #[inline(always)]
    pub fn slave(&mut self) -> SlaveW<'_, CtrlSpec> {
        SlaveW::new(self, 1)
    }
    #[doc = "Bit 2 - Automatic Acknowledge"]
    #[inline(always)]
    pub fn autoack(&mut self) -> AutoackW<'_, CtrlSpec> {
        AutoackW::new(self, 2)
    }
    #[doc = "Bit 3 - Automatic STOP when Empty"]
    #[inline(always)]
    pub fn autose(&mut self) -> AutoseW<'_, CtrlSpec> {
        AutoseW::new(self, 3)
    }
    #[doc = "Bit 4 - Automatic STOP on NACK"]
    #[inline(always)]
    pub fn autosn(&mut self) -> AutosnW<'_, CtrlSpec> {
        AutosnW::new(self, 4)
    }
    #[doc = "Bit 5 - Arbitration Disable"]
    #[inline(always)]
    pub fn arbdis(&mut self) -> ArbdisW<'_, CtrlSpec> {
        ArbdisW::new(self, 5)
    }
    #[doc = "Bit 6 - General Call Address Match Enable"]
    #[inline(always)]
    pub fn gcamen(&mut self) -> GcamenW<'_, CtrlSpec> {
        GcamenW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Clock Low High Ratio"]
    #[inline(always)]
    pub fn clhr(&mut self) -> ClhrW<'_, CtrlSpec> {
        ClhrW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Bus Idle Timeout"]
    #[inline(always)]
    pub fn bito(&mut self) -> BitoW<'_, CtrlSpec> {
        BitoW::new(self, 12)
    }
    #[doc = "Bit 15 - Go Idle on Bus Idle Timeout"]
    #[inline(always)]
    pub fn gibito(&mut self) -> GibitoW<'_, CtrlSpec> {
        GibitoW::new(self, 15)
    }
    #[doc = "Bits 16:18 - Clock Low Timeout"]
    #[inline(always)]
    pub fn clto(&mut self) -> CltoW<'_, CtrlSpec> {
        CltoW::new(self, 16)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
