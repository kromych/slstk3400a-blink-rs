#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `EMVREG` reader - Energy Mode Voltage Regulator Control"]
pub type EmvregR = crate::BitReader;
#[doc = "Field `EMVREG` writer - Energy Mode Voltage Regulator Control"]
pub type EmvregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM2BLOCK` reader - Energy Mode 2 Block"]
pub type Em2blockR = crate::BitReader;
#[doc = "Field `EM2BLOCK` writer - Energy Mode 2 Block"]
pub type Em2blockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM4CTRL` reader - Energy Mode 4 Control"]
pub type Em4ctrlR = crate::FieldReader;
#[doc = "Field `EM4CTRL` writer - Energy Mode 4 Control"]
pub type Em4ctrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Energy Mode Voltage Regulator Control"]
    #[inline(always)]
    pub fn emvreg(&self) -> EmvregR {
        EmvregR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Energy Mode 2 Block"]
    #[inline(always)]
    pub fn em2block(&self) -> Em2blockR {
        Em2blockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Energy Mode 4 Control"]
    #[inline(always)]
    pub fn em4ctrl(&self) -> Em4ctrlR {
        Em4ctrlR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Energy Mode Voltage Regulator Control"]
    #[inline(always)]
    pub fn emvreg(&mut self) -> EmvregW<'_, CtrlSpec> {
        EmvregW::new(self, 0)
    }
    #[doc = "Bit 1 - Energy Mode 2 Block"]
    #[inline(always)]
    pub fn em2block(&mut self) -> Em2blockW<'_, CtrlSpec> {
        Em2blockW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Energy Mode 4 Control"]
    #[inline(always)]
    pub fn em4ctrl(&mut self) -> Em4ctrlW<'_, CtrlSpec> {
        Em4ctrlW::new(self, 2)
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
