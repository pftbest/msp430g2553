#[doc = "Register `ADC10SA` reader"]
pub struct R(crate::R<ADC10SA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC10SA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ADC10SA_SPEC>> for R {
    fn from(reader: crate::R<ADC10SA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC10SA` writer"]
pub struct W(crate::W<ADC10SA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC10SA_SPEC>;
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
impl core::convert::From<crate::W<ADC10SA_SPEC>> for W {
    fn from(writer: crate::W<ADC10SA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC10SA` reader - ADC10 Data Transfer Start Address register"]
pub struct ADC10SA_R(crate::FieldReader<u16, u16>);
impl ADC10SA_R {
    pub(crate) fn new(bits: u16) -> Self {
        ADC10SA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC10SA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC10SA` writer - ADC10 Data Transfer Start Address register"]
pub struct ADC10SA_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10SA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - ADC10 Data Transfer Start Address register"]
    #[inline(always)]
    pub fn adc10sa(&self) -> ADC10SA_R {
        ADC10SA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADC10 Data Transfer Start Address register"]
    #[inline(always)]
    pub fn adc10sa(&mut self) -> ADC10SA_W {
        ADC10SA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC10 Data Transfer Start Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc10sa](index.html) module"]
pub struct ADC10SA_SPEC;
impl crate::RegisterSpec for ADC10SA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc10sa::R](R) reader structure"]
impl crate::Readable for ADC10SA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc10sa::W](W) writer structure"]
impl crate::Writable for ADC10SA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC10SA to value 0"]
impl crate::Resettable for ADC10SA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
