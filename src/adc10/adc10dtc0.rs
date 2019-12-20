#[doc = "Reader of register ADC10DTC0"]
pub type R = crate::R<u8, super::ADC10DTC0>;
#[doc = "Writer for register ADC10DTC0"]
pub type W = crate::W<u8, super::ADC10DTC0>;
#[doc = "Register ADC10DTC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC10DTC0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC10FETCH`"]
pub type ADC10FETCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC10FETCH`"]
pub struct ADC10FETCH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10FETCH_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ADC10B1`"]
pub type ADC10B1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC10B1`"]
pub struct ADC10B1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10B1_W<'a> {
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
            (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `ADC10CT`"]
pub type ADC10CT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC10CT`"]
pub struct ADC10CT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10CT_W<'a> {
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
            (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `ADC10TB`"]
pub type ADC10TB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC10TB`"]
pub struct ADC10TB_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10TB_W<'a> {
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
            (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - This bit should normally be reset"]
    #[inline(always)]
    pub fn adc10fetch(&self) -> ADC10FETCH_R {
        ADC10FETCH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC10 block one"]
    #[inline(always)]
    pub fn adc10b1(&self) -> ADC10B1_R {
        ADC10B1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC10 continuous transfer"]
    #[inline(always)]
    pub fn adc10ct(&self) -> ADC10CT_R {
        ADC10CT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC10 two-block mode"]
    #[inline(always)]
    pub fn adc10tb(&self) -> ADC10TB_R {
        ADC10TB_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit should normally be reset"]
    #[inline(always)]
    pub fn adc10fetch(&mut self) -> ADC10FETCH_W {
        ADC10FETCH_W { w: self }
    }
    #[doc = "Bit 1 - ADC10 block one"]
    #[inline(always)]
    pub fn adc10b1(&mut self) -> ADC10B1_W {
        ADC10B1_W { w: self }
    }
    #[doc = "Bit 2 - ADC10 continuous transfer"]
    #[inline(always)]
    pub fn adc10ct(&mut self) -> ADC10CT_W {
        ADC10CT_W { w: self }
    }
    #[doc = "Bit 3 - ADC10 two-block mode"]
    #[inline(always)]
    pub fn adc10tb(&mut self) -> ADC10TB_W {
        ADC10TB_W { w: self }
    }
}
