#[doc = "Register `IE1` reader"]
pub struct R(crate::R<IE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IE1_SPEC>> for R {
    fn from(reader: crate::R<IE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IE1` writer"]
pub struct W(crate::W<IE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IE1_SPEC>;
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
impl core::convert::From<crate::W<IE1_SPEC>> for W {
    fn from(writer: crate::W<IE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTIE` reader - Watchdog Interrupt Enable"]
pub struct WDTIE_R(crate::FieldReader<bool, bool>);
impl WDTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTIE` writer - Watchdog Interrupt Enable"]
pub struct WDTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTIE_W<'a> {
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
#[doc = "Field `OFIE` reader - Osc. Fault Interrupt Enable"]
pub struct OFIE_R(crate::FieldReader<bool, bool>);
impl OFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFIE` writer - Osc. Fault Interrupt Enable"]
pub struct OFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OFIE_W<'a> {
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
#[doc = "Field `NMIIE` reader - NMI Interrupt Enable"]
pub struct NMIIE_R(crate::FieldReader<bool, bool>);
impl NMIIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        NMIIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NMIIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NMIIE` writer - NMI Interrupt Enable"]
pub struct NMIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIIE_W<'a> {
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
            (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `ACCVIE` reader - Flash Access Violation Interrupt Enable"]
pub struct ACCVIE_R(crate::FieldReader<bool, bool>);
impl ACCVIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACCVIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACCVIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACCVIE` writer - Flash Access Violation Interrupt Enable"]
pub struct ACCVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCVIE_W<'a> {
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
            (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn wdtie(&self) -> WDTIE_R {
        WDTIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Osc. Fault Interrupt Enable"]
    #[inline(always)]
    pub fn ofie(&self) -> OFIE_R {
        OFIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NMI Interrupt Enable"]
    #[inline(always)]
    pub fn nmiie(&self) -> NMIIE_R {
        NMIIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Flash Access Violation Interrupt Enable"]
    #[inline(always)]
    pub fn accvie(&self) -> ACCVIE_R {
        ACCVIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn wdtie(&mut self) -> WDTIE_W {
        WDTIE_W { w: self }
    }
    #[doc = "Bit 1 - Osc. Fault Interrupt Enable"]
    #[inline(always)]
    pub fn ofie(&mut self) -> OFIE_W {
        OFIE_W { w: self }
    }
    #[doc = "Bit 4 - NMI Interrupt Enable"]
    #[inline(always)]
    pub fn nmiie(&mut self) -> NMIIE_W {
        NMIIE_W { w: self }
    }
    #[doc = "Bit 5 - Flash Access Violation Interrupt Enable"]
    #[inline(always)]
    pub fn accvie(&mut self) -> ACCVIE_W {
        ACCVIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie1](index.html) module"]
pub struct IE1_SPEC;
impl crate::RegisterSpec for IE1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ie1::R](R) reader structure"]
impl crate::Readable for IE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ie1::W](W) writer structure"]
impl crate::Writable for IE1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IE1 to value 0"]
impl crate::Resettable for IE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
