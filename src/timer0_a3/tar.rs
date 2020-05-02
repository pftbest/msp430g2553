#[doc = "Reader of register TAR"]
pub type R = crate::R<u16, super::TAR>;
#[doc = "Writer for register TAR"]
pub type W = crate::W<u16, super::TAR>;
#[doc = "Register TAR `reset()`'s with value 0"]
impl crate::ResetValue for super::TAR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TAR`"]
pub type TAR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TAR`"]
pub struct TAR_W<'a> {
    w: &'a mut W,
}
impl<'a> TAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timer A Counter Register"]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer A Counter Register"]
    #[inline(always)]
    pub fn tar(&mut self) -> TAR_W {
        TAR_W { w: self }
    }
}
