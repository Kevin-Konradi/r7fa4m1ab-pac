#[doc = "Register `SNZEDCR` reader"]
pub type R = crate::R<SnzedcrSpec>;
#[doc = "Register `SNZEDCR` writer"]
pub type W = crate::W<SnzedcrSpec>;
#[doc = "AGT1 Underflow Snooze End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Agtunfed {
    #[doc = "0: Disable the Snooze End request"]
    _0 = 0,
    #[doc = "1: Enable the Snooze End request"]
    _1 = 1,
}
impl From<Agtunfed> for bool {
    #[inline(always)]
    fn from(variant: Agtunfed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AGTUNFED` reader - AGT1 Underflow Snooze End Enable"]
pub type AgtunfedR = crate::BitReader<Agtunfed>;
impl AgtunfedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Agtunfed {
        match self.bits {
            false => Agtunfed::_0,
            true => Agtunfed::_1,
        }
    }
    #[doc = "Disable the Snooze End request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Agtunfed::_0
    }
    #[doc = "Enable the Snooze End request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Agtunfed::_1
    }
}
#[doc = "Field `AGTUNFED` writer - AGT1 Underflow Snooze End Enable"]
pub type AgtunfedW<'a, REG> = crate::BitWriter<'a, REG, Agtunfed>;
impl<'a, REG> AgtunfedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the Snooze End request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Agtunfed::_0)
    }
    #[doc = "Enable the Snooze End request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Agtunfed::_1)
    }
}
#[doc = "Last DTC Transmission Completion Snooze End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtczred {
    #[doc = "0: Disable the Snooze End request"]
    _0 = 0,
    #[doc = "1: Enable the Snooze End request"]
    _1 = 1,
}
impl From<Dtczred> for bool {
    #[inline(always)]
    fn from(variant: Dtczred) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTCZRED` reader - Last DTC Transmission Completion Snooze End Enable"]
pub type DtczredR = crate::BitReader<Dtczred>;
impl DtczredR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtczred {
        match self.bits {
            false => Dtczred::_0,
            true => Dtczred::_1,
        }
    }
    #[doc = "Disable the Snooze End request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dtczred::_0
    }
    #[doc = "Enable the Snooze End request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dtczred::_1
    }
}
#[doc = "Field `DTCZRED` writer - Last DTC Transmission Completion Snooze End Enable"]
pub type DtczredW<'a, REG> = crate::BitWriter<'a, REG, Dtczred>;
impl<'a, REG> DtczredW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the Snooze End request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtczred::_0)
    }
    #[doc = "Enable the Snooze End request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtczred::_1)
    }
}
#[doc = "Not Last DTC Transmission Completion Snooze End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtcnzred {
    #[doc = "0: Disable the Snooze End request"]
    _0 = 0,
    #[doc = "1: Enable the Snooze End request"]
    _1 = 1,
}
impl From<Dtcnzred> for bool {
    #[inline(always)]
    fn from(variant: Dtcnzred) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTCNZRED` reader - Not Last DTC Transmission Completion Snooze End Enable"]
pub type DtcnzredR = crate::BitReader<Dtcnzred>;
impl DtcnzredR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtcnzred {
        match self.bits {
            false => Dtcnzred::_0,
            true => Dtcnzred::_1,
        }
    }
    #[doc = "Disable the Snooze End request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dtcnzred::_0
    }
    #[doc = "Enable the Snooze End request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dtcnzred::_1
    }
}
#[doc = "Field `DTCNZRED` writer - Not Last DTC Transmission Completion Snooze End Enable"]
pub type DtcnzredW<'a, REG> = crate::BitWriter<'a, REG, Dtcnzred>;
impl<'a, REG> DtcnzredW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the Snooze End request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcnzred::_0)
    }
    #[doc = "Enable the Snooze End request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcnzred::_1)
    }
}
#[doc = "ADC140 Compare Match Snooze End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ad0mated {
    #[doc = "0: Disable the Snooze End request"]
    _0 = 0,
    #[doc = "1: Enable the Snooze End request"]
    _1 = 1,
}
impl From<Ad0mated> for bool {
    #[inline(always)]
    fn from(variant: Ad0mated) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AD0MATED` reader - ADC140 Compare Match Snooze End Enable"]
pub type Ad0matedR = crate::BitReader<Ad0mated>;
impl Ad0matedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ad0mated {
        match self.bits {
            false => Ad0mated::_0,
            true => Ad0mated::_1,
        }
    }
    #[doc = "Disable the Snooze End request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ad0mated::_0
    }
    #[doc = "Enable the Snooze End request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ad0mated::_1
    }
}
#[doc = "Field `AD0MATED` writer - ADC140 Compare Match Snooze End Enable"]
pub type Ad0matedW<'a, REG> = crate::BitWriter<'a, REG, Ad0mated>;
impl<'a, REG> Ad0matedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the Snooze End request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ad0mated::_0)
    }
    #[doc = "Enable the Snooze End request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ad0mated::_1)
    }
}
#[doc = "ADC140 Compare Mismatch Snooze End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ad0umted {
    #[doc = "0: Disable the Snooze End request"]
    _0 = 0,
    #[doc = "1: Enable the Snooze End request"]
    _1 = 1,
}
impl From<Ad0umted> for bool {
    #[inline(always)]
    fn from(variant: Ad0umted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AD0UMTED` reader - ADC140 Compare Mismatch Snooze End Enable"]
pub type Ad0umtedR = crate::BitReader<Ad0umted>;
impl Ad0umtedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ad0umted {
        match self.bits {
            false => Ad0umted::_0,
            true => Ad0umted::_1,
        }
    }
    #[doc = "Disable the Snooze End request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ad0umted::_0
    }
    #[doc = "Enable the Snooze End request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ad0umted::_1
    }
}
#[doc = "Field `AD0UMTED` writer - ADC140 Compare Mismatch Snooze End Enable"]
pub type Ad0umtedW<'a, REG> = crate::BitWriter<'a, REG, Ad0umted>;
impl<'a, REG> Ad0umtedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the Snooze End request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ad0umted::_0)
    }
    #[doc = "Enable the Snooze End request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ad0umted::_1)
    }
}
#[doc = "SCI0 Address Mismatch Snooze End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sci0umted {
    #[doc = "0: Disable the Snooze End request"]
    _0 = 0,
    #[doc = "1: Enable the Snooze End request"]
    _1 = 1,
}
impl From<Sci0umted> for bool {
    #[inline(always)]
    fn from(variant: Sci0umted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCI0UMTED` reader - SCI0 Address Mismatch Snooze End Enable"]
pub type Sci0umtedR = crate::BitReader<Sci0umted>;
impl Sci0umtedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sci0umted {
        match self.bits {
            false => Sci0umted::_0,
            true => Sci0umted::_1,
        }
    }
    #[doc = "Disable the Snooze End request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sci0umted::_0
    }
    #[doc = "Enable the Snooze End request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sci0umted::_1
    }
}
#[doc = "Field `SCI0UMTED` writer - SCI0 Address Mismatch Snooze End Enable"]
pub type Sci0umtedW<'a, REG> = crate::BitWriter<'a, REG, Sci0umted>;
impl<'a, REG> Sci0umtedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the Snooze End request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sci0umted::_0)
    }
    #[doc = "Enable the Snooze End request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sci0umted::_1)
    }
}
impl R {
    #[doc = "Bit 0 - AGT1 Underflow Snooze End Enable"]
    #[inline(always)]
    pub fn agtunfed(&self) -> AgtunfedR {
        AgtunfedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Last DTC Transmission Completion Snooze End Enable"]
    #[inline(always)]
    pub fn dtczred(&self) -> DtczredR {
        DtczredR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Not Last DTC Transmission Completion Snooze End Enable"]
    #[inline(always)]
    pub fn dtcnzred(&self) -> DtcnzredR {
        DtcnzredR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC140 Compare Match Snooze End Enable"]
    #[inline(always)]
    pub fn ad0mated(&self) -> Ad0matedR {
        Ad0matedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC140 Compare Mismatch Snooze End Enable"]
    #[inline(always)]
    pub fn ad0umted(&self) -> Ad0umtedR {
        Ad0umtedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - SCI0 Address Mismatch Snooze End Enable"]
    #[inline(always)]
    pub fn sci0umted(&self) -> Sci0umtedR {
        Sci0umtedR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SNZEDCR")
            .field("sci0umted", &self.sci0umted())
            .field("ad0umted", &self.ad0umted())
            .field("ad0mated", &self.ad0mated())
            .field("dtcnzred", &self.dtcnzred())
            .field("dtczred", &self.dtczred())
            .field("agtunfed", &self.agtunfed())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - AGT1 Underflow Snooze End Enable"]
    #[inline(always)]
    pub fn agtunfed(&mut self) -> AgtunfedW<SnzedcrSpec> {
        AgtunfedW::new(self, 0)
    }
    #[doc = "Bit 1 - Last DTC Transmission Completion Snooze End Enable"]
    #[inline(always)]
    pub fn dtczred(&mut self) -> DtczredW<SnzedcrSpec> {
        DtczredW::new(self, 1)
    }
    #[doc = "Bit 2 - Not Last DTC Transmission Completion Snooze End Enable"]
    #[inline(always)]
    pub fn dtcnzred(&mut self) -> DtcnzredW<SnzedcrSpec> {
        DtcnzredW::new(self, 2)
    }
    #[doc = "Bit 3 - ADC140 Compare Match Snooze End Enable"]
    #[inline(always)]
    pub fn ad0mated(&mut self) -> Ad0matedW<SnzedcrSpec> {
        Ad0matedW::new(self, 3)
    }
    #[doc = "Bit 4 - ADC140 Compare Mismatch Snooze End Enable"]
    #[inline(always)]
    pub fn ad0umted(&mut self) -> Ad0umtedW<SnzedcrSpec> {
        Ad0umtedW::new(self, 4)
    }
    #[doc = "Bit 7 - SCI0 Address Mismatch Snooze End Enable"]
    #[inline(always)]
    pub fn sci0umted(&mut self) -> Sci0umtedW<SnzedcrSpec> {
        Sci0umtedW::new(self, 7)
    }
}
#[doc = "Snooze End Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`snzedcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzedcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SnzedcrSpec;
impl crate::RegisterSpec for SnzedcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`snzedcr::R`](R) reader structure"]
impl crate::Readable for SnzedcrSpec {}
#[doc = "`write(|w| ..)` method takes [`snzedcr::W`](W) writer structure"]
impl crate::Writable for SnzedcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SNZEDCR to value 0"]
impl crate::Resettable for SnzedcrSpec {}
