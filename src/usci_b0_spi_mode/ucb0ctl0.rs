#[doc = "Register `UCB0CTL0` reader"]
pub struct R(crate::R<UCB0CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UCB0CTL0_SPEC>> for R {
    fn from(reader: crate::R<UCB0CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0CTL0` writer"]
pub struct W(crate::W<UCB0CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0CTL0_SPEC>;
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
impl core::convert::From<crate::W<UCB0CTL0_SPEC>> for W {
    fn from(writer: crate::W<UCB0CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCSYNC` reader - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
pub struct UCSYNC_R(crate::FieldReader<bool, bool>);
impl UCSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCSYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCSYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSYNC` writer - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
pub struct UCSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSYNC_W<'a> {
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
#[doc = "Sync. Mode: USCI Mode 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UCMODE_A {
    #[doc = "0: Sync. Mode: USCI Mode: 0"]
    UCMODE_0 = 0,
    #[doc = "1: Sync. Mode: USCI Mode: 1"]
    UCMODE_1 = 1,
    #[doc = "2: Sync. Mode: USCI Mode: 2"]
    UCMODE_2 = 2,
    #[doc = "3: Sync. Mode: USCI Mode: 3"]
    UCMODE_3 = 3,
}
impl From<UCMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: UCMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UCMODE` reader - Sync. Mode: USCI Mode 1"]
pub struct UCMODE_R(crate::FieldReader<u8, UCMODE_A>);
impl UCMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        UCMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCMODE_A {
        match self.bits {
            0 => UCMODE_A::UCMODE_0,
            1 => UCMODE_A::UCMODE_1,
            2 => UCMODE_A::UCMODE_2,
            3 => UCMODE_A::UCMODE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCMODE_0`"]
    #[inline(always)]
    pub fn is_ucmode_0(&self) -> bool {
        **self == UCMODE_A::UCMODE_0
    }
    #[doc = "Checks if the value of the field is `UCMODE_1`"]
    #[inline(always)]
    pub fn is_ucmode_1(&self) -> bool {
        **self == UCMODE_A::UCMODE_1
    }
    #[doc = "Checks if the value of the field is `UCMODE_2`"]
    #[inline(always)]
    pub fn is_ucmode_2(&self) -> bool {
        **self == UCMODE_A::UCMODE_2
    }
    #[doc = "Checks if the value of the field is `UCMODE_3`"]
    #[inline(always)]
    pub fn is_ucmode_3(&self) -> bool {
        **self == UCMODE_A::UCMODE_3
    }
}
impl core::ops::Deref for UCMODE_R {
    type Target = crate::FieldReader<u8, UCMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCMODE` writer - Sync. Mode: USCI Mode 1"]
pub struct UCMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCMODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Sync. Mode: USCI Mode: 0"]
    #[inline(always)]
    pub fn ucmode_0(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_0)
    }
    #[doc = "Sync. Mode: USCI Mode: 1"]
    #[inline(always)]
    pub fn ucmode_1(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_1)
    }
    #[doc = "Sync. Mode: USCI Mode: 2"]
    #[inline(always)]
    pub fn ucmode_2(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_2)
    }
    #[doc = "Sync. Mode: USCI Mode: 3"]
    #[inline(always)]
    pub fn ucmode_3(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 1)) | ((value as u8 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `UCMST` reader - Sync. Mode: Master Select"]
pub struct UCMST_R(crate::FieldReader<bool, bool>);
impl UCMST_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCMST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCMST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCMST` writer - Sync. Mode: Master Select"]
pub struct UCMST_W<'a> {
    w: &'a mut W,
}
impl<'a> UCMST_W<'a> {
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
#[doc = "Field `UC7BIT` reader - Sync. Mode: Data Bits 0:8-bits / 1:7-bits"]
pub struct UC7BIT_R(crate::FieldReader<bool, bool>);
impl UC7BIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        UC7BIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UC7BIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UC7BIT` writer - Sync. Mode: Data Bits 0:8-bits / 1:7-bits"]
pub struct UC7BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> UC7BIT_W<'a> {
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
#[doc = "Field `UCMSB` reader - Sync. Mode: MSB first 0:LSB / 1:MSB"]
pub struct UCMSB_R(crate::FieldReader<bool, bool>);
impl UCMSB_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCMSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCMSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCMSB` writer - Sync. Mode: MSB first 0:LSB / 1:MSB"]
pub struct UCMSB_W<'a> {
    w: &'a mut W,
}
impl<'a> UCMSB_W<'a> {
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
#[doc = "Field `UCCKPL` reader - Sync. Mode: Clock Polarity"]
pub struct UCCKPL_R(crate::FieldReader<bool, bool>);
impl UCCKPL_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCCKPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCCKPL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCCKPL` writer - Sync. Mode: Clock Polarity"]
pub struct UCCKPL_W<'a> {
    w: &'a mut W,
}
impl<'a> UCCKPL_W<'a> {
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
            (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `UCCKPH` reader - Sync. Mode: Clock Phase"]
pub struct UCCKPH_R(crate::FieldReader<bool, bool>);
impl UCCKPH_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCCKPH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCCKPH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCCKPH` writer - Sync. Mode: Clock Phase"]
pub struct UCCKPH_W<'a> {
    w: &'a mut W,
}
impl<'a> UCCKPH_W<'a> {
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
            (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
    #[inline(always)]
    pub fn ucsync(&self) -> UCSYNC_R {
        UCSYNC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Sync. Mode: USCI Mode 1"]
    #[inline(always)]
    pub fn ucmode(&self) -> UCMODE_R {
        UCMODE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Sync. Mode: Master Select"]
    #[inline(always)]
    pub fn ucmst(&self) -> UCMST_R {
        UCMST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Sync. Mode: Data Bits 0:8-bits / 1:7-bits"]
    #[inline(always)]
    pub fn uc7bit(&self) -> UC7BIT_R {
        UC7BIT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Sync. Mode: MSB first 0:LSB / 1:MSB"]
    #[inline(always)]
    pub fn ucmsb(&self) -> UCMSB_R {
        UCMSB_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Sync. Mode: Clock Polarity"]
    #[inline(always)]
    pub fn ucckpl(&self) -> UCCKPL_R {
        UCCKPL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Sync. Mode: Clock Phase"]
    #[inline(always)]
    pub fn ucckph(&self) -> UCCKPH_R {
        UCCKPH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
    #[inline(always)]
    pub fn ucsync(&mut self) -> UCSYNC_W {
        UCSYNC_W { w: self }
    }
    #[doc = "Bits 1:2 - Sync. Mode: USCI Mode 1"]
    #[inline(always)]
    pub fn ucmode(&mut self) -> UCMODE_W {
        UCMODE_W { w: self }
    }
    #[doc = "Bit 3 - Sync. Mode: Master Select"]
    #[inline(always)]
    pub fn ucmst(&mut self) -> UCMST_W {
        UCMST_W { w: self }
    }
    #[doc = "Bit 4 - Sync. Mode: Data Bits 0:8-bits / 1:7-bits"]
    #[inline(always)]
    pub fn uc7bit(&mut self) -> UC7BIT_W {
        UC7BIT_W { w: self }
    }
    #[doc = "Bit 5 - Sync. Mode: MSB first 0:LSB / 1:MSB"]
    #[inline(always)]
    pub fn ucmsb(&mut self) -> UCMSB_W {
        UCMSB_W { w: self }
    }
    #[doc = "Bit 6 - Sync. Mode: Clock Polarity"]
    #[inline(always)]
    pub fn ucckpl(&mut self) -> UCCKPL_W {
        UCCKPL_W { w: self }
    }
    #[doc = "Bit 7 - Sync. Mode: Clock Phase"]
    #[inline(always)]
    pub fn ucckph(&mut self) -> UCCKPH_W {
        UCCKPH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B0 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0ctl0](index.html) module"]
pub struct UCB0CTL0_SPEC;
impl crate::RegisterSpec for UCB0CTL0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucb0ctl0::R](R) reader structure"]
impl crate::Readable for UCB0CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0ctl0::W](W) writer structure"]
impl crate::Writable for UCB0CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0CTL0 to value 0"]
impl crate::Resettable for UCB0CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
