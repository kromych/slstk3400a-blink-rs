#[doc = "Register `DIEPTXF3` reader"]
pub type R = crate::R<DIEPTXF3_SPEC>;
#[doc = "Register `DIEPTXF3` writer"]
pub type W = crate::W<DIEPTXF3_SPEC>;
#[doc = "Field `INEPNTXFSTADDR` reader - IN Endpoint FIFO 3 Transmit RAM Start Address"]
pub type INEPNTXFSTADDR_R = crate::FieldReader<u16>;
#[doc = "Field `INEPNTXFSTADDR` writer - IN Endpoint FIFO 3 Transmit RAM Start Address"]
pub type INEPNTXFSTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `INEPNTXFDEP` reader - IN Endpoint TxFIFO Depth"]
pub type INEPNTXFDEP_R = crate::FieldReader<u16>;
#[doc = "Field `INEPNTXFDEP` writer - IN Endpoint TxFIFO Depth"]
pub type INEPNTXFDEP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:11 - IN Endpoint FIFO 3 Transmit RAM Start Address"]
    #[inline(always)]
    pub fn inepntxfstaddr(&self) -> INEPNTXFSTADDR_R {
        INEPNTXFSTADDR_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:25 - IN Endpoint TxFIFO Depth"]
    #[inline(always)]
    pub fn inepntxfdep(&self) -> INEPNTXFDEP_R {
        INEPNTXFDEP_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - IN Endpoint FIFO 3 Transmit RAM Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn inepntxfstaddr(&mut self) -> INEPNTXFSTADDR_W<DIEPTXF3_SPEC> {
        INEPNTXFSTADDR_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - IN Endpoint TxFIFO Depth"]
    #[inline(always)]
    #[must_use]
    pub fn inepntxfdep(&mut self) -> INEPNTXFDEP_W<DIEPTXF3_SPEC> {
        INEPNTXFDEP_W::new(self, 16)
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
#[doc = "Device IN Endpoint Transmit FIFO 3 Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTXF3_SPEC;
impl crate::RegisterSpec for DIEPTXF3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptxf3::R`](R) reader structure"]
impl crate::Readable for DIEPTXF3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dieptxf3::W`](W) writer structure"]
impl crate::Writable for DIEPTXF3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPTXF3 to value 0x0200_0800"]
impl crate::Resettable for DIEPTXF3_SPEC {
    const RESET_VALUE: u32 = 0x0200_0800;
}
