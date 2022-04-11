#[doc = "Register `ADPLL_FSM_CTRL_REG` reader"]
pub struct R(crate::R<ADPLL_FSM_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_FSM_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_FSM_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_FSM_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_FSM_CTRL_REG` writer"]
pub struct W(crate::W<ADPLL_FSM_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_FSM_CTRL_REG_SPEC>;
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
impl From<crate::W<ADPLL_FSM_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_FSM_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TVPASETTLE` reader - "]
pub struct TVPASETTLE_R(crate::FieldReader<u8, u8>);
impl TVPASETTLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TVPASETTLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TVPASETTLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TVPASETTLE` writer - "]
pub struct TVPASETTLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TVPASETTLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
#[doc = "Field `TSETTLE` reader - "]
pub struct TSETTLE_R(crate::FieldReader<u8, u8>);
impl TSETTLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TSETTLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSETTLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSETTLE` writer - "]
pub struct TSETTLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSETTLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `TPASETTLE` reader - "]
pub struct TPASETTLE_R(crate::FieldReader<u8, u8>);
impl TPASETTLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TPASETTLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPASETTLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPASETTLE` writer - "]
pub struct TPASETTLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TPASETTLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `TMOD` reader - "]
pub struct TMOD_R(crate::FieldReader<u8, u8>);
impl TMOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMOD` writer - "]
pub struct TMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `TFINE` reader - "]
pub struct TFINE_R(crate::FieldReader<u8, u8>);
impl TFINE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TFINE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFINE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFINE` writer - "]
pub struct TFINE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFINE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `TMEDIUM` reader - "]
pub struct TMEDIUM_R(crate::FieldReader<u8, u8>);
impl TMEDIUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TMEDIUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMEDIUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMEDIUM` writer - "]
pub struct TMEDIUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TMEDIUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `TCOARSE` reader - "]
pub struct TCOARSE_R(crate::FieldReader<u8, u8>);
impl TCOARSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TCOARSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCOARSE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCOARSE` writer - "]
pub struct TCOARSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCOARSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn tvpasettle(&self) -> TVPASETTLE_R {
        TVPASETTLE_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn tsettle(&self) -> TSETTLE_R {
        TSETTLE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn tpasettle(&self) -> TPASETTLE_R {
        TPASETTLE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn tmod(&self) -> TMOD_R {
        TMOD_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn tfine(&self) -> TFINE_R {
        TFINE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn tmedium(&self) -> TMEDIUM_R {
        TMEDIUM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn tcoarse(&self) -> TCOARSE_R {
        TCOARSE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn tvpasettle(&mut self) -> TVPASETTLE_W {
        TVPASETTLE_W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn tsettle(&mut self) -> TSETTLE_W {
        TSETTLE_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn tpasettle(&mut self) -> TPASETTLE_W {
        TPASETTLE_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn tmod(&mut self) -> TMOD_W {
        TMOD_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn tfine(&mut self) -> TFINE_W {
        TFINE_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn tmedium(&mut self) -> TMEDIUM_W {
        TMEDIUM_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn tcoarse(&mut self) -> TCOARSE_W {
        TCOARSE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_fsm_ctrl_reg](index.html) module"]
pub struct ADPLL_FSM_CTRL_REG_SPEC;
impl crate::RegisterSpec for ADPLL_FSM_CTRL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_fsm_ctrl_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_FSM_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_fsm_ctrl_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_FSM_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_FSM_CTRL_REG to value 0x08f8_ae84"]
impl crate::Resettable for ADPLL_FSM_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08f8_ae84
    }
}
