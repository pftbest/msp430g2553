#[doc = "Register `ADC10MEM` reader"]
pub struct R(crate::R<ADC10MEM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC10MEM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC10MEM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC10MEM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC10MEM` writer"]
pub struct W(crate::W<ADC10MEM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC10MEM_SPEC>;
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
impl From<crate::W<ADC10MEM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC10MEM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC10MEM` reader - ADC10 Memory register"]
pub type ADC10MEM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADC10MEM` writer - ADC10 Memory register"]
pub type ADC10MEM_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADC10MEM_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - ADC10 Memory register"]
    #[inline(always)]
    pub fn adc10mem(&self) -> ADC10MEM_R {
        ADC10MEM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADC10 Memory register"]
    #[inline(always)]
    pub fn adc10mem(&mut self) -> ADC10MEM_W<0> {
        ADC10MEM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "ADC10 Memory\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc10mem](index.html) module"]
pub struct ADC10MEM_SPEC;
impl crate::RegisterSpec for ADC10MEM_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc10mem::R](R) reader structure"]
impl crate::Readable for ADC10MEM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc10mem::W](W) writer structure"]
impl crate::Writable for ADC10MEM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC10MEM to value 0"]
impl crate::Resettable for ADC10MEM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
