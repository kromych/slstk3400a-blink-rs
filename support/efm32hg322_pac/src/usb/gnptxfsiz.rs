#[doc = "Register `GNPTXFSIZ` reader"]
pub type R = crate::R<GNPTXFSIZ_SPEC>;
#[doc = "Register `GNPTXFSIZ` writer"]
pub type W = crate::W<GNPTXFSIZ_SPEC>;
#[doc = "Field `NPTXFSTADDR` reader - Non-periodic Transmit RAM Start Address host only"]
pub type NPTXFSTADDR_R = crate::FieldReader<u16>;
#[doc = "Field `NPTXFSTADDR` writer - Non-periodic Transmit RAM Start Address host only"]
pub type NPTXFSTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `NPTXFINEPTXF0DEP` reader - Non-periodic TxFIFO Depth host only / IN Endpoint TxFIFO 0 Depth"]
pub type NPTXFINEPTXF0DEP_R = crate::FieldReader<u16>;
#[doc = "Field `NPTXFINEPTXF0DEP` writer - Non-periodic TxFIFO Depth host only / IN Endpoint TxFIFO 0 Depth"]
pub type NPTXFINEPTXF0DEP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:9 - Non-periodic Transmit RAM Start Address host only"]
    #[inline(always)]
    pub fn nptxfstaddr(&self) -> NPTXFSTADDR_R {
        NPTXFSTADDR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:31 - Non-periodic TxFIFO Depth host only / IN Endpoint TxFIFO 0 Depth"]
    #[inline(always)]
    pub fn nptxfineptxf0dep(&self) -> NPTXFINEPTXF0DEP_R {
        NPTXFINEPTXF0DEP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Non-periodic Transmit RAM Start Address host only"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfstaddr(&mut self) -> NPTXFSTADDR_W<GNPTXFSIZ_SPEC> {
        NPTXFSTADDR_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Non-periodic TxFIFO Depth host only / IN Endpoint TxFIFO 0 Depth"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfineptxf0dep(&mut self) -> NPTXFINEPTXF0DEP_W<GNPTXFSIZ_SPEC> {
        NPTXFINEPTXF0DEP_W::new(self, 16)
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
#[doc = "Non-periodic Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxfsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gnptxfsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GNPTXFSIZ_SPEC;
impl crate::RegisterSpec for GNPTXFSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gnptxfsiz::R`](R) reader structure"]
impl crate::Readable for GNPTXFSIZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gnptxfsiz::W`](W) writer structure"]
impl crate::Writable for GNPTXFSIZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GNPTXFSIZ to value 0x0200_0200"]
impl crate::Resettable for GNPTXFSIZ_SPEC {
    const RESET_VALUE: u32 = 0x0200_0200;
}
