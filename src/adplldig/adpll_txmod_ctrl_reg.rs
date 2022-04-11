#[doc = "Register `ADPLL_TXMOD_CTRL_REG` reader"]
pub struct R(crate::R<ADPLL_TXMOD_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_TXMOD_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_TXMOD_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_TXMOD_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_TXMOD_CTRL_REG` writer"]
pub struct W(crate::W<ADPLL_TXMOD_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_TXMOD_CTRL_REG_SPEC>;
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
impl From<crate::W<ADPLL_TXMOD_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_TXMOD_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INV_CKMODEXT` reader - "]
pub struct INV_CKMODEXT_R(crate::FieldReader<bool, bool>);
impl INV_CKMODEXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INV_CKMODEXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INV_CKMODEXT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INV_CKMODEXT` writer - "]
pub struct INV_CKMODEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_CKMODEXT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `TX_MODE` reader - "]
pub struct TX_MODE_R(crate::FieldReader<u8, u8>);
impl TX_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_MODE` writer - "]
pub struct TX_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
#[doc = "Field `EO_PACKET_DIS` reader - "]
pub struct EO_PACKET_DIS_R(crate::FieldReader<bool, bool>);
impl EO_PACKET_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EO_PACKET_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EO_PACKET_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EO_PACKET_DIS` writer - "]
pub struct EO_PACKET_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EO_PACKET_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `MOD_INDEX` reader - "]
pub struct MOD_INDEX_R(crate::FieldReader<u8, u8>);
impl MOD_INDEX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MOD_INDEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOD_INDEX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOD_INDEX` writer - "]
pub struct MOD_INDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_INDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u32 & 3) << 2);
        self.w
    }
}
#[doc = "Field `TX_DATA_INV` reader - "]
pub struct TX_DATA_INV_R(crate::FieldReader<bool, bool>);
impl TX_DATA_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_DATA_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DATA_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DATA_INV` writer - "]
pub struct TX_DATA_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DATA_INV_W<'a> {
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
#[doc = "Field `BT_SEL` reader - "]
pub struct BT_SEL_R(crate::FieldReader<bool, bool>);
impl BT_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BT_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BT_SEL` writer - "]
pub struct BT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BT_SEL_W<'a> {
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
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn inv_ckmodext(&self) -> INV_CKMODEXT_R {
        INV_CKMODEXT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn tx_mode(&self) -> TX_MODE_R {
        TX_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn eo_packet_dis(&self) -> EO_PACKET_DIS_R {
        EO_PACKET_DIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn mod_index(&self) -> MOD_INDEX_R {
        MOD_INDEX_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_data_inv(&self) -> TX_DATA_INV_R {
        TX_DATA_INV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bt_sel(&self) -> BT_SEL_R {
        BT_SEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn inv_ckmodext(&mut self) -> INV_CKMODEXT_W {
        INV_CKMODEXT_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn tx_mode(&mut self) -> TX_MODE_W {
        TX_MODE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn eo_packet_dis(&mut self) -> EO_PACKET_DIS_W {
        EO_PACKET_DIS_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn mod_index(&mut self) -> MOD_INDEX_W {
        MOD_INDEX_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_data_inv(&mut self) -> TX_DATA_INV_W {
        TX_DATA_INV_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bt_sel(&mut self) -> BT_SEL_W {
        BT_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_txmod_ctrl_reg](index.html) module"]
pub struct ADPLL_TXMOD_CTRL_REG_SPEC;
impl crate::RegisterSpec for ADPLL_TXMOD_CTRL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_txmod_ctrl_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_TXMOD_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_txmod_ctrl_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_TXMOD_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_TXMOD_CTRL_REG to value 0"]
impl crate::Resettable for ADPLL_TXMOD_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
