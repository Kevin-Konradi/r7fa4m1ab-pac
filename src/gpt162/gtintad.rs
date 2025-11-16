#[doc = "Register `GTINTAD` reader"]
pub type R = crate::R<GtintadSpec>;
#[doc = "Register `GTINTAD` writer"]
pub type W = crate::W<GtintadSpec>;
#[doc = "Output Disable Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Grp {
    #[doc = "0: Group A output disable request"]
    _00 = 0,
    #[doc = "1: Group B output disable request"]
    _01 = 1,
    #[doc = "2: Setting prohibited"]
    Others = 2,
}
impl From<Grp> for u8 {
    #[inline(always)]
    fn from(variant: Grp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Grp {
    type Ux = u8;
}
impl crate::IsEnum for Grp {}
#[doc = "Field `GRP` reader - Output Disable Source Select"]
pub type GrpR = crate::FieldReader<Grp>;
impl GrpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Grp {
        match self.bits {
            0 => Grp::_00,
            1 => Grp::_01,
            _ => Grp::Others,
        }
    }
    #[doc = "Group A output disable request"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Grp::_00
    }
    #[doc = "Group B output disable request"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Grp::_01
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Grp::Others)
    }
}
#[doc = "Field `GRP` writer - Output Disable Source Select"]
pub type GrpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Grp, crate::Safe>;
impl<'a, REG> GrpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Group A output disable request"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Grp::_00)
    }
    #[doc = "Group B output disable request"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Grp::_01)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Grp::Others)
    }
}
#[doc = "Same Time Output Level High Disable Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Grpabh {
    #[doc = "0: Same time output level high disable request is disabled."]
    _0 = 0,
    #[doc = "1: Same time output level high disable request is enabled."]
    _1 = 1,
}
impl From<Grpabh> for bool {
    #[inline(always)]
    fn from(variant: Grpabh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GRPABH` reader - Same Time Output Level High Disable Request Enable"]
pub type GrpabhR = crate::BitReader<Grpabh>;
impl GrpabhR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Grpabh {
        match self.bits {
            false => Grpabh::_0,
            true => Grpabh::_1,
        }
    }
    #[doc = "Same time output level high disable request is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Grpabh::_0
    }
    #[doc = "Same time output level high disable request is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Grpabh::_1
    }
}
#[doc = "Field `GRPABH` writer - Same Time Output Level High Disable Request Enable"]
pub type GrpabhW<'a, REG> = crate::BitWriter<'a, REG, Grpabh>;
impl<'a, REG> GrpabhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Same time output level high disable request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Grpabh::_0)
    }
    #[doc = "Same time output level high disable request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Grpabh::_1)
    }
}
#[doc = "Same Time Output Level Low Disable Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Grpabl {
    #[doc = "0: Same time output level low disable request is disabled."]
    _0 = 0,
    #[doc = "1: Same time output level low disable request is enabled."]
    _1 = 1,
}
impl From<Grpabl> for bool {
    #[inline(always)]
    fn from(variant: Grpabl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GRPABL` reader - Same Time Output Level Low Disable Request Enable"]
pub type GrpablR = crate::BitReader<Grpabl>;
impl GrpablR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Grpabl {
        match self.bits {
            false => Grpabl::_0,
            true => Grpabl::_1,
        }
    }
    #[doc = "Same time output level low disable request is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Grpabl::_0
    }
    #[doc = "Same time output level low disable request is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Grpabl::_1
    }
}
#[doc = "Field `GRPABL` writer - Same Time Output Level Low Disable Request Enable"]
pub type GrpablW<'a, REG> = crate::BitWriter<'a, REG, Grpabl>;
impl<'a, REG> GrpablW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Same time output level low disable request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Grpabl::_0)
    }
    #[doc = "Same time output level low disable request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Grpabl::_1)
    }
}
impl R {
    #[doc = "Bits 24:25 - Output Disable Source Select"]
    #[inline(always)]
    pub fn grp(&self) -> GrpR {
        GrpR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 29 - Same Time Output Level High Disable Request Enable"]
    #[inline(always)]
    pub fn grpabh(&self) -> GrpabhR {
        GrpabhR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Same Time Output Level Low Disable Request Enable"]
    #[inline(always)]
    pub fn grpabl(&self) -> GrpablR {
        GrpablR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTINTAD")
            .field("grpabl", &self.grpabl())
            .field("grpabh", &self.grpabh())
            .field("grp", &self.grp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 24:25 - Output Disable Source Select"]
    #[inline(always)]
    pub fn grp(&mut self) -> GrpW<GtintadSpec> {
        GrpW::new(self, 24)
    }
    #[doc = "Bit 29 - Same Time Output Level High Disable Request Enable"]
    #[inline(always)]
    pub fn grpabh(&mut self) -> GrpabhW<GtintadSpec> {
        GrpabhW::new(self, 29)
    }
    #[doc = "Bit 30 - Same Time Output Level Low Disable Request Enable"]
    #[inline(always)]
    pub fn grpabl(&mut self) -> GrpablW<GtintadSpec> {
        GrpablW::new(self, 30)
    }
}
#[doc = "General PWM Timer Interrupt Output Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtintad::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtintad::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtintadSpec;
impl crate::RegisterSpec for GtintadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtintad::R`](R) reader structure"]
impl crate::Readable for GtintadSpec {}
#[doc = "`write(|w| ..)` method takes [`gtintad::W`](W) writer structure"]
impl crate::Writable for GtintadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTINTAD to value 0"]
impl crate::Resettable for GtintadSpec {}
