#[doc = "Reader of register TLV_CHECKSUM"]
pub type R = crate::R<u16, super::TLV_CHECKSUM>;
#[doc = "Writer for register TLV_CHECKSUM"]
pub type W = crate::W<u16, super::TLV_CHECKSUM>;
#[doc = "Register TLV_CHECKSUM `reset()`'s with value 0"]
impl crate::ResetValue for super::TLV_CHECKSUM {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TLV_CHECKSUM`"]
pub type TLV_CHECKSUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TLV_CHECKSUM`"]
pub struct TLV_CHECKSUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TLV_CHECKSUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - TLV CHECK SUM register"]
    #[inline(always)]
    pub fn tlv_checksum(&self) -> TLV_CHECKSUM_R {
        TLV_CHECKSUM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TLV CHECK SUM register"]
    #[inline(always)]
    pub fn tlv_checksum(&mut self) -> TLV_CHECKSUM_W {
        TLV_CHECKSUM_W { w: self }
    }
}
