#[doc = "Register `ADPLL_DIV_CTRL_REG` reader"]
pub struct R(crate::R<ADPLL_DIV_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_DIV_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_DIV_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_DIV_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_DIV_CTRL_REG` writer"]
pub struct W(crate::W<ADPLL_DIV_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_DIV_CTRL_REG_SPEC>;
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
impl From<crate::W<ADPLL_DIV_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_DIV_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDIV_TRIM` reader - "]
pub struct TXDIV_TRIM_R(crate::FieldReader<u16, u16>);
impl TXDIV_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TXDIV_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDIV_TRIM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDIV_TRIM` writer - "]
pub struct TXDIV_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDIV_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 17)) | ((value as u32 & 0x01ff) << 17);
        self.w
    }
}
#[doc = "Field `RXDIV_TRIM` reader - "]
pub struct RXDIV_TRIM_R(crate::FieldReader<u16, u16>);
impl RXDIV_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RXDIV_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDIV_TRIM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDIV_TRIM` writer - "]
pub struct RXDIV_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDIV_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 8)) | ((value as u32 & 0x01ff) << 8);
        self.w
    }
}
#[doc = "Field `RXDIV_FB_EN_TX` reader - "]
pub struct RXDIV_FB_EN_TX_R(crate::FieldReader<bool, bool>);
impl RXDIV_FB_EN_TX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXDIV_FB_EN_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDIV_FB_EN_TX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDIV_FB_EN_TX` writer - "]
pub struct RXDIV_FB_EN_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDIV_FB_EN_TX_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `RXDIV_FB_EN_RX` reader - "]
pub struct RXDIV_FB_EN_RX_R(crate::FieldReader<bool, bool>);
impl RXDIV_FB_EN_RX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXDIV_FB_EN_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDIV_FB_EN_RX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDIV_FB_EN_RX` writer - "]
pub struct RXDIV_FB_EN_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDIV_FB_EN_RX_W<'a> {
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
#[doc = "Field `FBDIV_EN` reader - "]
pub struct FBDIV_EN_R(crate::FieldReader<bool, bool>);
impl FBDIV_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FBDIV_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBDIV_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FBDIV_EN` writer - "]
pub struct FBDIV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FBDIV_EN_W<'a> {
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
    #[doc = "Bits 17:25"]
    #[inline(always)]
    pub fn txdiv_trim(&self) -> TXDIV_TRIM_R {
        TXDIV_TRIM_R::new(((self.bits >> 17) & 0x01ff) as u16)
    }
    #[doc = "Bits 8:16"]
    #[inline(always)]
    pub fn rxdiv_trim(&self) -> RXDIV_TRIM_R {
        RXDIV_TRIM_R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rxdiv_fb_en_tx(&self) -> RXDIV_FB_EN_TX_R {
        RXDIV_FB_EN_TX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rxdiv_fb_en_rx(&self) -> RXDIV_FB_EN_RX_R {
        RXDIV_FB_EN_RX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fbdiv_en(&self) -> FBDIV_EN_R {
        FBDIV_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 17:25"]
    #[inline(always)]
    pub fn txdiv_trim(&mut self) -> TXDIV_TRIM_W {
        TXDIV_TRIM_W { w: self }
    }
    #[doc = "Bits 8:16"]
    #[inline(always)]
    pub fn rxdiv_trim(&mut self) -> RXDIV_TRIM_W {
        RXDIV_TRIM_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rxdiv_fb_en_tx(&mut self) -> RXDIV_FB_EN_TX_W {
        RXDIV_FB_EN_TX_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rxdiv_fb_en_rx(&mut self) -> RXDIV_FB_EN_RX_W {
        RXDIV_FB_EN_RX_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fbdiv_en(&mut self) -> FBDIV_EN_W {
        FBDIV_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_div_ctrl_reg](index.html) module"]
pub struct ADPLL_DIV_CTRL_REG_SPEC;
impl crate::RegisterSpec for ADPLL_DIV_CTRL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_div_ctrl_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_DIV_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_div_ctrl_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_DIV_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_DIV_CTRL_REG to value 0x0198_ff03"]
impl crate::Resettable for ADPLL_DIV_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0198_ff03
    }
}
