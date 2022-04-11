#[doc = "Register `ADPLL_KDCO_CAL_CTRL2_REG` reader"]
pub struct R(crate::R<ADPLL_KDCO_CAL_CTRL2_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_KDCO_CAL_CTRL2_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_KDCO_CAL_CTRL2_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_KDCO_CAL_CTRL2_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_KDCO_CAL_CTRL2_REG` writer"]
pub struct W(crate::W<ADPLL_KDCO_CAL_CTRL2_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_KDCO_CAL_CTRL2_REG_SPEC>;
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
impl From<crate::W<ADPLL_KDCO_CAL_CTRL2_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_KDCO_CAL_CTRL2_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KDCOESTDEV` reader - "]
pub struct KDCOESTDEV_R(crate::FieldReader<u8, u8>);
impl KDCOESTDEV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KDCOESTDEV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KDCOESTDEV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KDCOESTDEV` writer - "]
pub struct KDCOESTDEV_W<'a> {
    w: &'a mut W,
}
impl<'a> KDCOESTDEV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 30)) | ((value as u32 & 3) << 30);
        self.w
    }
}
#[doc = "Field `KDCOCALTX` reader - "]
pub struct KDCOCALTX_R(crate::FieldReader<bool, bool>);
impl KDCOCALTX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KDCOCALTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KDCOCALTX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KDCOCALTX` writer - "]
pub struct KDCOCALTX_W<'a> {
    w: &'a mut W,
}
impl<'a> KDCOCALTX_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 29)) | ((value as u32 & 1) << 29);
        self.w
    }
}
#[doc = "Field `KDCOCALRX` reader - "]
pub struct KDCOCALRX_R(crate::FieldReader<bool, bool>);
impl KDCOCALRX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KDCOCALRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KDCOCALRX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KDCOCALRX` writer - "]
pub struct KDCOCALRX_W<'a> {
    w: &'a mut W,
}
impl<'a> KDCOCALRX_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 28)) | ((value as u32 & 1) << 28);
        self.w
    }
}
#[doc = "Field `KDCOLFCALEN` reader - "]
pub struct KDCOLFCALEN_R(crate::FieldReader<bool, bool>);
impl KDCOLFCALEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KDCOLFCALEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KDCOLFCALEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KDCOLFCALEN` writer - "]
pub struct KDCOLFCALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> KDCOLFCALEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 27)) | ((value as u32 & 1) << 27);
        self.w
    }
}
#[doc = "Field `TKDCOCAL` reader - "]
pub struct TKDCOCAL_R(crate::FieldReader<u8, u8>);
impl TKDCOCAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TKDCOCAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TKDCOCAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TKDCOCAL` writer - "]
pub struct TKDCOCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TKDCOCAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 24)) | ((value as u32 & 7) << 24);
        self.w
    }
}
#[doc = "Field `KDCOCN_IC` reader - "]
pub struct KDCOCN_IC_R(crate::FieldReader<u8, u8>);
impl KDCOCN_IC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KDCOCN_IC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KDCOCN_IC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KDCOCN_IC` writer - "]
pub struct KDCOCN_IC_W<'a> {
    w: &'a mut W,
}
impl<'a> KDCOCN_IC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `KMOD_ALPHA_1M` reader - "]
pub struct KMOD_ALPHA_1M_R(crate::FieldReader<u8, u8>);
impl KMOD_ALPHA_1M_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KMOD_ALPHA_1M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KMOD_ALPHA_1M_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KMOD_ALPHA_1M` writer - "]
pub struct KMOD_ALPHA_1M_W<'a> {
    w: &'a mut W,
}
impl<'a> KMOD_ALPHA_1M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn kdcoestdev(&self) -> KDCOESTDEV_R {
        KDCOESTDEV_R::new(((self.bits >> 30) & 3) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn kdcocaltx(&self) -> KDCOCALTX_R {
        KDCOCALTX_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn kdcocalrx(&self) -> KDCOCALRX_R {
        KDCOCALRX_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn kdcolfcalen(&self) -> KDCOLFCALEN_R {
        KDCOLFCALEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn tkdcocal(&self) -> TKDCOCAL_R {
        TKDCOCAL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn kdcocn_ic(&self) -> KDCOCN_IC_R {
        KDCOCN_IC_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn kmod_alpha_1m(&self) -> KMOD_ALPHA_1M_R {
        KMOD_ALPHA_1M_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn kdcoestdev(&mut self) -> KDCOESTDEV_W {
        KDCOESTDEV_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn kdcocaltx(&mut self) -> KDCOCALTX_W {
        KDCOCALTX_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn kdcocalrx(&mut self) -> KDCOCALRX_W {
        KDCOCALRX_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn kdcolfcalen(&mut self) -> KDCOLFCALEN_W {
        KDCOLFCALEN_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn tkdcocal(&mut self) -> TKDCOCAL_W {
        TKDCOCAL_W { w: self }
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn kdcocn_ic(&mut self) -> KDCOCN_IC_W {
        KDCOCN_IC_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn kmod_alpha_1m(&mut self) -> KMOD_ALPHA_1M_W {
        KMOD_ALPHA_1M_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_kdco_cal_ctrl2_reg](index.html) module"]
pub struct ADPLL_KDCO_CAL_CTRL2_REG_SPEC;
impl crate::RegisterSpec for ADPLL_KDCO_CAL_CTRL2_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_kdco_cal_ctrl2_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_KDCO_CAL_CTRL2_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_kdco_cal_ctrl2_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_KDCO_CAL_CTRL2_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_KDCO_CAL_CTRL2_REG to value 0x4c14_000a"]
impl crate::Resettable for ADPLL_KDCO_CAL_CTRL2_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4c14_000a
    }
}
