#[doc = "Register `UCB0I2CIE` reader"]
pub struct R(crate::R<UCB0I2CIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0I2CIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB0I2CIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB0I2CIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0I2CIE` writer"]
pub struct W(crate::W<UCB0I2CIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0I2CIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<UCB0I2CIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB0I2CIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCALIE` reader - Arbitration Lost interrupt enable"]
pub type UCALIE_R = crate::BitReader<bool>;
#[doc = "Field `UCALIE` writer - Arbitration Lost interrupt enable"]
pub type UCALIE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, UCB0I2CIE_SPEC, bool, O>;
#[doc = "Field `UCSTTIE` reader - START Condition interrupt enable"]
pub type UCSTTIE_R = crate::BitReader<bool>;
#[doc = "Field `UCSTTIE` writer - START Condition interrupt enable"]
pub type UCSTTIE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, UCB0I2CIE_SPEC, bool, O>;
#[doc = "Field `UCSTPIE` reader - STOP Condition interrupt enable"]
pub type UCSTPIE_R = crate::BitReader<bool>;
#[doc = "Field `UCSTPIE` writer - STOP Condition interrupt enable"]
pub type UCSTPIE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, UCB0I2CIE_SPEC, bool, O>;
#[doc = "Field `UCNACKIE` reader - NACK Condition interrupt enable"]
pub type UCNACKIE_R = crate::BitReader<bool>;
#[doc = "Field `UCNACKIE` writer - NACK Condition interrupt enable"]
pub type UCNACKIE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, UCB0I2CIE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Arbitration Lost interrupt enable"]
    #[inline(always)]
    pub fn ucalie(&self) -> UCALIE_R {
        UCALIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - START Condition interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&self) -> UCSTTIE_R {
        UCSTTIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - STOP Condition interrupt enable"]
    #[inline(always)]
    pub fn ucstpie(&self) -> UCSTPIE_R {
        UCSTPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NACK Condition interrupt enable"]
    #[inline(always)]
    pub fn ucnackie(&self) -> UCNACKIE_R {
        UCNACKIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Arbitration Lost interrupt enable"]
    #[inline(always)]
    pub fn ucalie(&mut self) -> UCALIE_W<0> {
        UCALIE_W::new(self)
    }
    #[doc = "Bit 1 - START Condition interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&mut self) -> UCSTTIE_W<1> {
        UCSTTIE_W::new(self)
    }
    #[doc = "Bit 2 - STOP Condition interrupt enable"]
    #[inline(always)]
    pub fn ucstpie(&mut self) -> UCSTPIE_W<2> {
        UCSTPIE_W::new(self)
    }
    #[doc = "Bit 3 - NACK Condition interrupt enable"]
    #[inline(always)]
    pub fn ucnackie(&mut self) -> UCNACKIE_W<3> {
        UCNACKIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B0 I2C Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0i2cie](index.html) module"]
pub struct UCB0I2CIE_SPEC;
impl crate::RegisterSpec for UCB0I2CIE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucb0i2cie::R](R) reader structure"]
impl crate::Readable for UCB0I2CIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0i2cie::W](W) writer structure"]
impl crate::Writable for UCB0I2CIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0I2CIE to value 0"]
impl crate::Resettable for UCB0I2CIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
