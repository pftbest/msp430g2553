#[doc = "Register `ADC10DTC1` reader"]
pub struct R(crate::R<ADC10DTC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC10DTC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC10DTC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC10DTC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC10DTC1` writer"]
pub struct W(crate::W<ADC10DTC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC10DTC1_SPEC>;
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
impl From<crate::W<ADC10DTC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC10DTC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC10DTC1` reader - ADC10 Data Transfer Control 1 register"]
pub type ADC10DTC1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC10DTC1` writer - ADC10 Data Transfer Control 1 register"]
pub type ADC10DTC1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, ADC10DTC1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - ADC10 Data Transfer Control 1 register"]
    #[inline(always)]
    pub fn adc10dtc1(&self) -> ADC10DTC1_R {
        ADC10DTC1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADC10 Data Transfer Control 1 register"]
    #[inline(always)]
    pub fn adc10dtc1(&mut self) -> ADC10DTC1_W<0> {
        ADC10DTC1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "ADC10 Data Transfer Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc10dtc1](index.html) module"]
pub struct ADC10DTC1_SPEC;
impl crate::RegisterSpec for ADC10DTC1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adc10dtc1::R](R) reader structure"]
impl crate::Readable for ADC10DTC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc10dtc1::W](W) writer structure"]
impl crate::Writable for ADC10DTC1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC10DTC1 to value 0"]
impl crate::Resettable for ADC10DTC1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
