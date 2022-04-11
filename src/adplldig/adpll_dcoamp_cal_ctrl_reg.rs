#[doc = "Register `ADPLL_DCOAMP_CAL_CTRL_REG` reader"]
pub struct R(crate::R<ADPLL_DCOAMP_CAL_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_DCOAMP_CAL_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_DCOAMP_CAL_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_DCOAMP_CAL_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_DCOAMP_CAL_CTRL_REG` writer"]
pub struct W(crate::W<ADPLL_DCOAMP_CAL_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_DCOAMP_CAL_CTRL_REG_SPEC>;
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
impl From<crate::W<ADPLL_DCOAMP_CAL_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_DCOAMP_CAL_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCOAMPIC_LP_TX` reader - "]
pub struct DCOAMPIC_LP_TX_R(crate::FieldReader<u8, u8>);
impl DCOAMPIC_LP_TX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DCOAMPIC_LP_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCOAMPIC_LP_TX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCOAMPIC_LP_TX` writer - "]
pub struct DCOAMPIC_LP_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> DCOAMPIC_LP_TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `DCOAMPIC_LP_RX` reader - "]
pub struct DCOAMPIC_LP_RX_R(crate::FieldReader<u8, u8>);
impl DCOAMPIC_LP_RX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DCOAMPIC_LP_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCOAMPIC_LP_RX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCOAMPIC_LP_RX` writer - "]
pub struct DCOAMPIC_LP_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> DCOAMPIC_LP_RX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `DCOAMPIC_HP_TX` reader - "]
pub struct DCOAMPIC_HP_TX_R(crate::FieldReader<u8, u8>);
impl DCOAMPIC_HP_TX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DCOAMPIC_HP_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCOAMPIC_HP_TX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCOAMPIC_HP_TX` writer - "]
pub struct DCOAMPIC_HP_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> DCOAMPIC_HP_TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `DCOAMPIC_HP_RX` reader - "]
pub struct DCOAMPIC_HP_RX_R(crate::FieldReader<u8, u8>);
impl DCOAMPIC_HP_RX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DCOAMPIC_HP_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCOAMPIC_HP_RX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCOAMPIC_HP_RX` writer - "]
pub struct DCOAMPIC_HP_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> DCOAMPIC_HP_RX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `DCOAMPTM` reader - "]
pub struct DCOAMPTM_R(crate::FieldReader<bool, bool>);
impl DCOAMPTM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCOAMPTM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCOAMPTM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCOAMPTM` writer - "]
pub struct DCOAMPTM_W<'a> {
    w: &'a mut W,
}
impl<'a> DCOAMPTM_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `AMPCALEN` reader - "]
pub struct AMPCALEN_R(crate::FieldReader<bool, bool>);
impl AMPCALEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AMPCALEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMPCALEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMPCALEN` writer - "]
pub struct AMPCALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AMPCALEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `KMEDIUM` reader - "]
pub struct KMEDIUM_R(crate::FieldReader<u8, u8>);
impl KMEDIUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KMEDIUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KMEDIUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KMEDIUM` writer - "]
pub struct KMEDIUM_W<'a> {
    w: &'a mut W,
}
impl<'a> KMEDIUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 3)) | ((value as u32 & 7) << 3);
        self.w
    }
}
#[doc = "Field `KCOARSE` reader - "]
pub struct KCOARSE_R(crate::FieldReader<u8, u8>);
impl KCOARSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KCOARSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KCOARSE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KCOARSE` writer - "]
pub struct KCOARSE_W<'a> {
    w: &'a mut W,
}
impl<'a> KCOARSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u32 & 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn dcoampic_lp_tx(&self) -> DCOAMPIC_LP_TX_R {
        DCOAMPIC_LP_TX_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn dcoampic_lp_rx(&self) -> DCOAMPIC_LP_RX_R {
        DCOAMPIC_LP_RX_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn dcoampic_hp_tx(&self) -> DCOAMPIC_HP_TX_R {
        DCOAMPIC_HP_TX_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn dcoampic_hp_rx(&self) -> DCOAMPIC_HP_RX_R {
        DCOAMPIC_HP_RX_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dcoamptm(&self) -> DCOAMPTM_R {
        DCOAMPTM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ampcalen(&self) -> AMPCALEN_R {
        AMPCALEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn kmedium(&self) -> KMEDIUM_R {
        KMEDIUM_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn kcoarse(&self) -> KCOARSE_R {
        KCOARSE_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn dcoampic_lp_tx(&mut self) -> DCOAMPIC_LP_TX_W {
        DCOAMPIC_LP_TX_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn dcoampic_lp_rx(&mut self) -> DCOAMPIC_LP_RX_W {
        DCOAMPIC_LP_RX_W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn dcoampic_hp_tx(&mut self) -> DCOAMPIC_HP_TX_W {
        DCOAMPIC_HP_TX_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn dcoampic_hp_rx(&mut self) -> DCOAMPIC_HP_RX_W {
        DCOAMPIC_HP_RX_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dcoamptm(&mut self) -> DCOAMPTM_W {
        DCOAMPTM_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ampcalen(&mut self) -> AMPCALEN_W {
        AMPCALEN_W { w: self }
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn kmedium(&mut self) -> KMEDIUM_W {
        KMEDIUM_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn kcoarse(&mut self) -> KCOARSE_W {
        KCOARSE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_dcoamp_cal_ctrl_reg](index.html) module"]
pub struct ADPLL_DCOAMP_CAL_CTRL_REG_SPEC;
impl crate::RegisterSpec for ADPLL_DCOAMP_CAL_CTRL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_dcoamp_cal_ctrl_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_DCOAMP_CAL_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_dcoamp_cal_ctrl_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_DCOAMP_CAL_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_DCOAMP_CAL_CTRL_REG to value 0x7777_0011"]
impl crate::Resettable for ADPLL_DCOAMP_CAL_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7777_0011
    }
}
