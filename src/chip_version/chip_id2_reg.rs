#[doc = "Register `CHIP_ID2_REG` reader"]
pub struct R(crate::R<CHIP_ID2_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHIP_ID2_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHIP_ID2_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHIP_ID2_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHIP_ID2_REG` writer"]
pub struct W(crate::W<CHIP_ID2_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHIP_ID2_REG_SPEC>;
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
impl From<crate::W<CHIP_ID2_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHIP_ID2_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHIP_ID2` reader - Second character of device type \"2632\" in ASCII."]
pub struct CHIP_ID2_R(crate::FieldReader<u8, u8>);
impl CHIP_ID2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHIP_ID2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHIP_ID2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Second character of device type \"2632\" in ASCII."]
    #[inline(always)]
    pub fn chip_id2(&self) -> CHIP_ID2_R {
        CHIP_ID2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Chip identification register 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chip_id2_reg](index.html) module"]
pub struct CHIP_ID2_REG_SPEC;
impl crate::RegisterSpec for CHIP_ID2_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [chip_id2_reg::R](R) reader structure"]
impl crate::Readable for CHIP_ID2_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chip_id2_reg::W](W) writer structure"]
impl crate::Writable for CHIP_ID2_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHIP_ID2_REG to value 0x36"]
impl crate::Resettable for CHIP_ID2_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x36
    }
}
