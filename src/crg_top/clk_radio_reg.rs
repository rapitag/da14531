#[doc = "Register `CLK_RADIO_REG` reader"]
pub struct R(crate::R<CLK_RADIO_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_RADIO_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_RADIO_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_RADIO_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_RADIO_REG` writer"]
pub struct W(crate::W<CLK_RADIO_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_RADIO_REG_SPEC>;
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
impl From<crate::W<CLK_RADIO_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_RADIO_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLE_ENABLE` reader - Enable the BLE core clocks"]
pub struct BLE_ENABLE_R(crate::FieldReader<bool, bool>);
impl BLE_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BLE_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLE_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLE_ENABLE` writer - Enable the BLE core clocks"]
pub struct BLE_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_ENABLE_W<'a> {
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
#[doc = "Field `BLE_LP_RESET` reader - Reset for the BLE LP timer"]
pub struct BLE_LP_RESET_R(crate::FieldReader<bool, bool>);
impl BLE_LP_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BLE_LP_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLE_LP_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLE_LP_RESET` writer - Reset for the BLE LP timer"]
pub struct BLE_LP_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_LP_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u16 & 1) << 6);
        self.w
    }
}
#[doc = "Field `BLE_DIV` reader - Division factor for BLE core blocks 0x0: divide by 1 0x1: divide by 2 0x2: divide by 4 0x3: divide by 8 The programmed frequency should not be lower than 8 MHz and not faster than the programmed CPU clock frequency. Refer also to BLE_CNTL2_REG\\[BLE_CLK_SEL\\]."]
pub struct BLE_DIV_R(crate::FieldReader<u8, u8>);
impl BLE_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BLE_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLE_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLE_DIV` writer - Division factor for BLE core blocks 0x0: divide by 1 0x1: divide by 2 0x2: divide by 4 0x3: divide by 8 The programmed frequency should not be lower than 8 MHz and not faster than the programmed CPU clock frequency. Refer also to BLE_CNTL2_REG\\[BLE_CLK_SEL\\]."]
pub struct BLE_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u16 & 3) << 4);
        self.w
    }
}
#[doc = "Field `RFCU_ENABLE` reader - Enable the RF control Unit clock"]
pub struct RFCU_ENABLE_R(crate::FieldReader<bool, bool>);
impl RFCU_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFCU_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFCU_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFCU_ENABLE` writer - Enable the RF control Unit clock"]
pub struct RFCU_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCU_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u16 & 1) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Enable the BLE core clocks"]
    #[inline(always)]
    pub fn ble_enable(&self) -> BLE_ENABLE_R {
        BLE_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset for the BLE LP timer"]
    #[inline(always)]
    pub fn ble_lp_reset(&self) -> BLE_LP_RESET_R {
        BLE_LP_RESET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Division factor for BLE core blocks 0x0: divide by 1 0x1: divide by 2 0x2: divide by 4 0x3: divide by 8 The programmed frequency should not be lower than 8 MHz and not faster than the programmed CPU clock frequency. Refer also to BLE_CNTL2_REG\\[BLE_CLK_SEL\\]."]
    #[inline(always)]
    pub fn ble_div(&self) -> BLE_DIV_R {
        BLE_DIV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 3 - Enable the RF control Unit clock"]
    #[inline(always)]
    pub fn rfcu_enable(&self) -> RFCU_ENABLE_R {
        RFCU_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Enable the BLE core clocks"]
    #[inline(always)]
    pub fn ble_enable(&mut self) -> BLE_ENABLE_W {
        BLE_ENABLE_W { w: self }
    }
    #[doc = "Bit 6 - Reset for the BLE LP timer"]
    #[inline(always)]
    pub fn ble_lp_reset(&mut self) -> BLE_LP_RESET_W {
        BLE_LP_RESET_W { w: self }
    }
    #[doc = "Bits 4:5 - Division factor for BLE core blocks 0x0: divide by 1 0x1: divide by 2 0x2: divide by 4 0x3: divide by 8 The programmed frequency should not be lower than 8 MHz and not faster than the programmed CPU clock frequency. Refer also to BLE_CNTL2_REG\\[BLE_CLK_SEL\\]."]
    #[inline(always)]
    pub fn ble_div(&mut self) -> BLE_DIV_W {
        BLE_DIV_W { w: self }
    }
    #[doc = "Bit 3 - Enable the RF control Unit clock"]
    #[inline(always)]
    pub fn rfcu_enable(&mut self) -> RFCU_ENABLE_W {
        RFCU_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Radio PLL control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_radio_reg](index.html) module"]
pub struct CLK_RADIO_REG_SPEC;
impl crate::RegisterSpec for CLK_RADIO_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [clk_radio_reg::R](R) reader structure"]
impl crate::Readable for CLK_RADIO_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_radio_reg::W](W) writer structure"]
impl crate::Writable for CLK_RADIO_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_RADIO_REG to value 0x40"]
impl crate::Resettable for CLK_RADIO_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
