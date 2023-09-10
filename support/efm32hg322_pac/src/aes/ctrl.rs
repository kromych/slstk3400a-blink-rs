#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `DECRYPT` reader - Decryption/Encryption Mode"]
pub type DECRYPT_R = crate::BitReader;
#[doc = "Field `DECRYPT` writer - Decryption/Encryption Mode"]
pub type DECRYPT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATASTART` reader - AES_DATA Write Start"]
pub type DATASTART_R = crate::BitReader;
#[doc = "Field `DATASTART` writer - AES_DATA Write Start"]
pub type DATASTART_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XORSTART` reader - AES_XORDATA Write Start"]
pub type XORSTART_R = crate::BitReader;
#[doc = "Field `XORSTART` writer - AES_XORDATA Write Start"]
pub type XORSTART_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BYTEORDER` reader - Configure byte order in data and key registers"]
pub type BYTEORDER_R = crate::BitReader;
#[doc = "Field `BYTEORDER` writer - Configure byte order in data and key registers"]
pub type BYTEORDER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Decryption/Encryption Mode"]
    #[inline(always)]
    pub fn decrypt(&self) -> DECRYPT_R {
        DECRYPT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - AES_DATA Write Start"]
    #[inline(always)]
    pub fn datastart(&self) -> DATASTART_R {
        DATASTART_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AES_XORDATA Write Start"]
    #[inline(always)]
    pub fn xorstart(&self) -> XORSTART_R {
        XORSTART_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configure byte order in data and key registers"]
    #[inline(always)]
    pub fn byteorder(&self) -> BYTEORDER_R {
        BYTEORDER_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Decryption/Encryption Mode"]
    #[inline(always)]
    #[must_use]
    pub fn decrypt(&mut self) -> DECRYPT_W<CTRL_SPEC, 0> {
        DECRYPT_W::new(self)
    }
    #[doc = "Bit 4 - AES_DATA Write Start"]
    #[inline(always)]
    #[must_use]
    pub fn datastart(&mut self) -> DATASTART_W<CTRL_SPEC, 4> {
        DATASTART_W::new(self)
    }
    #[doc = "Bit 5 - AES_XORDATA Write Start"]
    #[inline(always)]
    #[must_use]
    pub fn xorstart(&mut self) -> XORSTART_W<CTRL_SPEC, 5> {
        XORSTART_W::new(self)
    }
    #[doc = "Bit 6 - Configure byte order in data and key registers"]
    #[inline(always)]
    #[must_use]
    pub fn byteorder(&mut self) -> BYTEORDER_W<CTRL_SPEC, 6> {
        BYTEORDER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
