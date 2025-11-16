#[doc = "Register `POEGG%s` reader"]
pub type R = crate::R<PoeggSpec>;
#[doc = "Register `POEGG%s` writer"]
pub type W = crate::W<PoeggSpec>;
#[doc = "Port Input Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidf {
    #[doc = "0: No output-disable request from the GTETRGn pin has occurred"]
    _0 = 0,
    #[doc = "1: Output-disable request from the GTETRGn pin occurred."]
    _1 = 1,
}
impl From<Pidf> for bool {
    #[inline(always)]
    fn from(variant: Pidf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIDF` reader - Port Input Detection Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type PidfR = crate::BitReader<Pidf>;
impl PidfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pidf {
        match self.bits {
            false => Pidf::_0,
            true => Pidf::_1,
        }
    }
    #[doc = "No output-disable request from the GTETRGn pin has occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidf::_0
    }
    #[doc = "Output-disable request from the GTETRGn pin occurred."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidf::_1
    }
}
#[doc = "Field `PIDF` writer - Port Input Detection Flag"]
pub type PidfW<'a, REG> = crate::BitWriter0C<'a, REG, Pidf>;
impl<'a, REG> PidfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No output-disable request from the GTETRGn pin has occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pidf::_0)
    }
    #[doc = "Output-disable request from the GTETRGn pin occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pidf::_1)
    }
}
#[doc = "Output-disable Request Detection Flag from GPT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iocf {
    #[doc = "0: No output-disable request from the GPT disable request has occurred"]
    _0 = 0,
    #[doc = "1: Output-disable request from the GPT disable request occurred."]
    _1 = 1,
}
impl From<Iocf> for bool {
    #[inline(always)]
    fn from(variant: Iocf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOCF` reader - Output-disable Request Detection Flag from GPT\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type IocfR = crate::BitReader<Iocf>;
impl IocfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iocf {
        match self.bits {
            false => Iocf::_0,
            true => Iocf::_1,
        }
    }
    #[doc = "No output-disable request from the GPT disable request has occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iocf::_0
    }
    #[doc = "Output-disable request from the GPT disable request occurred."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iocf::_1
    }
}
#[doc = "Field `IOCF` writer - Output-disable Request Detection Flag from GPT"]
pub type IocfW<'a, REG> = crate::BitWriter0C<'a, REG, Iocf>;
impl<'a, REG> IocfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No output-disable request from the GPT disable request has occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iocf::_0)
    }
    #[doc = "Output-disable request from the GPT disable request occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iocf::_1)
    }
}
#[doc = "Oscillation Stop Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ostpf {
    #[doc = "0: No output-disable request from oscillation stop detection has occurred"]
    _0 = 0,
    #[doc = "1: Output-disable request from oscillation stop detection occurred."]
    _1 = 1,
}
impl From<Ostpf> for bool {
    #[inline(always)]
    fn from(variant: Ostpf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSTPF` reader - Oscillation Stop Detection Flag\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type OstpfR = crate::BitReader<Ostpf>;
impl OstpfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ostpf {
        match self.bits {
            false => Ostpf::_0,
            true => Ostpf::_1,
        }
    }
    #[doc = "No output-disable request from oscillation stop detection has occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ostpf::_0
    }
    #[doc = "Output-disable request from oscillation stop detection occurred."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ostpf::_1
    }
}
#[doc = "Field `OSTPF` writer - Oscillation Stop Detection Flag"]
pub type OstpfW<'a, REG> = crate::BitWriter0C<'a, REG, Ostpf>;
impl<'a, REG> OstpfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No output-disable request from oscillation stop detection has occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ostpf::_0)
    }
    #[doc = "Output-disable request from oscillation stop detection occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ostpf::_1)
    }
}
#[doc = "Software Stop Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssf {
    #[doc = "0: No output-disable request from software has occurred"]
    _0 = 0,
    #[doc = "1: Output-disable request from software occurred."]
    _1 = 1,
}
impl From<Ssf> for bool {
    #[inline(always)]
    fn from(variant: Ssf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSF` reader - Software Stop Flag"]
pub type SsfR = crate::BitReader<Ssf>;
impl SsfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssf {
        match self.bits {
            false => Ssf::_0,
            true => Ssf::_1,
        }
    }
    #[doc = "No output-disable request from software has occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ssf::_0
    }
    #[doc = "Output-disable request from software occurred."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ssf::_1
    }
}
#[doc = "Field `SSF` writer - Software Stop Flag"]
pub type SsfW<'a, REG> = crate::BitWriter<'a, REG, Ssf>;
impl<'a, REG> SsfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No output-disable request from software has occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssf::_0)
    }
    #[doc = "Output-disable request from software occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssf::_1)
    }
}
#[doc = "Port Input Detection Enable Note: Can be modified only once after a reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pide {
    #[doc = "0: Output-disable request from the GTETRG pins disabled"]
    _0 = 0,
    #[doc = "1: Output-disable request from the GTETRG pins enabled."]
    _1 = 1,
}
impl From<Pide> for bool {
    #[inline(always)]
    fn from(variant: Pide) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIDE` reader - Port Input Detection Enable Note: Can be modified only once after a reset."]
pub type PideR = crate::BitReader<Pide>;
impl PideR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pide {
        match self.bits {
            false => Pide::_0,
            true => Pide::_1,
        }
    }
    #[doc = "Output-disable request from the GTETRG pins disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pide::_0
    }
    #[doc = "Output-disable request from the GTETRG pins enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pide::_1
    }
}
#[doc = "Field `PIDE` writer - Port Input Detection Enable Note: Can be modified only once after a reset."]
pub type PideW<'a, REG> = crate::BitWriter<'a, REG, Pide>;
impl<'a, REG> PideW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output-disable request from the GTETRG pins disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pide::_0)
    }
    #[doc = "Output-disable request from the GTETRG pins enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pide::_1)
    }
}
#[doc = "Output-disable Request Enable from GPT Note: Can be modified only once after a reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ioce {
    #[doc = "0: Output-disable request from the GPT disable request disabled"]
    _0 = 0,
    #[doc = "1: Output-disable request from the GPT disable request enabled."]
    _1 = 1,
}
impl From<Ioce> for bool {
    #[inline(always)]
    fn from(variant: Ioce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOCE` reader - Output-disable Request Enable from GPT Note: Can be modified only once after a reset."]
pub type IoceR = crate::BitReader<Ioce>;
impl IoceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ioce {
        match self.bits {
            false => Ioce::_0,
            true => Ioce::_1,
        }
    }
    #[doc = "Output-disable request from the GPT disable request disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ioce::_0
    }
    #[doc = "Output-disable request from the GPT disable request enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ioce::_1
    }
}
#[doc = "Field `IOCE` writer - Output-disable Request Enable from GPT Note: Can be modified only once after a reset."]
pub type IoceW<'a, REG> = crate::BitWriter<'a, REG, Ioce>;
impl<'a, REG> IoceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output-disable request from the GPT disable request disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ioce::_0)
    }
    #[doc = "Output-disable request from the GPT disable request enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ioce::_1)
    }
}
#[doc = "Oscillation Stop Detection Enable Note: Can be modified only once after a reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ostpe {
    #[doc = "0: A output-disable request from the oscillation stop detection disabled."]
    _0 = 0,
    #[doc = "1: A output-disable request from the oscillation stop detection enabled."]
    _1 = 1,
}
impl From<Ostpe> for bool {
    #[inline(always)]
    fn from(variant: Ostpe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSTPE` reader - Oscillation Stop Detection Enable Note: Can be modified only once after a reset."]
pub type OstpeR = crate::BitReader<Ostpe>;
impl OstpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ostpe {
        match self.bits {
            false => Ostpe::_0,
            true => Ostpe::_1,
        }
    }
    #[doc = "A output-disable request from the oscillation stop detection disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ostpe::_0
    }
    #[doc = "A output-disable request from the oscillation stop detection enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ostpe::_1
    }
}
#[doc = "Field `OSTPE` writer - Oscillation Stop Detection Enable Note: Can be modified only once after a reset."]
pub type OstpeW<'a, REG> = crate::BitWriter<'a, REG, Ostpe>;
impl<'a, REG> OstpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A output-disable request from the oscillation stop detection disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ostpe::_0)
    }
    #[doc = "A output-disable request from the oscillation stop detection enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ostpe::_1)
    }
}
#[doc = "GTETRG Input Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum St {
    #[doc = "0: GTETRG input after filtering is 0."]
    _0 = 0,
    #[doc = "1: GTETRG input after filtering is 1."]
    _1 = 1,
}
impl From<St> for bool {
    #[inline(always)]
    fn from(variant: St) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ST` reader - GTETRG Input Status Flag"]
pub type StR = crate::BitReader<St>;
impl StR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> St {
        match self.bits {
            false => St::_0,
            true => St::_1,
        }
    }
    #[doc = "GTETRG input after filtering is 0."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == St::_0
    }
    #[doc = "GTETRG input after filtering is 1."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == St::_1
    }
}
#[doc = "GTETRG Input Reverse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inv {
    #[doc = "0: GTETRG Input"]
    _0 = 0,
    #[doc = "1: GTETRG Input Reversed."]
    _1 = 1,
}
impl From<Inv> for bool {
    #[inline(always)]
    fn from(variant: Inv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INV` reader - GTETRG Input Reverse"]
pub type InvR = crate::BitReader<Inv>;
impl InvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inv {
        match self.bits {
            false => Inv::_0,
            true => Inv::_1,
        }
    }
    #[doc = "GTETRG Input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Inv::_0
    }
    #[doc = "GTETRG Input Reversed."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Inv::_1
    }
}
#[doc = "Field `INV` writer - GTETRG Input Reverse"]
pub type InvW<'a, REG> = crate::BitWriter<'a, REG, Inv>;
impl<'a, REG> InvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GTETRG Input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Inv::_0)
    }
    #[doc = "GTETRG Input Reversed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Inv::_1)
    }
}
#[doc = "Noise Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nfen {
    #[doc = "0: Filtering noise disabled"]
    _0 = 0,
    #[doc = "1: Filtering noise enabled"]
    _1 = 1,
}
impl From<Nfen> for bool {
    #[inline(always)]
    fn from(variant: Nfen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NFEN` reader - Noise Filter Enable"]
pub type NfenR = crate::BitReader<Nfen>;
impl NfenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nfen {
        match self.bits {
            false => Nfen::_0,
            true => Nfen::_1,
        }
    }
    #[doc = "Filtering noise disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nfen::_0
    }
    #[doc = "Filtering noise enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nfen::_1
    }
}
#[doc = "Field `NFEN` writer - Noise Filter Enable"]
pub type NfenW<'a, REG> = crate::BitWriter<'a, REG, Nfen>;
impl<'a, REG> NfenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Filtering noise disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nfen::_0)
    }
    #[doc = "Filtering noise enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nfen::_1)
    }
}
#[doc = "Noise Filter Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nfcs {
    #[doc = "0: Sampling GTETRG pin input level for three times in every PCLKB."]
    _00 = 0,
    #[doc = "1: Sampling GTETRG pin input level for three times in every PCLKB /8."]
    _01 = 1,
    #[doc = "2: Sampling GTETRG pin input level for three times in every PCLKB /32."]
    _10 = 2,
    #[doc = "3: Sampling GTETRG pin input level for three times in every PCLKB /128."]
    _11 = 3,
}
impl From<Nfcs> for u8 {
    #[inline(always)]
    fn from(variant: Nfcs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nfcs {
    type Ux = u8;
}
impl crate::IsEnum for Nfcs {}
#[doc = "Field `NFCS` reader - Noise Filter Clock Select"]
pub type NfcsR = crate::FieldReader<Nfcs>;
impl NfcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nfcs {
        match self.bits {
            0 => Nfcs::_00,
            1 => Nfcs::_01,
            2 => Nfcs::_10,
            3 => Nfcs::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Sampling GTETRG pin input level for three times in every PCLKB."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Nfcs::_00
    }
    #[doc = "Sampling GTETRG pin input level for three times in every PCLKB /8."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Nfcs::_01
    }
    #[doc = "Sampling GTETRG pin input level for three times in every PCLKB /32."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Nfcs::_10
    }
    #[doc = "Sampling GTETRG pin input level for three times in every PCLKB /128."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Nfcs::_11
    }
}
#[doc = "Field `NFCS` writer - Noise Filter Clock Select"]
pub type NfcsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Nfcs, crate::Safe>;
impl<'a, REG> NfcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sampling GTETRG pin input level for three times in every PCLKB."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcs::_00)
    }
    #[doc = "Sampling GTETRG pin input level for three times in every PCLKB /8."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcs::_01)
    }
    #[doc = "Sampling GTETRG pin input level for three times in every PCLKB /32."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcs::_10)
    }
    #[doc = "Sampling GTETRG pin input level for three times in every PCLKB /128."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcs::_11)
    }
}
impl R {
    #[doc = "Bit 0 - Port Input Detection Flag"]
    #[inline(always)]
    pub fn pidf(&self) -> PidfR {
        PidfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output-disable Request Detection Flag from GPT"]
    #[inline(always)]
    pub fn iocf(&self) -> IocfR {
        IocfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Oscillation Stop Detection Flag"]
    #[inline(always)]
    pub fn ostpf(&self) -> OstpfR {
        OstpfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software Stop Flag"]
    #[inline(always)]
    pub fn ssf(&self) -> SsfR {
        SsfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port Input Detection Enable Note: Can be modified only once after a reset."]
    #[inline(always)]
    pub fn pide(&self) -> PideR {
        PideR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Output-disable Request Enable from GPT Note: Can be modified only once after a reset."]
    #[inline(always)]
    pub fn ioce(&self) -> IoceR {
        IoceR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Oscillation Stop Detection Enable Note: Can be modified only once after a reset."]
    #[inline(always)]
    pub fn ostpe(&self) -> OstpeR {
        OstpeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - GTETRG Input Status Flag"]
    #[inline(always)]
    pub fn st(&self) -> StR {
        StR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 28 - GTETRG Input Reverse"]
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Noise Filter Enable"]
    #[inline(always)]
    pub fn nfen(&self) -> NfenR {
        NfenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Noise Filter Clock Select"]
    #[inline(always)]
    pub fn nfcs(&self) -> NfcsR {
        NfcsR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POEGG")
            .field("nfcs", &self.nfcs())
            .field("nfen", &self.nfen())
            .field("inv", &self.inv())
            .field("st", &self.st())
            .field("ostpe", &self.ostpe())
            .field("ioce", &self.ioce())
            .field("pide", &self.pide())
            .field("ssf", &self.ssf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Port Input Detection Flag"]
    #[inline(always)]
    pub fn pidf(&mut self) -> PidfW<PoeggSpec> {
        PidfW::new(self, 0)
    }
    #[doc = "Bit 1 - Output-disable Request Detection Flag from GPT"]
    #[inline(always)]
    pub fn iocf(&mut self) -> IocfW<PoeggSpec> {
        IocfW::new(self, 1)
    }
    #[doc = "Bit 2 - Oscillation Stop Detection Flag"]
    #[inline(always)]
    pub fn ostpf(&mut self) -> OstpfW<PoeggSpec> {
        OstpfW::new(self, 2)
    }
    #[doc = "Bit 3 - Software Stop Flag"]
    #[inline(always)]
    pub fn ssf(&mut self) -> SsfW<PoeggSpec> {
        SsfW::new(self, 3)
    }
    #[doc = "Bit 4 - Port Input Detection Enable Note: Can be modified only once after a reset."]
    #[inline(always)]
    pub fn pide(&mut self) -> PideW<PoeggSpec> {
        PideW::new(self, 4)
    }
    #[doc = "Bit 5 - Output-disable Request Enable from GPT Note: Can be modified only once after a reset."]
    #[inline(always)]
    pub fn ioce(&mut self) -> IoceW<PoeggSpec> {
        IoceW::new(self, 5)
    }
    #[doc = "Bit 6 - Oscillation Stop Detection Enable Note: Can be modified only once after a reset."]
    #[inline(always)]
    pub fn ostpe(&mut self) -> OstpeW<PoeggSpec> {
        OstpeW::new(self, 6)
    }
    #[doc = "Bit 28 - GTETRG Input Reverse"]
    #[inline(always)]
    pub fn inv(&mut self) -> InvW<PoeggSpec> {
        InvW::new(self, 28)
    }
    #[doc = "Bit 29 - Noise Filter Enable"]
    #[inline(always)]
    pub fn nfen(&mut self) -> NfenW<PoeggSpec> {
        NfenW::new(self, 29)
    }
    #[doc = "Bits 30:31 - Noise Filter Clock Select"]
    #[inline(always)]
    pub fn nfcs(&mut self) -> NfcsW<PoeggSpec> {
        NfcsW::new(self, 30)
    }
}
#[doc = "POEG Group %s Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`poegg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poegg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PoeggSpec;
impl crate::RegisterSpec for PoeggSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`poegg::R`](R) reader structure"]
impl crate::Readable for PoeggSpec {}
#[doc = "`write(|w| ..)` method takes [`poegg::W`](W) writer structure"]
impl crate::Writable for PoeggSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x07;
}
#[doc = "`reset()` method sets POEGG%s to value 0"]
impl crate::Resettable for PoeggSpec {}
