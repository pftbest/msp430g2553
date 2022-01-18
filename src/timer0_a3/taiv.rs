#[doc = "Register `TAIV` reader"]
pub struct R(crate::R<TAIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAIV` writer"]
pub struct W(crate::W<TAIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAIV_SPEC>;
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
impl From<crate::W<TAIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Timer A Interrupt Vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TAIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Capture/Compare 1"]
    TACCR1 = 2,
    #[doc = "4: Capture/Compare 2"]
    TACCR2 = 4,
    #[doc = "10: Timer overflow"]
    TAIFG = 10,
}
impl From<TAIV_A> for u8 {
    #[inline(always)]
    fn from(variant: TAIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TAIV` reader - Timer A Interrupt Vector value"]
pub struct TAIV_R(crate::FieldReader<u8, TAIV_A>);
impl TAIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TAIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TAIV_A> {
        match self.bits {
            0 => Some(TAIV_A::NONE),
            2 => Some(TAIV_A::TACCR1),
            4 => Some(TAIV_A::TACCR2),
            10 => Some(TAIV_A::TAIFG),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == TAIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `TACCR1`"]
    #[inline(always)]
    pub fn is_taccr1(&self) -> bool {
        **self == TAIV_A::TACCR1
    }
    #[doc = "Checks if the value of the field is `TACCR2`"]
    #[inline(always)]
    pub fn is_taccr2(&self) -> bool {
        **self == TAIV_A::TACCR2
    }
    #[doc = "Checks if the value of the field is `TAIFG`"]
    #[inline(always)]
    pub fn is_taifg(&self) -> bool {
        **self == TAIV_A::TAIFG
    }
}
impl core::ops::Deref for TAIV_R {
    type Target = crate::FieldReader<u8, TAIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAIV` writer - Timer A Interrupt Vector value"]
pub struct TAIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TAIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(TAIV_A::NONE)
    }
    #[doc = "Capture/Compare 1"]
    #[inline(always)]
    pub fn taccr1(self) -> &'a mut W {
        self.variant(TAIV_A::TACCR1)
    }
    #[doc = "Capture/Compare 2"]
    #[inline(always)]
    pub fn taccr2(self) -> &'a mut W {
        self.variant(TAIV_A::TACCR2)
    }
    #[doc = "Timer overflow"]
    #[inline(always)]
    pub fn taifg(self) -> &'a mut W {
        self.variant(TAIV_A::TAIFG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u16 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Timer A Interrupt Vector value"]
    #[inline(always)]
    pub fn taiv(&self) -> TAIV_R {
        TAIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Timer A Interrupt Vector value"]
    #[inline(always)]
    pub fn taiv(&mut self) -> TAIV_W {
        TAIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer0_A3 Interrupt Vector Word\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taiv](index.html) module"]
pub struct TAIV_SPEC;
impl crate::RegisterSpec for TAIV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [taiv::R](R) reader structure"]
impl crate::Readable for TAIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [taiv::W](W) writer structure"]
impl crate::Writable for TAIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAIV to value 0"]
impl crate::Resettable for TAIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
