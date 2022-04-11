#[doc = "Register `POWER_AON_CTRL_REG` reader"]
pub struct R(crate::R<POWER_AON_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWER_AON_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWER_AON_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWER_AON_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POWER_AON_CTRL_REG` writer"]
pub struct W(crate::W<POWER_AON_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWER_AON_CTRL_REG_SPEC>;
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
impl From<crate::W<POWER_AON_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWER_AON_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCE_RUNNING_COMP_DIS` reader - "]
pub struct FORCE_RUNNING_COMP_DIS_R(crate::FieldReader<bool, bool>);
impl FORCE_RUNNING_COMP_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_RUNNING_COMP_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_RUNNING_COMP_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_RUNNING_COMP_DIS` writer - "]
pub struct FORCE_RUNNING_COMP_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_RUNNING_COMP_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u16 & 1) << 14);
        self.w
    }
}
#[doc = "Field `LDO_RET_TRIM` reader - VDD clamp level setting for hibernation mode"]
pub struct LDO_RET_TRIM_R(crate::FieldReader<u8, u8>);
impl LDO_RET_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LDO_RET_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_RET_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_RET_TRIM` writer - VDD clamp level setting for hibernation mode"]
pub struct LDO_RET_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_RET_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | ((value as u16 & 0x0f) << 10);
        self.w
    }
}
#[doc = "Field `CMP_VCONT_SLP_DISABLE` reader - Disable vcont comparator in SLP"]
pub struct CMP_VCONT_SLP_DISABLE_R(crate::FieldReader<bool, bool>);
impl CMP_VCONT_SLP_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMP_VCONT_SLP_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_VCONT_SLP_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_VCONT_SLP_DISABLE` writer - Disable vcont comparator in SLP"]
pub struct CMP_VCONT_SLP_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_VCONT_SLP_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u16 & 1) << 9);
        self.w
    }
}
#[doc = "Field `BOOST_MODE_FORCE` reader - 0x:automatic selection of boost mode 11: force boost mode 10: force buck mode"]
pub struct BOOST_MODE_FORCE_R(crate::FieldReader<u8, u8>);
impl BOOST_MODE_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BOOST_MODE_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOST_MODE_FORCE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOST_MODE_FORCE` writer - 0x:automatic selection of boost mode 11: force boost mode 10: force buck mode"]
pub struct BOOST_MODE_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOST_MODE_FORCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 7)) | ((value as u16 & 3) << 7);
        self.w
    }
}
#[doc = "Field `CHARGE_VBAT_DISABLE` reader - Do not charge vbat high in boost mode"]
pub struct CHARGE_VBAT_DISABLE_R(crate::FieldReader<bool, bool>);
impl CHARGE_VBAT_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHARGE_VBAT_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHARGE_VBAT_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHARGE_VBAT_DISABLE` writer - Do not charge vbat high in boost mode"]
pub struct CHARGE_VBAT_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHARGE_VBAT_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u16 & 1) << 6);
        self.w
    }
}
#[doc = "Field `RC32K_LOW_SPEED_FORCE` reader - "]
pub struct RC32K_LOW_SPEED_FORCE_R(crate::FieldReader<bool, bool>);
impl RC32K_LOW_SPEED_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RC32K_LOW_SPEED_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32K_LOW_SPEED_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RC32K_LOW_SPEED_FORCE` writer - "]
pub struct RC32K_LOW_SPEED_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32K_LOW_SPEED_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u16 & 1) << 5);
        self.w
    }
}
#[doc = "Field `RC32K_HIGH_SPEED_FORCE` reader - "]
pub struct RC32K_HIGH_SPEED_FORCE_R(crate::FieldReader<bool, bool>);
impl RC32K_HIGH_SPEED_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RC32K_HIGH_SPEED_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32K_HIGH_SPEED_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RC32K_HIGH_SPEED_FORCE` writer - "]
pub struct RC32K_HIGH_SPEED_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32K_HIGH_SPEED_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u16 & 1) << 4);
        self.w
    }
}
#[doc = "Field `POR_VBAT_HIGH_RST_MASK` reader - Mask rst from por_vbat_high"]
pub struct POR_VBAT_HIGH_RST_MASK_R(crate::FieldReader<bool, bool>);
impl POR_VBAT_HIGH_RST_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POR_VBAT_HIGH_RST_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POR_VBAT_HIGH_RST_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POR_VBAT_HIGH_RST_MASK` writer - Mask rst from por_vbat_high"]
pub struct POR_VBAT_HIGH_RST_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> POR_VBAT_HIGH_RST_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u16 & 1) << 3);
        self.w
    }
}
#[doc = "Field `POR_VBAT_LOW_RST_MASK` reader - Mask rst from por_vbat_low"]
pub struct POR_VBAT_LOW_RST_MASK_R(crate::FieldReader<bool, bool>);
impl POR_VBAT_LOW_RST_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POR_VBAT_LOW_RST_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POR_VBAT_LOW_RST_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POR_VBAT_LOW_RST_MASK` writer - Mask rst from por_vbat_low"]
pub struct POR_VBAT_LOW_RST_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> POR_VBAT_LOW_RST_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u16 & 1) << 2);
        self.w
    }
}
#[doc = "Field `VBAT_HL_CONNECT_RES_CTRL` reader - 00: OFF 01: Forced ON 10: Active: automatic control, Sleep: forced ON 11: Automatic control"]
pub struct VBAT_HL_CONNECT_RES_CTRL_R(crate::FieldReader<u8, u8>);
impl VBAT_HL_CONNECT_RES_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VBAT_HL_CONNECT_RES_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBAT_HL_CONNECT_RES_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBAT_HL_CONNECT_RES_CTRL` writer - 00: OFF 01: Forced ON 10: Active: automatic control, Sleep: forced ON 11: Automatic control"]
pub struct VBAT_HL_CONNECT_RES_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> VBAT_HL_CONNECT_RES_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u16 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn force_running_comp_dis(&self) -> FORCE_RUNNING_COMP_DIS_R {
        FORCE_RUNNING_COMP_DIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 10:13 - VDD clamp level setting for hibernation mode"]
    #[inline(always)]
    pub fn ldo_ret_trim(&self) -> LDO_RET_TRIM_R {
        LDO_RET_TRIM_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Disable vcont comparator in SLP"]
    #[inline(always)]
    pub fn cmp_vcont_slp_disable(&self) -> CMP_VCONT_SLP_DISABLE_R {
        CMP_VCONT_SLP_DISABLE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 7:8 - 0x:automatic selection of boost mode 11: force boost mode 10: force buck mode"]
    #[inline(always)]
    pub fn boost_mode_force(&self) -> BOOST_MODE_FORCE_R {
        BOOST_MODE_FORCE_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 6 - Do not charge vbat high in boost mode"]
    #[inline(always)]
    pub fn charge_vbat_disable(&self) -> CHARGE_VBAT_DISABLE_R {
        CHARGE_VBAT_DISABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rc32k_low_speed_force(&self) -> RC32K_LOW_SPEED_FORCE_R {
        RC32K_LOW_SPEED_FORCE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rc32k_high_speed_force(&self) -> RC32K_HIGH_SPEED_FORCE_R {
        RC32K_HIGH_SPEED_FORCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask rst from por_vbat_high"]
    #[inline(always)]
    pub fn por_vbat_high_rst_mask(&self) -> POR_VBAT_HIGH_RST_MASK_R {
        POR_VBAT_HIGH_RST_MASK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask rst from por_vbat_low"]
    #[inline(always)]
    pub fn por_vbat_low_rst_mask(&self) -> POR_VBAT_LOW_RST_MASK_R {
        POR_VBAT_LOW_RST_MASK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 0:1 - 00: OFF 01: Forced ON 10: Active: automatic control, Sleep: forced ON 11: Automatic control"]
    #[inline(always)]
    pub fn vbat_hl_connect_res_ctrl(&self) -> VBAT_HL_CONNECT_RES_CTRL_R {
        VBAT_HL_CONNECT_RES_CTRL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn force_running_comp_dis(&mut self) -> FORCE_RUNNING_COMP_DIS_W {
        FORCE_RUNNING_COMP_DIS_W { w: self }
    }
    #[doc = "Bits 10:13 - VDD clamp level setting for hibernation mode"]
    #[inline(always)]
    pub fn ldo_ret_trim(&mut self) -> LDO_RET_TRIM_W {
        LDO_RET_TRIM_W { w: self }
    }
    #[doc = "Bit 9 - Disable vcont comparator in SLP"]
    #[inline(always)]
    pub fn cmp_vcont_slp_disable(&mut self) -> CMP_VCONT_SLP_DISABLE_W {
        CMP_VCONT_SLP_DISABLE_W { w: self }
    }
    #[doc = "Bits 7:8 - 0x:automatic selection of boost mode 11: force boost mode 10: force buck mode"]
    #[inline(always)]
    pub fn boost_mode_force(&mut self) -> BOOST_MODE_FORCE_W {
        BOOST_MODE_FORCE_W { w: self }
    }
    #[doc = "Bit 6 - Do not charge vbat high in boost mode"]
    #[inline(always)]
    pub fn charge_vbat_disable(&mut self) -> CHARGE_VBAT_DISABLE_W {
        CHARGE_VBAT_DISABLE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rc32k_low_speed_force(&mut self) -> RC32K_LOW_SPEED_FORCE_W {
        RC32K_LOW_SPEED_FORCE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rc32k_high_speed_force(&mut self) -> RC32K_HIGH_SPEED_FORCE_W {
        RC32K_HIGH_SPEED_FORCE_W { w: self }
    }
    #[doc = "Bit 3 - Mask rst from por_vbat_high"]
    #[inline(always)]
    pub fn por_vbat_high_rst_mask(&mut self) -> POR_VBAT_HIGH_RST_MASK_W {
        POR_VBAT_HIGH_RST_MASK_W { w: self }
    }
    #[doc = "Bit 2 - Mask rst from por_vbat_low"]
    #[inline(always)]
    pub fn por_vbat_low_rst_mask(&mut self) -> POR_VBAT_LOW_RST_MASK_W {
        POR_VBAT_LOW_RST_MASK_W { w: self }
    }
    #[doc = "Bits 0:1 - 00: OFF 01: Forced ON 10: Active: automatic control, Sleep: forced ON 11: Automatic control"]
    #[inline(always)]
    pub fn vbat_hl_connect_res_ctrl(&mut self) -> VBAT_HL_CONNECT_RES_CTRL_W {
        VBAT_HL_CONNECT_RES_CTRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power_aon_ctrl_reg](index.html) module"]
pub struct POWER_AON_CTRL_REG_SPEC;
impl crate::RegisterSpec for POWER_AON_CTRL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [power_aon_ctrl_reg::R](R) reader structure"]
impl crate::Readable for POWER_AON_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [power_aon_ctrl_reg::W](W) writer structure"]
impl crate::Writable for POWER_AON_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POWER_AON_CTRL_REG to value 0x08"]
impl crate::Resettable for POWER_AON_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
