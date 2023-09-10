#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `START` writer - Send start condition"]
pub type START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STOP` writer - Send stop condition"]
pub type STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACK` writer - Send ACK"]
pub type ACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NACK` writer - Send NACK"]
pub type NACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CONT` writer - Continue transmission"]
pub type CONT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ABORT` writer - Abort transmission"]
pub type ABORT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLEARTX` writer - Clear TX"]
pub type CLEARTX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLEARPC` writer - Clear Pending Commands"]
pub type CLEARPC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Send start condition"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CMD_SPEC, 0> {
        START_W::new(self)
    }
    #[doc = "Bit 1 - Send stop condition"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<CMD_SPEC, 1> {
        STOP_W::new(self)
    }
    #[doc = "Bit 2 - Send ACK"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<CMD_SPEC, 2> {
        ACK_W::new(self)
    }
    #[doc = "Bit 3 - Send NACK"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<CMD_SPEC, 3> {
        NACK_W::new(self)
    }
    #[doc = "Bit 4 - Continue transmission"]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<CMD_SPEC, 4> {
        CONT_W::new(self)
    }
    #[doc = "Bit 5 - Abort transmission"]
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> ABORT_W<CMD_SPEC, 5> {
        ABORT_W::new(self)
    }
    #[doc = "Bit 6 - Clear TX"]
    #[inline(always)]
    #[must_use]
    pub fn cleartx(&mut self) -> CLEARTX_W<CMD_SPEC, 6> {
        CLEARTX_W::new(self)
    }
    #[doc = "Bit 7 - Clear Pending Commands"]
    #[inline(always)]
    #[must_use]
    pub fn clearpc(&mut self) -> CLEARPC_W<CMD_SPEC, 7> {
        CLEARPC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
