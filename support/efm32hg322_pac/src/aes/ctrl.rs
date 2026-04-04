#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `DECRYPT` reader - Decryption/Encryption Mode"]
pub type DecryptR = crate::BitReader;
#[doc = "Field `DECRYPT` writer - Decryption/Encryption Mode"]
pub type DecryptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATASTART` reader - AES_DATA Write Start"]
pub type DatastartR = crate::BitReader;
#[doc = "Field `DATASTART` writer - AES_DATA Write Start"]
pub type DatastartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XORSTART` reader - AES_XORDATA Write Start"]
pub type XorstartR = crate::BitReader;
#[doc = "Field `XORSTART` writer - AES_XORDATA Write Start"]
pub type XorstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYTEORDER` reader - Configure byte order in data and key registers"]
pub type ByteorderR = crate::BitReader;
#[doc = "Field `BYTEORDER` writer - Configure byte order in data and key registers"]
pub type ByteorderW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Decryption/Encryption Mode"]
    #[inline(always)]
    pub fn decrypt(&self) -> DecryptR {
        DecryptR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - AES_DATA Write Start"]
    #[inline(always)]
    pub fn datastart(&self) -> DatastartR {
        DatastartR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AES_XORDATA Write Start"]
    #[inline(always)]
    pub fn xorstart(&self) -> XorstartR {
        XorstartR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configure byte order in data and key registers"]
    #[inline(always)]
    pub fn byteorder(&self) -> ByteorderR {
        ByteorderR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Decryption/Encryption Mode"]
    #[inline(always)]
    pub fn decrypt(&mut self) -> DecryptW<'_, CtrlSpec> {
        DecryptW::new(self, 0)
    }
    #[doc = "Bit 4 - AES_DATA Write Start"]
    #[inline(always)]
    pub fn datastart(&mut self) -> DatastartW<'_, CtrlSpec> {
        DatastartW::new(self, 4)
    }
    #[doc = "Bit 5 - AES_XORDATA Write Start"]
    #[inline(always)]
    pub fn xorstart(&mut self) -> XorstartW<'_, CtrlSpec> {
        XorstartW::new(self, 5)
    }
    #[doc = "Bit 6 - Configure byte order in data and key registers"]
    #[inline(always)]
    pub fn byteorder(&mut self) -> ByteorderW<'_, CtrlSpec> {
        ByteorderW::new(self, 6)
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
