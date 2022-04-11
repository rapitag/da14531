#[doc = "Register `ADPLL_MISC_CTRL_REG` reader"]
pub struct R(crate::R<ADPLL_MISC_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_MISC_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_MISC_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_MISC_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_MISC_CTRL_REG` writer"]
pub struct W(crate::W<ADPLL_MISC_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_MISC_CTRL_REG_SPEC>;
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
impl From<crate::W<ADPLL_MISC_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_MISC_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHR_FRAC_PRESET_VAL` reader - "]
pub struct PHR_FRAC_PRESET_VAL_R(crate::FieldReader<u16, u16>);
impl PHR_FRAC_PRESET_VAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PHR_FRAC_PRESET_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHR_FRAC_PRESET_VAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHR_FRAC_PRESET_VAL` writer - "]
pub struct PHR_FRAC_PRESET_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PHR_FRAC_PRESET_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 8)) | ((value as u32 & 0xffff) << 8);
        self.w
    }
}
#[doc = "Field `ENFCWMOD` reader - "]
pub struct ENFCWMOD_R(crate::FieldReader<bool, bool>);
impl ENFCWMOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENFCWMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENFCWMOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENFCWMOD` writer - "]
pub struct ENFCWMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> ENFCWMOD_W<'a> {
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
#[doc = "Field `ENRESIDUE` reader - "]
pub struct ENRESIDUE_R(crate::FieldReader<bool, bool>);
impl ENRESIDUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENRESIDUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENRESIDUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENRESIDUE` writer - "]
pub struct ENRESIDUE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENRESIDUE_W<'a> {
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
#[doc = "Field `MODDLY` reader - "]
pub struct MODDLY_R(crate::FieldReader<u8, u8>);
impl MODDLY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MODDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODDLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODDLY` writer - "]
pub struct MODDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> MODDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `RESDLY` reader - "]
pub struct RESDLY_R(crate::FieldReader<u8, u8>);
impl RESDLY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RESDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESDLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESDLY` writer - "]
pub struct RESDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> RESDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u32 & 3) << 2);
        self.w
    }
}
#[doc = "Field `DLYFCWDT` reader - "]
pub struct DLYFCWDT_R(crate::FieldReader<u8, u8>);
impl DLYFCWDT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DLYFCWDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLYFCWDT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYFCWDT` writer - "]
pub struct DLYFCWDT_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYFCWDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:23"]
    #[inline(always)]
    pub fn phr_frac_preset_val(&self) -> PHR_FRAC_PRESET_VAL_R {
        PHR_FRAC_PRESET_VAL_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn enfcwmod(&self) -> ENFCWMOD_R {
        ENFCWMOD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn enresidue(&self) -> ENRESIDUE_R {
        ENRESIDUE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn moddly(&self) -> MODDLY_R {
        MODDLY_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn resdly(&self) -> RESDLY_R {
        RESDLY_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dlyfcwdt(&self) -> DLYFCWDT_R {
        DLYFCWDT_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 8:23"]
    #[inline(always)]
    pub fn phr_frac_preset_val(&mut self) -> PHR_FRAC_PRESET_VAL_W {
        PHR_FRAC_PRESET_VAL_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn enfcwmod(&mut self) -> ENFCWMOD_W {
        ENFCWMOD_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn enresidue(&mut self) -> ENRESIDUE_W {
        ENRESIDUE_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn moddly(&mut self) -> MODDLY_W {
        MODDLY_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn resdly(&mut self) -> RESDLY_W {
        RESDLY_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dlyfcwdt(&mut self) -> DLYFCWDT_W {
        DLYFCWDT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_misc_ctrl_reg](index.html) module"]
pub struct ADPLL_MISC_CTRL_REG_SPEC;
impl crate::RegisterSpec for ADPLL_MISC_CTRL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_misc_ctrl_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_MISC_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_misc_ctrl_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_MISC_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_MISC_CTRL_REG to value 0xda"]
impl crate::Resettable for ADPLL_MISC_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xda
    }
}
