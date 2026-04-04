#[doc = "Register `XORDATA` reader"]
pub type R = crate::R<XordataSpec>;
#[doc = "Register `XORDATA` writer"]
pub type W = crate::W<XordataSpec>;
#[doc = "Field `XORDATA` reader - XOR Data Access"]
pub type XordataR = crate::FieldReader<u32>;
#[doc = "Field `XORDATA` writer - XOR Data Access"]
pub type XordataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - XOR Data Access"]
    #[inline(always)]
    pub fn xordata(&self) -> XordataR {
        XordataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - XOR Data Access"]
    #[inline(always)]
    pub fn xordata(&mut self) -> XordataW<'_, XordataSpec> {
        XordataW::new(self, 0)
    }
}
#[doc = "XORDATA Register\n\nYou can [`read`](crate::Reg::read) this register and get [`xordata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xordata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct XordataSpec;
impl crate::RegisterSpec for XordataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xordata::R`](R) reader structure"]
impl crate::Readable for XordataSpec {}
#[doc = "`write(|w| ..)` method takes [`xordata::W`](W) writer structure"]
impl crate::Writable for XordataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XORDATA to value 0"]
impl crate::Resettable for XordataSpec {}
