#[doc = "Register `P0_RESET_DATA_REG` reader"]
pub struct R(crate::R<P0_RESET_DATA_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P0_RESET_DATA_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P0_RESET_DATA_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P0_RESET_DATA_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P0_RESET_DATA_REG` writer"]
pub struct W(crate::W<P0_RESET_DATA_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P0_RESET_DATA_REG_SPEC>;
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
impl From<crate::W<P0_RESET_DATA_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P0_RESET_DATA_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P0_RESET` writer - Writing a 1 to P0\\[x\\]
sets P0\\[x\\]
to 0. Writing 0 is discarded, reading returns 0."]
pub struct P0_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_RESET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u16 & 0x0fff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:11 - Writing a 1 to P0\\[x\\]
sets P0\\[x\\]
to 0. Writing 0 is discarded, reading returns 0."]
    #[inline(always)]
    pub fn p0_reset(&mut self) -> P0_RESET_W {
        P0_RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "P0 Reset port pins Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p0_reset_data_reg](index.html) module"]
pub struct P0_RESET_DATA_REG_SPEC;
impl crate::RegisterSpec for P0_RESET_DATA_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [p0_reset_data_reg::R](R) reader structure"]
impl crate::Readable for P0_RESET_DATA_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p0_reset_data_reg::W](W) writer structure"]
impl crate::Writable for P0_RESET_DATA_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P0_RESET_DATA_REG to value 0"]
impl crate::Resettable for P0_RESET_DATA_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
