#[doc = "Register `TACTL` reader"]
pub struct R(crate::R<TACTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TACTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TACTL_SPEC>> for R {
    fn from(reader: crate::R<TACTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TACTL` writer"]
pub struct W(crate::W<TACTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TACTL_SPEC>;
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
impl core::convert::From<crate::W<TACTL_SPEC>> for W {
    fn from(writer: crate::W<TACTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAIFG` reader - Timer A counter interrupt flag"]
pub struct TAIFG_R(crate::FieldReader<bool, bool>);
impl TAIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAIFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAIFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAIFG` writer - Timer A counter interrupt flag"]
pub struct TAIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAIFG_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Field `TAIE` reader - Timer A counter interrupt enable"]
pub struct TAIE_R(crate::FieldReader<bool, bool>);
impl TAIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAIE` writer - Timer A counter interrupt enable"]
pub struct TAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAIE_W<'a> {
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
            (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `TACLR` reader - Timer A counter clear"]
pub struct TACLR_R(crate::FieldReader<bool, bool>);
impl TACLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TACLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TACLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TACLR` writer - Timer A counter clear"]
pub struct TACLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TACLR_W<'a> {
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
            (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Timer A mode control 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MC_A {
    #[doc = "0: Timer A mode control: 0 - Stop"]
    MC_0 = 0,
    #[doc = "1: Timer A mode control: 1 - Up to CCR0"]
    MC_1 = 1,
    #[doc = "2: Timer A mode control: 2 - Continous up"]
    MC_2 = 2,
    #[doc = "3: Timer A mode control: 3 - Up/Down"]
    MC_3 = 3,
}
impl From<MC_A> for u8 {
    #[inline(always)]
    fn from(variant: MC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MC` reader - Timer A mode control 1"]
pub struct MC_R(crate::FieldReader<u8, MC_A>);
impl MC_R {
    pub(crate) fn new(bits: u8) -> Self {
        MC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MC_A {
        match self.bits {
            0 => MC_A::MC_0,
            1 => MC_A::MC_1,
            2 => MC_A::MC_2,
            3 => MC_A::MC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MC_0`"]
    #[inline(always)]
    pub fn is_mc_0(&self) -> bool {
        **self == MC_A::MC_0
    }
    #[doc = "Checks if the value of the field is `MC_1`"]
    #[inline(always)]
    pub fn is_mc_1(&self) -> bool {
        **self == MC_A::MC_1
    }
    #[doc = "Checks if the value of the field is `MC_2`"]
    #[inline(always)]
    pub fn is_mc_2(&self) -> bool {
        **self == MC_A::MC_2
    }
    #[doc = "Checks if the value of the field is `MC_3`"]
    #[inline(always)]
    pub fn is_mc_3(&self) -> bool {
        **self == MC_A::MC_3
    }
}
impl core::ops::Deref for MC_R {
    type Target = crate::FieldReader<u8, MC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MC` writer - Timer A mode control 1"]
pub struct MC_W<'a> {
    w: &'a mut W,
}
impl<'a> MC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Timer A mode control: 0 - Stop"]
    #[inline(always)]
    pub fn mc_0(self) -> &'a mut W {
        self.variant(MC_A::MC_0)
    }
    #[doc = "Timer A mode control: 1 - Up to CCR0"]
    #[inline(always)]
    pub fn mc_1(self) -> &'a mut W {
        self.variant(MC_A::MC_1)
    }
    #[doc = "Timer A mode control: 2 - Continous up"]
    #[inline(always)]
    pub fn mc_2(self) -> &'a mut W {
        self.variant(MC_A::MC_2)
    }
    #[doc = "Timer A mode control: 3 - Up/Down"]
    #[inline(always)]
    pub fn mc_3(self) -> &'a mut W {
        self.variant(MC_A::MC_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 4)) | ((value as u16 & 0x03) << 4);
        self.w
    }
}
#[doc = "Timer A clock input divider 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ID_A {
    #[doc = "0: Timer A input divider: 0 - /1"]
    ID_0 = 0,
    #[doc = "1: Timer A input divider: 1 - /2"]
    ID_1 = 1,
    #[doc = "2: Timer A input divider: 2 - /4"]
    ID_2 = 2,
    #[doc = "3: Timer A input divider: 3 - /8"]
    ID_3 = 3,
}
impl From<ID_A> for u8 {
    #[inline(always)]
    fn from(variant: ID_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ID` reader - Timer A clock input divider 1"]
pub struct ID_R(crate::FieldReader<u8, ID_A>);
impl ID_R {
    pub(crate) fn new(bits: u8) -> Self {
        ID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ID_A {
        match self.bits {
            0 => ID_A::ID_0,
            1 => ID_A::ID_1,
            2 => ID_A::ID_2,
            3 => ID_A::ID_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ID_0`"]
    #[inline(always)]
    pub fn is_id_0(&self) -> bool {
        **self == ID_A::ID_0
    }
    #[doc = "Checks if the value of the field is `ID_1`"]
    #[inline(always)]
    pub fn is_id_1(&self) -> bool {
        **self == ID_A::ID_1
    }
    #[doc = "Checks if the value of the field is `ID_2`"]
    #[inline(always)]
    pub fn is_id_2(&self) -> bool {
        **self == ID_A::ID_2
    }
    #[doc = "Checks if the value of the field is `ID_3`"]
    #[inline(always)]
    pub fn is_id_3(&self) -> bool {
        **self == ID_A::ID_3
    }
}
impl core::ops::Deref for ID_R {
    type Target = crate::FieldReader<u8, ID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ID` writer - Timer A clock input divider 1"]
pub struct ID_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ID_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Timer A input divider: 0 - /1"]
    #[inline(always)]
    pub fn id_0(self) -> &'a mut W {
        self.variant(ID_A::ID_0)
    }
    #[doc = "Timer A input divider: 1 - /2"]
    #[inline(always)]
    pub fn id_1(self) -> &'a mut W {
        self.variant(ID_A::ID_1)
    }
    #[doc = "Timer A input divider: 2 - /4"]
    #[inline(always)]
    pub fn id_2(self) -> &'a mut W {
        self.variant(ID_A::ID_2)
    }
    #[doc = "Timer A input divider: 3 - /8"]
    #[inline(always)]
    pub fn id_3(self) -> &'a mut W {
        self.variant(ID_A::ID_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 6)) | ((value as u16 & 0x03) << 6);
        self.w
    }
}
#[doc = "Timer A clock source select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TASSEL_A {
    #[doc = "0: Timer A clock source select: 0 - TACLK"]
    TASSEL_0 = 0,
    #[doc = "1: Timer A clock source select: 1 - ACLK"]
    TASSEL_1 = 1,
    #[doc = "2: Timer A clock source select: 2 - SMCLK"]
    TASSEL_2 = 2,
    #[doc = "3: Timer A clock source select: 3 - INCLK"]
    TASSEL_3 = 3,
}
impl From<TASSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TASSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TASSEL` reader - Timer A clock source select 1"]
pub struct TASSEL_R(crate::FieldReader<u8, TASSEL_A>);
impl TASSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TASSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TASSEL_A {
        match self.bits {
            0 => TASSEL_A::TASSEL_0,
            1 => TASSEL_A::TASSEL_1,
            2 => TASSEL_A::TASSEL_2,
            3 => TASSEL_A::TASSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TASSEL_0`"]
    #[inline(always)]
    pub fn is_tassel_0(&self) -> bool {
        **self == TASSEL_A::TASSEL_0
    }
    #[doc = "Checks if the value of the field is `TASSEL_1`"]
    #[inline(always)]
    pub fn is_tassel_1(&self) -> bool {
        **self == TASSEL_A::TASSEL_1
    }
    #[doc = "Checks if the value of the field is `TASSEL_2`"]
    #[inline(always)]
    pub fn is_tassel_2(&self) -> bool {
        **self == TASSEL_A::TASSEL_2
    }
    #[doc = "Checks if the value of the field is `TASSEL_3`"]
    #[inline(always)]
    pub fn is_tassel_3(&self) -> bool {
        **self == TASSEL_A::TASSEL_3
    }
}
impl core::ops::Deref for TASSEL_R {
    type Target = crate::FieldReader<u8, TASSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TASSEL` writer - Timer A clock source select 1"]
pub struct TASSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TASSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TASSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Timer A clock source select: 0 - TACLK"]
    #[inline(always)]
    pub fn tassel_0(self) -> &'a mut W {
        self.variant(TASSEL_A::TASSEL_0)
    }
    #[doc = "Timer A clock source select: 1 - ACLK"]
    #[inline(always)]
    pub fn tassel_1(self) -> &'a mut W {
        self.variant(TASSEL_A::TASSEL_1)
    }
    #[doc = "Timer A clock source select: 2 - SMCLK"]
    #[inline(always)]
    pub fn tassel_2(self) -> &'a mut W {
        self.variant(TASSEL_A::TASSEL_2)
    }
    #[doc = "Timer A clock source select: 3 - INCLK"]
    #[inline(always)]
    pub fn tassel_3(self) -> &'a mut W {
        self.variant(TASSEL_A::TASSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 8)) | ((value as u16 & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Timer A counter interrupt flag"]
    #[inline(always)]
    pub fn taifg(&self) -> TAIFG_R {
        TAIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer A counter interrupt enable"]
    #[inline(always)]
    pub fn taie(&self) -> TAIE_R {
        TAIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer A counter clear"]
    #[inline(always)]
    pub fn taclr(&self) -> TACLR_R {
        TACLR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Timer A mode control 1"]
    #[inline(always)]
    pub fn mc(&self) -> MC_R {
        MC_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Timer A clock input divider 1"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Timer A clock source select 1"]
    #[inline(always)]
    pub fn tassel(&self) -> TASSEL_R {
        TASSEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Timer A counter interrupt flag"]
    #[inline(always)]
    pub fn taifg(&mut self) -> TAIFG_W {
        TAIFG_W { w: self }
    }
    #[doc = "Bit 1 - Timer A counter interrupt enable"]
    #[inline(always)]
    pub fn taie(&mut self) -> TAIE_W {
        TAIE_W { w: self }
    }
    #[doc = "Bit 2 - Timer A counter clear"]
    #[inline(always)]
    pub fn taclr(&mut self) -> TACLR_W {
        TACLR_W { w: self }
    }
    #[doc = "Bits 4:5 - Timer A mode control 1"]
    #[inline(always)]
    pub fn mc(&mut self) -> MC_W {
        MC_W { w: self }
    }
    #[doc = "Bits 6:7 - Timer A clock input divider 1"]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W { w: self }
    }
    #[doc = "Bits 8:9 - Timer A clock source select 1"]
    #[inline(always)]
    pub fn tassel(&mut self) -> TASSEL_W {
        TASSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer0_A3 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tactl](index.html) module"]
pub struct TACTL_SPEC;
impl crate::RegisterSpec for TACTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tactl::R](R) reader structure"]
impl crate::Readable for TACTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tactl::W](W) writer structure"]
impl crate::Writable for TACTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TACTL to value 0"]
impl crate::Resettable for TACTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
