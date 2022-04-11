#[doc = "Register `ADPLL_ATTR_CTRL_REG` reader"]
pub struct R(crate::R<ADPLL_ATTR_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_ATTR_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_ATTR_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_ATTR_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_ATTR_CTRL_REG` writer"]
pub struct W(crate::W<ADPLL_ATTR_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_ATTR_CTRL_REG_SPEC>;
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
impl From<crate::W<ADPLL_ATTR_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_ATTR_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWR_MODE_TX` reader - "]
pub struct PWR_MODE_TX_R(crate::FieldReader<bool, bool>);
impl PWR_MODE_TX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_MODE_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_MODE_TX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWR_MODE_TX` writer - "]
pub struct PWR_MODE_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_MODE_TX_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `PWR_MODE_RX` reader - "]
pub struct PWR_MODE_RX_R(crate::FieldReader<bool, bool>);
impl PWR_MODE_RX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_MODE_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_MODE_RX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWR_MODE_RX` writer - "]
pub struct PWR_MODE_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_MODE_RX_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pwr_mode_tx(&self) -> PWR_MODE_TX_R {
        PWR_MODE_TX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pwr_mode_rx(&self) -> PWR_MODE_RX_R {
        PWR_MODE_RX_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pwr_mode_tx(&mut self) -> PWR_MODE_TX_W {
        PWR_MODE_TX_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pwr_mode_rx(&mut self) -> PWR_MODE_RX_W {
        PWR_MODE_RX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_attr_ctrl_reg](index.html) module"]
pub struct ADPLL_ATTR_CTRL_REG_SPEC;
impl crate::RegisterSpec for ADPLL_ATTR_CTRL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_attr_ctrl_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_ATTR_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_attr_ctrl_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_ATTR_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_ATTR_CTRL_REG to value 0x03"]
impl crate::Resettable for ADPLL_ATTR_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
