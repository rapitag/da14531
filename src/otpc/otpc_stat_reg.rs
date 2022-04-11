#[doc = "Register `OTPC_STAT_REG` reader"]
pub struct R(crate::R<OTPC_STAT_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTPC_STAT_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTPC_STAT_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTPC_STAT_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTPC_STAT_REG` writer"]
pub struct W(crate::W<OTPC_STAT_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTPC_STAT_REG_SPEC>;
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
impl From<crate::W<OTPC_STAT_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTPC_STAT_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OTPC_STAT_MRDY` reader - Indicates the progress of the transition from a mode of operation to a new mode of operation. 0 : There is a transition in progress in a new mode of operation . Wait until the transition to be completed. 1 : The transition to the new mode of operation has been completed. The function that has been enabled by the new mode can be used. A new mode can be applied. This status bit gets the value zero every time where the OTPC_MODE_REG\\[MODE\\]
is changing. Do not try to use or change any function of the controller until this status bit to become equal to 1."]
pub struct OTPC_STAT_MRDY_R(crate::FieldReader<bool, bool>);
impl OTPC_STAT_MRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OTPC_STAT_MRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTPC_STAT_MRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTPC_STAT_PBUF_EMPTY` reader - Indicates the status of the programming buffer (PBUF). 0 : The PBUF contains the address and the data of a programming request. The OTPC_PADDR_REG and the OTPC_PWORD_REG should not be written as long as this status bit is zero. 1 : The PBUF is empty and a new programming request can be registered in the PBUF by using the OTPC_PADDR_REG and the OTPC_PWORD_REG registers. This status bit gets the value zero every time where a programming is triggered by the OTPC_PADDR_REG (only if the PROG mode is active)."]
pub struct OTPC_STAT_PBUF_EMPTY_R(crate::FieldReader<bool, bool>);
impl OTPC_STAT_PBUF_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OTPC_STAT_PBUF_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTPC_STAT_PBUF_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTPC_STAT_PRDY` reader - Indicates the state of the programming process. 0: The controller is busy. A programming is in progress. 1: The logic which performs programming is idle."]
pub struct OTPC_STAT_PRDY_R(crate::FieldReader<bool, bool>);
impl OTPC_STAT_PRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OTPC_STAT_PRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTPC_STAT_PRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 2 - Indicates the progress of the transition from a mode of operation to a new mode of operation. 0 : There is a transition in progress in a new mode of operation . Wait until the transition to be completed. 1 : The transition to the new mode of operation has been completed. The function that has been enabled by the new mode can be used. A new mode can be applied. This status bit gets the value zero every time where the OTPC_MODE_REG\\[MODE\\]
is changing. Do not try to use or change any function of the controller until this status bit to become equal to 1."]
    #[inline(always)]
    pub fn otpc_stat_mrdy(&self) -> OTPC_STAT_MRDY_R {
        OTPC_STAT_MRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates the status of the programming buffer (PBUF). 0 : The PBUF contains the address and the data of a programming request. The OTPC_PADDR_REG and the OTPC_PWORD_REG should not be written as long as this status bit is zero. 1 : The PBUF is empty and a new programming request can be registered in the PBUF by using the OTPC_PADDR_REG and the OTPC_PWORD_REG registers. This status bit gets the value zero every time where a programming is triggered by the OTPC_PADDR_REG (only if the PROG mode is active)."]
    #[inline(always)]
    pub fn otpc_stat_pbuf_empty(&self) -> OTPC_STAT_PBUF_EMPTY_R {
        OTPC_STAT_PBUF_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Indicates the state of the programming process. 0: The controller is busy. A programming is in progress. 1: The logic which performs programming is idle."]
    #[inline(always)]
    pub fn otpc_stat_prdy(&self) -> OTPC_STAT_PRDY_R {
        OTPC_STAT_PRDY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otpc_stat_reg](index.html) module"]
pub struct OTPC_STAT_REG_SPEC;
impl crate::RegisterSpec for OTPC_STAT_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otpc_stat_reg::R](R) reader structure"]
impl crate::Readable for OTPC_STAT_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otpc_stat_reg::W](W) writer structure"]
impl crate::Writable for OTPC_STAT_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTPC_STAT_REG to value 0x07"]
impl crate::Resettable for OTPC_STAT_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
