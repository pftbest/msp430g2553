#[doc = "Reader of register ADC10CTL0"]
pub type R = crate::R<u16, super::ADC10CTL0>;
#[doc = "Writer for register ADC10CTL0"]
pub type W = crate::W<u16, super::ADC10CTL0>;
#[doc = "Register ADC10CTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC10CTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC10SC`"]
pub type ADC10SC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC10SC`"]
pub struct ADC10SC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10SC_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ENC`"]
pub type ENC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENC`"]
pub struct ENC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENC_W<'a> {
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
            (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `ADC10IFG`"]
pub type ADC10IFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC10IFG`"]
pub struct ADC10IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10IFG_W<'a> {
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
            (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `ADC10IE`"]
pub type ADC10IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC10IE`"]
pub struct ADC10IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10IE_W<'a> {
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
            (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `ADC10ON`"]
pub type ADC10ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC10ON`"]
pub struct ADC10ON_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10ON_W<'a> {
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
            (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `REFON`"]
pub type REFON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFON`"]
pub struct REFON_W<'a> {
    w: &'a mut W,
}
impl<'a> REFON_W<'a> {
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
            (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `REF2_5V`"]
pub type REF2_5V_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REF2_5V`"]
pub struct REF2_5V_W<'a> {
    w: &'a mut W,
}
impl<'a> REF2_5V_W<'a> {
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
            (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `MSC`"]
pub type MSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSC`"]
pub struct MSC_W<'a> {
    w: &'a mut W,
}
impl<'a> MSC_W<'a> {
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
            (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `REFBURST`"]
pub type REFBURST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFBURST`"]
pub struct REFBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> REFBURST_W<'a> {
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
            (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `REFOUT`"]
pub type REFOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFOUT`"]
pub struct REFOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> REFOUT_W<'a> {
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
            (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `ADC10SR`"]
pub type ADC10SR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC10SR`"]
pub struct ADC10SR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10SR_W<'a> {
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
            (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "ADC10 Sample Hold Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC10SHT_A {
    #[doc = "0: 4 x ADC10CLKs"]
    ADC10SHT_0 = 0,
    #[doc = "1: 8 x ADC10CLKs"]
    ADC10SHT_1 = 1,
    #[doc = "2: 16 x ADC10CLKs"]
    ADC10SHT_2 = 2,
    #[doc = "3: 64 x ADC10CLKs"]
    ADC10SHT_3 = 3,
}
impl From<ADC10SHT_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC10SHT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC10SHT`"]
pub type ADC10SHT_R = crate::R<u8, ADC10SHT_A>;
impl ADC10SHT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC10SHT_A {
        match self.bits {
            0 => ADC10SHT_A::ADC10SHT_0,
            1 => ADC10SHT_A::ADC10SHT_1,
            2 => ADC10SHT_A::ADC10SHT_2,
            3 => ADC10SHT_A::ADC10SHT_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC10SHT_0`"]
    #[inline(always)]
    pub fn is_adc10sht_0(&self) -> bool {
        *self == ADC10SHT_A::ADC10SHT_0
    }
    #[doc = "Checks if the value of the field is `ADC10SHT_1`"]
    #[inline(always)]
    pub fn is_adc10sht_1(&self) -> bool {
        *self == ADC10SHT_A::ADC10SHT_1
    }
    #[doc = "Checks if the value of the field is `ADC10SHT_2`"]
    #[inline(always)]
    pub fn is_adc10sht_2(&self) -> bool {
        *self == ADC10SHT_A::ADC10SHT_2
    }
    #[doc = "Checks if the value of the field is `ADC10SHT_3`"]
    #[inline(always)]
    pub fn is_adc10sht_3(&self) -> bool {
        *self == ADC10SHT_A::ADC10SHT_3
    }
}
#[doc = "Write proxy for field `ADC10SHT`"]
pub struct ADC10SHT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10SHT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC10SHT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "4 x ADC10CLKs"]
    #[inline(always)]
    pub fn adc10sht_0(self) -> &'a mut W {
        self.variant(ADC10SHT_A::ADC10SHT_0)
    }
    #[doc = "8 x ADC10CLKs"]
    #[inline(always)]
    pub fn adc10sht_1(self) -> &'a mut W {
        self.variant(ADC10SHT_A::ADC10SHT_1)
    }
    #[doc = "16 x ADC10CLKs"]
    #[inline(always)]
    pub fn adc10sht_2(self) -> &'a mut W {
        self.variant(ADC10SHT_A::ADC10SHT_2)
    }
    #[doc = "64 x ADC10CLKs"]
    #[inline(always)]
    pub fn adc10sht_3(self) -> &'a mut W {
        self.variant(ADC10SHT_A::ADC10SHT_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 11)) | (((value as u16) & 0x03) << 11);
        self.w
    }
}
#[doc = "ADC10 Reference Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SREF_A {
    #[doc = "0: VR+ = AVCC and VR- = AVSS"]
    SREF_0 = 0,
    #[doc = "1: VR+ = VREF+ and VR- = AVSS"]
    SREF_1 = 1,
    #[doc = "2: VR+ = VEREF+ and VR- = AVSS"]
    SREF_2 = 2,
    #[doc = "3: VR+ = VEREF+ and VR- = AVSS"]
    SREF_3 = 3,
    #[doc = "4: VR+ = AVCC and VR- = VREF-/VEREF-"]
    SREF_4 = 4,
    #[doc = "5: VR+ = VREF+ and VR- = VREF-/VEREF-"]
    SREF_5 = 5,
    #[doc = "6: VR+ = VEREF+ and VR- = VREF-/VEREF-"]
    SREF_6 = 6,
    #[doc = "7: VR+ = VEREF+ and VR- = VREF-/VEREF-"]
    SREF_7 = 7,
}
impl From<SREF_A> for u8 {
    #[inline(always)]
    fn from(variant: SREF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SREF`"]
pub type SREF_R = crate::R<u8, SREF_A>;
impl SREF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SREF_A {
        match self.bits {
            0 => SREF_A::SREF_0,
            1 => SREF_A::SREF_1,
            2 => SREF_A::SREF_2,
            3 => SREF_A::SREF_3,
            4 => SREF_A::SREF_4,
            5 => SREF_A::SREF_5,
            6 => SREF_A::SREF_6,
            7 => SREF_A::SREF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SREF_0`"]
    #[inline(always)]
    pub fn is_sref_0(&self) -> bool {
        *self == SREF_A::SREF_0
    }
    #[doc = "Checks if the value of the field is `SREF_1`"]
    #[inline(always)]
    pub fn is_sref_1(&self) -> bool {
        *self == SREF_A::SREF_1
    }
    #[doc = "Checks if the value of the field is `SREF_2`"]
    #[inline(always)]
    pub fn is_sref_2(&self) -> bool {
        *self == SREF_A::SREF_2
    }
    #[doc = "Checks if the value of the field is `SREF_3`"]
    #[inline(always)]
    pub fn is_sref_3(&self) -> bool {
        *self == SREF_A::SREF_3
    }
    #[doc = "Checks if the value of the field is `SREF_4`"]
    #[inline(always)]
    pub fn is_sref_4(&self) -> bool {
        *self == SREF_A::SREF_4
    }
    #[doc = "Checks if the value of the field is `SREF_5`"]
    #[inline(always)]
    pub fn is_sref_5(&self) -> bool {
        *self == SREF_A::SREF_5
    }
    #[doc = "Checks if the value of the field is `SREF_6`"]
    #[inline(always)]
    pub fn is_sref_6(&self) -> bool {
        *self == SREF_A::SREF_6
    }
    #[doc = "Checks if the value of the field is `SREF_7`"]
    #[inline(always)]
    pub fn is_sref_7(&self) -> bool {
        *self == SREF_A::SREF_7
    }
}
#[doc = "Write proxy for field `SREF`"]
pub struct SREF_W<'a> {
    w: &'a mut W,
}
impl<'a> SREF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SREF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "VR+ = AVCC and VR- = AVSS"]
    #[inline(always)]
    pub fn sref_0(self) -> &'a mut W {
        self.variant(SREF_A::SREF_0)
    }
    #[doc = "VR+ = VREF+ and VR- = AVSS"]
    #[inline(always)]
    pub fn sref_1(self) -> &'a mut W {
        self.variant(SREF_A::SREF_1)
    }
    #[doc = "VR+ = VEREF+ and VR- = AVSS"]
    #[inline(always)]
    pub fn sref_2(self) -> &'a mut W {
        self.variant(SREF_A::SREF_2)
    }
    #[doc = "VR+ = VEREF+ and VR- = AVSS"]
    #[inline(always)]
    pub fn sref_3(self) -> &'a mut W {
        self.variant(SREF_A::SREF_3)
    }
    #[doc = "VR+ = AVCC and VR- = VREF-/VEREF-"]
    #[inline(always)]
    pub fn sref_4(self) -> &'a mut W {
        self.variant(SREF_A::SREF_4)
    }
    #[doc = "VR+ = VREF+ and VR- = VREF-/VEREF-"]
    #[inline(always)]
    pub fn sref_5(self) -> &'a mut W {
        self.variant(SREF_A::SREF_5)
    }
    #[doc = "VR+ = VEREF+ and VR- = VREF-/VEREF-"]
    #[inline(always)]
    pub fn sref_6(self) -> &'a mut W {
        self.variant(SREF_A::SREF_6)
    }
    #[doc = "VR+ = VEREF+ and VR- = VREF-/VEREF-"]
    #[inline(always)]
    pub fn sref_7(self) -> &'a mut W {
        self.variant(SREF_A::SREF_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x07 << 13)) | (((value as u16) & 0x07) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ADC10 Start Conversion"]
    #[inline(always)]
    pub fn adc10sc(&self) -> ADC10SC_R {
        ADC10SC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC10 Enable Conversion"]
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC10 Interrupt Flag"]
    #[inline(always)]
    pub fn adc10ifg(&self) -> ADC10IFG_R {
        ADC10IFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC10 Interrupt Enalbe"]
    #[inline(always)]
    pub fn adc10ie(&self) -> ADC10IE_R {
        ADC10IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC10 On/Enable"]
    #[inline(always)]
    pub fn adc10on(&self) -> ADC10ON_R {
        ADC10ON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC10 Reference on"]
    #[inline(always)]
    pub fn refon(&self) -> REFON_R {
        REFON_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ADC10 Ref 0:1.5V / 1:2.5V"]
    #[inline(always)]
    pub fn ref2_5v(&self) -> REF2_5V_R {
        REF2_5V_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC10 Multiple SampleConversion"]
    #[inline(always)]
    pub fn msc(&self) -> MSC_R {
        MSC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC10 Reference Burst Mode"]
    #[inline(always)]
    pub fn refburst(&self) -> REFBURST_R {
        REFBURST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC10 Enalbe output of Ref."]
    #[inline(always)]
    pub fn refout(&self) -> REFOUT_R {
        REFOUT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ADC10 Sampling Rate 0:200ksps / 1:50ksps"]
    #[inline(always)]
    pub fn adc10sr(&self) -> ADC10SR_R {
        ADC10SR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:12 - ADC10 Sample Hold Select Bit: 0"]
    #[inline(always)]
    pub fn adc10sht(&self) -> ADC10SHT_R {
        ADC10SHT_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 13:15 - ADC10 Reference Select Bit: 0"]
    #[inline(always)]
    pub fn sref(&self) -> SREF_R {
        SREF_R::new(((self.bits >> 13) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC10 Start Conversion"]
    #[inline(always)]
    pub fn adc10sc(&mut self) -> ADC10SC_W {
        ADC10SC_W { w: self }
    }
    #[doc = "Bit 1 - ADC10 Enable Conversion"]
    #[inline(always)]
    pub fn enc(&mut self) -> ENC_W {
        ENC_W { w: self }
    }
    #[doc = "Bit 2 - ADC10 Interrupt Flag"]
    #[inline(always)]
    pub fn adc10ifg(&mut self) -> ADC10IFG_W {
        ADC10IFG_W { w: self }
    }
    #[doc = "Bit 3 - ADC10 Interrupt Enalbe"]
    #[inline(always)]
    pub fn adc10ie(&mut self) -> ADC10IE_W {
        ADC10IE_W { w: self }
    }
    #[doc = "Bit 4 - ADC10 On/Enable"]
    #[inline(always)]
    pub fn adc10on(&mut self) -> ADC10ON_W {
        ADC10ON_W { w: self }
    }
    #[doc = "Bit 5 - ADC10 Reference on"]
    #[inline(always)]
    pub fn refon(&mut self) -> REFON_W {
        REFON_W { w: self }
    }
    #[doc = "Bit 6 - ADC10 Ref 0:1.5V / 1:2.5V"]
    #[inline(always)]
    pub fn ref2_5v(&mut self) -> REF2_5V_W {
        REF2_5V_W { w: self }
    }
    #[doc = "Bit 7 - ADC10 Multiple SampleConversion"]
    #[inline(always)]
    pub fn msc(&mut self) -> MSC_W {
        MSC_W { w: self }
    }
    #[doc = "Bit 8 - ADC10 Reference Burst Mode"]
    #[inline(always)]
    pub fn refburst(&mut self) -> REFBURST_W {
        REFBURST_W { w: self }
    }
    #[doc = "Bit 9 - ADC10 Enalbe output of Ref."]
    #[inline(always)]
    pub fn refout(&mut self) -> REFOUT_W {
        REFOUT_W { w: self }
    }
    #[doc = "Bit 10 - ADC10 Sampling Rate 0:200ksps / 1:50ksps"]
    #[inline(always)]
    pub fn adc10sr(&mut self) -> ADC10SR_W {
        ADC10SR_W { w: self }
    }
    #[doc = "Bits 11:12 - ADC10 Sample Hold Select Bit: 0"]
    #[inline(always)]
    pub fn adc10sht(&mut self) -> ADC10SHT_W {
        ADC10SHT_W { w: self }
    }
    #[doc = "Bits 13:15 - ADC10 Reference Select Bit: 0"]
    #[inline(always)]
    pub fn sref(&mut self) -> SREF_W {
        SREF_W { w: self }
    }
}
