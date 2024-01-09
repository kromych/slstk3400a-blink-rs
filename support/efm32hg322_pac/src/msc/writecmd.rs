#[doc = "Register `WRITECMD` writer"]
pub type W = crate::W<WRITECMD_SPEC>;
#[doc = "Field `LADDRIM` writer - Load MSC_ADDRB into ADDR"]
pub type LADDRIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERASEPAGE` writer - Erase Page"]
pub type ERASEPAGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITEEND` writer - End Write Mode"]
pub type WRITEEND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITEONCE` writer - Word Write-Once Trigger"]
pub type WRITEONCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITETRIG` writer - Word Write Sequence Trigger"]
pub type WRITETRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERASEABORT` writer - Abort erase sequence"]
pub type ERASEABORT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERASEMAIN0` writer - Mass erase region 0"]
pub type ERASEMAIN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARWDATA` writer - Clear WDATA state"]
pub type CLEARWDATA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Load MSC_ADDRB into ADDR"]
    #[inline(always)]
    #[must_use]
    pub fn laddrim(&mut self) -> LADDRIM_W<WRITECMD_SPEC> {
        LADDRIM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Erase Page"]
    #[inline(always)]
    #[must_use]
    pub fn erasepage(&mut self) -> ERASEPAGE_W<WRITECMD_SPEC> {
        ERASEPAGE_W::new(self, 1)
    }
    #[doc = "Bit 2 - End Write Mode"]
    #[inline(always)]
    #[must_use]
    pub fn writeend(&mut self) -> WRITEEND_W<WRITECMD_SPEC> {
        WRITEEND_W::new(self, 2)
    }
    #[doc = "Bit 3 - Word Write-Once Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn writeonce(&mut self) -> WRITEONCE_W<WRITECMD_SPEC> {
        WRITEONCE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Word Write Sequence Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn writetrig(&mut self) -> WRITETRIG_W<WRITECMD_SPEC> {
        WRITETRIG_W::new(self, 4)
    }
    #[doc = "Bit 5 - Abort erase sequence"]
    #[inline(always)]
    #[must_use]
    pub fn eraseabort(&mut self) -> ERASEABORT_W<WRITECMD_SPEC> {
        ERASEABORT_W::new(self, 5)
    }
    #[doc = "Bit 8 - Mass erase region 0"]
    #[inline(always)]
    #[must_use]
    pub fn erasemain0(&mut self) -> ERASEMAIN0_W<WRITECMD_SPEC> {
        ERASEMAIN0_W::new(self, 8)
    }
    #[doc = "Bit 12 - Clear WDATA state"]
    #[inline(always)]
    #[must_use]
    pub fn clearwdata(&mut self) -> CLEARWDATA_W<WRITECMD_SPEC> {
        CLEARWDATA_W::new(self, 12)
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
#[doc = "Write Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`writecmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRITECMD_SPEC;
impl crate::RegisterSpec for WRITECMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`writecmd::W`](W) writer structure"]
impl crate::Writable for WRITECMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRITECMD to value 0"]
impl crate::Resettable for WRITECMD_SPEC {
    const RESET_VALUE: u32 = 0;
}
