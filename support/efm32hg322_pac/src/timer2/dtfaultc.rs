#[doc = "Register `DTFAULTC` writer"]
pub type W = crate::W<DTFAULTC_SPEC>;
#[doc = "Field `DTPRS0FC` writer - DTI PRS0 Fault Clear"]
pub type DTPRS0FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTPRS1FC` writer - DTI PRS1 Fault Clear"]
pub type DTPRS1FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTDBGFC` writer - DTI Debugger Fault Clear"]
pub type DTDBGFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TLOCKUPFC` writer - DTI Lockup Fault Clear"]
pub type TLOCKUPFC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - DTI PRS0 Fault Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dtprs0fc(&mut self) -> DTPRS0FC_W<DTFAULTC_SPEC> {
        DTPRS0FC_W::new(self, 0)
    }
    #[doc = "Bit 1 - DTI PRS1 Fault Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dtprs1fc(&mut self) -> DTPRS1FC_W<DTFAULTC_SPEC> {
        DTPRS1FC_W::new(self, 1)
    }
    #[doc = "Bit 2 - DTI Debugger Fault Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dtdbgfc(&mut self) -> DTDBGFC_W<DTFAULTC_SPEC> {
        DTDBGFC_W::new(self, 2)
    }
    #[doc = "Bit 3 - DTI Lockup Fault Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tlockupfc(&mut self) -> TLOCKUPFC_W<DTFAULTC_SPEC> {
        TLOCKUPFC_W::new(self, 3)
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
#[doc = "DTI Fault Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtfaultc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTFAULTC_SPEC;
impl crate::RegisterSpec for DTFAULTC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dtfaultc::W`](W) writer structure"]
impl crate::Writable for DTFAULTC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTFAULTC to value 0"]
impl crate::Resettable for DTFAULTC_SPEC {
    const RESET_VALUE: u32 = 0;
}
