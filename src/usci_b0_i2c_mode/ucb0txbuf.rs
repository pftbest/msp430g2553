#[doc = "Register `UCB0TXBUF` reader"]
pub struct R(crate::R<UCB0TXBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0TXBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB0TXBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB0TXBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0TXBUF` writer"]
pub struct W(crate::W<UCB0TXBUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0TXBUF_SPEC>;
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
impl From<crate::W<UCB0TXBUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB0TXBUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCB0TXBUF` reader - USCI B0 Transmit Buffer register"]
pub struct UCB0TXBUF_R(crate::FieldReader<u8, u8>);
impl UCB0TXBUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UCB0TXBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCB0TXBUF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCB0TXBUF` writer - USCI B0 Transmit Buffer register"]
pub struct UCB0TXBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> UCB0TXBUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value as u8;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - USCI B0 Transmit Buffer register"]
    #[inline(always)]
    pub fn ucb0txbuf(&self) -> UCB0TXBUF_R {
        UCB0TXBUF_R::new(self.bits as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - USCI B0 Transmit Buffer register"]
    #[inline(always)]
    pub fn ucb0txbuf(&mut self) -> UCB0TXBUF_W {
        UCB0TXBUF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "USCI B0 Transmit Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0txbuf](index.html) module"]
pub struct UCB0TXBUF_SPEC;
impl crate::RegisterSpec for UCB0TXBUF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucb0txbuf::R](R) reader structure"]
impl crate::Readable for UCB0TXBUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0txbuf::W](W) writer structure"]
impl crate::Writable for UCB0TXBUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0TXBUF to value 0"]
impl crate::Resettable for UCB0TXBUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
