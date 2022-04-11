#[doc = "Register `ADPLL_INIT_CTRL_REG` reader"]
pub struct R(crate::R<ADPLL_INIT_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_INIT_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_INIT_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_INIT_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_INIT_CTRL_REG` writer"]
pub struct W(crate::W<ADPLL_INIT_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_INIT_CTRL_REG_SPEC>;
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
impl From<crate::W<ADPLL_INIT_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_INIT_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCOMODIC` reader - "]
pub struct DCOMODIC_R(crate::FieldReader<u16, u16>);
impl DCOMODIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DCOMODIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCOMODIC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCOMODIC` writer - "]
pub struct DCOMODIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DCOMODIC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 16)) | ((value as u32 & 0x1fff) << 16);
        self.w
    }
}
#[doc = "Field `DCOFINEIC` reader - "]
pub struct DCOFINEIC_R(crate::FieldReader<u8, u8>);
impl DCOFINEIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DCOFINEIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCOFINEIC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCOFINEIC` writer - "]
pub struct DCOFINEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DCOFINEIC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `DCOMEDIUMIC` reader - "]
pub struct DCOMEDIUMIC_R(crate::FieldReader<u8, u8>);
impl DCOMEDIUMIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DCOMEDIUMIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCOMEDIUMIC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCOMEDIUMIC` writer - "]
pub struct DCOMEDIUMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DCOMEDIUMIC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 4)) | ((value as u32 & 7) << 4);
        self.w
    }
}
#[doc = "Field `DCOCOARSEIC` reader - "]
pub struct DCOCOARSEIC_R(crate::FieldReader<u8, u8>);
impl DCOCOARSEIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DCOCOARSEIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCOCOARSEIC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCOCOARSEIC` writer - "]
pub struct DCOCOARSEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DCOCOARSEIC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:28"]
    #[inline(always)]
    pub fn dcomodic(&self) -> DCOMODIC_R {
        DCOMODIC_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn dcofineic(&self) -> DCOFINEIC_R {
        DCOFINEIC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn dcomediumic(&self) -> DCOMEDIUMIC_R {
        DCOMEDIUMIC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dcocoarseic(&self) -> DCOCOARSEIC_R {
        DCOCOARSEIC_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:28"]
    #[inline(always)]
    pub fn dcomodic(&mut self) -> DCOMODIC_W {
        DCOMODIC_W { w: self }
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn dcofineic(&mut self) -> DCOFINEIC_W {
        DCOFINEIC_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn dcomediumic(&mut self) -> DCOMEDIUMIC_W {
        DCOMEDIUMIC_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dcocoarseic(&mut self) -> DCOCOARSEIC_W {
        DCOCOARSEIC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_init_ctrl_reg](index.html) module"]
pub struct ADPLL_INIT_CTRL_REG_SPEC;
impl crate::RegisterSpec for ADPLL_INIT_CTRL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_init_ctrl_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_INIT_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_init_ctrl_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_INIT_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_INIT_CTRL_REG to value 0x05"]
impl crate::Resettable for ADPLL_INIT_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x05
    }
}
