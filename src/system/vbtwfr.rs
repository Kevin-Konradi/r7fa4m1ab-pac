#[doc = "Register `VBTWFR` reader"]
pub type R = crate::R<VbtwfrSpec>;
#[doc = "Register `VBTWFR` writer"]
pub type W = crate::W<VbtwfrSpec>;
#[doc = "VBATWIO0 Wakeup Trigger Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vch0f {
    #[doc = "0: No wakeup trigger by the VBATWIO0 pin is generated"]
    _0 = 0,
    #[doc = "1: A wakeup trigger by the VBATWIO0 pin is generated"]
    _1 = 1,
}
impl From<Vch0f> for bool {
    #[inline(always)]
    fn from(variant: Vch0f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCH0F` reader - VBATWIO0 Wakeup Trigger Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Vch0fR = crate::BitReader<Vch0f>;
impl Vch0fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vch0f {
        match self.bits {
            false => Vch0f::_0,
            true => Vch0f::_1,
        }
    }
    #[doc = "No wakeup trigger by the VBATWIO0 pin is generated"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vch0f::_0
    }
    #[doc = "A wakeup trigger by the VBATWIO0 pin is generated"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vch0f::_1
    }
}
#[doc = "Field `VCH0F` writer - VBATWIO0 Wakeup Trigger Flag"]
pub type Vch0fW<'a, REG> = crate::BitWriter0C<'a, REG, Vch0f>;
impl<'a, REG> Vch0fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No wakeup trigger by the VBATWIO0 pin is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vch0f::_0)
    }
    #[doc = "A wakeup trigger by the VBATWIO0 pin is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vch0f::_1)
    }
}
#[doc = "VBATWIO1 Wakeup Trigger Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vch1f {
    #[doc = "0: No wakeup trigger by the VBATWIO1 pin is generated"]
    _0 = 0,
    #[doc = "1: A wakeup trigger by the VBATWIO1 pin is generated"]
    _1 = 1,
}
impl From<Vch1f> for bool {
    #[inline(always)]
    fn from(variant: Vch1f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCH1F` reader - VBATWIO1 Wakeup Trigger Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Vch1fR = crate::BitReader<Vch1f>;
impl Vch1fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vch1f {
        match self.bits {
            false => Vch1f::_0,
            true => Vch1f::_1,
        }
    }
    #[doc = "No wakeup trigger by the VBATWIO1 pin is generated"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vch1f::_0
    }
    #[doc = "A wakeup trigger by the VBATWIO1 pin is generated"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vch1f::_1
    }
}
#[doc = "Field `VCH1F` writer - VBATWIO1 Wakeup Trigger Flag"]
pub type Vch1fW<'a, REG> = crate::BitWriter0C<'a, REG, Vch1f>;
impl<'a, REG> Vch1fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No wakeup trigger by the VBATWIO1 pin is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vch1f::_0)
    }
    #[doc = "A wakeup trigger by the VBATWIO1 pin is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vch1f::_1)
    }
}
#[doc = "VBATWIO2 Wakeup Trigger Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vch2f {
    #[doc = "0: No wakeup trigger by the VBATWIO2 pin is generated"]
    _0 = 0,
    #[doc = "1: A wakeup trigger by the VBATWIO2 pin is generated"]
    _1 = 1,
}
impl From<Vch2f> for bool {
    #[inline(always)]
    fn from(variant: Vch2f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCH2F` reader - VBATWIO2 Wakeup Trigger Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Vch2fR = crate::BitReader<Vch2f>;
impl Vch2fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vch2f {
        match self.bits {
            false => Vch2f::_0,
            true => Vch2f::_1,
        }
    }
    #[doc = "No wakeup trigger by the VBATWIO2 pin is generated"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vch2f::_0
    }
    #[doc = "A wakeup trigger by the VBATWIO2 pin is generated"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vch2f::_1
    }
}
#[doc = "Field `VCH2F` writer - VBATWIO2 Wakeup Trigger Flag"]
pub type Vch2fW<'a, REG> = crate::BitWriter0C<'a, REG, Vch2f>;
impl<'a, REG> Vch2fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No wakeup trigger by the VBATWIO2 pin is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vch2f::_0)
    }
    #[doc = "A wakeup trigger by the VBATWIO2 pin is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vch2f::_1)
    }
}
#[doc = "VBATT RTC-Interval Wakeup Trigger Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vrtcif {
    #[doc = "0: No wakeup trigger by the RTC interval is generated"]
    _0 = 0,
    #[doc = "1: A wakeup trigger by the RTC interval is generated"]
    _1 = 1,
}
impl From<Vrtcif> for bool {
    #[inline(always)]
    fn from(variant: Vrtcif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VRTCIF` reader - VBATT RTC-Interval Wakeup Trigger Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type VrtcifR = crate::BitReader<Vrtcif>;
impl VrtcifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vrtcif {
        match self.bits {
            false => Vrtcif::_0,
            true => Vrtcif::_1,
        }
    }
    #[doc = "No wakeup trigger by the RTC interval is generated"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vrtcif::_0
    }
    #[doc = "A wakeup trigger by the RTC interval is generated"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vrtcif::_1
    }
}
#[doc = "Field `VRTCIF` writer - VBATT RTC-Interval Wakeup Trigger Flag"]
pub type VrtcifW<'a, REG> = crate::BitWriter0C<'a, REG, Vrtcif>;
impl<'a, REG> VrtcifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No wakeup trigger by the RTC interval is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vrtcif::_0)
    }
    #[doc = "A wakeup trigger by the RTC interval is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vrtcif::_1)
    }
}
#[doc = "VBATT RTC-Alarm Wakeup Trigger Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vrtcaf {
    #[doc = "0: No wakeup trigger by the RTC alarm is generated"]
    _0 = 0,
    #[doc = "1: A wakeup trigger by the RTC alarm is generated"]
    _1 = 1,
}
impl From<Vrtcaf> for bool {
    #[inline(always)]
    fn from(variant: Vrtcaf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VRTCAF` reader - VBATT RTC-Alarm Wakeup Trigger Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type VrtcafR = crate::BitReader<Vrtcaf>;
impl VrtcafR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vrtcaf {
        match self.bits {
            false => Vrtcaf::_0,
            true => Vrtcaf::_1,
        }
    }
    #[doc = "No wakeup trigger by the RTC alarm is generated"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vrtcaf::_0
    }
    #[doc = "A wakeup trigger by the RTC alarm is generated"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vrtcaf::_1
    }
}
#[doc = "Field `VRTCAF` writer - VBATT RTC-Alarm Wakeup Trigger Flag"]
pub type VrtcafW<'a, REG> = crate::BitWriter0C<'a, REG, Vrtcaf>;
impl<'a, REG> VrtcafW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No wakeup trigger by the RTC alarm is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vrtcaf::_0)
    }
    #[doc = "A wakeup trigger by the RTC alarm is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vrtcaf::_1)
    }
}
impl R {
    #[doc = "Bit 0 - VBATWIO0 Wakeup Trigger Flag"]
    #[inline(always)]
    pub fn vch0f(&self) -> Vch0fR {
        Vch0fR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBATWIO1 Wakeup Trigger Flag"]
    #[inline(always)]
    pub fn vch1f(&self) -> Vch1fR {
        Vch1fR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VBATWIO2 Wakeup Trigger Flag"]
    #[inline(always)]
    pub fn vch2f(&self) -> Vch2fR {
        Vch2fR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VBATT RTC-Interval Wakeup Trigger Flag"]
    #[inline(always)]
    pub fn vrtcif(&self) -> VrtcifR {
        VrtcifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VBATT RTC-Alarm Wakeup Trigger Flag"]
    #[inline(always)]
    pub fn vrtcaf(&self) -> VrtcafR {
        VrtcafR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VBTWFR").finish()
    }
}
impl W {
    #[doc = "Bit 0 - VBATWIO0 Wakeup Trigger Flag"]
    #[inline(always)]
    pub fn vch0f(&mut self) -> Vch0fW<VbtwfrSpec> {
        Vch0fW::new(self, 0)
    }
    #[doc = "Bit 1 - VBATWIO1 Wakeup Trigger Flag"]
    #[inline(always)]
    pub fn vch1f(&mut self) -> Vch1fW<VbtwfrSpec> {
        Vch1fW::new(self, 1)
    }
    #[doc = "Bit 2 - VBATWIO2 Wakeup Trigger Flag"]
    #[inline(always)]
    pub fn vch2f(&mut self) -> Vch2fW<VbtwfrSpec> {
        Vch2fW::new(self, 2)
    }
    #[doc = "Bit 3 - VBATT RTC-Interval Wakeup Trigger Flag"]
    #[inline(always)]
    pub fn vrtcif(&mut self) -> VrtcifW<VbtwfrSpec> {
        VrtcifW::new(self, 3)
    }
    #[doc = "Bit 4 - VBATT RTC-Alarm Wakeup Trigger Flag"]
    #[inline(always)]
    pub fn vrtcaf(&mut self) -> VrtcafW<VbtwfrSpec> {
        VrtcafW::new(self, 4)
    }
}
#[doc = "VBATT Wakeup trigger source Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtwfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtwfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbtwfrSpec;
impl crate::RegisterSpec for VbtwfrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vbtwfr::R`](R) reader structure"]
impl crate::Readable for VbtwfrSpec {}
#[doc = "`write(|w| ..)` method takes [`vbtwfr::W`](W) writer structure"]
impl crate::Writable for VbtwfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x1f;
}
#[doc = "`reset()` method sets VBTWFR to value 0"]
impl crate::Resettable for VbtwfrSpec {}
