#[doc = "Register `TXDATAX` writer"]
pub type W = crate::W<TXDATAX_SPEC>;
#[doc = "Field `TXDATAX` writer - TX Data"]
pub type TXDATAX_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `UBRXAT` writer - Unblock RX After Transmission"]
pub type UBRXAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTRIAT` writer - Set TXTRI After Transmission"]
pub type TXTRIAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBREAK` writer - Transmit Data As Break"]
pub type TXBREAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDISAT` writer - Clear TXEN After Transmission"]
pub type TXDISAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXENAT` writer - Enable RX After Transmission"]
pub type RXENAT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:8 - TX Data"]
    #[inline(always)]
    #[must_use]
    pub fn txdatax(&mut self) -> TXDATAX_W<TXDATAX_SPEC> {
        TXDATAX_W::new(self, 0)
    }
    #[doc = "Bit 11 - Unblock RX After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn ubrxat(&mut self) -> UBRXAT_W<TXDATAX_SPEC> {
        UBRXAT_W::new(self, 11)
    }
    #[doc = "Bit 12 - Set TXTRI After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn txtriat(&mut self) -> TXTRIAT_W<TXDATAX_SPEC> {
        TXTRIAT_W::new(self, 12)
    }
    #[doc = "Bit 13 - Transmit Data As Break"]
    #[inline(always)]
    #[must_use]
    pub fn txbreak(&mut self) -> TXBREAK_W<TXDATAX_SPEC> {
        TXBREAK_W::new(self, 13)
    }
    #[doc = "Bit 14 - Clear TXEN After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn txdisat(&mut self) -> TXDISAT_W<TXDATAX_SPEC> {
        TXDISAT_W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable RX After Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn rxenat(&mut self) -> RXENAT_W<TXDATAX_SPEC> {
        RXENAT_W::new(self, 15)
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
#[doc = "TX Buffer Data Extended Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdatax::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDATAX_SPEC;
impl crate::RegisterSpec for TXDATAX_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`txdatax::W`](W) writer structure"]
impl crate::Writable for TXDATAX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDATAX to value 0"]
impl crate::Resettable for TXDATAX_SPEC {
    const RESET_VALUE: u32 = 0;
}
