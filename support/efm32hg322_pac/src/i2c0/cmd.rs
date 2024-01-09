#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `START` writer - Send start condition"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` writer - Send stop condition"]
pub type STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` writer - Send ACK"]
pub type ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` writer - Send NACK"]
pub type NACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONT` writer - Continue transmission"]
pub type CONT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORT` writer - Abort transmission"]
pub type ABORT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARTX` writer - Clear TX"]
pub type CLEARTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARPC` writer - Clear Pending Commands"]
pub type CLEARPC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Send start condition"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CMD_SPEC> {
        START_W::new(self, 0)
    }
    #[doc = "Bit 1 - Send stop condition"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<CMD_SPEC> {
        STOP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Send ACK"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<CMD_SPEC> {
        ACK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Send NACK"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<CMD_SPEC> {
        NACK_W::new(self, 3)
    }
    #[doc = "Bit 4 - Continue transmission"]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<CMD_SPEC> {
        CONT_W::new(self, 4)
    }
    #[doc = "Bit 5 - Abort transmission"]
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> ABORT_W<CMD_SPEC> {
        ABORT_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear TX"]
    #[inline(always)]
    #[must_use]
    pub fn cleartx(&mut self) -> CLEARTX_W<CMD_SPEC> {
        CLEARTX_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear Pending Commands"]
    #[inline(always)]
    #[must_use]
    pub fn clearpc(&mut self) -> CLEARPC_W<CMD_SPEC> {
        CLEARPC_W::new(self, 7)
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
#[doc = "Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: u32 = 0;
}
