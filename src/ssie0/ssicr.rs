#[doc = "Register `SSICR` reader"]
pub type R = crate::R<SsicrSpec>;
#[doc = "Register `SSICR` writer"]
pub type W = crate::W<SsicrSpec>;
#[doc = "Receive Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ren {
    #[doc = "0: Disables the receive operation."]
    _0 = 0,
    #[doc = "1: Enables the receive operation."]
    _1 = 1,
}
impl From<Ren> for bool {
    #[inline(always)]
    fn from(variant: Ren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REN` reader - Receive Enable"]
pub type RenR = crate::BitReader<Ren>;
impl RenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ren {
        match self.bits {
            false => Ren::_0,
            true => Ren::_1,
        }
    }
    #[doc = "Disables the receive operation."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ren::_0
    }
    #[doc = "Enables the receive operation."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ren::_1
    }
}
#[doc = "Field `REN` writer - Receive Enable"]
pub type RenW<'a, REG> = crate::BitWriter<'a, REG, Ren>;
impl<'a, REG> RenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the receive operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ren::_0)
    }
    #[doc = "Enables the receive operation."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ren::_1)
    }
}
#[doc = "Transmit Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ten {
    #[doc = "0: Disables the transmit operation."]
    _0 = 0,
    #[doc = "1: Enables the transmit operation."]
    _1 = 1,
}
impl From<Ten> for bool {
    #[inline(always)]
    fn from(variant: Ten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEN` reader - Transmit Enable"]
pub type TenR = crate::BitReader<Ten>;
impl TenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ten {
        match self.bits {
            false => Ten::_0,
            true => Ten::_1,
        }
    }
    #[doc = "Disables the transmit operation."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ten::_0
    }
    #[doc = "Enables the transmit operation."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ten::_1
    }
}
#[doc = "Field `TEN` writer - Transmit Enable"]
pub type TenW<'a, REG> = crate::BitWriter<'a, REG, Ten>;
impl<'a, REG> TenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the transmit operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ten::_0)
    }
    #[doc = "Enables the transmit operation."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ten::_1)
    }
}
#[doc = "Mute Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Muen {
    #[doc = "0: Disables muting on the next frame boundary"]
    _0 = 0,
    #[doc = "1: Enables muting on the next frame boundary."]
    _1 = 1,
}
impl From<Muen> for bool {
    #[inline(always)]
    fn from(variant: Muen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUEN` reader - Mute Enable"]
pub type MuenR = crate::BitReader<Muen>;
impl MuenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Muen {
        match self.bits {
            false => Muen::_0,
            true => Muen::_1,
        }
    }
    #[doc = "Disables muting on the next frame boundary"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Muen::_0
    }
    #[doc = "Enables muting on the next frame boundary."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Muen::_1
    }
}
#[doc = "Field `MUEN` writer - Mute Enable"]
pub type MuenW<'a, REG> = crate::BitWriter<'a, REG, Muen>;
impl<'a, REG> MuenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables muting on the next frame boundary"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Muen::_0)
    }
    #[doc = "Enables muting on the next frame boundary."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Muen::_1)
    }
}
#[doc = "Selects Bit Clock Division Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckdv {
    #[doc = "0: AUDIO_MCK"]
    _0000 = 0,
    #[doc = "1: AUDIO_MCK/2"]
    _0001 = 1,
    #[doc = "2: AUDIO_MCK/4"]
    _0010 = 2,
    #[doc = "3: AUDIO_MCK/8"]
    _0011 = 3,
    #[doc = "4: AUDIO_MCK/16"]
    _0100 = 4,
    #[doc = "5: AUDIO_MCK/32"]
    _0101 = 5,
    #[doc = "6: AUDIO_MCK/64"]
    _0110 = 6,
    #[doc = "7: AUDIO_MCK/128"]
    _0111 = 7,
    #[doc = "8: AUDIO_MCK/6"]
    _1000 = 8,
    #[doc = "9: AUDIO_MCK/12"]
    _1001 = 9,
    #[doc = "10: AUDIO_MCK/24"]
    _1010 = 10,
    #[doc = "11: AUDIO_MCK/48"]
    _1011 = 11,
    #[doc = "12: AUDIO_MCK/96"]
    _1100 = 12,
    #[doc = "13: Setting prohibited"]
    Others = 13,
}
impl From<Ckdv> for u8 {
    #[inline(always)]
    fn from(variant: Ckdv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ckdv {
    type Ux = u8;
}
impl crate::IsEnum for Ckdv {}
#[doc = "Field `CKDV` reader - Selects Bit Clock Division Ratio"]
pub type CkdvR = crate::FieldReader<Ckdv>;
impl CkdvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckdv {
        match self.bits {
            0 => Ckdv::_0000,
            1 => Ckdv::_0001,
            2 => Ckdv::_0010,
            3 => Ckdv::_0011,
            4 => Ckdv::_0100,
            5 => Ckdv::_0101,
            6 => Ckdv::_0110,
            7 => Ckdv::_0111,
            8 => Ckdv::_1000,
            9 => Ckdv::_1001,
            10 => Ckdv::_1010,
            11 => Ckdv::_1011,
            12 => Ckdv::_1100,
            _ => Ckdv::Others,
        }
    }
    #[doc = "AUDIO_MCK"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == Ckdv::_0000
    }
    #[doc = "AUDIO_MCK/2"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == Ckdv::_0001
    }
    #[doc = "AUDIO_MCK/4"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == Ckdv::_0010
    }
    #[doc = "AUDIO_MCK/8"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == Ckdv::_0011
    }
    #[doc = "AUDIO_MCK/16"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == Ckdv::_0100
    }
    #[doc = "AUDIO_MCK/32"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == Ckdv::_0101
    }
    #[doc = "AUDIO_MCK/64"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == Ckdv::_0110
    }
    #[doc = "AUDIO_MCK/128"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == Ckdv::_0111
    }
    #[doc = "AUDIO_MCK/6"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == Ckdv::_1000
    }
    #[doc = "AUDIO_MCK/12"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == Ckdv::_1001
    }
    #[doc = "AUDIO_MCK/24"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == Ckdv::_1010
    }
    #[doc = "AUDIO_MCK/48"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == Ckdv::_1011
    }
    #[doc = "AUDIO_MCK/96"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == Ckdv::_1100
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Ckdv::Others)
    }
}
#[doc = "Field `CKDV` writer - Selects Bit Clock Division Ratio"]
pub type CkdvW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ckdv, crate::Safe>;
impl<'a, REG> CkdvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AUDIO_MCK"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::_0000)
    }
    #[doc = "AUDIO_MCK/2"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::_0001)
    }
    #[doc = "AUDIO_MCK/4"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::_0010)
    }
    #[doc = "AUDIO_MCK/8"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::_0011)
    }
    #[doc = "AUDIO_MCK/16"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::_0100)
    }
    #[doc = "AUDIO_MCK/32"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::_0101)
    }
    #[doc = "AUDIO_MCK/64"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::_0110)
    }
    #[doc = "AUDIO_MCK/128"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::_0111)
    }
    #[doc = "AUDIO_MCK/6"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::_1000)
    }
    #[doc = "AUDIO_MCK/12"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::_1001)
    }
    #[doc = "AUDIO_MCK/24"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::_1010)
    }
    #[doc = "AUDIO_MCK/48"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::_1011)
    }
    #[doc = "AUDIO_MCK/96"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::_1100)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdv::Others)
    }
}
#[doc = "Selects Serial Data Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Del {
    #[doc = "0: Delay of one cycle of SSIBCK between SSILRCK/SSIFS and SSITXD0/SSIRXD0"]
    _0 = 0,
    #[doc = "1: No delay between SSILRCK/SSIFS and SSITXD0/SSIRXD0 In the monaural format, this bit controls the waveform of SSILRCK/SSIFS."]
    _1 = 1,
}
impl From<Del> for bool {
    #[inline(always)]
    fn from(variant: Del) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEL` reader - Selects Serial Data Delay"]
pub type DelR = crate::BitReader<Del>;
impl DelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Del {
        match self.bits {
            false => Del::_0,
            true => Del::_1,
        }
    }
    #[doc = "Delay of one cycle of SSIBCK between SSILRCK/SSIFS and SSITXD0/SSIRXD0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Del::_0
    }
    #[doc = "No delay between SSILRCK/SSIFS and SSITXD0/SSIRXD0 In the monaural format, this bit controls the waveform of SSILRCK/SSIFS."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Del::_1
    }
}
#[doc = "Field `DEL` writer - Selects Serial Data Delay"]
pub type DelW<'a, REG> = crate::BitWriter<'a, REG, Del>;
impl<'a, REG> DelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Delay of one cycle of SSIBCK between SSILRCK/SSIFS and SSITXD0/SSIRXD0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Del::_0)
    }
    #[doc = "No delay between SSILRCK/SSIFS and SSITXD0/SSIRXD0 In the monaural format, this bit controls the waveform of SSILRCK/SSIFS."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Del::_1)
    }
}
#[doc = "Selects Placement Data Alignment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdta {
    #[doc = "0: Left-justifies placement data (SSIFTDR, SSIFRDR)"]
    _0 = 0,
    #[doc = "1: Right-justifies placement data (SSIFTDR, SSIFRDR)."]
    _1 = 1,
}
impl From<Pdta> for bool {
    #[inline(always)]
    fn from(variant: Pdta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDTA` reader - Selects Placement Data Alignment"]
pub type PdtaR = crate::BitReader<Pdta>;
impl PdtaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdta {
        match self.bits {
            false => Pdta::_0,
            true => Pdta::_1,
        }
    }
    #[doc = "Left-justifies placement data (SSIFTDR, SSIFRDR)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdta::_0
    }
    #[doc = "Right-justifies placement data (SSIFTDR, SSIFRDR)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdta::_1
    }
}
#[doc = "Field `PDTA` writer - Selects Placement Data Alignment"]
pub type PdtaW<'a, REG> = crate::BitWriter<'a, REG, Pdta>;
impl<'a, REG> PdtaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Left-justifies placement data (SSIFTDR, SSIFRDR)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdta::_0)
    }
    #[doc = "Right-justifies placement data (SSIFTDR, SSIFRDR)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdta::_1)
    }
}
#[doc = "Selects Serial Data Alignment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdta {
    #[doc = "0: Transmits and receives serial data first and then padding bits"]
    _0 = 0,
    #[doc = "1: Transmit and receives padding bits first and then serial data."]
    _1 = 1,
}
impl From<Sdta> for bool {
    #[inline(always)]
    fn from(variant: Sdta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDTA` reader - Selects Serial Data Alignment"]
pub type SdtaR = crate::BitReader<Sdta>;
impl SdtaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdta {
        match self.bits {
            false => Sdta::_0,
            true => Sdta::_1,
        }
    }
    #[doc = "Transmits and receives serial data first and then padding bits"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sdta::_0
    }
    #[doc = "Transmit and receives padding bits first and then serial data."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sdta::_1
    }
}
#[doc = "Field `SDTA` writer - Selects Serial Data Alignment"]
pub type SdtaW<'a, REG> = crate::BitWriter<'a, REG, Sdta>;
impl<'a, REG> SdtaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmits and receives serial data first and then padding bits"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sdta::_0)
    }
    #[doc = "Transmit and receives padding bits first and then serial data."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdta::_1)
    }
}
#[doc = "Selects Serial Padding Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spdp {
    #[doc = "0: Padding data is at a low level"]
    _0 = 0,
    #[doc = "1: Padding data is at a high level."]
    _1 = 1,
}
impl From<Spdp> for bool {
    #[inline(always)]
    fn from(variant: Spdp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPDP` reader - Selects Serial Padding Polarity"]
pub type SpdpR = crate::BitReader<Spdp>;
impl SpdpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spdp {
        match self.bits {
            false => Spdp::_0,
            true => Spdp::_1,
        }
    }
    #[doc = "Padding data is at a low level"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Spdp::_0
    }
    #[doc = "Padding data is at a high level."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Spdp::_1
    }
}
#[doc = "Field `SPDP` writer - Selects Serial Padding Polarity"]
pub type SpdpW<'a, REG> = crate::BitWriter<'a, REG, Spdp>;
impl<'a, REG> SpdpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Padding data is at a low level"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Spdp::_0)
    }
    #[doc = "Padding data is at a high level."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Spdp::_1)
    }
}
#[doc = "Selects the Initial Value and Polarity of LR Clock/Frame Synchronization Signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lrckp {
    #[doc = "0: The initial value is at a high level The start trigger for a frame is synchronized with a falling edge of SSILRCK/SSIFS"]
    _0 = 0,
    #[doc = "1: The initial value is at a low level The start trigger for a frame is synchronized with a rising edge of SSILRCK/SSIFS."]
    _1 = 1,
}
impl From<Lrckp> for bool {
    #[inline(always)]
    fn from(variant: Lrckp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LRCKP` reader - Selects the Initial Value and Polarity of LR Clock/Frame Synchronization Signal"]
pub type LrckpR = crate::BitReader<Lrckp>;
impl LrckpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lrckp {
        match self.bits {
            false => Lrckp::_0,
            true => Lrckp::_1,
        }
    }
    #[doc = "The initial value is at a high level The start trigger for a frame is synchronized with a falling edge of SSILRCK/SSIFS"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lrckp::_0
    }
    #[doc = "The initial value is at a low level The start trigger for a frame is synchronized with a rising edge of SSILRCK/SSIFS."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lrckp::_1
    }
}
#[doc = "Field `LRCKP` writer - Selects the Initial Value and Polarity of LR Clock/Frame Synchronization Signal"]
pub type LrckpW<'a, REG> = crate::BitWriter<'a, REG, Lrckp>;
impl<'a, REG> LrckpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The initial value is at a high level The start trigger for a frame is synchronized with a falling edge of SSILRCK/SSIFS"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lrckp::_0)
    }
    #[doc = "The initial value is at a low level The start trigger for a frame is synchronized with a rising edge of SSILRCK/SSIFS."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lrckp::_1)
    }
}
#[doc = "Selects Bit Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bckp {
    #[doc = "0: SSILRCK/SSIFS and SSITXD0/SSIRXD0 change at a falling edge (SSILRCK/SSIFS and SSIRXD0 are sampled at a rising edge of SSIBCK)"]
    _0 = 0,
    #[doc = "1: SSILRCK/SSIFS and SSITXD0/SSIRXD0 change at a rising edge (SSILRCK/SSIFS and SSIRXD0 are sampled at a falling edge of SSIBCK)."]
    _1 = 1,
}
impl From<Bckp> for bool {
    #[inline(always)]
    fn from(variant: Bckp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCKP` reader - Selects Bit Clock Polarity"]
pub type BckpR = crate::BitReader<Bckp>;
impl BckpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bckp {
        match self.bits {
            false => Bckp::_0,
            true => Bckp::_1,
        }
    }
    #[doc = "SSILRCK/SSIFS and SSITXD0/SSIRXD0 change at a falling edge (SSILRCK/SSIFS and SSIRXD0 are sampled at a rising edge of SSIBCK)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bckp::_0
    }
    #[doc = "SSILRCK/SSIFS and SSITXD0/SSIRXD0 change at a rising edge (SSILRCK/SSIFS and SSIRXD0 are sampled at a falling edge of SSIBCK)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bckp::_1
    }
}
#[doc = "Field `BCKP` writer - Selects Bit Clock Polarity"]
pub type BckpW<'a, REG> = crate::BitWriter<'a, REG, Bckp>;
impl<'a, REG> BckpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SSILRCK/SSIFS and SSITXD0/SSIRXD0 change at a falling edge (SSILRCK/SSIFS and SSIRXD0 are sampled at a rising edge of SSIBCK)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bckp::_0)
    }
    #[doc = "SSILRCK/SSIFS and SSITXD0/SSIRXD0 change at a rising edge (SSILRCK/SSIFS and SSIRXD0 are sampled at a falling edge of SSIBCK)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bckp::_1)
    }
}
#[doc = "Master Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mst {
    #[doc = "0: Slave-mode communication"]
    _0 = 0,
    #[doc = "1: Master-mode communication."]
    _1 = 1,
}
impl From<Mst> for bool {
    #[inline(always)]
    fn from(variant: Mst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MST` reader - Master Enable"]
pub type MstR = crate::BitReader<Mst>;
impl MstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mst {
        match self.bits {
            false => Mst::_0,
            true => Mst::_1,
        }
    }
    #[doc = "Slave-mode communication"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mst::_0
    }
    #[doc = "Master-mode communication."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mst::_1
    }
}
#[doc = "Field `MST` writer - Master Enable"]
pub type MstW<'a, REG> = crate::BitWriter<'a, REG, Mst>;
impl<'a, REG> MstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave-mode communication"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mst::_0)
    }
    #[doc = "Master-mode communication."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mst::_1)
    }
}
#[doc = "Selects System Word Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Swl {
    #[doc = "0: 8 bits"]
    _000 = 0,
    #[doc = "1: 16 bits"]
    _001 = 1,
    #[doc = "2: 24 bits"]
    _010 = 2,
    #[doc = "3: 32 bits"]
    _011 = 3,
    #[doc = "4: 48 bits"]
    _100 = 4,
    #[doc = "5: 64 bits"]
    _101 = 5,
    #[doc = "6: 128 bits"]
    _110 = 6,
    #[doc = "7: 256 bits."]
    _111 = 7,
}
impl From<Swl> for u8 {
    #[inline(always)]
    fn from(variant: Swl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Swl {
    type Ux = u8;
}
impl crate::IsEnum for Swl {}
#[doc = "Field `SWL` reader - Selects System Word Length"]
pub type SwlR = crate::FieldReader<Swl>;
impl SwlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swl {
        match self.bits {
            0 => Swl::_000,
            1 => Swl::_001,
            2 => Swl::_010,
            3 => Swl::_011,
            4 => Swl::_100,
            5 => Swl::_101,
            6 => Swl::_110,
            7 => Swl::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Swl::_000
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Swl::_001
    }
    #[doc = "24 bits"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Swl::_010
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Swl::_011
    }
    #[doc = "48 bits"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Swl::_100
    }
    #[doc = "64 bits"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Swl::_101
    }
    #[doc = "128 bits"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Swl::_110
    }
    #[doc = "256 bits."]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Swl::_111
    }
}
#[doc = "Field `SWL` writer - Selects System Word Length"]
pub type SwlW<'a, REG> = crate::FieldWriter<'a, REG, 3, Swl, crate::Safe>;
impl<'a, REG> SwlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Swl::_000)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Swl::_001)
    }
    #[doc = "24 bits"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Swl::_010)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Swl::_011)
    }
    #[doc = "48 bits"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Swl::_100)
    }
    #[doc = "64 bits"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Swl::_101)
    }
    #[doc = "128 bits"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Swl::_110)
    }
    #[doc = "256 bits."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Swl::_111)
    }
}
#[doc = "Selects Data Word Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dwl {
    #[doc = "0: 8 bits"]
    _000 = 0,
    #[doc = "1: 16 bits"]
    _001 = 1,
    #[doc = "2: 18 bits"]
    _010 = 2,
    #[doc = "3: 20 bits"]
    _011 = 3,
    #[doc = "4: 22 bits"]
    _100 = 4,
    #[doc = "5: 24 bits"]
    _101 = 5,
    #[doc = "6: 32 bits"]
    _110 = 6,
    #[doc = "7: Settings other than above are prohibited."]
    Others = 7,
}
impl From<Dwl> for u8 {
    #[inline(always)]
    fn from(variant: Dwl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dwl {
    type Ux = u8;
}
impl crate::IsEnum for Dwl {}
#[doc = "Field `DWL` reader - Selects Data Word Length"]
pub type DwlR = crate::FieldReader<Dwl>;
impl DwlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dwl {
        match self.bits {
            0 => Dwl::_000,
            1 => Dwl::_001,
            2 => Dwl::_010,
            3 => Dwl::_011,
            4 => Dwl::_100,
            5 => Dwl::_101,
            6 => Dwl::_110,
            7 => Dwl::Others,
            _ => unreachable!(),
        }
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Dwl::_000
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Dwl::_001
    }
    #[doc = "18 bits"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Dwl::_010
    }
    #[doc = "20 bits"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Dwl::_011
    }
    #[doc = "22 bits"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Dwl::_100
    }
    #[doc = "24 bits"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Dwl::_101
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Dwl::_110
    }
    #[doc = "Settings other than above are prohibited."]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == Dwl::Others
    }
}
#[doc = "Field `DWL` writer - Selects Data Word Length"]
pub type DwlW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dwl, crate::Safe>;
impl<'a, REG> DwlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Dwl::_000)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Dwl::_001)
    }
    #[doc = "18 bits"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Dwl::_010)
    }
    #[doc = "20 bits"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Dwl::_011)
    }
    #[doc = "22 bits"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Dwl::_100)
    }
    #[doc = "24 bits"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Dwl::_101)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Dwl::_110)
    }
    #[doc = "Settings other than above are prohibited."]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Dwl::Others)
    }
}
#[doc = "Idle Mode Interrupt Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iien {
    #[doc = "0: Disables idle mode interrupt output"]
    _0 = 0,
    #[doc = "1: Enables idle mode interrupt output."]
    _1 = 1,
}
impl From<Iien> for bool {
    #[inline(always)]
    fn from(variant: Iien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IIEN` reader - Idle Mode Interrupt Output Enable"]
pub type IienR = crate::BitReader<Iien>;
impl IienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iien {
        match self.bits {
            false => Iien::_0,
            true => Iien::_1,
        }
    }
    #[doc = "Disables idle mode interrupt output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iien::_0
    }
    #[doc = "Enables idle mode interrupt output."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iien::_1
    }
}
#[doc = "Field `IIEN` writer - Idle Mode Interrupt Output Enable"]
pub type IienW<'a, REG> = crate::BitWriter<'a, REG, Iien>;
impl<'a, REG> IienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables idle mode interrupt output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iien::_0)
    }
    #[doc = "Enables idle mode interrupt output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iien::_1)
    }
}
#[doc = "Receive Overflow Interrupt Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Roien {
    #[doc = "0: Disables receive overflow interrupt output"]
    _0 = 0,
    #[doc = "1: Enables receive overflow interrupt output."]
    _1 = 1,
}
impl From<Roien> for bool {
    #[inline(always)]
    fn from(variant: Roien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROIEN` reader - Receive Overflow Interrupt Output Enable"]
pub type RoienR = crate::BitReader<Roien>;
impl RoienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Roien {
        match self.bits {
            false => Roien::_0,
            true => Roien::_1,
        }
    }
    #[doc = "Disables receive overflow interrupt output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Roien::_0
    }
    #[doc = "Enables receive overflow interrupt output."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Roien::_1
    }
}
#[doc = "Field `ROIEN` writer - Receive Overflow Interrupt Output Enable"]
pub type RoienW<'a, REG> = crate::BitWriter<'a, REG, Roien>;
impl<'a, REG> RoienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables receive overflow interrupt output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Roien::_0)
    }
    #[doc = "Enables receive overflow interrupt output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Roien::_1)
    }
}
#[doc = "Receive Underflow Interrupt Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ruien {
    #[doc = "0: Disables receive underflow interrupt output"]
    _0 = 0,
    #[doc = "1: Enables receive underflow interrupt output."]
    _1 = 1,
}
impl From<Ruien> for bool {
    #[inline(always)]
    fn from(variant: Ruien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUIEN` reader - Receive Underflow Interrupt Output Enable"]
pub type RuienR = crate::BitReader<Ruien>;
impl RuienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ruien {
        match self.bits {
            false => Ruien::_0,
            true => Ruien::_1,
        }
    }
    #[doc = "Disables receive underflow interrupt output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ruien::_0
    }
    #[doc = "Enables receive underflow interrupt output."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ruien::_1
    }
}
#[doc = "Field `RUIEN` writer - Receive Underflow Interrupt Output Enable"]
pub type RuienW<'a, REG> = crate::BitWriter<'a, REG, Ruien>;
impl<'a, REG> RuienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables receive underflow interrupt output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ruien::_0)
    }
    #[doc = "Enables receive underflow interrupt output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ruien::_1)
    }
}
#[doc = "Transmit Overflow Interrupt Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Toien {
    #[doc = "0: Disables transmit overflow interrupt output"]
    _0 = 0,
    #[doc = "1: Enables transmit overflow interrupt output."]
    _1 = 1,
}
impl From<Toien> for bool {
    #[inline(always)]
    fn from(variant: Toien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOIEN` reader - Transmit Overflow Interrupt Output Enable"]
pub type ToienR = crate::BitReader<Toien>;
impl ToienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Toien {
        match self.bits {
            false => Toien::_0,
            true => Toien::_1,
        }
    }
    #[doc = "Disables transmit overflow interrupt output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Toien::_0
    }
    #[doc = "Enables transmit overflow interrupt output."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Toien::_1
    }
}
#[doc = "Field `TOIEN` writer - Transmit Overflow Interrupt Output Enable"]
pub type ToienW<'a, REG> = crate::BitWriter<'a, REG, Toien>;
impl<'a, REG> ToienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables transmit overflow interrupt output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Toien::_0)
    }
    #[doc = "Enables transmit overflow interrupt output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Toien::_1)
    }
}
#[doc = "Transmit Underflow Interrupt Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tuien {
    #[doc = "0: Disables transmit underflow interrupt output"]
    _0 = 0,
    #[doc = "1: Enables transmit underflow interrupt output."]
    _1 = 1,
}
impl From<Tuien> for bool {
    #[inline(always)]
    fn from(variant: Tuien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TUIEN` reader - Transmit Underflow Interrupt Output Enable"]
pub type TuienR = crate::BitReader<Tuien>;
impl TuienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tuien {
        match self.bits {
            false => Tuien::_0,
            true => Tuien::_1,
        }
    }
    #[doc = "Disables transmit underflow interrupt output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tuien::_0
    }
    #[doc = "Enables transmit underflow interrupt output."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tuien::_1
    }
}
#[doc = "Field `TUIEN` writer - Transmit Underflow Interrupt Output Enable"]
pub type TuienW<'a, REG> = crate::BitWriter<'a, REG, Tuien>;
impl<'a, REG> TuienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables transmit underflow interrupt output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tuien::_0)
    }
    #[doc = "Enables transmit underflow interrupt output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tuien::_1)
    }
}
#[doc = "Selects an Audio Clock for Master-mode Communication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cks {
    #[doc = "0: Selects the AUDIO_CLK input"]
    _0 = 0,
    #[doc = "1: Selects the GTIOC1A (GPT output)."]
    _1 = 1,
}
impl From<Cks> for bool {
    #[inline(always)]
    fn from(variant: Cks) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKS` reader - Selects an Audio Clock for Master-mode Communication"]
pub type CksR = crate::BitReader<Cks>;
impl CksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cks {
        match self.bits {
            false => Cks::_0,
            true => Cks::_1,
        }
    }
    #[doc = "Selects the AUDIO_CLK input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cks::_0
    }
    #[doc = "Selects the GTIOC1A (GPT output)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cks::_1
    }
}
#[doc = "Field `CKS` writer - Selects an Audio Clock for Master-mode Communication"]
pub type CksW<'a, REG> = crate::BitWriter<'a, REG, Cks>;
impl<'a, REG> CksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selects the AUDIO_CLK input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_0)
    }
    #[doc = "Selects the GTIOC1A (GPT output)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Receive Enable"]
    #[inline(always)]
    pub fn ren(&self) -> RenR {
        RenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Enable"]
    #[inline(always)]
    pub fn ten(&self) -> TenR {
        TenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Mute Enable"]
    #[inline(always)]
    pub fn muen(&self) -> MuenR {
        MuenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Selects Bit Clock Division Ratio"]
    #[inline(always)]
    pub fn ckdv(&self) -> CkdvR {
        CkdvR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Selects Serial Data Delay"]
    #[inline(always)]
    pub fn del(&self) -> DelR {
        DelR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Selects Placement Data Alignment"]
    #[inline(always)]
    pub fn pdta(&self) -> PdtaR {
        PdtaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects Serial Data Alignment"]
    #[inline(always)]
    pub fn sdta(&self) -> SdtaR {
        SdtaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Selects Serial Padding Polarity"]
    #[inline(always)]
    pub fn spdp(&self) -> SpdpR {
        SpdpR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Selects the Initial Value and Polarity of LR Clock/Frame Synchronization Signal"]
    #[inline(always)]
    pub fn lrckp(&self) -> LrckpR {
        LrckpR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Selects Bit Clock Polarity"]
    #[inline(always)]
    pub fn bckp(&self) -> BckpR {
        BckpR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Master Enable"]
    #[inline(always)]
    pub fn mst(&self) -> MstR {
        MstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Selects System Word Length"]
    #[inline(always)]
    pub fn swl(&self) -> SwlR {
        SwlR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - Selects Data Word Length"]
    #[inline(always)]
    pub fn dwl(&self) -> DwlR {
        DwlR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bit 25 - Idle Mode Interrupt Output Enable"]
    #[inline(always)]
    pub fn iien(&self) -> IienR {
        IienR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Receive Overflow Interrupt Output Enable"]
    #[inline(always)]
    pub fn roien(&self) -> RoienR {
        RoienR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Receive Underflow Interrupt Output Enable"]
    #[inline(always)]
    pub fn ruien(&self) -> RuienR {
        RuienR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Transmit Overflow Interrupt Output Enable"]
    #[inline(always)]
    pub fn toien(&self) -> ToienR {
        ToienR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Transmit Underflow Interrupt Output Enable"]
    #[inline(always)]
    pub fn tuien(&self) -> TuienR {
        TuienR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Selects an Audio Clock for Master-mode Communication"]
    #[inline(always)]
    pub fn cks(&self) -> CksR {
        CksR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSICR")
            .field("cks", &self.cks())
            .field("tuien", &self.tuien())
            .field("toien", &self.toien())
            .field("ruien", &self.ruien())
            .field("roien", &self.roien())
            .field("iien", &self.iien())
            .field("dwl", &self.dwl())
            .field("swl", &self.swl())
            .field("mst", &self.mst())
            .field("bckp", &self.bckp())
            .field("lrckp", &self.lrckp())
            .field("spdp", &self.spdp())
            .field("sdta", &self.sdta())
            .field("pdta", &self.pdta())
            .field("del", &self.del())
            .field("ckdv", &self.ckdv())
            .field("muen", &self.muen())
            .field("ten", &self.ten())
            .field("ren", &self.ren())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Receive Enable"]
    #[inline(always)]
    pub fn ren(&mut self) -> RenW<SsicrSpec> {
        RenW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Enable"]
    #[inline(always)]
    pub fn ten(&mut self) -> TenW<SsicrSpec> {
        TenW::new(self, 1)
    }
    #[doc = "Bit 3 - Mute Enable"]
    #[inline(always)]
    pub fn muen(&mut self) -> MuenW<SsicrSpec> {
        MuenW::new(self, 3)
    }
    #[doc = "Bits 4:7 - Selects Bit Clock Division Ratio"]
    #[inline(always)]
    pub fn ckdv(&mut self) -> CkdvW<SsicrSpec> {
        CkdvW::new(self, 4)
    }
    #[doc = "Bit 8 - Selects Serial Data Delay"]
    #[inline(always)]
    pub fn del(&mut self) -> DelW<SsicrSpec> {
        DelW::new(self, 8)
    }
    #[doc = "Bit 9 - Selects Placement Data Alignment"]
    #[inline(always)]
    pub fn pdta(&mut self) -> PdtaW<SsicrSpec> {
        PdtaW::new(self, 9)
    }
    #[doc = "Bit 10 - Selects Serial Data Alignment"]
    #[inline(always)]
    pub fn sdta(&mut self) -> SdtaW<SsicrSpec> {
        SdtaW::new(self, 10)
    }
    #[doc = "Bit 11 - Selects Serial Padding Polarity"]
    #[inline(always)]
    pub fn spdp(&mut self) -> SpdpW<SsicrSpec> {
        SpdpW::new(self, 11)
    }
    #[doc = "Bit 12 - Selects the Initial Value and Polarity of LR Clock/Frame Synchronization Signal"]
    #[inline(always)]
    pub fn lrckp(&mut self) -> LrckpW<SsicrSpec> {
        LrckpW::new(self, 12)
    }
    #[doc = "Bit 13 - Selects Bit Clock Polarity"]
    #[inline(always)]
    pub fn bckp(&mut self) -> BckpW<SsicrSpec> {
        BckpW::new(self, 13)
    }
    #[doc = "Bit 14 - Master Enable"]
    #[inline(always)]
    pub fn mst(&mut self) -> MstW<SsicrSpec> {
        MstW::new(self, 14)
    }
    #[doc = "Bits 16:18 - Selects System Word Length"]
    #[inline(always)]
    pub fn swl(&mut self) -> SwlW<SsicrSpec> {
        SwlW::new(self, 16)
    }
    #[doc = "Bits 19:21 - Selects Data Word Length"]
    #[inline(always)]
    pub fn dwl(&mut self) -> DwlW<SsicrSpec> {
        DwlW::new(self, 19)
    }
    #[doc = "Bit 25 - Idle Mode Interrupt Output Enable"]
    #[inline(always)]
    pub fn iien(&mut self) -> IienW<SsicrSpec> {
        IienW::new(self, 25)
    }
    #[doc = "Bit 26 - Receive Overflow Interrupt Output Enable"]
    #[inline(always)]
    pub fn roien(&mut self) -> RoienW<SsicrSpec> {
        RoienW::new(self, 26)
    }
    #[doc = "Bit 27 - Receive Underflow Interrupt Output Enable"]
    #[inline(always)]
    pub fn ruien(&mut self) -> RuienW<SsicrSpec> {
        RuienW::new(self, 27)
    }
    #[doc = "Bit 28 - Transmit Overflow Interrupt Output Enable"]
    #[inline(always)]
    pub fn toien(&mut self) -> ToienW<SsicrSpec> {
        ToienW::new(self, 28)
    }
    #[doc = "Bit 29 - Transmit Underflow Interrupt Output Enable"]
    #[inline(always)]
    pub fn tuien(&mut self) -> TuienW<SsicrSpec> {
        TuienW::new(self, 29)
    }
    #[doc = "Bit 30 - Selects an Audio Clock for Master-mode Communication"]
    #[inline(always)]
    pub fn cks(&mut self) -> CksW<SsicrSpec> {
        CksW::new(self, 30)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsicrSpec;
impl crate::RegisterSpec for SsicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssicr::R`](R) reader structure"]
impl crate::Readable for SsicrSpec {}
#[doc = "`write(|w| ..)` method takes [`ssicr::W`](W) writer structure"]
impl crate::Writable for SsicrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSICR to value 0"]
impl crate::Resettable for SsicrSpec {}
