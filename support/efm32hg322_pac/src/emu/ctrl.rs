#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `EMVREG` reader - Energy Mode Voltage Regulator Control"]
pub type EMVREG_R = crate::BitReader;
#[doc = "Field `EMVREG` writer - Energy Mode Voltage Regulator Control"]
pub type EMVREG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM2BLOCK` reader - Energy Mode 2 Block"]
pub type EM2BLOCK_R = crate::BitReader;
#[doc = "Field `EM2BLOCK` writer - Energy Mode 2 Block"]
pub type EM2BLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM4CTRL` reader - Energy Mode 4 Control"]
pub type EM4CTRL_R = crate::FieldReader;
#[doc = "Field `EM4CTRL` writer - Energy Mode 4 Control"]
pub type EM4CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Energy Mode Voltage Regulator Control"]
    #[inline(always)]
    pub fn emvreg(&self) -> EMVREG_R {
        EMVREG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Energy Mode 2 Block"]
    #[inline(always)]
    pub fn em2block(&self) -> EM2BLOCK_R {
        EM2BLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Energy Mode 4 Control"]
    #[inline(always)]
    pub fn em4ctrl(&self) -> EM4CTRL_R {
        EM4CTRL_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Energy Mode Voltage Regulator Control"]
    #[inline(always)]
    #[must_use]
    pub fn emvreg(&mut self) -> EMVREG_W<CTRL_SPEC> {
        EMVREG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Energy Mode 2 Block"]
    #[inline(always)]
    #[must_use]
    pub fn em2block(&mut self) -> EM2BLOCK_W<CTRL_SPEC> {
        EM2BLOCK_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Energy Mode 4 Control"]
    #[inline(always)]
    #[must_use]
    pub fn em4ctrl(&mut self) -> EM4CTRL_W<CTRL_SPEC> {
        EM4CTRL_W::new(self, 2)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
