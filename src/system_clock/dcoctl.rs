#[doc = "Reader of register DCOCTL"]
pub type R = crate::R<u8, super::DCOCTL>;
#[doc = "Writer for register DCOCTL"]
pub type W = crate::W<u8, super::DCOCTL>;
#[doc = "Register DCOCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::DCOCTL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DCOCTL`"]
pub type DCOCTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DCOCTL`"]
pub struct DCOCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> DCOCTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DCO`"]
pub type DCO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DCO`"]
pub struct DCO_W<'a> {
    w: &'a mut W,
}
impl<'a> DCO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x07 << 5)) | (((value as u8) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `MOD`"]
pub type MOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MOD`"]
pub struct MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u8) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DCO Clock Frequency Control register"]
    #[inline(always)]
    pub fn dcoctl(&self) -> DCOCTL_R {
        DCOCTL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 5:7 - DCO Select Bit 0"]
    #[inline(always)]
    pub fn dco(&self) -> DCO_R {
        DCO_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 0:4 - Modulation Bit 0"]
    #[inline(always)]
    pub fn mod_(&self) -> MOD_R {
        MOD_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DCO Clock Frequency Control register"]
    #[inline(always)]
    pub fn dcoctl(&mut self) -> DCOCTL_W {
        DCOCTL_W { w: self }
    }
    #[doc = "Bits 5:7 - DCO Select Bit 0"]
    #[inline(always)]
    pub fn dco(&mut self) -> DCO_W {
        DCO_W { w: self }
    }
    #[doc = "Bits 0:4 - Modulation Bit 0"]
    #[inline(always)]
    pub fn mod_(&mut self) -> MOD_W {
        MOD_W { w: self }
    }
}
