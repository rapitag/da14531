#[doc = "Register `BLE_BLEMPRIO0_REG` reader"]
pub struct R(crate::R<BLE_BLEMPRIO0_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_BLEMPRIO0_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_BLEMPRIO0_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_BLEMPRIO0_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_BLEMPRIO0_REG` writer"]
pub struct W(crate::W<BLE_BLEMPRIO0_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_BLEMPRIO0_REG_SPEC>;
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
impl From<crate::W<BLE_BLEMPRIO0_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_BLEMPRIO0_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLEM7` reader - Set Priority value for Passive Scanning"]
pub struct BLEM7_R(crate::FieldReader<u8, u8>);
impl BLEM7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BLEM7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEM7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEM7` writer - Set Priority value for Passive Scanning"]
pub struct BLEM7_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEM7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `BLEM6` reader - Set Priority value for Non-Connectable Advertising"]
pub struct BLEM6_R(crate::FieldReader<u8, u8>);
impl BLEM6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BLEM6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEM6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEM6` writer - Set Priority value for Non-Connectable Advertising"]
pub struct BLEM6_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEM6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `BLEM5` reader - Set Priority value for Connectable Advertising BLE message"]
pub struct BLEM5_R(crate::FieldReader<u8, u8>);
impl BLEM5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BLEM5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEM5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEM5` writer - Set Priority value for Connectable Advertising BLE message"]
pub struct BLEM5_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEM5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `BLEM4` reader - Set Priority value for Active Scanning BLE message"]
pub struct BLEM4_R(crate::FieldReader<u8, u8>);
impl BLEM4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BLEM4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEM4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEM4` writer - Set Priority value for Active Scanning BLE message"]
pub struct BLEM4_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEM4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `BLEM3` reader - Set Priority value for Initiating (Scanning) BLE message"]
pub struct BLEM3_R(crate::FieldReader<u8, u8>);
impl BLEM3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BLEM3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEM3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEM3` writer - Set Priority value for Initiating (Scanning) BLE message"]
pub struct BLEM3_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEM3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `BLEM2` reader - Set Priority value for Data Channel transmission BLE message"]
pub struct BLEM2_R(crate::FieldReader<u8, u8>);
impl BLEM2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BLEM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEM2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEM2` writer - Set Priority value for Data Channel transmission BLE message"]
pub struct BLEM2_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEM2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `BLEM1` reader - Set Priority value for LLCP BLE message"]
pub struct BLEM1_R(crate::FieldReader<u8, u8>);
impl BLEM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BLEM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEM1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEM1` writer - Set Priority value for LLCP BLE message"]
pub struct BLEM1_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEM1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `BLEM0` reader - Set Priority value for Initiating (Connection Request Response) BLE message"]
pub struct BLEM0_R(crate::FieldReader<u8, u8>);
impl BLEM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BLEM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEM0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEM0` writer - Set Priority value for Initiating (Connection Request Response) BLE message"]
pub struct BLEM0_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEM0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - Set Priority value for Passive Scanning"]
    #[inline(always)]
    pub fn blem7(&self) -> BLEM7_R {
        BLEM7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Set Priority value for Non-Connectable Advertising"]
    #[inline(always)]
    pub fn blem6(&self) -> BLEM6_R {
        BLEM6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Set Priority value for Connectable Advertising BLE message"]
    #[inline(always)]
    pub fn blem5(&self) -> BLEM5_R {
        BLEM5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Set Priority value for Active Scanning BLE message"]
    #[inline(always)]
    pub fn blem4(&self) -> BLEM4_R {
        BLEM4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Set Priority value for Initiating (Scanning) BLE message"]
    #[inline(always)]
    pub fn blem3(&self) -> BLEM3_R {
        BLEM3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Set Priority value for Data Channel transmission BLE message"]
    #[inline(always)]
    pub fn blem2(&self) -> BLEM2_R {
        BLEM2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Set Priority value for LLCP BLE message"]
    #[inline(always)]
    pub fn blem1(&self) -> BLEM1_R {
        BLEM1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Set Priority value for Initiating (Connection Request Response) BLE message"]
    #[inline(always)]
    pub fn blem0(&self) -> BLEM0_R {
        BLEM0_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - Set Priority value for Passive Scanning"]
    #[inline(always)]
    pub fn blem7(&mut self) -> BLEM7_W {
        BLEM7_W { w: self }
    }
    #[doc = "Bits 24:27 - Set Priority value for Non-Connectable Advertising"]
    #[inline(always)]
    pub fn blem6(&mut self) -> BLEM6_W {
        BLEM6_W { w: self }
    }
    #[doc = "Bits 20:23 - Set Priority value for Connectable Advertising BLE message"]
    #[inline(always)]
    pub fn blem5(&mut self) -> BLEM5_W {
        BLEM5_W { w: self }
    }
    #[doc = "Bits 16:19 - Set Priority value for Active Scanning BLE message"]
    #[inline(always)]
    pub fn blem4(&mut self) -> BLEM4_W {
        BLEM4_W { w: self }
    }
    #[doc = "Bits 12:15 - Set Priority value for Initiating (Scanning) BLE message"]
    #[inline(always)]
    pub fn blem3(&mut self) -> BLEM3_W {
        BLEM3_W { w: self }
    }
    #[doc = "Bits 8:11 - Set Priority value for Data Channel transmission BLE message"]
    #[inline(always)]
    pub fn blem2(&mut self) -> BLEM2_W {
        BLEM2_W { w: self }
    }
    #[doc = "Bits 4:7 - Set Priority value for LLCP BLE message"]
    #[inline(always)]
    pub fn blem1(&mut self) -> BLEM1_W {
        BLEM1_W { w: self }
    }
    #[doc = "Bits 0:3 - Set Priority value for Initiating (Connection Request Response) BLE message"]
    #[inline(always)]
    pub fn blem0(&mut self) -> BLEM0_W {
        BLEM0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Coexistence interface Priority 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_blemprio0_reg](index.html) module"]
pub struct BLE_BLEMPRIO0_REG_SPEC;
impl crate::RegisterSpec for BLE_BLEMPRIO0_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_blemprio0_reg::R](R) reader structure"]
impl crate::Readable for BLE_BLEMPRIO0_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_blemprio0_reg::W](W) writer structure"]
impl crate::Writable for BLE_BLEMPRIO0_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_BLEMPRIO0_REG to value 0x3489_adef"]
impl crate::Resettable for BLE_BLEMPRIO0_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3489_adef
    }
}
