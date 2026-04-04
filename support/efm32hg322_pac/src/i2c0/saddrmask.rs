#[doc = "Register `SADDRMASK` reader"]
pub type R = crate::R<SaddrmaskSpec>;
#[doc = "Register `SADDRMASK` writer"]
pub type W = crate::W<SaddrmaskSpec>;
#[doc = "Field `MASK` reader - Slave Address Mask"]
pub type MaskR = crate::FieldReader;
#[doc = "Field `MASK` writer - Slave Address Mask"]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 1:7 - Slave Address Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - Slave Address Mask"]
    #[inline(always)]
    pub fn mask(&mut self) -> MaskW<'_, SaddrmaskSpec> {
        MaskW::new(self, 1)
    }
}
#[doc = "Slave Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`saddrmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddrmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaddrmaskSpec;
impl crate::RegisterSpec for SaddrmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saddrmask::R`](R) reader structure"]
impl crate::Readable for SaddrmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`saddrmask::W`](W) writer structure"]
impl crate::Writable for SaddrmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SADDRMASK to value 0"]
impl crate::Resettable for SaddrmaskSpec {}
