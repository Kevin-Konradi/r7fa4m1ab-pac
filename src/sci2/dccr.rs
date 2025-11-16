#[doc = "Register `DCCR` reader"]
pub type R = crate::R<DccrSpec>;
#[doc = "Register `DCCR` writer"]
pub type W = crate::W<DccrSpec>;
#[doc = "Data Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcmf {
    #[doc = "0: No matched"]
    _0 = 0,
    #[doc = "1: Matched"]
    _1 = 1,
}
impl From<Dcmf> for bool {
    #[inline(always)]
    fn from(variant: Dcmf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMF` reader - Data Compare Match Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type DcmfR = crate::BitReader<Dcmf>;
impl DcmfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcmf {
        match self.bits {
            false => Dcmf::_0,
            true => Dcmf::_1,
        }
    }
    #[doc = "No matched"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dcmf::_0
    }
    #[doc = "Matched"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dcmf::_1
    }
}
#[doc = "Field `DCMF` writer - Data Compare Match Flag"]
pub type DcmfW<'a, REG> = crate::BitWriter0C<'a, REG, Dcmf>;
impl<'a, REG> DcmfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No matched"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dcmf::_0)
    }
    #[doc = "Matched"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcmf::_1)
    }
}
#[doc = "Data Compare Match Parity Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dper {
    #[doc = "0: No parity error occurred"]
    _0 = 0,
    #[doc = "1: A parity error has occurred"]
    _1 = 1,
}
impl From<Dper> for bool {
    #[inline(always)]
    fn from(variant: Dper) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPER` reader - Data Compare Match Parity Error Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type DperR = crate::BitReader<Dper>;
impl DperR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dper {
        match self.bits {
            false => Dper::_0,
            true => Dper::_1,
        }
    }
    #[doc = "No parity error occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dper::_0
    }
    #[doc = "A parity error has occurred"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dper::_1
    }
}
#[doc = "Field `DPER` writer - Data Compare Match Parity Error Flag"]
pub type DperW<'a, REG> = crate::BitWriter0C<'a, REG, Dper>;
impl<'a, REG> DperW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No parity error occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dper::_0)
    }
    #[doc = "A parity error has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dper::_1)
    }
}
#[doc = "Data Compare Match Framing Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dfer {
    #[doc = "0: No framing error occurred"]
    _0 = 0,
    #[doc = "1: A framing error has occurred"]
    _1 = 1,
}
impl From<Dfer> for bool {
    #[inline(always)]
    fn from(variant: Dfer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFER` reader - Data Compare Match Framing Error Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type DferR = crate::BitReader<Dfer>;
impl DferR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dfer {
        match self.bits {
            false => Dfer::_0,
            true => Dfer::_1,
        }
    }
    #[doc = "No framing error occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dfer::_0
    }
    #[doc = "A framing error has occurred"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dfer::_1
    }
}
#[doc = "Field `DFER` writer - Data Compare Match Framing Error Flag"]
pub type DferW<'a, REG> = crate::BitWriter0C<'a, REG, Dfer>;
impl<'a, REG> DferW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No framing error occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dfer::_0)
    }
    #[doc = "A framing error has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dfer::_1)
    }
}
#[doc = "ID frame select (Valid only in asynchronous mode(including multi-processor)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idsel {
    #[doc = "0: Always compare data regardless of the value of the MPB bit."]
    _0 = 0,
    #[doc = "1: Compare data when the MPB bit is 1 (ID frame) only."]
    _1 = 1,
}
impl From<Idsel> for bool {
    #[inline(always)]
    fn from(variant: Idsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDSEL` reader - ID frame select (Valid only in asynchronous mode(including multi-processor)"]
pub type IdselR = crate::BitReader<Idsel>;
impl IdselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idsel {
        match self.bits {
            false => Idsel::_0,
            true => Idsel::_1,
        }
    }
    #[doc = "Always compare data regardless of the value of the MPB bit."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Idsel::_0
    }
    #[doc = "Compare data when the MPB bit is 1 (ID frame) only."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Idsel::_1
    }
}
#[doc = "Field `IDSEL` writer - ID frame select (Valid only in asynchronous mode(including multi-processor)"]
pub type IdselW<'a, REG> = crate::BitWriter<'a, REG, Idsel>;
impl<'a, REG> IdselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Always compare data regardless of the value of the MPB bit."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Idsel::_0)
    }
    #[doc = "Compare data when the MPB bit is 1 (ID frame) only."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Idsel::_1)
    }
}
#[doc = "Data Compare Match Enable (Valid only in asynchronous mode(including multi-processor)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcme {
    #[doc = "0: Address match function is disabled."]
    _0 = 0,
    #[doc = "1: Address match function is enabled"]
    _1 = 1,
}
impl From<Dcme> for bool {
    #[inline(always)]
    fn from(variant: Dcme) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCME` reader - Data Compare Match Enable (Valid only in asynchronous mode(including multi-processor)"]
pub type DcmeR = crate::BitReader<Dcme>;
impl DcmeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcme {
        match self.bits {
            false => Dcme::_0,
            true => Dcme::_1,
        }
    }
    #[doc = "Address match function is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dcme::_0
    }
    #[doc = "Address match function is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dcme::_1
    }
}
#[doc = "Field `DCME` writer - Data Compare Match Enable (Valid only in asynchronous mode(including multi-processor)"]
pub type DcmeW<'a, REG> = crate::BitWriter<'a, REG, Dcme>;
impl<'a, REG> DcmeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Address match function is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dcme::_0)
    }
    #[doc = "Address match function is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcme::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Data Compare Match Flag"]
    #[inline(always)]
    pub fn dcmf(&self) -> DcmfR {
        DcmfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Data Compare Match Parity Error Flag"]
    #[inline(always)]
    pub fn dper(&self) -> DperR {
        DperR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Compare Match Framing Error Flag"]
    #[inline(always)]
    pub fn dfer(&self) -> DferR {
        DferR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - ID frame select (Valid only in asynchronous mode(including multi-processor)"]
    #[inline(always)]
    pub fn idsel(&self) -> IdselR {
        IdselR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data Compare Match Enable (Valid only in asynchronous mode(including multi-processor)"]
    #[inline(always)]
    pub fn dcme(&self) -> DcmeR {
        DcmeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCCR")
            .field("dcme", &self.dcme())
            .field("idsel", &self.idsel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Data Compare Match Flag"]
    #[inline(always)]
    pub fn dcmf(&mut self) -> DcmfW<DccrSpec> {
        DcmfW::new(self, 0)
    }
    #[doc = "Bit 3 - Data Compare Match Parity Error Flag"]
    #[inline(always)]
    pub fn dper(&mut self) -> DperW<DccrSpec> {
        DperW::new(self, 3)
    }
    #[doc = "Bit 4 - Data Compare Match Framing Error Flag"]
    #[inline(always)]
    pub fn dfer(&mut self) -> DferW<DccrSpec> {
        DferW::new(self, 4)
    }
    #[doc = "Bit 6 - ID frame select (Valid only in asynchronous mode(including multi-processor)"]
    #[inline(always)]
    pub fn idsel(&mut self) -> IdselW<DccrSpec> {
        IdselW::new(self, 6)
    }
    #[doc = "Bit 7 - Data Compare Match Enable (Valid only in asynchronous mode(including multi-processor)"]
    #[inline(always)]
    pub fn dcme(&mut self) -> DcmeW<DccrSpec> {
        DcmeW::new(self, 7)
    }
}
#[doc = "Data Compare Match Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DccrSpec;
impl crate::RegisterSpec for DccrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dccr::R`](R) reader structure"]
impl crate::Readable for DccrSpec {}
#[doc = "`write(|w| ..)` method takes [`dccr::W`](W) writer structure"]
impl crate::Writable for DccrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x19;
}
#[doc = "`reset()` method sets DCCR to value 0x40"]
impl crate::Resettable for DccrSpec {
    const RESET_VALUE: u8 = 0x40;
}
