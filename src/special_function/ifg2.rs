#[doc = "Register `IFG2` reader"]
pub struct R(crate::R<IFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IFG2_SPEC>> for R {
    fn from(reader: crate::R<IFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFG2` writer"]
pub struct W(crate::W<IFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFG2_SPEC>;
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
impl core::convert::From<crate::W<IFG2_SPEC>> for W {
    fn from(writer: crate::W<IFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCA0RXIFG` reader - UCA0RXIFG"]
pub struct UCA0RXIFG_R(crate::FieldReader<bool, bool>);
impl UCA0RXIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCA0RXIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCA0RXIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCA0RXIFG` writer - UCA0RXIFG"]
pub struct UCA0RXIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCA0RXIFG_W<'a> {
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
#[doc = "Field `UCA0TXIFG` reader - UCA0TXIFG"]
pub struct UCA0TXIFG_R(crate::FieldReader<bool, bool>);
impl UCA0TXIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCA0TXIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCA0TXIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCA0TXIFG` writer - UCA0TXIFG"]
pub struct UCA0TXIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCA0TXIFG_W<'a> {
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
#[doc = "Field `UCB0RXIFG` reader - UCB0RXIFG"]
pub struct UCB0RXIFG_R(crate::FieldReader<bool, bool>);
impl UCB0RXIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCB0RXIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCB0RXIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCB0RXIFG` writer - UCB0RXIFG"]
pub struct UCB0RXIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCB0RXIFG_W<'a> {
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
#[doc = "Field `UCB0TXIFG` reader - UCB0TXIFG"]
pub struct UCB0TXIFG_R(crate::FieldReader<bool, bool>);
impl UCB0TXIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCB0TXIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCB0TXIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCB0TXIFG` writer - UCB0TXIFG"]
pub struct UCB0TXIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCB0TXIFG_W<'a> {
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
    #[doc = "Bit 0 - UCA0RXIFG"]
    #[inline(always)]
    pub fn uca0rxifg(&self) -> UCA0RXIFG_R {
        UCA0RXIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - UCA0TXIFG"]
    #[inline(always)]
    pub fn uca0txifg(&self) -> UCA0TXIFG_R {
        UCA0TXIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - UCB0RXIFG"]
    #[inline(always)]
    pub fn ucb0rxifg(&self) -> UCB0RXIFG_R {
        UCB0RXIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - UCB0TXIFG"]
    #[inline(always)]
    pub fn ucb0txifg(&self) -> UCB0TXIFG_R {
        UCB0TXIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UCA0RXIFG"]
    #[inline(always)]
    pub fn uca0rxifg(&mut self) -> UCA0RXIFG_W {
        UCA0RXIFG_W { w: self }
    }
    #[doc = "Bit 1 - UCA0TXIFG"]
    #[inline(always)]
    pub fn uca0txifg(&mut self) -> UCA0TXIFG_W {
        UCA0TXIFG_W { w: self }
    }
    #[doc = "Bit 2 - UCB0RXIFG"]
    #[inline(always)]
    pub fn ucb0rxifg(&mut self) -> UCB0RXIFG_W {
        UCB0RXIFG_W { w: self }
    }
    #[doc = "Bit 3 - UCB0TXIFG"]
    #[inline(always)]
    pub fn ucb0txifg(&mut self) -> UCB0TXIFG_W {
        UCB0TXIFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifg2](index.html) module"]
pub struct IFG2_SPEC;
impl crate::RegisterSpec for IFG2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ifg2::R](R) reader structure"]
impl crate::Readable for IFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ifg2::W](W) writer structure"]
impl crate::Writable for IFG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFG2 to value 0"]
impl crate::Resettable for IFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
