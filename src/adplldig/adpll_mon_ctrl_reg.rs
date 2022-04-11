#[doc = "Register `ADPLL_MON_CTRL_REG` reader"]
pub struct R(crate::R<ADPLL_MON_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_MON_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_MON_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_MON_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_MON_CTRL_REG` writer"]
pub struct W(crate::W<ADPLL_MON_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_MON_CTRL_REG_SPEC>;
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
impl From<crate::W<ADPLL_MON_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_MON_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QUALMONFRCEN` reader - "]
pub struct QUALMONFRCEN_R(crate::FieldReader<bool, bool>);
impl QUALMONFRCEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QUALMONFRCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QUALMONFRCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QUALMONFRCEN` writer - "]
pub struct QUALMONFRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> QUALMONFRCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "Field `QUALMONTRHLD` reader - "]
pub struct QUALMONTRHLD_R(crate::FieldReader<u8, u8>);
impl QUALMONTRHLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        QUALMONTRHLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QUALMONTRHLD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QUALMONTRHLD` writer - "]
pub struct QUALMONTRHLD_W<'a> {
    w: &'a mut W,
}
impl<'a> QUALMONTRHLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
#[doc = "Field `QUALMONWND` reader - "]
pub struct QUALMONWND_R(crate::FieldReader<u8, u8>);
impl QUALMONWND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        QUALMONWND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QUALMONWND_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QUALMONWND` writer - "]
pub struct QUALMONWND_W<'a> {
    w: &'a mut W,
}
impl<'a> QUALMONWND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | ((value as u32 & 0x3f) << 18);
        self.w
    }
}
#[doc = "Field `QUALMONMOD` reader - "]
pub struct QUALMONMOD_R(crate::FieldReader<u8, u8>);
impl QUALMONMOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        QUALMONMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QUALMONMOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QUALMONMOD` writer - "]
pub struct QUALMONMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> QUALMONMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 16)) | ((value as u32 & 3) << 16);
        self.w
    }
}
#[doc = "Field `HOLD_STATE` reader - "]
pub struct HOLD_STATE_R(crate::FieldReader<u8, u8>);
impl HOLD_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HOLD_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOLD_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOLD_STATE` writer - "]
pub struct HOLD_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> HOLD_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `RFMEAS_MODE` reader - "]
pub struct RFMEAS_MODE_R(crate::FieldReader<bool, bool>);
impl RFMEAS_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFMEAS_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFMEAS_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFMEAS_MODE` writer - "]
pub struct RFMEAS_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFMEAS_MODE_W<'a> {
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
#[doc = "Field `ENRFMEAS` reader - "]
pub struct ENRFMEAS_R(crate::FieldReader<bool, bool>);
impl ENRFMEAS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENRFMEAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENRFMEAS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENRFMEAS` writer - "]
pub struct ENRFMEAS_W<'a> {
    w: &'a mut W,
}
impl<'a> ENRFMEAS_W<'a> {
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
#[doc = "Field `TMREN` reader - "]
pub struct TMREN_R(crate::FieldReader<bool, bool>);
impl TMREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TMREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMREN` writer - "]
pub struct TMREN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMREN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `TFREQMEAS` reader - "]
pub struct TFREQMEAS_R(crate::FieldReader<u8, u8>);
impl TFREQMEAS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TFREQMEAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFREQMEAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFREQMEAS` writer - "]
pub struct TFREQMEAS_W<'a> {
    w: &'a mut W,
}
impl<'a> TFREQMEAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn qualmonfrcen(&self) -> QUALMONFRCEN_R {
        QUALMONFRCEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn qualmontrhld(&self) -> QUALMONTRHLD_R {
        QUALMONTRHLD_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn qualmonwnd(&self) -> QUALMONWND_R {
        QUALMONWND_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn qualmonmod(&self) -> QUALMONMOD_R {
        QUALMONMOD_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn hold_state(&self) -> HOLD_STATE_R {
        HOLD_STATE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rfmeas_mode(&self) -> RFMEAS_MODE_R {
        RFMEAS_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn enrfmeas(&self) -> ENRFMEAS_R {
        ENRFMEAS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tmren(&self) -> TMREN_R {
        TMREN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn tfreqmeas(&self) -> TFREQMEAS_R {
        TFREQMEAS_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn qualmonfrcen(&mut self) -> QUALMONFRCEN_W {
        QUALMONFRCEN_W { w: self }
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn qualmontrhld(&mut self) -> QUALMONTRHLD_W {
        QUALMONTRHLD_W { w: self }
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn qualmonwnd(&mut self) -> QUALMONWND_W {
        QUALMONWND_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn qualmonmod(&mut self) -> QUALMONMOD_W {
        QUALMONMOD_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn hold_state(&mut self) -> HOLD_STATE_W {
        HOLD_STATE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rfmeas_mode(&mut self) -> RFMEAS_MODE_W {
        RFMEAS_MODE_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn enrfmeas(&mut self) -> ENRFMEAS_W {
        ENRFMEAS_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tmren(&mut self) -> TMREN_W {
        TMREN_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn tfreqmeas(&mut self) -> TFREQMEAS_W {
        TFREQMEAS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_mon_ctrl_reg](index.html) module"]
pub struct ADPLL_MON_CTRL_REG_SPEC;
impl crate::RegisterSpec for ADPLL_MON_CTRL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_mon_ctrl_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_MON_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_mon_ctrl_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_MON_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_MON_CTRL_REG to value 0x00fc_0f83"]
impl crate::Resettable for ADPLL_MON_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00fc_0f83
    }
}
