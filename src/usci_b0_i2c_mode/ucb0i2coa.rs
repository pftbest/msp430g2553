#[doc = "Register `UCB0I2COA` reader"]
pub struct R(crate::R<UCB0I2COA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0I2COA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UCB0I2COA_SPEC>> for R {
    fn from(reader: crate::R<UCB0I2COA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0I2COA` writer"]
pub struct W(crate::W<UCB0I2COA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0I2COA_SPEC>;
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
impl core::convert::From<crate::W<UCB0I2COA_SPEC>> for W {
    fn from(writer: crate::W<UCB0I2COA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCGCEN` reader - I2C General Call enable"]
pub struct UCGCEN_R(crate::FieldReader<bool, bool>);
impl UCGCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCGCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCGCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCGCEN` writer - I2C General Call enable"]
pub struct UCGCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCGCEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 15)) | ((value as u16 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `UCOA` reader - I2C Own Address 0"]
pub struct UCOA_R(crate::FieldReader<u16, u16>);
impl UCOA_R {
    pub(crate) fn new(bits: u16) -> Self {
        UCOA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCOA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCOA` writer - I2C Own Address 0"]
pub struct UCOA_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u16 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - I2C General Call enable"]
    #[inline(always)]
    pub fn ucgcen(&self) -> UCGCEN_R {
        UCGCEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 0:9 - I2C Own Address 0"]
    #[inline(always)]
    pub fn ucoa(&self) -> UCOA_R {
        UCOA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 15 - I2C General Call enable"]
    #[inline(always)]
    pub fn ucgcen(&mut self) -> UCGCEN_W {
        UCGCEN_W { w: self }
    }
    #[doc = "Bits 0:9 - I2C Own Address 0"]
    #[inline(always)]
    pub fn ucoa(&mut self) -> UCOA_W {
        UCOA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B0 I2C Own Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0i2coa](index.html) module"]
pub struct UCB0I2COA_SPEC;
impl crate::RegisterSpec for UCB0I2COA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucb0i2coa::R](R) reader structure"]
impl crate::Readable for UCB0I2COA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0i2coa::W](W) writer structure"]
impl crate::Writable for UCB0I2COA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0I2COA to value 0"]
impl crate::Resettable for UCB0I2COA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
