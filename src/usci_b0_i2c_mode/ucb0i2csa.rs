#[doc = "Register `UCB0I2CSA` reader"]
pub struct R(crate::R<UCB0I2CSA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0I2CSA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UCB0I2CSA_SPEC>> for R {
    fn from(reader: crate::R<UCB0I2CSA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0I2CSA` writer"]
pub struct W(crate::W<UCB0I2CSA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0I2CSA_SPEC>;
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
impl core::convert::From<crate::W<UCB0I2CSA_SPEC>> for W {
    fn from(writer: crate::W<UCB0I2CSA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCSA` reader - I2C Slave Address 0"]
pub struct UCSA_R(crate::FieldReader<u16, u16>);
impl UCSA_R {
    pub(crate) fn new(bits: u16) -> Self {
        UCSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCSA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSA` writer - I2C Slave Address 0"]
pub struct UCSA_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u16 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - I2C Slave Address 0"]
    #[inline(always)]
    pub fn ucsa(&self) -> UCSA_R {
        UCSA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - I2C Slave Address 0"]
    #[inline(always)]
    pub fn ucsa(&mut self) -> UCSA_W {
        UCSA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B0 I2C Slave Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0i2csa](index.html) module"]
pub struct UCB0I2CSA_SPEC;
impl crate::RegisterSpec for UCB0I2CSA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucb0i2csa::R](R) reader structure"]
impl crate::Readable for UCB0I2CSA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0i2csa::W](W) writer structure"]
impl crate::Writable for UCB0I2CSA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0I2CSA to value 0"]
impl crate::Resettable for UCB0I2CSA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
