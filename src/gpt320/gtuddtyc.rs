#[doc = "Register `GTUDDTYC` reader"]
pub type R = crate::R<GtuddtycSpec>;
#[doc = "Register `GTUDDTYC` writer"]
pub type W = crate::W<GtuddtycSpec>;
#[doc = "Count Direction Setting\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ud {
    #[doc = "0: GTCNT counts down."]
    _0 = 0,
    #[doc = "1: GTCNT counts up."]
    _1 = 1,
}
impl From<Ud> for bool {
    #[inline(always)]
    fn from(variant: Ud) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UD` reader - Count Direction Setting"]
pub type UdR = crate::BitReader<Ud>;
impl UdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ud {
        match self.bits {
            false => Ud::_0,
            true => Ud::_1,
        }
    }
    #[doc = "GTCNT counts down."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ud::_0
    }
    #[doc = "GTCNT counts up."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ud::_1
    }
}
#[doc = "Field `UD` writer - Count Direction Setting"]
pub type UdW<'a, REG> = crate::BitWriter<'a, REG, Ud>;
impl<'a, REG> UdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTCNT counts down."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ud::_0)
    }
    #[doc = "GTCNT counts up."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ud::_1)
    }
}
#[doc = "Forcible Count Direction Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Udf {
    #[doc = "0: Not forcibly set"]
    _0 = 0,
    #[doc = "1: Forcibly set"]
    _1 = 1,
}
impl From<Udf> for bool {
    #[inline(always)]
    fn from(variant: Udf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDF` reader - Forcible Count Direction Setting"]
pub type UdfR = crate::BitReader<Udf>;
impl UdfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Udf {
        match self.bits {
            false => Udf::_0,
            true => Udf::_1,
        }
    }
    #[doc = "Not forcibly set"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Udf::_0
    }
    #[doc = "Forcibly set"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Udf::_1
    }
}
#[doc = "Field `UDF` writer - Forcible Count Direction Setting"]
pub type UdfW<'a, REG> = crate::BitWriter<'a, REG, Udf>;
impl<'a, REG> UdfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not forcibly set"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Udf::_0)
    }
    #[doc = "Forcibly set"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Udf::_1)
    }
}
#[doc = "GTIOCA Output Duty Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Oadty {
    #[doc = "0: GTIOCA pin duty is depend on compare match"]
    _00 = 0,
    #[doc = "1: GTIOCA pin duty is depend on compare match"]
    _01 = 1,
    #[doc = "2: GTIOCA pin duty 0 percent"]
    _10 = 2,
    #[doc = "3: GTIOCA pin duty 100 percent"]
    _11 = 3,
}
impl From<Oadty> for u8 {
    #[inline(always)]
    fn from(variant: Oadty) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Oadty {
    type Ux = u8;
}
impl crate::IsEnum for Oadty {}
#[doc = "Field `OADTY` reader - GTIOCA Output Duty Setting"]
pub type OadtyR = crate::FieldReader<Oadty>;
impl OadtyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oadty {
        match self.bits {
            0 => Oadty::_00,
            1 => Oadty::_01,
            2 => Oadty::_10,
            3 => Oadty::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "GTIOCA pin duty is depend on compare match"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Oadty::_00
    }
    #[doc = "GTIOCA pin duty is depend on compare match"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Oadty::_01
    }
    #[doc = "GTIOCA pin duty 0 percent"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Oadty::_10
    }
    #[doc = "GTIOCA pin duty 100 percent"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Oadty::_11
    }
}
#[doc = "Field `OADTY` writer - GTIOCA Output Duty Setting"]
pub type OadtyW<'a, REG> = crate::FieldWriter<'a, REG, 2, Oadty, crate::Safe>;
impl<'a, REG> OadtyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GTIOCA pin duty is depend on compare match"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Oadty::_00)
    }
    #[doc = "GTIOCA pin duty is depend on compare match"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Oadty::_01)
    }
    #[doc = "GTIOCA pin duty 0 percent"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Oadty::_10)
    }
    #[doc = "GTIOCA pin duty 100 percent"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Oadty::_11)
    }
}
#[doc = "Forcible GTIOCA Output Duty Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oadtyf {
    #[doc = "0: Not forcibly set"]
    _0 = 0,
    #[doc = "1: Forcibly set"]
    _1 = 1,
}
impl From<Oadtyf> for bool {
    #[inline(always)]
    fn from(variant: Oadtyf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OADTYF` reader - Forcible GTIOCA Output Duty Setting"]
pub type OadtyfR = crate::BitReader<Oadtyf>;
impl OadtyfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oadtyf {
        match self.bits {
            false => Oadtyf::_0,
            true => Oadtyf::_1,
        }
    }
    #[doc = "Not forcibly set"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Oadtyf::_0
    }
    #[doc = "Forcibly set"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Oadtyf::_1
    }
}
#[doc = "Field `OADTYF` writer - Forcible GTIOCA Output Duty Setting"]
pub type OadtyfW<'a, REG> = crate::BitWriter<'a, REG, Oadtyf>;
impl<'a, REG> OadtyfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not forcibly set"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Oadtyf::_0)
    }
    #[doc = "Forcibly set"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Oadtyf::_1)
    }
}
#[doc = "GTIOCA Output Value Selecting after Releasing 0 percent/100 percent Duty Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oadtyr {
    #[doc = "0: Apply output value set in 0 percent/100 percent duty to GTIOA\\[3:2\\] function after releasing 0 percent/100 percent duty setting."]
    _0 = 0,
    #[doc = "1: Apply masked compare match output value to GTIOA\\[3:2\\] function after releasing 0 percent/100 percent duty setting."]
    _1 = 1,
}
impl From<Oadtyr> for bool {
    #[inline(always)]
    fn from(variant: Oadtyr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OADTYR` reader - GTIOCA Output Value Selecting after Releasing 0 percent/100 percent Duty Setting"]
pub type OadtyrR = crate::BitReader<Oadtyr>;
impl OadtyrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oadtyr {
        match self.bits {
            false => Oadtyr::_0,
            true => Oadtyr::_1,
        }
    }
    #[doc = "Apply output value set in 0 percent/100 percent duty to GTIOA\\[3:2\\] function after releasing 0 percent/100 percent duty setting."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Oadtyr::_0
    }
    #[doc = "Apply masked compare match output value to GTIOA\\[3:2\\] function after releasing 0 percent/100 percent duty setting."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Oadtyr::_1
    }
}
#[doc = "Field `OADTYR` writer - GTIOCA Output Value Selecting after Releasing 0 percent/100 percent Duty Setting"]
pub type OadtyrW<'a, REG> = crate::BitWriter<'a, REG, Oadtyr>;
impl<'a, REG> OadtyrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Apply output value set in 0 percent/100 percent duty to GTIOA\\[3:2\\] function after releasing 0 percent/100 percent duty setting."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Oadtyr::_0)
    }
    #[doc = "Apply masked compare match output value to GTIOA\\[3:2\\] function after releasing 0 percent/100 percent duty setting."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Oadtyr::_1)
    }
}
#[doc = "GTIOCB Output Duty Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Obdty {
    #[doc = "0: GTIOCB pin duty is depend on compare match"]
    _00 = 0,
    #[doc = "1: GTIOCB pin duty is depend on compare match"]
    _01 = 1,
    #[doc = "2: GTIOCB pin duty 0 percent"]
    _10 = 2,
    #[doc = "3: GTIOCB pin duty 100 percent"]
    _11 = 3,
}
impl From<Obdty> for u8 {
    #[inline(always)]
    fn from(variant: Obdty) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Obdty {
    type Ux = u8;
}
impl crate::IsEnum for Obdty {}
#[doc = "Field `OBDTY` reader - GTIOCB Output Duty Setting"]
pub type ObdtyR = crate::FieldReader<Obdty>;
impl ObdtyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Obdty {
        match self.bits {
            0 => Obdty::_00,
            1 => Obdty::_01,
            2 => Obdty::_10,
            3 => Obdty::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "GTIOCB pin duty is depend on compare match"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Obdty::_00
    }
    #[doc = "GTIOCB pin duty is depend on compare match"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Obdty::_01
    }
    #[doc = "GTIOCB pin duty 0 percent"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Obdty::_10
    }
    #[doc = "GTIOCB pin duty 100 percent"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Obdty::_11
    }
}
#[doc = "Field `OBDTY` writer - GTIOCB Output Duty Setting"]
pub type ObdtyW<'a, REG> = crate::FieldWriter<'a, REG, 2, Obdty, crate::Safe>;
impl<'a, REG> ObdtyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GTIOCB pin duty is depend on compare match"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Obdty::_00)
    }
    #[doc = "GTIOCB pin duty is depend on compare match"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Obdty::_01)
    }
    #[doc = "GTIOCB pin duty 0 percent"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Obdty::_10)
    }
    #[doc = "GTIOCB pin duty 100 percent"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Obdty::_11)
    }
}
#[doc = "Forcible GTIOCB Output Duty Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Obdtyf {
    #[doc = "0: Not forcibly set"]
    _0 = 0,
    #[doc = "1: Forcibly set"]
    _1 = 1,
}
impl From<Obdtyf> for bool {
    #[inline(always)]
    fn from(variant: Obdtyf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OBDTYF` reader - Forcible GTIOCB Output Duty Setting"]
pub type ObdtyfR = crate::BitReader<Obdtyf>;
impl ObdtyfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Obdtyf {
        match self.bits {
            false => Obdtyf::_0,
            true => Obdtyf::_1,
        }
    }
    #[doc = "Not forcibly set"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Obdtyf::_0
    }
    #[doc = "Forcibly set"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Obdtyf::_1
    }
}
#[doc = "Field `OBDTYF` writer - Forcible GTIOCB Output Duty Setting"]
pub type ObdtyfW<'a, REG> = crate::BitWriter<'a, REG, Obdtyf>;
impl<'a, REG> ObdtyfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not forcibly set"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Obdtyf::_0)
    }
    #[doc = "Forcibly set"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Obdtyf::_1)
    }
}
#[doc = "GTIOCB Output Value Selecting after Releasing 0 percent/100 percent Duty Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Obdtyr {
    #[doc = "0: Apply output value set in 0 percent/100 percent duty to GTIOB\\[3:2\\] function after releasing 0 percent/100 percent duty setting."]
    _0 = 0,
    #[doc = "1: Apply masked compare match output value to GTIOB\\[3:2\\] function after releasing 0 percent/100 percent duty setting."]
    _1 = 1,
}
impl From<Obdtyr> for bool {
    #[inline(always)]
    fn from(variant: Obdtyr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OBDTYR` reader - GTIOCB Output Value Selecting after Releasing 0 percent/100 percent Duty Setting"]
pub type ObdtyrR = crate::BitReader<Obdtyr>;
impl ObdtyrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Obdtyr {
        match self.bits {
            false => Obdtyr::_0,
            true => Obdtyr::_1,
        }
    }
    #[doc = "Apply output value set in 0 percent/100 percent duty to GTIOB\\[3:2\\] function after releasing 0 percent/100 percent duty setting."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Obdtyr::_0
    }
    #[doc = "Apply masked compare match output value to GTIOB\\[3:2\\] function after releasing 0 percent/100 percent duty setting."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Obdtyr::_1
    }
}
#[doc = "Field `OBDTYR` writer - GTIOCB Output Value Selecting after Releasing 0 percent/100 percent Duty Setting"]
pub type ObdtyrW<'a, REG> = crate::BitWriter<'a, REG, Obdtyr>;
impl<'a, REG> ObdtyrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Apply output value set in 0 percent/100 percent duty to GTIOB\\[3:2\\] function after releasing 0 percent/100 percent duty setting."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Obdtyr::_0)
    }
    #[doc = "Apply masked compare match output value to GTIOB\\[3:2\\] function after releasing 0 percent/100 percent duty setting."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Obdtyr::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Count Direction Setting"]
    #[inline(always)]
    pub fn ud(&self) -> UdR {
        UdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Forcible Count Direction Setting"]
    #[inline(always)]
    pub fn udf(&self) -> UdfR {
        UdfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:17 - GTIOCA Output Duty Setting"]
    #[inline(always)]
    pub fn oadty(&self) -> OadtyR {
        OadtyR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Forcible GTIOCA Output Duty Setting"]
    #[inline(always)]
    pub fn oadtyf(&self) -> OadtyfR {
        OadtyfR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GTIOCA Output Value Selecting after Releasing 0 percent/100 percent Duty Setting"]
    #[inline(always)]
    pub fn oadtyr(&self) -> OadtyrR {
        OadtyrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:25 - GTIOCB Output Duty Setting"]
    #[inline(always)]
    pub fn obdty(&self) -> ObdtyR {
        ObdtyR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Forcible GTIOCB Output Duty Setting"]
    #[inline(always)]
    pub fn obdtyf(&self) -> ObdtyfR {
        ObdtyfR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - GTIOCB Output Value Selecting after Releasing 0 percent/100 percent Duty Setting"]
    #[inline(always)]
    pub fn obdtyr(&self) -> ObdtyrR {
        ObdtyrR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTUDDTYC")
            .field("obdtyr", &self.obdtyr())
            .field("obdtyf", &self.obdtyf())
            .field("obdty", &self.obdty())
            .field("oadtyr", &self.oadtyr())
            .field("oadtyf", &self.oadtyf())
            .field("oadty", &self.oadty())
            .field("udf", &self.udf())
            .field("ud", &self.ud())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Count Direction Setting"]
    #[inline(always)]
    pub fn ud(&mut self) -> UdW<GtuddtycSpec> {
        UdW::new(self, 0)
    }
    #[doc = "Bit 1 - Forcible Count Direction Setting"]
    #[inline(always)]
    pub fn udf(&mut self) -> UdfW<GtuddtycSpec> {
        UdfW::new(self, 1)
    }
    #[doc = "Bits 16:17 - GTIOCA Output Duty Setting"]
    #[inline(always)]
    pub fn oadty(&mut self) -> OadtyW<GtuddtycSpec> {
        OadtyW::new(self, 16)
    }
    #[doc = "Bit 18 - Forcible GTIOCA Output Duty Setting"]
    #[inline(always)]
    pub fn oadtyf(&mut self) -> OadtyfW<GtuddtycSpec> {
        OadtyfW::new(self, 18)
    }
    #[doc = "Bit 19 - GTIOCA Output Value Selecting after Releasing 0 percent/100 percent Duty Setting"]
    #[inline(always)]
    pub fn oadtyr(&mut self) -> OadtyrW<GtuddtycSpec> {
        OadtyrW::new(self, 19)
    }
    #[doc = "Bits 24:25 - GTIOCB Output Duty Setting"]
    #[inline(always)]
    pub fn obdty(&mut self) -> ObdtyW<GtuddtycSpec> {
        ObdtyW::new(self, 24)
    }
    #[doc = "Bit 26 - Forcible GTIOCB Output Duty Setting"]
    #[inline(always)]
    pub fn obdtyf(&mut self) -> ObdtyfW<GtuddtycSpec> {
        ObdtyfW::new(self, 26)
    }
    #[doc = "Bit 27 - GTIOCB Output Value Selecting after Releasing 0 percent/100 percent Duty Setting"]
    #[inline(always)]
    pub fn obdtyr(&mut self) -> ObdtyrW<GtuddtycSpec> {
        ObdtyrW::new(self, 27)
    }
}
#[doc = "General PWM Timer Count Direction and Duty Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtuddtyc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtuddtyc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtuddtycSpec;
impl crate::RegisterSpec for GtuddtycSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtuddtyc::R`](R) reader structure"]
impl crate::Readable for GtuddtycSpec {}
#[doc = "`write(|w| ..)` method takes [`gtuddtyc::W`](W) writer structure"]
impl crate::Writable for GtuddtycSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTUDDTYC to value 0x01"]
impl crate::Resettable for GtuddtycSpec {
    const RESET_VALUE: u32 = 0x01;
}
