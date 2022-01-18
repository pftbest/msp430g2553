#[doc = "Register `IFG1` reader"]
pub struct R(crate::R<IFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFG1` writer"]
pub struct W(crate::W<IFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFG1_SPEC>;
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
impl From<crate::W<IFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTIFG` reader - Watchdog Interrupt Flag"]
pub struct WDTIFG_R(crate::FieldReader<bool, bool>);
impl WDTIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDTIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTIFG` writer - Watchdog Interrupt Flag"]
pub struct WDTIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTIFG_W<'a> {
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
#[doc = "Field `OFIFG` reader - Osc. Fault Interrupt Flag"]
pub struct OFIFG_R(crate::FieldReader<bool, bool>);
impl OFIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OFIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFIFG` writer - Osc. Fault Interrupt Flag"]
pub struct OFIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> OFIFG_W<'a> {
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
#[doc = "Field `PORIFG` reader - Power On Interrupt Flag"]
pub struct PORIFG_R(crate::FieldReader<bool, bool>);
impl PORIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PORIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORIFG` writer - Power On Interrupt Flag"]
pub struct PORIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PORIFG_W<'a> {
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
#[doc = "Field `RSTIFG` reader - Reset Interrupt Flag"]
pub struct RSTIFG_R(crate::FieldReader<bool, bool>);
impl RSTIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RSTIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTIFG` writer - Reset Interrupt Flag"]
pub struct RSTIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTIFG_W<'a> {
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
#[doc = "Field `NMIIFG` reader - NMI Interrupt Flag"]
pub struct NMIIFG_R(crate::FieldReader<bool, bool>);
impl NMIIFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NMIIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NMIIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NMIIFG` writer - NMI Interrupt Flag"]
pub struct NMIIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIIFG_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Watchdog Interrupt Flag"]
    #[inline(always)]
    pub fn wdtifg(&self) -> WDTIFG_R {
        WDTIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Osc. Fault Interrupt Flag"]
    #[inline(always)]
    pub fn ofifg(&self) -> OFIFG_R {
        OFIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Power On Interrupt Flag"]
    #[inline(always)]
    pub fn porifg(&self) -> PORIFG_R {
        PORIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reset Interrupt Flag"]
    #[inline(always)]
    pub fn rstifg(&self) -> RSTIFG_R {
        RSTIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NMI Interrupt Flag"]
    #[inline(always)]
    pub fn nmiifg(&self) -> NMIIFG_R {
        NMIIFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Interrupt Flag"]
    #[inline(always)]
    pub fn wdtifg(&mut self) -> WDTIFG_W {
        WDTIFG_W { w: self }
    }
    #[doc = "Bit 1 - Osc. Fault Interrupt Flag"]
    #[inline(always)]
    pub fn ofifg(&mut self) -> OFIFG_W {
        OFIFG_W { w: self }
    }
    #[doc = "Bit 2 - Power On Interrupt Flag"]
    #[inline(always)]
    pub fn porifg(&mut self) -> PORIFG_W {
        PORIFG_W { w: self }
    }
    #[doc = "Bit 3 - Reset Interrupt Flag"]
    #[inline(always)]
    pub fn rstifg(&mut self) -> RSTIFG_W {
        RSTIFG_W { w: self }
    }
    #[doc = "Bit 4 - NMI Interrupt Flag"]
    #[inline(always)]
    pub fn nmiifg(&mut self) -> NMIIFG_W {
        NMIIFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifg1](index.html) module"]
pub struct IFG1_SPEC;
impl crate::RegisterSpec for IFG1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ifg1::R](R) reader structure"]
impl crate::Readable for IFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ifg1::W](W) writer structure"]
impl crate::Writable for IFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFG1 to value 0"]
impl crate::Resettable for IFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
