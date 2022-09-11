#[doc = "Register `UCA0BR1` reader"]
pub struct R(crate::R<UCA0BR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA0BR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA0BR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA0BR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA0BR1` writer"]
pub struct W(crate::W<UCA0BR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA0BR1_SPEC>;
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
impl From<crate::W<UCA0BR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA0BR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCA0BR1` reader - USCI A0 Baud Rate 1 register"]
pub type UCA0BR1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UCA0BR1` writer - USCI A0 Baud Rate 1 register"]
pub type UCA0BR1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, UCA0BR1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - USCI A0 Baud Rate 1 register"]
    #[inline(always)]
    pub fn uca0br1(&self) -> UCA0BR1_R {
        UCA0BR1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - USCI A0 Baud Rate 1 register"]
    #[inline(always)]
    pub fn uca0br1(&mut self) -> UCA0BR1_W<0> {
        UCA0BR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "USCI A0 Baud Rate 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0br1](index.html) module"]
pub struct UCA0BR1_SPEC;
impl crate::RegisterSpec for UCA0BR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca0br1::R](R) reader structure"]
impl crate::Readable for UCA0BR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca0br1::W](W) writer structure"]
impl crate::Writable for UCA0BR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA0BR1 to value 0"]
impl crate::Resettable for UCA0BR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
