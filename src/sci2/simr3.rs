#[doc = "Register `SIMR3` reader"]
pub type R = crate::R<Simr3Spec>;
#[doc = "Register `SIMR3` writer"]
pub type W = crate::W<Simr3Spec>;
#[doc = "Start Condition Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iicstareq {
    #[doc = "0: A start condition is not generated."]
    _0 = 0,
    #[doc = "1: A start condition is generated."]
    _1 = 1,
}
impl From<Iicstareq> for bool {
    #[inline(always)]
    fn from(variant: Iicstareq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IICSTAREQ` reader - Start Condition Generation"]
pub type IicstareqR = crate::BitReader<Iicstareq>;
impl IicstareqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iicstareq {
        match self.bits {
            false => Iicstareq::_0,
            true => Iicstareq::_1,
        }
    }
    #[doc = "A start condition is not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iicstareq::_0
    }
    #[doc = "A start condition is generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iicstareq::_1
    }
}
#[doc = "Field `IICSTAREQ` writer - Start Condition Generation"]
pub type IicstareqW<'a, REG> = crate::BitWriter<'a, REG, Iicstareq>;
impl<'a, REG> IicstareqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A start condition is not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iicstareq::_0)
    }
    #[doc = "A start condition is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iicstareq::_1)
    }
}
#[doc = "Restart Condition Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iicrstareq {
    #[doc = "0: A restart condition is not generated."]
    _0 = 0,
    #[doc = "1: A restart condition is generated."]
    _1 = 1,
}
impl From<Iicrstareq> for bool {
    #[inline(always)]
    fn from(variant: Iicrstareq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IICRSTAREQ` reader - Restart Condition Generation"]
pub type IicrstareqR = crate::BitReader<Iicrstareq>;
impl IicrstareqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iicrstareq {
        match self.bits {
            false => Iicrstareq::_0,
            true => Iicrstareq::_1,
        }
    }
    #[doc = "A restart condition is not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iicrstareq::_0
    }
    #[doc = "A restart condition is generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iicrstareq::_1
    }
}
#[doc = "Field `IICRSTAREQ` writer - Restart Condition Generation"]
pub type IicrstareqW<'a, REG> = crate::BitWriter<'a, REG, Iicrstareq>;
impl<'a, REG> IicrstareqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A restart condition is not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iicrstareq::_0)
    }
    #[doc = "A restart condition is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iicrstareq::_1)
    }
}
#[doc = "Stop Condition Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iicstpreq {
    #[doc = "0: A stop condition is not generated."]
    _0 = 0,
    #[doc = "1: A stop condition is generated."]
    _1 = 1,
}
impl From<Iicstpreq> for bool {
    #[inline(always)]
    fn from(variant: Iicstpreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IICSTPREQ` reader - Stop Condition Generation"]
pub type IicstpreqR = crate::BitReader<Iicstpreq>;
impl IicstpreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iicstpreq {
        match self.bits {
            false => Iicstpreq::_0,
            true => Iicstpreq::_1,
        }
    }
    #[doc = "A stop condition is not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iicstpreq::_0
    }
    #[doc = "A stop condition is generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iicstpreq::_1
    }
}
#[doc = "Field `IICSTPREQ` writer - Stop Condition Generation"]
pub type IicstpreqW<'a, REG> = crate::BitWriter<'a, REG, Iicstpreq>;
impl<'a, REG> IicstpreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A stop condition is not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iicstpreq::_0)
    }
    #[doc = "A stop condition is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iicstpreq::_1)
    }
}
#[doc = "Issuing of Start, Restart, or Stop Condition Completed Flag (When 0 is written to IICSTIF, it is cleared to 0.)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iicstif {
    #[doc = "0: There are no requests for generating conditions or a condition is being generated."]
    _0 = 0,
    #[doc = "1: A start, restart, or stop condition is completely generated."]
    _1 = 1,
}
impl From<Iicstif> for bool {
    #[inline(always)]
    fn from(variant: Iicstif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IICSTIF` reader - Issuing of Start, Restart, or Stop Condition Completed Flag (When 0 is written to IICSTIF, it is cleared to 0.)\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type IicstifR = crate::BitReader<Iicstif>;
impl IicstifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iicstif {
        match self.bits {
            false => Iicstif::_0,
            true => Iicstif::_1,
        }
    }
    #[doc = "There are no requests for generating conditions or a condition is being generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iicstif::_0
    }
    #[doc = "A start, restart, or stop condition is completely generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iicstif::_1
    }
}
#[doc = "Field `IICSTIF` writer - Issuing of Start, Restart, or Stop Condition Completed Flag (When 0 is written to IICSTIF, it is cleared to 0.)"]
pub type IicstifW<'a, REG> = crate::BitWriter0C<'a, REG, Iicstif>;
impl<'a, REG> IicstifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "There are no requests for generating conditions or a condition is being generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iicstif::_0)
    }
    #[doc = "A start, restart, or stop condition is completely generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iicstif::_1)
    }
}
#[doc = "SDA Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iicsdas {
    #[doc = "0: Serial data output"]
    _00 = 0,
    #[doc = "1: Generate a start, restart, or stop condition."]
    _01 = 1,
    #[doc = "2: Output the low level on the SSDAn pin."]
    _10 = 2,
    #[doc = "3: Place the SSDAn pin in the high-impedance state."]
    _11 = 3,
}
impl From<Iicsdas> for u8 {
    #[inline(always)]
    fn from(variant: Iicsdas) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iicsdas {
    type Ux = u8;
}
impl crate::IsEnum for Iicsdas {}
#[doc = "Field `IICSDAS` reader - SDA Output Select"]
pub type IicsdasR = crate::FieldReader<Iicsdas>;
impl IicsdasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iicsdas {
        match self.bits {
            0 => Iicsdas::_00,
            1 => Iicsdas::_01,
            2 => Iicsdas::_10,
            3 => Iicsdas::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Serial data output"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Iicsdas::_00
    }
    #[doc = "Generate a start, restart, or stop condition."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Iicsdas::_01
    }
    #[doc = "Output the low level on the SSDAn pin."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Iicsdas::_10
    }
    #[doc = "Place the SSDAn pin in the high-impedance state."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Iicsdas::_11
    }
}
#[doc = "Field `IICSDAS` writer - SDA Output Select"]
pub type IicsdasW<'a, REG> = crate::FieldWriter<'a, REG, 2, Iicsdas, crate::Safe>;
impl<'a, REG> IicsdasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Serial data output"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Iicsdas::_00)
    }
    #[doc = "Generate a start, restart, or stop condition."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Iicsdas::_01)
    }
    #[doc = "Output the low level on the SSDAn pin."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Iicsdas::_10)
    }
    #[doc = "Place the SSDAn pin in the high-impedance state."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Iicsdas::_11)
    }
}
#[doc = "SCL Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iicscls {
    #[doc = "0: Serial clock output"]
    _00 = 0,
    #[doc = "1: Generate a start, restart, or stop condition."]
    _01 = 1,
    #[doc = "2: Output the low level on the SSCLn pin."]
    _10 = 2,
    #[doc = "3: Place the SSCLn pin in the high-impedance state."]
    _11 = 3,
}
impl From<Iicscls> for u8 {
    #[inline(always)]
    fn from(variant: Iicscls) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iicscls {
    type Ux = u8;
}
impl crate::IsEnum for Iicscls {}
#[doc = "Field `IICSCLS` reader - SCL Output Select"]
pub type IicsclsR = crate::FieldReader<Iicscls>;
impl IicsclsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iicscls {
        match self.bits {
            0 => Iicscls::_00,
            1 => Iicscls::_01,
            2 => Iicscls::_10,
            3 => Iicscls::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Serial clock output"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Iicscls::_00
    }
    #[doc = "Generate a start, restart, or stop condition."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Iicscls::_01
    }
    #[doc = "Output the low level on the SSCLn pin."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Iicscls::_10
    }
    #[doc = "Place the SSCLn pin in the high-impedance state."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Iicscls::_11
    }
}
#[doc = "Field `IICSCLS` writer - SCL Output Select"]
pub type IicsclsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Iicscls, crate::Safe>;
impl<'a, REG> IicsclsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Serial clock output"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Iicscls::_00)
    }
    #[doc = "Generate a start, restart, or stop condition."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Iicscls::_01)
    }
    #[doc = "Output the low level on the SSCLn pin."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Iicscls::_10)
    }
    #[doc = "Place the SSCLn pin in the high-impedance state."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Iicscls::_11)
    }
}
impl R {
    #[doc = "Bit 0 - Start Condition Generation"]
    #[inline(always)]
    pub fn iicstareq(&self) -> IicstareqR {
        IicstareqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Restart Condition Generation"]
    #[inline(always)]
    pub fn iicrstareq(&self) -> IicrstareqR {
        IicrstareqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop Condition Generation"]
    #[inline(always)]
    pub fn iicstpreq(&self) -> IicstpreqR {
        IicstpreqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Issuing of Start, Restart, or Stop Condition Completed Flag (When 0 is written to IICSTIF, it is cleared to 0.)"]
    #[inline(always)]
    pub fn iicstif(&self) -> IicstifR {
        IicstifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - SDA Output Select"]
    #[inline(always)]
    pub fn iicsdas(&self) -> IicsdasR {
        IicsdasR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - SCL Output Select"]
    #[inline(always)]
    pub fn iicscls(&self) -> IicsclsR {
        IicsclsR::new((self.bits >> 6) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIMR3")
            .field("iicscls", &self.iicscls())
            .field("iicsdas", &self.iicsdas())
            .field("iicstpreq", &self.iicstpreq())
            .field("iicrstareq", &self.iicrstareq())
            .field("iicstareq", &self.iicstareq())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Start Condition Generation"]
    #[inline(always)]
    pub fn iicstareq(&mut self) -> IicstareqW<Simr3Spec> {
        IicstareqW::new(self, 0)
    }
    #[doc = "Bit 1 - Restart Condition Generation"]
    #[inline(always)]
    pub fn iicrstareq(&mut self) -> IicrstareqW<Simr3Spec> {
        IicrstareqW::new(self, 1)
    }
    #[doc = "Bit 2 - Stop Condition Generation"]
    #[inline(always)]
    pub fn iicstpreq(&mut self) -> IicstpreqW<Simr3Spec> {
        IicstpreqW::new(self, 2)
    }
    #[doc = "Bit 3 - Issuing of Start, Restart, or Stop Condition Completed Flag (When 0 is written to IICSTIF, it is cleared to 0.)"]
    #[inline(always)]
    pub fn iicstif(&mut self) -> IicstifW<Simr3Spec> {
        IicstifW::new(self, 3)
    }
    #[doc = "Bits 4:5 - SDA Output Select"]
    #[inline(always)]
    pub fn iicsdas(&mut self) -> IicsdasW<Simr3Spec> {
        IicsdasW::new(self, 4)
    }
    #[doc = "Bits 6:7 - SCL Output Select"]
    #[inline(always)]
    pub fn iicscls(&mut self) -> IicsclsW<Simr3Spec> {
        IicsclsW::new(self, 6)
    }
}
#[doc = "I2C Mode Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`simr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Simr3Spec;
impl crate::RegisterSpec for Simr3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`simr3::R`](R) reader structure"]
impl crate::Readable for Simr3Spec {}
#[doc = "`write(|w| ..)` method takes [`simr3::W`](W) writer structure"]
impl crate::Writable for Simr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x08;
}
#[doc = "`reset()` method sets SIMR3 to value 0"]
impl crate::Resettable for Simr3Spec {}
