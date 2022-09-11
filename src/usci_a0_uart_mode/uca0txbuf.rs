#[doc = "Register `UCA0TXBUF` reader"]
pub struct R(crate::R<UCA0TXBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA0TXBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA0TXBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA0TXBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA0TXBUF` writer"]
pub struct W(crate::W<UCA0TXBUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA0TXBUF_SPEC>;
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
impl From<crate::W<UCA0TXBUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA0TXBUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCA0TXBUF` reader - USCI A0 Transmit Buffer register"]
pub type UCA0TXBUF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UCA0TXBUF` writer - USCI A0 Transmit Buffer register"]
pub type UCA0TXBUF_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, UCA0TXBUF_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - USCI A0 Transmit Buffer register"]
    #[inline(always)]
    pub fn uca0txbuf(&self) -> UCA0TXBUF_R {
        UCA0TXBUF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - USCI A0 Transmit Buffer register"]
    #[inline(always)]
    pub fn uca0txbuf(&mut self) -> UCA0TXBUF_W<0> {
        UCA0TXBUF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "USCI A0 Transmit Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0txbuf](index.html) module"]
pub struct UCA0TXBUF_SPEC;
impl crate::RegisterSpec for UCA0TXBUF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca0txbuf::R](R) reader structure"]
impl crate::Readable for UCA0TXBUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca0txbuf::W](W) writer structure"]
impl crate::Writable for UCA0TXBUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA0TXBUF to value 0"]
impl crate::Resettable for UCA0TXBUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
