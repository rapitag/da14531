#[doc = "Register `BANDGAP_REG` reader"]
pub struct R(crate::R<BANDGAP_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BANDGAP_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BANDGAP_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BANDGAP_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BANDGAP_REG` writer"]
pub struct W(crate::W<BANDGAP_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BANDGAP_REG_SPEC>;
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
impl From<crate::W<BANDGAP_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BANDGAP_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BGR_ITRIM` reader - Trim setting for bandgap bias current 10000 -> -25% .... 11111 -> ~0% 00000 -> ~0% (typ) ... 01111 -> +32%"]
pub struct BGR_ITRIM_R(crate::FieldReader<u8, u8>);
impl BGR_ITRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BGR_ITRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BGR_ITRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BGR_ITRIM` writer - Trim setting for bandgap bias current 10000 -> -25% .... 11111 -> ~0% 00000 -> ~0% (typ) ... 01111 -> +32%"]
pub struct BGR_ITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> BGR_ITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u16 & 0x1f) << 5);
        self.w
    }
}
#[doc = "Field `BGR_TRIM` reader - Trim setting for bandgap voltage 10000 -> -6.4% .... 11111 -> ~0% 00000 -> ~0% (typ) ... 01111 -> +5.8%"]
pub struct BGR_TRIM_R(crate::FieldReader<u8, u8>);
impl BGR_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BGR_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BGR_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BGR_TRIM` writer - Trim setting for bandgap voltage 10000 -> -6.4% .... 11111 -> ~0% 00000 -> ~0% (typ) ... 01111 -> +5.8%"]
pub struct BGR_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> BGR_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u16 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:9 - Trim setting for bandgap bias current 10000 -> -25% .... 11111 -> ~0% 00000 -> ~0% (typ) ... 01111 -> +32%"]
    #[inline(always)]
    pub fn bgr_itrim(&self) -> BGR_ITRIM_R {
        BGR_ITRIM_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - Trim setting for bandgap voltage 10000 -> -6.4% .... 11111 -> ~0% 00000 -> ~0% (typ) ... 01111 -> +5.8%"]
    #[inline(always)]
    pub fn bgr_trim(&self) -> BGR_TRIM_R {
        BGR_TRIM_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 5:9 - Trim setting for bandgap bias current 10000 -> -25% .... 11111 -> ~0% 00000 -> ~0% (typ) ... 01111 -> +32%"]
    #[inline(always)]
    pub fn bgr_itrim(&mut self) -> BGR_ITRIM_W {
        BGR_ITRIM_W { w: self }
    }
    #[doc = "Bits 0:4 - Trim setting for bandgap voltage 10000 -> -6.4% .... 11111 -> ~0% 00000 -> ~0% (typ) ... 01111 -> +5.8%"]
    #[inline(always)]
    pub fn bgr_trim(&mut self) -> BGR_TRIM_W {
        BGR_TRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bandgap trimming\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bandgap_reg](index.html) module"]
pub struct BANDGAP_REG_SPEC;
impl crate::RegisterSpec for BANDGAP_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [bandgap_reg::R](R) reader structure"]
impl crate::Readable for BANDGAP_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bandgap_reg::W](W) writer structure"]
impl crate::Writable for BANDGAP_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BANDGAP_REG to value 0"]
impl crate::Resettable for BANDGAP_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
