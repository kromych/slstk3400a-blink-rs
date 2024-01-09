#[doc = "Register `CHUSEBURSTS` reader"]
pub type R = crate::R<CHUSEBURSTS_SPEC>;
#[doc = "Register `CHUSEBURSTS` writer"]
pub type W = crate::W<CHUSEBURSTS_SPEC>;
#[doc = "Field `CH0USEBURSTS` reader - Channel 0 Useburst Set"]
pub type CH0USEBURSTS_R = crate::BitReader;
#[doc = "Field `CH0USEBURSTS` writer - Channel 0 Useburst Set"]
pub type CH0USEBURSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1USEBURSTS` reader - Channel 1 Useburst Set"]
pub type CH1USEBURSTS_R = crate::BitReader;
#[doc = "Field `CH1USEBURSTS` writer - Channel 1 Useburst Set"]
pub type CH1USEBURSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2USEBURSTS` reader - Channel 2 Useburst Set"]
pub type CH2USEBURSTS_R = crate::BitReader;
#[doc = "Field `CH2USEBURSTS` writer - Channel 2 Useburst Set"]
pub type CH2USEBURSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3USEBURSTS` reader - Channel 3 Useburst Set"]
pub type CH3USEBURSTS_R = crate::BitReader;
#[doc = "Field `CH3USEBURSTS` writer - Channel 3 Useburst Set"]
pub type CH3USEBURSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4USEBURSTS` reader - Channel 4 Useburst Set"]
pub type CH4USEBURSTS_R = crate::BitReader;
#[doc = "Field `CH4USEBURSTS` writer - Channel 4 Useburst Set"]
pub type CH4USEBURSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5USEBURSTS` reader - Channel 5 Useburst Set"]
pub type CH5USEBURSTS_R = crate::BitReader;
#[doc = "Field `CH5USEBURSTS` writer - Channel 5 Useburst Set"]
pub type CH5USEBURSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel 0 Useburst Set"]
    #[inline(always)]
    pub fn ch0usebursts(&self) -> CH0USEBURSTS_R {
        CH0USEBURSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Useburst Set"]
    #[inline(always)]
    pub fn ch1usebursts(&self) -> CH1USEBURSTS_R {
        CH1USEBURSTS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Useburst Set"]
    #[inline(always)]
    pub fn ch2usebursts(&self) -> CH2USEBURSTS_R {
        CH2USEBURSTS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Useburst Set"]
    #[inline(always)]
    pub fn ch3usebursts(&self) -> CH3USEBURSTS_R {
        CH3USEBURSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Useburst Set"]
    #[inline(always)]
    pub fn ch4usebursts(&self) -> CH4USEBURSTS_R {
        CH4USEBURSTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Useburst Set"]
    #[inline(always)]
    pub fn ch5usebursts(&self) -> CH5USEBURSTS_R {
        CH5USEBURSTS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Useburst Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch0usebursts(&mut self) -> CH0USEBURSTS_W<CHUSEBURSTS_SPEC> {
        CH0USEBURSTS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Useburst Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch1usebursts(&mut self) -> CH1USEBURSTS_W<CHUSEBURSTS_SPEC> {
        CH1USEBURSTS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Useburst Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch2usebursts(&mut self) -> CH2USEBURSTS_W<CHUSEBURSTS_SPEC> {
        CH2USEBURSTS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Useburst Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch3usebursts(&mut self) -> CH3USEBURSTS_W<CHUSEBURSTS_SPEC> {
        CH3USEBURSTS_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Useburst Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch4usebursts(&mut self) -> CH4USEBURSTS_W<CHUSEBURSTS_SPEC> {
        CH4USEBURSTS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Useburst Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch5usebursts(&mut self) -> CH5USEBURSTS_W<CHUSEBURSTS_SPEC> {
        CH5USEBURSTS_W::new(self, 5)
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
#[doc = "Channel Useburst Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chusebursts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chusebursts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHUSEBURSTS_SPEC;
impl crate::RegisterSpec for CHUSEBURSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chusebursts::R`](R) reader structure"]
impl crate::Readable for CHUSEBURSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chusebursts::W`](W) writer structure"]
impl crate::Writable for CHUSEBURSTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHUSEBURSTS to value 0"]
impl crate::Resettable for CHUSEBURSTS_SPEC {
    const RESET_VALUE: u32 = 0;
}
