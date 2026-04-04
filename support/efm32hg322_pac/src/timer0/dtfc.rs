#[doc = "Register `DTFC` reader"]
pub type R = crate::R<DtfcSpec>;
#[doc = "Register `DTFC` writer"]
pub type W = crate::W<DtfcSpec>;
#[doc = "DTI PRS Fault Source 0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dtprs0fsel {
    #[doc = "0: PRS Channel 0 selected as fault source 0"]
    Prsch0 = 0,
    #[doc = "1: PRS Channel 1 selected as fault source 0"]
    Prsch1 = 1,
    #[doc = "2: PRS Channel 2 selected as fault source 0"]
    Prsch2 = 2,
    #[doc = "3: PRS Channel 3 selected as fault source 0"]
    Prsch3 = 3,
    #[doc = "4: PRS Channel 4 selected as fault source 0"]
    Prsch4 = 4,
    #[doc = "5: PRS Channel 5 selected as fault source 0"]
    Prsch5 = 5,
    #[doc = "6: PRS Channel 6 selected as fault source 0"]
    Prsch6 = 6,
    #[doc = "7: PRS Channel 7 selected as fault source 0"]
    Prsch7 = 7,
}
impl From<Dtprs0fsel> for u8 {
    #[inline(always)]
    fn from(variant: Dtprs0fsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dtprs0fsel {
    type Ux = u8;
}
impl crate::IsEnum for Dtprs0fsel {}
#[doc = "Field `DTPRS0FSEL` reader - DTI PRS Fault Source 0 Select"]
pub type Dtprs0fselR = crate::FieldReader<Dtprs0fsel>;
impl Dtprs0fselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtprs0fsel {
        match self.bits {
            0 => Dtprs0fsel::Prsch0,
            1 => Dtprs0fsel::Prsch1,
            2 => Dtprs0fsel::Prsch2,
            3 => Dtprs0fsel::Prsch3,
            4 => Dtprs0fsel::Prsch4,
            5 => Dtprs0fsel::Prsch5,
            6 => Dtprs0fsel::Prsch6,
            7 => Dtprs0fsel::Prsch7,
            _ => unreachable!(),
        }
    }
    #[doc = "PRS Channel 0 selected as fault source 0"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Dtprs0fsel::Prsch0
    }
    #[doc = "PRS Channel 1 selected as fault source 0"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Dtprs0fsel::Prsch1
    }
    #[doc = "PRS Channel 2 selected as fault source 0"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Dtprs0fsel::Prsch2
    }
    #[doc = "PRS Channel 3 selected as fault source 0"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Dtprs0fsel::Prsch3
    }
    #[doc = "PRS Channel 4 selected as fault source 0"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Dtprs0fsel::Prsch4
    }
    #[doc = "PRS Channel 5 selected as fault source 0"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Dtprs0fsel::Prsch5
    }
    #[doc = "PRS Channel 6 selected as fault source 0"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == Dtprs0fsel::Prsch6
    }
    #[doc = "PRS Channel 7 selected as fault source 0"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == Dtprs0fsel::Prsch7
    }
}
#[doc = "Field `DTPRS0FSEL` writer - DTI PRS Fault Source 0 Select"]
pub type Dtprs0fselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dtprs0fsel, crate::Safe>;
impl<'a, REG> Dtprs0fselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as fault source 0"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprs0fsel::Prsch0)
    }
    #[doc = "PRS Channel 1 selected as fault source 0"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprs0fsel::Prsch1)
    }
    #[doc = "PRS Channel 2 selected as fault source 0"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprs0fsel::Prsch2)
    }
    #[doc = "PRS Channel 3 selected as fault source 0"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprs0fsel::Prsch3)
    }
    #[doc = "PRS Channel 4 selected as fault source 0"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprs0fsel::Prsch4)
    }
    #[doc = "PRS Channel 5 selected as fault source 0"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprs0fsel::Prsch5)
    }
    #[doc = "PRS Channel 6 selected as fault source 0"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprs0fsel::Prsch6)
    }
    #[doc = "PRS Channel 7 selected as fault source 0"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprs0fsel::Prsch7)
    }
}
#[doc = "DTI PRS Fault Source 1 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dtprs1fsel {
    #[doc = "0: PRS Channel 0 selected as fault source 1"]
    Prsch0 = 0,
    #[doc = "1: PRS Channel 1 selected as fault source 1"]
    Prsch1 = 1,
    #[doc = "2: PRS Channel 2 selected as fault source 1"]
    Prsch2 = 2,
    #[doc = "3: PRS Channel 3 selected as fault source 1"]
    Prsch3 = 3,
    #[doc = "4: PRS Channel 4 selected as fault source 1"]
    Prsch4 = 4,
    #[doc = "5: PRS Channel 5 selected as fault source 1"]
    Prsch5 = 5,
    #[doc = "6: PRS Channel 6 selected as fault source 1"]
    Prsch6 = 6,
    #[doc = "7: PRS Channel 7 selected as fault source 1"]
    Prsch7 = 7,
}
impl From<Dtprs1fsel> for u8 {
    #[inline(always)]
    fn from(variant: Dtprs1fsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dtprs1fsel {
    type Ux = u8;
}
impl crate::IsEnum for Dtprs1fsel {}
#[doc = "Field `DTPRS1FSEL` reader - DTI PRS Fault Source 1 Select"]
pub type Dtprs1fselR = crate::FieldReader<Dtprs1fsel>;
impl Dtprs1fselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtprs1fsel {
        match self.bits {
            0 => Dtprs1fsel::Prsch0,
            1 => Dtprs1fsel::Prsch1,
            2 => Dtprs1fsel::Prsch2,
            3 => Dtprs1fsel::Prsch3,
            4 => Dtprs1fsel::Prsch4,
            5 => Dtprs1fsel::Prsch5,
            6 => Dtprs1fsel::Prsch6,
            7 => Dtprs1fsel::Prsch7,
            _ => unreachable!(),
        }
    }
    #[doc = "PRS Channel 0 selected as fault source 1"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == Dtprs1fsel::Prsch0
    }
    #[doc = "PRS Channel 1 selected as fault source 1"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == Dtprs1fsel::Prsch1
    }
    #[doc = "PRS Channel 2 selected as fault source 1"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == Dtprs1fsel::Prsch2
    }
    #[doc = "PRS Channel 3 selected as fault source 1"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == Dtprs1fsel::Prsch3
    }
    #[doc = "PRS Channel 4 selected as fault source 1"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == Dtprs1fsel::Prsch4
    }
    #[doc = "PRS Channel 5 selected as fault source 1"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == Dtprs1fsel::Prsch5
    }
    #[doc = "PRS Channel 6 selected as fault source 1"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == Dtprs1fsel::Prsch6
    }
    #[doc = "PRS Channel 7 selected as fault source 1"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == Dtprs1fsel::Prsch7
    }
}
#[doc = "Field `DTPRS1FSEL` writer - DTI PRS Fault Source 1 Select"]
pub type Dtprs1fselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dtprs1fsel, crate::Safe>;
impl<'a, REG> Dtprs1fselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprs1fsel::Prsch0)
    }
    #[doc = "PRS Channel 1 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprs1fsel::Prsch1)
    }
    #[doc = "PRS Channel 2 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprs1fsel::Prsch2)
    }
    #[doc = "PRS Channel 3 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprs1fsel::Prsch3)
    }
    #[doc = "PRS Channel 4 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprs1fsel::Prsch4)
    }
    #[doc = "PRS Channel 5 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprs1fsel::Prsch5)
    }
    #[doc = "PRS Channel 6 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprs1fsel::Prsch6)
    }
    #[doc = "PRS Channel 7 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(Dtprs1fsel::Prsch7)
    }
}
#[doc = "DTI Fault Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dtfa {
    #[doc = "0: No action on fault"]
    None = 0,
    #[doc = "1: Set outputs inactive"]
    Inactive = 1,
    #[doc = "2: Clear outputs"]
    Clear = 2,
    #[doc = "3: Tristate outputs"]
    Tristate = 3,
}
impl From<Dtfa> for u8 {
    #[inline(always)]
    fn from(variant: Dtfa) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dtfa {
    type Ux = u8;
}
impl crate::IsEnum for Dtfa {}
#[doc = "Field `DTFA` reader - DTI Fault Action"]
pub type DtfaR = crate::FieldReader<Dtfa>;
impl DtfaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtfa {
        match self.bits {
            0 => Dtfa::None,
            1 => Dtfa::Inactive,
            2 => Dtfa::Clear,
            3 => Dtfa::Tristate,
            _ => unreachable!(),
        }
    }
    #[doc = "No action on fault"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Dtfa::None
    }
    #[doc = "Set outputs inactive"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Dtfa::Inactive
    }
    #[doc = "Clear outputs"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Dtfa::Clear
    }
    #[doc = "Tristate outputs"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == Dtfa::Tristate
    }
}
#[doc = "Field `DTFA` writer - DTI Fault Action"]
pub type DtfaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dtfa, crate::Safe>;
impl<'a, REG> DtfaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action on fault"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Dtfa::None)
    }
    #[doc = "Set outputs inactive"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Dtfa::Inactive)
    }
    #[doc = "Clear outputs"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Dtfa::Clear)
    }
    #[doc = "Tristate outputs"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut crate::W<REG> {
        self.variant(Dtfa::Tristate)
    }
}
#[doc = "Field `DTPRS0FEN` reader - DTI PRS 0 Fault Enable"]
pub type Dtprs0fenR = crate::BitReader;
#[doc = "Field `DTPRS0FEN` writer - DTI PRS 0 Fault Enable"]
pub type Dtprs0fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTPRS1FEN` reader - DTI PRS 1 Fault Enable"]
pub type Dtprs1fenR = crate::BitReader;
#[doc = "Field `DTPRS1FEN` writer - DTI PRS 1 Fault Enable"]
pub type Dtprs1fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTDBGFEN` reader - DTI Debugger Fault Enable"]
pub type DtdbgfenR = crate::BitReader;
#[doc = "Field `DTDBGFEN` writer - DTI Debugger Fault Enable"]
pub type DtdbgfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTLOCKUPFEN` reader - DTI Lockup Fault Enable"]
pub type DtlockupfenR = crate::BitReader;
#[doc = "Field `DTLOCKUPFEN` writer - DTI Lockup Fault Enable"]
pub type DtlockupfenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - DTI PRS Fault Source 0 Select"]
    #[inline(always)]
    pub fn dtprs0fsel(&self) -> Dtprs0fselR {
        Dtprs0fselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - DTI PRS Fault Source 1 Select"]
    #[inline(always)]
    pub fn dtprs1fsel(&self) -> Dtprs1fselR {
        Dtprs1fselR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:17 - DTI Fault Action"]
    #[inline(always)]
    pub fn dtfa(&self) -> DtfaR {
        DtfaR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - DTI PRS 0 Fault Enable"]
    #[inline(always)]
    pub fn dtprs0fen(&self) -> Dtprs0fenR {
        Dtprs0fenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DTI PRS 1 Fault Enable"]
    #[inline(always)]
    pub fn dtprs1fen(&self) -> Dtprs1fenR {
        Dtprs1fenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DTI Debugger Fault Enable"]
    #[inline(always)]
    pub fn dtdbgfen(&self) -> DtdbgfenR {
        DtdbgfenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DTI Lockup Fault Enable"]
    #[inline(always)]
    pub fn dtlockupfen(&self) -> DtlockupfenR {
        DtlockupfenR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - DTI PRS Fault Source 0 Select"]
    #[inline(always)]
    pub fn dtprs0fsel(&mut self) -> Dtprs0fselW<'_, DtfcSpec> {
        Dtprs0fselW::new(self, 0)
    }
    #[doc = "Bits 8:10 - DTI PRS Fault Source 1 Select"]
    #[inline(always)]
    pub fn dtprs1fsel(&mut self) -> Dtprs1fselW<'_, DtfcSpec> {
        Dtprs1fselW::new(self, 8)
    }
    #[doc = "Bits 16:17 - DTI Fault Action"]
    #[inline(always)]
    pub fn dtfa(&mut self) -> DtfaW<'_, DtfcSpec> {
        DtfaW::new(self, 16)
    }
    #[doc = "Bit 24 - DTI PRS 0 Fault Enable"]
    #[inline(always)]
    pub fn dtprs0fen(&mut self) -> Dtprs0fenW<'_, DtfcSpec> {
        Dtprs0fenW::new(self, 24)
    }
    #[doc = "Bit 25 - DTI PRS 1 Fault Enable"]
    #[inline(always)]
    pub fn dtprs1fen(&mut self) -> Dtprs1fenW<'_, DtfcSpec> {
        Dtprs1fenW::new(self, 25)
    }
    #[doc = "Bit 26 - DTI Debugger Fault Enable"]
    #[inline(always)]
    pub fn dtdbgfen(&mut self) -> DtdbgfenW<'_, DtfcSpec> {
        DtdbgfenW::new(self, 26)
    }
    #[doc = "Bit 27 - DTI Lockup Fault Enable"]
    #[inline(always)]
    pub fn dtlockupfen(&mut self) -> DtlockupfenW<'_, DtfcSpec> {
        DtlockupfenW::new(self, 27)
    }
}
#[doc = "DTI Fault Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtfcSpec;
impl crate::RegisterSpec for DtfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtfc::R`](R) reader structure"]
impl crate::Readable for DtfcSpec {}
#[doc = "`write(|w| ..)` method takes [`dtfc::W`](W) writer structure"]
impl crate::Writable for DtfcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTFC to value 0"]
impl crate::Resettable for DtfcSpec {}
