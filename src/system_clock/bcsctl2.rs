#[doc = "Register `BCSCTL2` reader"]
pub struct R(crate::R<BCSCTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCSCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCSCTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCSCTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCSCTL2` writer"]
pub struct W(crate::W<BCSCTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCSCTL2_SPEC>;
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
impl From<crate::W<BCSCTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCSCTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SMCLK Divider 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVS_A {
    #[doc = "0: SMCLK Divider 0: /1"]
    DIVS_0 = 0,
    #[doc = "1: SMCLK Divider 1: /2"]
    DIVS_1 = 1,
    #[doc = "2: SMCLK Divider 2: /4"]
    DIVS_2 = 2,
    #[doc = "3: SMCLK Divider 3: /8"]
    DIVS_3 = 3,
}
impl From<DIVS_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DIVS` reader - SMCLK Divider 0"]
pub struct DIVS_R(crate::FieldReader<u8, DIVS_A>);
impl DIVS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIVS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVS_A {
        match self.bits {
            0 => DIVS_A::DIVS_0,
            1 => DIVS_A::DIVS_1,
            2 => DIVS_A::DIVS_2,
            3 => DIVS_A::DIVS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVS_0`"]
    #[inline(always)]
    pub fn is_divs_0(&self) -> bool {
        **self == DIVS_A::DIVS_0
    }
    #[doc = "Checks if the value of the field is `DIVS_1`"]
    #[inline(always)]
    pub fn is_divs_1(&self) -> bool {
        **self == DIVS_A::DIVS_1
    }
    #[doc = "Checks if the value of the field is `DIVS_2`"]
    #[inline(always)]
    pub fn is_divs_2(&self) -> bool {
        **self == DIVS_A::DIVS_2
    }
    #[doc = "Checks if the value of the field is `DIVS_3`"]
    #[inline(always)]
    pub fn is_divs_3(&self) -> bool {
        **self == DIVS_A::DIVS_3
    }
}
impl core::ops::Deref for DIVS_R {
    type Target = crate::FieldReader<u8, DIVS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVS` writer - SMCLK Divider 0"]
pub struct DIVS_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "SMCLK Divider 0: /1"]
    #[inline(always)]
    pub fn divs_0(self) -> &'a mut W {
        self.variant(DIVS_A::DIVS_0)
    }
    #[doc = "SMCLK Divider 1: /2"]
    #[inline(always)]
    pub fn divs_1(self) -> &'a mut W {
        self.variant(DIVS_A::DIVS_1)
    }
    #[doc = "SMCLK Divider 2: /4"]
    #[inline(always)]
    pub fn divs_2(self) -> &'a mut W {
        self.variant(DIVS_A::DIVS_2)
    }
    #[doc = "SMCLK Divider 3: /8"]
    #[inline(always)]
    pub fn divs_3(self) -> &'a mut W {
        self.variant(DIVS_A::DIVS_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 1)) | ((value as u8 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `SELS` reader - SMCLK Source Select 0:DCOCLK / 1:XT2CLK/LFXTCLK"]
pub struct SELS_R(crate::FieldReader<bool, bool>);
impl SELS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SELS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SELS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELS` writer - SMCLK Source Select 0:DCOCLK / 1:XT2CLK/LFXTCLK"]
pub struct SELS_W<'a> {
    w: &'a mut W,
}
impl<'a> SELS_W<'a> {
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
#[doc = "MCLK Divider 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVM_A {
    #[doc = "0: MCLK Divider 0: /1"]
    DIVM_0 = 0,
    #[doc = "1: MCLK Divider 1: /2"]
    DIVM_1 = 1,
    #[doc = "2: MCLK Divider 2: /4"]
    DIVM_2 = 2,
    #[doc = "3: MCLK Divider 3: /8"]
    DIVM_3 = 3,
}
impl From<DIVM_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DIVM` reader - MCLK Divider 0"]
pub struct DIVM_R(crate::FieldReader<u8, DIVM_A>);
impl DIVM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIVM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVM_A {
        match self.bits {
            0 => DIVM_A::DIVM_0,
            1 => DIVM_A::DIVM_1,
            2 => DIVM_A::DIVM_2,
            3 => DIVM_A::DIVM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVM_0`"]
    #[inline(always)]
    pub fn is_divm_0(&self) -> bool {
        **self == DIVM_A::DIVM_0
    }
    #[doc = "Checks if the value of the field is `DIVM_1`"]
    #[inline(always)]
    pub fn is_divm_1(&self) -> bool {
        **self == DIVM_A::DIVM_1
    }
    #[doc = "Checks if the value of the field is `DIVM_2`"]
    #[inline(always)]
    pub fn is_divm_2(&self) -> bool {
        **self == DIVM_A::DIVM_2
    }
    #[doc = "Checks if the value of the field is `DIVM_3`"]
    #[inline(always)]
    pub fn is_divm_3(&self) -> bool {
        **self == DIVM_A::DIVM_3
    }
}
impl core::ops::Deref for DIVM_R {
    type Target = crate::FieldReader<u8, DIVM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVM` writer - MCLK Divider 0"]
pub struct DIVM_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVM_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "MCLK Divider 0: /1"]
    #[inline(always)]
    pub fn divm_0(self) -> &'a mut W {
        self.variant(DIVM_A::DIVM_0)
    }
    #[doc = "MCLK Divider 1: /2"]
    #[inline(always)]
    pub fn divm_1(self) -> &'a mut W {
        self.variant(DIVM_A::DIVM_1)
    }
    #[doc = "MCLK Divider 2: /4"]
    #[inline(always)]
    pub fn divm_2(self) -> &'a mut W {
        self.variant(DIVM_A::DIVM_2)
    }
    #[doc = "MCLK Divider 3: /8"]
    #[inline(always)]
    pub fn divm_3(self) -> &'a mut W {
        self.variant(DIVM_A::DIVM_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 4)) | ((value as u8 & 0x03) << 4);
        self.w
    }
}
#[doc = "MCLK Source Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SELM_A {
    #[doc = "0: MCLK Source Select 0: DCOCLK"]
    SELM_0 = 0,
    #[doc = "1: MCLK Source Select 1: DCOCLK"]
    SELM_1 = 1,
    #[doc = "2: MCLK Source Select 2: XT2CLK/LFXTCLK"]
    SELM_2 = 2,
    #[doc = "3: MCLK Source Select 3: LFXTCLK"]
    SELM_3 = 3,
}
impl From<SELM_A> for u8 {
    #[inline(always)]
    fn from(variant: SELM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SELM` reader - MCLK Source Select 0"]
pub struct SELM_R(crate::FieldReader<u8, SELM_A>);
impl SELM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SELM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELM_A {
        match self.bits {
            0 => SELM_A::SELM_0,
            1 => SELM_A::SELM_1,
            2 => SELM_A::SELM_2,
            3 => SELM_A::SELM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SELM_0`"]
    #[inline(always)]
    pub fn is_selm_0(&self) -> bool {
        **self == SELM_A::SELM_0
    }
    #[doc = "Checks if the value of the field is `SELM_1`"]
    #[inline(always)]
    pub fn is_selm_1(&self) -> bool {
        **self == SELM_A::SELM_1
    }
    #[doc = "Checks if the value of the field is `SELM_2`"]
    #[inline(always)]
    pub fn is_selm_2(&self) -> bool {
        **self == SELM_A::SELM_2
    }
    #[doc = "Checks if the value of the field is `SELM_3`"]
    #[inline(always)]
    pub fn is_selm_3(&self) -> bool {
        **self == SELM_A::SELM_3
    }
}
impl core::ops::Deref for SELM_R {
    type Target = crate::FieldReader<u8, SELM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELM` writer - MCLK Source Select 0"]
pub struct SELM_W<'a> {
    w: &'a mut W,
}
impl<'a> SELM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELM_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "MCLK Source Select 0: DCOCLK"]
    #[inline(always)]
    pub fn selm_0(self) -> &'a mut W {
        self.variant(SELM_A::SELM_0)
    }
    #[doc = "MCLK Source Select 1: DCOCLK"]
    #[inline(always)]
    pub fn selm_1(self) -> &'a mut W {
        self.variant(SELM_A::SELM_1)
    }
    #[doc = "MCLK Source Select 2: XT2CLK/LFXTCLK"]
    #[inline(always)]
    pub fn selm_2(self) -> &'a mut W {
        self.variant(SELM_A::SELM_2)
    }
    #[doc = "MCLK Source Select 3: LFXTCLK"]
    #[inline(always)]
    pub fn selm_3(self) -> &'a mut W {
        self.variant(SELM_A::SELM_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 6)) | ((value as u8 & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:2 - SMCLK Divider 0"]
    #[inline(always)]
    pub fn divs(&self) -> DIVS_R {
        DIVS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - SMCLK Source Select 0:DCOCLK / 1:XT2CLK/LFXTCLK"]
    #[inline(always)]
    pub fn sels(&self) -> SELS_R {
        SELS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - MCLK Divider 0"]
    #[inline(always)]
    pub fn divm(&self) -> DIVM_R {
        DIVM_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - MCLK Source Select 0"]
    #[inline(always)]
    pub fn selm(&self) -> SELM_R {
        SELM_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 1:2 - SMCLK Divider 0"]
    #[inline(always)]
    pub fn divs(&mut self) -> DIVS_W {
        DIVS_W { w: self }
    }
    #[doc = "Bit 3 - SMCLK Source Select 0:DCOCLK / 1:XT2CLK/LFXTCLK"]
    #[inline(always)]
    pub fn sels(&mut self) -> SELS_W {
        SELS_W { w: self }
    }
    #[doc = "Bits 4:5 - MCLK Divider 0"]
    #[inline(always)]
    pub fn divm(&mut self) -> DIVM_W {
        DIVM_W { w: self }
    }
    #[doc = "Bits 6:7 - MCLK Source Select 0"]
    #[inline(always)]
    pub fn selm(&mut self) -> SELM_W {
        SELM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Basic Clock System Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcsctl2](index.html) module"]
pub struct BCSCTL2_SPEC;
impl crate::RegisterSpec for BCSCTL2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bcsctl2::R](R) reader structure"]
impl crate::Readable for BCSCTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcsctl2::W](W) writer structure"]
impl crate::Writable for BCSCTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BCSCTL2 to value 0"]
impl crate::Resettable for BCSCTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
