#[doc = "Register `UCB0I2COA` reader"]
pub struct R(crate::R<UCB0I2COA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0I2COA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB0I2COA_SPEC>> for R {
    #[inline(always)]
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
impl From<crate::W<UCB0I2COA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB0I2COA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCOA` reader - I2C Own Address 0"]
pub type UCOA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `UCOA` writer - I2C Own Address 0"]
pub type UCOA_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, UCB0I2COA_SPEC, u16, u16, 10, O>;
#[doc = "Field `UCGCEN` reader - I2C General Call enable"]
pub type UCGCEN_R = crate::BitReader<bool>;
#[doc = "Field `UCGCEN` writer - I2C General Call enable"]
pub type UCGCEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, UCB0I2COA_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9 - I2C Own Address 0"]
    #[inline(always)]
    pub fn ucoa(&self) -> UCOA_R {
        UCOA_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - I2C General Call enable"]
    #[inline(always)]
    pub fn ucgcen(&self) -> UCGCEN_R {
        UCGCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - I2C Own Address 0"]
    #[inline(always)]
    pub fn ucoa(&mut self) -> UCOA_W<0> {
        UCOA_W::new(self)
    }
    #[doc = "Bit 15 - I2C General Call enable"]
    #[inline(always)]
    pub fn ucgcen(&mut self) -> UCGCEN_W<15> {
        UCGCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
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
