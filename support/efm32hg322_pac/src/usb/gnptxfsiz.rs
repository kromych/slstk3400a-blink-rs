#[doc = "Register `GNPTXFSIZ` reader"]
pub type R = crate::R<GnptxfsizSpec>;
#[doc = "Register `GNPTXFSIZ` writer"]
pub type W = crate::W<GnptxfsizSpec>;
#[doc = "Field `NPTXFSTADDR` reader - Non-periodic Transmit RAM Start Address host only"]
pub type NptxfstaddrR = crate::FieldReader<u16>;
#[doc = "Field `NPTXFSTADDR` writer - Non-periodic Transmit RAM Start Address host only"]
pub type NptxfstaddrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `NPTXFINEPTXF0DEP` reader - Non-periodic TxFIFO Depth host only / IN Endpoint TxFIFO 0 Depth"]
pub type Nptxfineptxf0depR = crate::FieldReader<u16>;
#[doc = "Field `NPTXFINEPTXF0DEP` writer - Non-periodic TxFIFO Depth host only / IN Endpoint TxFIFO 0 Depth"]
pub type Nptxfineptxf0depW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:9 - Non-periodic Transmit RAM Start Address host only"]
    #[inline(always)]
    pub fn nptxfstaddr(&self) -> NptxfstaddrR {
        NptxfstaddrR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:31 - Non-periodic TxFIFO Depth host only / IN Endpoint TxFIFO 0 Depth"]
    #[inline(always)]
    pub fn nptxfineptxf0dep(&self) -> Nptxfineptxf0depR {
        Nptxfineptxf0depR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Non-periodic Transmit RAM Start Address host only"]
    #[inline(always)]
    pub fn nptxfstaddr(&mut self) -> NptxfstaddrW<'_, GnptxfsizSpec> {
        NptxfstaddrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Non-periodic TxFIFO Depth host only / IN Endpoint TxFIFO 0 Depth"]
    #[inline(always)]
    pub fn nptxfineptxf0dep(&mut self) -> Nptxfineptxf0depW<'_, GnptxfsizSpec> {
        Nptxfineptxf0depW::new(self, 16)
    }
}
#[doc = "Non-periodic Transmit FIFO Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gnptxfsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gnptxfsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GnptxfsizSpec;
impl crate::RegisterSpec for GnptxfsizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gnptxfsiz::R`](R) reader structure"]
impl crate::Readable for GnptxfsizSpec {}
#[doc = "`write(|w| ..)` method takes [`gnptxfsiz::W`](W) writer structure"]
impl crate::Writable for GnptxfsizSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GNPTXFSIZ to value 0x0200_0200"]
impl crate::Resettable for GnptxfsizSpec {
    const RESET_VALUE: u32 = 0x0200_0200;
}
