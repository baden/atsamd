#[doc = "Reader of register DMACPUSEL0"]
pub type R = crate::R<u32, super::DMACPUSEL0>;
#[doc = "Writer for register DMACPUSEL0"]
pub type W = crate::W<u32, super::DMACPUSEL0>;
#[doc = "Register DMACPUSEL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACPUSEL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH0`"]
pub type CH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0`"]
pub struct CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `CH1`"]
pub type CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1`"]
pub struct CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CH2`"]
pub type CH2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2`"]
pub struct CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `CH3`"]
pub type CH3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3`"]
pub struct CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `CH4`"]
pub type CH4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH4`"]
pub struct CH4_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `CH5`"]
pub type CH5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH5`"]
pub struct CH5_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `CH6`"]
pub type CH6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH6`"]
pub struct CH6_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `CH7`"]
pub type CH7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH7`"]
pub struct CH7_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `CH8`"]
pub type CH8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH8`"]
pub struct CH8_W<'a> {
    w: &'a mut W,
}
impl<'a> CH8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `CH9`"]
pub type CH9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH9`"]
pub struct CH9_W<'a> {
    w: &'a mut W,
}
impl<'a> CH9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `CH10`"]
pub type CH10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH10`"]
pub struct CH10_W<'a> {
    w: &'a mut W,
}
impl<'a> CH10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `CH11`"]
pub type CH11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH11`"]
pub struct CH11_W<'a> {
    w: &'a mut W,
}
impl<'a> CH11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `CH12`"]
pub type CH12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH12`"]
pub struct CH12_W<'a> {
    w: &'a mut W,
}
impl<'a> CH12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `CH13`"]
pub type CH13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH13`"]
pub struct CH13_W<'a> {
    w: &'a mut W,
}
impl<'a> CH13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `CH14`"]
pub type CH14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH14`"]
pub struct CH14_W<'a> {
    w: &'a mut W,
}
impl<'a> CH14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `CH15`"]
pub type CH15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH15`"]
pub struct CH15_W<'a> {
    w: &'a mut W,
}
impl<'a> CH15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DMA Channel 0 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA Channel 1 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DMA Channel 2 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DMA Channel 3 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DMA Channel 4 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DMA Channel 5 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DMA Channel 6 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - DMA Channel 7 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DMA Channel 8 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch8(&self) -> CH8_R {
        CH8_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DMA Channel 9 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch9(&self) -> CH9_R {
        CH9_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - DMA Channel 10 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch10(&self) -> CH10_R {
        CH10_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 22 - DMA Channel 11 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch11(&self) -> CH11_R {
        CH11_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DMA Channel 12 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch12(&self) -> CH12_R {
        CH12_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - DMA Channel 13 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch13(&self) -> CH13_R {
        CH13_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DMA Channel 14 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch14(&self) -> CH14_R {
        CH14_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DMA Channel 15 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch15(&self) -> CH15_R {
        CH15_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Channel 0 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W {
        CH0_W { w: self }
    }
    #[doc = "Bit 2 - DMA Channel 1 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W {
        CH1_W { w: self }
    }
    #[doc = "Bit 4 - DMA Channel 2 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH2_W {
        CH2_W { w: self }
    }
    #[doc = "Bit 6 - DMA Channel 3 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch3(&mut self) -> CH3_W {
        CH3_W { w: self }
    }
    #[doc = "Bit 8 - DMA Channel 4 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch4(&mut self) -> CH4_W {
        CH4_W { w: self }
    }
    #[doc = "Bit 10 - DMA Channel 5 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch5(&mut self) -> CH5_W {
        CH5_W { w: self }
    }
    #[doc = "Bit 12 - DMA Channel 6 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch6(&mut self) -> CH6_W {
        CH6_W { w: self }
    }
    #[doc = "Bit 14 - DMA Channel 7 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch7(&mut self) -> CH7_W {
        CH7_W { w: self }
    }
    #[doc = "Bit 16 - DMA Channel 8 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch8(&mut self) -> CH8_W {
        CH8_W { w: self }
    }
    #[doc = "Bit 18 - DMA Channel 9 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch9(&mut self) -> CH9_W {
        CH9_W { w: self }
    }
    #[doc = "Bit 20 - DMA Channel 10 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch10(&mut self) -> CH10_W {
        CH10_W { w: self }
    }
    #[doc = "Bit 22 - DMA Channel 11 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch11(&mut self) -> CH11_W {
        CH11_W { w: self }
    }
    #[doc = "Bit 24 - DMA Channel 12 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch12(&mut self) -> CH12_W {
        CH12_W { w: self }
    }
    #[doc = "Bit 26 - DMA Channel 13 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch13(&mut self) -> CH13_W {
        CH13_W { w: self }
    }
    #[doc = "Bit 28 - DMA Channel 14 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch14(&mut self) -> CH14_W {
        CH14_W { w: self }
    }
    #[doc = "Bit 30 - DMA Channel 15 Interrupt CPU Select"]
    #[inline(always)]
    pub fn ch15(&mut self) -> CH15_W {
        CH15_W { w: self }
    }
}
