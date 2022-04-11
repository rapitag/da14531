#[doc = "Register `CLK_RCX_REG` reader"]
pub struct R(crate::R<CLK_RCX_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_RCX_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_RCX_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_RCX_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_RCX_REG` writer"]
pub struct W(crate::W<CLK_RCX_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_RCX_REG_SPEC>;
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
impl From<crate::W<CLK_RCX_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_RCX_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCX_BIAS` reader - LDO bias current. 0x0: minimum 0xF: maximum"]
pub struct RCX_BIAS_R(crate::FieldReader<u8, u8>);
impl RCX_BIAS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RCX_BIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCX_BIAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCX_BIAS` writer - LDO bias current. 0x0: minimum 0xF: maximum"]
pub struct RCX_BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> RCX_BIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u16 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `RCX_C0` reader - Add unit capacitance to RC-time delay."]
pub struct RCX_C0_R(crate::FieldReader<bool, bool>);
impl RCX_C0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCX_C0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCX_C0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCX_C0` writer - Add unit capacitance to RC-time delay."]
pub struct RCX_C0_W<'a> {
    w: &'a mut W,
}
impl<'a> RCX_C0_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u16 & 1) << 7);
        self.w
    }
}
#[doc = "Field `RCX_CADJUST` reader - Adjust capacitance part of RC-time delay. 0x00: minimum capacitance 0x1F: maximum capacitance"]
pub struct RCX_CADJUST_R(crate::FieldReader<u8, u8>);
impl RCX_CADJUST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RCX_CADJUST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCX_CADJUST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCX_CADJUST` writer - Adjust capacitance part of RC-time delay. 0x00: minimum capacitance 0x1F: maximum capacitance"]
pub struct RCX_CADJUST_W<'a> {
    w: &'a mut W,
}
impl<'a> RCX_CADJUST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 2)) | ((value as u16 & 0x1f) << 2);
        self.w
    }
}
#[doc = "Field `RCX_RADJUST` reader - Adjust resistance part of RC-time delay. Lower resistance increases power consumption. 0x0: maximum resistance 0x1: minimum resistance"]
pub struct RCX_RADJUST_R(crate::FieldReader<bool, bool>);
impl RCX_RADJUST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCX_RADJUST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCX_RADJUST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCX_RADJUST` writer - Adjust resistance part of RC-time delay. Lower resistance increases power consumption. 0x0: maximum resistance 0x1: minimum resistance"]
pub struct RCX_RADJUST_W<'a> {
    w: &'a mut W,
}
impl<'a> RCX_RADJUST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u16 & 1) << 1);
        self.w
    }
}
#[doc = "Field `RCX_ENABLE` reader - Enable the RCX oscillator"]
pub struct RCX_ENABLE_R(crate::FieldReader<bool, bool>);
impl RCX_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCX_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCX_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCX_ENABLE` writer - Enable the RCX oscillator"]
pub struct RCX_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RCX_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u16 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:11 - LDO bias current. 0x0: minimum 0xF: maximum"]
    #[inline(always)]
    pub fn rcx_bias(&self) -> RCX_BIAS_R {
        RCX_BIAS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Add unit capacitance to RC-time delay."]
    #[inline(always)]
    pub fn rcx_c0(&self) -> RCX_C0_R {
        RCX_C0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Adjust capacitance part of RC-time delay. 0x00: minimum capacitance 0x1F: maximum capacitance"]
    #[inline(always)]
    pub fn rcx_cadjust(&self) -> RCX_CADJUST_R {
        RCX_CADJUST_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 1 - Adjust resistance part of RC-time delay. Lower resistance increases power consumption. 0x0: maximum resistance 0x1: minimum resistance"]
    #[inline(always)]
    pub fn rcx_radjust(&self) -> RCX_RADJUST_R {
        RCX_RADJUST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Enable the RCX oscillator"]
    #[inline(always)]
    pub fn rcx_enable(&self) -> RCX_ENABLE_R {
        RCX_ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:11 - LDO bias current. 0x0: minimum 0xF: maximum"]
    #[inline(always)]
    pub fn rcx_bias(&mut self) -> RCX_BIAS_W {
        RCX_BIAS_W { w: self }
    }
    #[doc = "Bit 7 - Add unit capacitance to RC-time delay."]
    #[inline(always)]
    pub fn rcx_c0(&mut self) -> RCX_C0_W {
        RCX_C0_W { w: self }
    }
    #[doc = "Bits 2:6 - Adjust capacitance part of RC-time delay. 0x00: minimum capacitance 0x1F: maximum capacitance"]
    #[inline(always)]
    pub fn rcx_cadjust(&mut self) -> RCX_CADJUST_W {
        RCX_CADJUST_W { w: self }
    }
    #[doc = "Bit 1 - Adjust resistance part of RC-time delay. Lower resistance increases power consumption. 0x0: maximum resistance 0x1: minimum resistance"]
    #[inline(always)]
    pub fn rcx_radjust(&mut self) -> RCX_RADJUST_W {
        RCX_RADJUST_W { w: self }
    }
    #[doc = "Bit 0 - Enable the RCX oscillator"]
    #[inline(always)]
    pub fn rcx_enable(&mut self) -> RCX_ENABLE_W {
        RCX_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCX-oscillator control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_rcx_reg](index.html) module"]
pub struct CLK_RCX_REG_SPEC;
impl crate::RegisterSpec for CLK_RCX_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [clk_rcx_reg::R](R) reader structure"]
impl crate::Readable for CLK_RCX_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_rcx_reg::W](W) writer structure"]
impl crate::Writable for CLK_RCX_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_RCX_REG to value 0x0afc"]
impl crate::Resettable for CLK_RCX_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0afc
    }
}
