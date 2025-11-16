#[doc = "Register `AGTCR` reader"]
pub type R = crate::R<AgtcrSpec>;
#[doc = "Register `AGTCR` writer"]
pub type W = crate::W<AgtcrSpec>;
#[doc = "AGT count start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstart {
    #[doc = "0: Count stops"]
    _0 = 0,
    #[doc = "1: Count starts."]
    _1 = 1,
}
impl From<Tstart> for bool {
    #[inline(always)]
    fn from(variant: Tstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTART` reader - AGT count start"]
pub type TstartR = crate::BitReader<Tstart>;
impl TstartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstart {
        match self.bits {
            false => Tstart::_0,
            true => Tstart::_1,
        }
    }
    #[doc = "Count stops"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tstart::_0
    }
    #[doc = "Count starts."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tstart::_1
    }
}
#[doc = "Field `TSTART` writer - AGT count start"]
pub type TstartW<'a, REG> = crate::BitWriter<'a, REG, Tstart>;
impl<'a, REG> TstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Count stops"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tstart::_0)
    }
    #[doc = "Count starts."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tstart::_1)
    }
}
#[doc = "AGT count status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcstf {
    #[doc = "0: Count stops"]
    _0 = 0,
    #[doc = "1: Count in progress."]
    _1 = 1,
}
impl From<Tcstf> for bool {
    #[inline(always)]
    fn from(variant: Tcstf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCSTF` reader - AGT count status flag"]
pub type TcstfR = crate::BitReader<Tcstf>;
impl TcstfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcstf {
        match self.bits {
            false => Tcstf::_0,
            true => Tcstf::_1,
        }
    }
    #[doc = "Count stops"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcstf::_0
    }
    #[doc = "Count in progress."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcstf::_1
    }
}
#[doc = "AGT count forced stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstop {
    #[doc = "0: Writing is invalid"]
    _0 = 0,
    #[doc = "1: The count is forcibly stopped."]
    _1 = 1,
}
impl From<Tstop> for bool {
    #[inline(always)]
    fn from(variant: Tstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTOP` writer - AGT count forced stop"]
pub type TstopW<'a, REG> = crate::BitWriter<'a, REG, Tstop>;
impl<'a, REG> TstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing is invalid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tstop::_0)
    }
    #[doc = "The count is forcibly stopped."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tstop::_1)
    }
}
#[doc = "Active edge judgment flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tedgf {
    #[doc = "0: No active edge received"]
    _0 = 0,
    #[doc = "1: Active edge received."]
    _1 = 1,
}
impl From<Tedgf> for bool {
    #[inline(always)]
    fn from(variant: Tedgf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEDGF` reader - Active edge judgment flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type TedgfR = crate::BitReader<Tedgf>;
impl TedgfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tedgf {
        match self.bits {
            false => Tedgf::_0,
            true => Tedgf::_1,
        }
    }
    #[doc = "No active edge received"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tedgf::_0
    }
    #[doc = "Active edge received."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tedgf::_1
    }
}
#[doc = "Field `TEDGF` writer - Active edge judgment flag"]
pub type TedgfW<'a, REG> = crate::BitWriter0C<'a, REG, Tedgf>;
impl<'a, REG> TedgfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No active edge received"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tedgf::_0)
    }
    #[doc = "Active edge received."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tedgf::_1)
    }
}
#[doc = "Underflow flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tundf {
    #[doc = "0: No match"]
    _0 = 0,
    #[doc = "1: Match."]
    _1 = 1,
}
impl From<Tundf> for bool {
    #[inline(always)]
    fn from(variant: Tundf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TUNDF` reader - Underflow flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type TundfR = crate::BitReader<Tundf>;
impl TundfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tundf {
        match self.bits {
            false => Tundf::_0,
            true => Tundf::_1,
        }
    }
    #[doc = "No match"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tundf::_0
    }
    #[doc = "Match."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tundf::_1
    }
}
#[doc = "Field `TUNDF` writer - Underflow flag"]
pub type TundfW<'a, REG> = crate::BitWriter0C<'a, REG, Tundf>;
impl<'a, REG> TundfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No match"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tundf::_0)
    }
    #[doc = "Match."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tundf::_1)
    }
}
#[doc = "Compare match A flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcmaf {
    #[doc = "0: No match"]
    _0 = 0,
    #[doc = "1: Match."]
    _1 = 1,
}
impl From<Tcmaf> for bool {
    #[inline(always)]
    fn from(variant: Tcmaf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCMAF` reader - Compare match A flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type TcmafR = crate::BitReader<Tcmaf>;
impl TcmafR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcmaf {
        match self.bits {
            false => Tcmaf::_0,
            true => Tcmaf::_1,
        }
    }
    #[doc = "No match"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcmaf::_0
    }
    #[doc = "Match."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcmaf::_1
    }
}
#[doc = "Field `TCMAF` writer - Compare match A flag"]
pub type TcmafW<'a, REG> = crate::BitWriter0C<'a, REG, Tcmaf>;
impl<'a, REG> TcmafW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No match"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcmaf::_0)
    }
    #[doc = "Match."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcmaf::_1)
    }
}
#[doc = "Compare match B flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcmbf {
    #[doc = "0: No match"]
    _0 = 0,
    #[doc = "1: Match."]
    _1 = 1,
}
impl From<Tcmbf> for bool {
    #[inline(always)]
    fn from(variant: Tcmbf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCMBF` reader - Compare match B flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type TcmbfR = crate::BitReader<Tcmbf>;
impl TcmbfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcmbf {
        match self.bits {
            false => Tcmbf::_0,
            true => Tcmbf::_1,
        }
    }
    #[doc = "No match"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tcmbf::_0
    }
    #[doc = "Match."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tcmbf::_1
    }
}
#[doc = "Field `TCMBF` writer - Compare match B flag"]
pub type TcmbfW<'a, REG> = crate::BitWriter0C<'a, REG, Tcmbf>;
impl<'a, REG> TcmbfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No match"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcmbf::_0)
    }
    #[doc = "Match."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcmbf::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AGT count start"]
    #[inline(always)]
    pub fn tstart(&self) -> TstartR {
        TstartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AGT count status flag"]
    #[inline(always)]
    pub fn tcstf(&self) -> TcstfR {
        TcstfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Active edge judgment flag"]
    #[inline(always)]
    pub fn tedgf(&self) -> TedgfR {
        TedgfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Underflow flag"]
    #[inline(always)]
    pub fn tundf(&self) -> TundfR {
        TundfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Compare match A flag"]
    #[inline(always)]
    pub fn tcmaf(&self) -> TcmafR {
        TcmafR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Compare match B flag"]
    #[inline(always)]
    pub fn tcmbf(&self) -> TcmbfR {
        TcmbfR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGTCR")
            .field("tcstf", &self.tcstf())
            .field("tstart", &self.tstart())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - AGT count start"]
    #[inline(always)]
    pub fn tstart(&mut self) -> TstartW<AgtcrSpec> {
        TstartW::new(self, 0)
    }
    #[doc = "Bit 2 - AGT count forced stop"]
    #[inline(always)]
    pub fn tstop(&mut self) -> TstopW<AgtcrSpec> {
        TstopW::new(self, 2)
    }
    #[doc = "Bit 4 - Active edge judgment flag"]
    #[inline(always)]
    pub fn tedgf(&mut self) -> TedgfW<AgtcrSpec> {
        TedgfW::new(self, 4)
    }
    #[doc = "Bit 5 - Underflow flag"]
    #[inline(always)]
    pub fn tundf(&mut self) -> TundfW<AgtcrSpec> {
        TundfW::new(self, 5)
    }
    #[doc = "Bit 6 - Compare match A flag"]
    #[inline(always)]
    pub fn tcmaf(&mut self) -> TcmafW<AgtcrSpec> {
        TcmafW::new(self, 6)
    }
    #[doc = "Bit 7 - Compare match B flag"]
    #[inline(always)]
    pub fn tcmbf(&mut self) -> TcmbfW<AgtcrSpec> {
        TcmbfW::new(self, 7)
    }
}
#[doc = "AGT Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`agtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AgtcrSpec;
impl crate::RegisterSpec for AgtcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`agtcr::R`](R) reader structure"]
impl crate::Readable for AgtcrSpec {}
#[doc = "`write(|w| ..)` method takes [`agtcr::W`](W) writer structure"]
impl crate::Writable for AgtcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0xf0;
}
#[doc = "`reset()` method sets AGTCR to value 0"]
impl crate::Resettable for AgtcrSpec {}
