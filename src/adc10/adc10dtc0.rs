#[doc = "Register `ADC10DTC0` reader"]
pub struct R(crate::R<ADC10DTC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC10DTC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC10DTC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC10DTC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC10DTC0` writer"]
pub struct W(crate::W<ADC10DTC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC10DTC0_SPEC>;
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
impl From<crate::W<ADC10DTC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC10DTC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC10FETCH` reader - This bit should normally be reset"]
pub struct ADC10FETCH_R(crate::FieldReader<bool, bool>);
impl ADC10FETCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC10FETCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC10FETCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC10FETCH` writer - This bit should normally be reset"]
pub struct ADC10FETCH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10FETCH_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `ADC10B1` reader - ADC10 block one"]
pub struct ADC10B1_R(crate::FieldReader<bool, bool>);
impl ADC10B1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC10B1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC10B1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC10B1` writer - ADC10 block one"]
pub struct ADC10B1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10B1_W<'a> {
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
            (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `ADC10CT` reader - ADC10 continuous transfer"]
pub struct ADC10CT_R(crate::FieldReader<bool, bool>);
impl ADC10CT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC10CT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC10CT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC10CT` writer - ADC10 continuous transfer"]
pub struct ADC10CT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10CT_W<'a> {
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
            (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `ADC10TB` reader - ADC10 two-block mode"]
pub struct ADC10TB_R(crate::FieldReader<bool, bool>);
impl ADC10TB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC10TB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC10TB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC10TB` writer - ADC10 two-block mode"]
pub struct ADC10TB_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10TB_W<'a> {
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
            (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - This bit should normally be reset"]
    #[inline(always)]
    pub fn adc10fetch(&self) -> ADC10FETCH_R {
        ADC10FETCH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC10 block one"]
    #[inline(always)]
    pub fn adc10b1(&self) -> ADC10B1_R {
        ADC10B1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC10 continuous transfer"]
    #[inline(always)]
    pub fn adc10ct(&self) -> ADC10CT_R {
        ADC10CT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC10 two-block mode"]
    #[inline(always)]
    pub fn adc10tb(&self) -> ADC10TB_R {
        ADC10TB_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit should normally be reset"]
    #[inline(always)]
    pub fn adc10fetch(&mut self) -> ADC10FETCH_W {
        ADC10FETCH_W { w: self }
    }
    #[doc = "Bit 1 - ADC10 block one"]
    #[inline(always)]
    pub fn adc10b1(&mut self) -> ADC10B1_W {
        ADC10B1_W { w: self }
    }
    #[doc = "Bit 2 - ADC10 continuous transfer"]
    #[inline(always)]
    pub fn adc10ct(&mut self) -> ADC10CT_W {
        ADC10CT_W { w: self }
    }
    #[doc = "Bit 3 - ADC10 two-block mode"]
    #[inline(always)]
    pub fn adc10tb(&mut self) -> ADC10TB_W {
        ADC10TB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC10 Data Transfer Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc10dtc0](index.html) module"]
pub struct ADC10DTC0_SPEC;
impl crate::RegisterSpec for ADC10DTC0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adc10dtc0::R](R) reader structure"]
impl crate::Readable for ADC10DTC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc10dtc0::W](W) writer structure"]
impl crate::Writable for ADC10DTC0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC10DTC0 to value 0"]
impl crate::Resettable for ADC10DTC0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
