#[doc = "Register `ADC10MEM` reader"]
pub struct R(crate::R<ADC10MEM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC10MEM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ADC10MEM_SPEC>> for R {
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
impl core::convert::From<crate::W<ADC10MEM_SPEC>> for W {
    fn from(writer: crate::W<ADC10MEM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC10MEM` reader - ADC10 Memory register"]
pub struct ADC10MEM_R(crate::FieldReader<u16, u16>);
impl ADC10MEM_R {
    pub(crate) fn new(bits: u16) -> Self {
        ADC10MEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC10MEM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC10MEM` writer - ADC10 Memory register"]
pub struct ADC10MEM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10MEM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - ADC10 Memory register"]
    #[inline(always)]
    pub fn adc10mem(&self) -> ADC10MEM_R {
        ADC10MEM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADC10 Memory register"]
    #[inline(always)]
    pub fn adc10mem(&mut self) -> ADC10MEM_W {
        ADC10MEM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
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
