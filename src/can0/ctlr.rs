#[doc = "Register `CTLR` reader"]
pub type R = crate::R<CtlrSpec>;
#[doc = "Register `CTLR` writer"]
pub type W = crate::W<CtlrSpec>;
#[doc = "CAN Mailbox Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mbm {
    #[doc = "0: Normal mailbox mode"]
    _0 = 0,
    #[doc = "1: FIFO mailbox mode"]
    _1 = 1,
}
impl From<Mbm> for bool {
    #[inline(always)]
    fn from(variant: Mbm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MBM` reader - CAN Mailbox Mode Select"]
pub type MbmR = crate::BitReader<Mbm>;
impl MbmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mbm {
        match self.bits {
            false => Mbm::_0,
            true => Mbm::_1,
        }
    }
    #[doc = "Normal mailbox mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mbm::_0
    }
    #[doc = "FIFO mailbox mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mbm::_1
    }
}
#[doc = "Field `MBM` writer - CAN Mailbox Mode Select"]
pub type MbmW<'a, REG> = crate::BitWriter<'a, REG, Mbm>;
impl<'a, REG> MbmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mailbox mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mbm::_0)
    }
    #[doc = "FIFO mailbox mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mbm::_1)
    }
}
#[doc = "ID Format Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Idfm {
    #[doc = "0: Standard ID mode.All mailboxes (including FIFO mailboxes) handle only standard Ids."]
    _00 = 0,
    #[doc = "1: Extended ID mode.All mailboxes (including FIFO mailboxes) handle only extended IDs."]
    _01 = 1,
    #[doc = "2: Mixed ID mode.All mailboxes (including FIFO mailboxes) handle both standard IDs and extended IDs. Standard IDs or extended IDs are specified by using the IDE bit in the corresponding mailbox in normal mailbox mode. In FIFO mailbox mode, the IDE bit in the corresponding mailbox is used for mailboxes \\[0\\] to \\[23\\], the IDE bits in FIDCR0 and FIDCR1 are used for the receive FIFO, and the IDE bit in mailbox \\[24\\] is used for the transmit FIFO."]
    _10 = 2,
    #[doc = "3: Do not use this combination"]
    _11 = 3,
}
impl From<Idfm> for u8 {
    #[inline(always)]
    fn from(variant: Idfm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Idfm {
    type Ux = u8;
}
impl crate::IsEnum for Idfm {}
#[doc = "Field `IDFM` reader - ID Format Mode Select"]
pub type IdfmR = crate::FieldReader<Idfm>;
impl IdfmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idfm {
        match self.bits {
            0 => Idfm::_00,
            1 => Idfm::_01,
            2 => Idfm::_10,
            3 => Idfm::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Standard ID mode.All mailboxes (including FIFO mailboxes) handle only standard Ids."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Idfm::_00
    }
    #[doc = "Extended ID mode.All mailboxes (including FIFO mailboxes) handle only extended IDs."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Idfm::_01
    }
    #[doc = "Mixed ID mode.All mailboxes (including FIFO mailboxes) handle both standard IDs and extended IDs. Standard IDs or extended IDs are specified by using the IDE bit in the corresponding mailbox in normal mailbox mode. In FIFO mailbox mode, the IDE bit in the corresponding mailbox is used for mailboxes \\[0\\] to \\[23\\], the IDE bits in FIDCR0 and FIDCR1 are used for the receive FIFO, and the IDE bit in mailbox \\[24\\] is used for the transmit FIFO."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Idfm::_10
    }
    #[doc = "Do not use this combination"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Idfm::_11
    }
}
#[doc = "Field `IDFM` writer - ID Format Mode Select"]
pub type IdfmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Idfm, crate::Safe>;
impl<'a, REG> IdfmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Standard ID mode.All mailboxes (including FIFO mailboxes) handle only standard Ids."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Idfm::_00)
    }
    #[doc = "Extended ID mode.All mailboxes (including FIFO mailboxes) handle only extended IDs."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Idfm::_01)
    }
    #[doc = "Mixed ID mode.All mailboxes (including FIFO mailboxes) handle both standard IDs and extended IDs. Standard IDs or extended IDs are specified by using the IDE bit in the corresponding mailbox in normal mailbox mode. In FIFO mailbox mode, the IDE bit in the corresponding mailbox is used for mailboxes \\[0\\] to \\[23\\], the IDE bits in FIDCR0 and FIDCR1 are used for the receive FIFO, and the IDE bit in mailbox \\[24\\] is used for the transmit FIFO."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Idfm::_10)
    }
    #[doc = "Do not use this combination"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Idfm::_11)
    }
}
#[doc = "Message Lost Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mlm {
    #[doc = "0: Overwrite mode"]
    _0 = 0,
    #[doc = "1: Overrun mode"]
    _1 = 1,
}
impl From<Mlm> for bool {
    #[inline(always)]
    fn from(variant: Mlm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MLM` reader - Message Lost Mode Select"]
pub type MlmR = crate::BitReader<Mlm>;
impl MlmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mlm {
        match self.bits {
            false => Mlm::_0,
            true => Mlm::_1,
        }
    }
    #[doc = "Overwrite mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mlm::_0
    }
    #[doc = "Overrun mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mlm::_1
    }
}
#[doc = "Field `MLM` writer - Message Lost Mode Select"]
pub type MlmW<'a, REG> = crate::BitWriter<'a, REG, Mlm>;
impl<'a, REG> MlmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overwrite mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mlm::_0)
    }
    #[doc = "Overrun mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mlm::_1)
    }
}
#[doc = "Transmission Priority Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tpm {
    #[doc = "0: ID priority transmit mode"]
    _0 = 0,
    #[doc = "1: Mailbox number priority transmit mode"]
    _1 = 1,
}
impl From<Tpm> for bool {
    #[inline(always)]
    fn from(variant: Tpm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPM` reader - Transmission Priority Mode Select"]
pub type TpmR = crate::BitReader<Tpm>;
impl TpmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tpm {
        match self.bits {
            false => Tpm::_0,
            true => Tpm::_1,
        }
    }
    #[doc = "ID priority transmit mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tpm::_0
    }
    #[doc = "Mailbox number priority transmit mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tpm::_1
    }
}
#[doc = "Field `TPM` writer - Transmission Priority Mode Select"]
pub type TpmW<'a, REG> = crate::BitWriter<'a, REG, Tpm>;
impl<'a, REG> TpmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ID priority transmit mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tpm::_0)
    }
    #[doc = "Mailbox number priority transmit mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tpm::_1)
    }
}
#[doc = "Time Stamp Counter Reset Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsrc {
    #[doc = "0: Nothing occurred"]
    _0 = 0,
    #[doc = "1: Reset"]
    _1 = 1,
}
impl From<Tsrc> for bool {
    #[inline(always)]
    fn from(variant: Tsrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSRC` reader - Time Stamp Counter Reset Command"]
pub type TsrcR = crate::BitReader<Tsrc>;
impl TsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsrc {
        match self.bits {
            false => Tsrc::_0,
            true => Tsrc::_1,
        }
    }
    #[doc = "Nothing occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tsrc::_0
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tsrc::_1
    }
}
#[doc = "Field `TSRC` writer - Time Stamp Counter Reset Command"]
pub type TsrcW<'a, REG> = crate::BitWriter<'a, REG, Tsrc>;
impl<'a, REG> TsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Nothing occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tsrc::_0)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsrc::_1)
    }
}
#[doc = "Time Stamp Prescaler Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tsps {
    #[doc = "0: Every bit time"]
    _00 = 0,
    #[doc = "1: Every 2-bit time"]
    _01 = 1,
    #[doc = "2: Every 4-bit time"]
    _10 = 2,
    #[doc = "3: Every 8-bit time"]
    _11 = 3,
}
impl From<Tsps> for u8 {
    #[inline(always)]
    fn from(variant: Tsps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tsps {
    type Ux = u8;
}
impl crate::IsEnum for Tsps {}
#[doc = "Field `TSPS` reader - Time Stamp Prescaler Select"]
pub type TspsR = crate::FieldReader<Tsps>;
impl TspsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsps {
        match self.bits {
            0 => Tsps::_00,
            1 => Tsps::_01,
            2 => Tsps::_10,
            3 => Tsps::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Every bit time"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Tsps::_00
    }
    #[doc = "Every 2-bit time"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Tsps::_01
    }
    #[doc = "Every 4-bit time"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Tsps::_10
    }
    #[doc = "Every 8-bit time"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Tsps::_11
    }
}
#[doc = "Field `TSPS` writer - Time Stamp Prescaler Select"]
pub type TspsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tsps, crate::Safe>;
impl<'a, REG> TspsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Every bit time"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Tsps::_00)
    }
    #[doc = "Every 2-bit time"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Tsps::_01)
    }
    #[doc = "Every 4-bit time"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Tsps::_10)
    }
    #[doc = "Every 8-bit time"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Tsps::_11)
    }
}
#[doc = "CAN Operating Mode Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Canm {
    #[doc = "0: CAN operation mode"]
    _00 = 0,
    #[doc = "1: CAN reset mode"]
    _01 = 1,
    #[doc = "2: CAN halt mode"]
    _10 = 2,
    #[doc = "3: CAN reset mode (forcible transition)"]
    _11 = 3,
}
impl From<Canm> for u8 {
    #[inline(always)]
    fn from(variant: Canm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Canm {
    type Ux = u8;
}
impl crate::IsEnum for Canm {}
#[doc = "Field `CANM` reader - CAN Operating Mode Select"]
pub type CanmR = crate::FieldReader<Canm>;
impl CanmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Canm {
        match self.bits {
            0 => Canm::_00,
            1 => Canm::_01,
            2 => Canm::_10,
            3 => Canm::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "CAN operation mode"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Canm::_00
    }
    #[doc = "CAN reset mode"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Canm::_01
    }
    #[doc = "CAN halt mode"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Canm::_10
    }
    #[doc = "CAN reset mode (forcible transition)"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Canm::_11
    }
}
#[doc = "Field `CANM` writer - CAN Operating Mode Select"]
pub type CanmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Canm, crate::Safe>;
impl<'a, REG> CanmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CAN operation mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Canm::_00)
    }
    #[doc = "CAN reset mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Canm::_01)
    }
    #[doc = "CAN halt mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Canm::_10)
    }
    #[doc = "CAN reset mode (forcible transition)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Canm::_11)
    }
}
#[doc = "CAN Sleep Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slpm {
    #[doc = "0: Other than CAN sleep mode"]
    _0 = 0,
    #[doc = "1: CAN sleep mode"]
    _1 = 1,
}
impl From<Slpm> for bool {
    #[inline(always)]
    fn from(variant: Slpm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLPM` reader - CAN Sleep Mode"]
pub type SlpmR = crate::BitReader<Slpm>;
impl SlpmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slpm {
        match self.bits {
            false => Slpm::_0,
            true => Slpm::_1,
        }
    }
    #[doc = "Other than CAN sleep mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Slpm::_0
    }
    #[doc = "CAN sleep mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Slpm::_1
    }
}
#[doc = "Field `SLPM` writer - CAN Sleep Mode"]
pub type SlpmW<'a, REG> = crate::BitWriter<'a, REG, Slpm>;
impl<'a, REG> SlpmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Other than CAN sleep mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Slpm::_0)
    }
    #[doc = "CAN sleep mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Slpm::_1)
    }
}
#[doc = "Bus-Off Recovery Mode by a program request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bom {
    #[doc = "0: Normal mode (ISO11898-1 compliant)"]
    _00 = 0,
    #[doc = "1: Entry to CAN halt mode automatically at bus-off entry"]
    _01 = 1,
    #[doc = "2: Entry to CAN halt mode automatically at bus-off end"]
    _10 = 2,
    #[doc = "3: Entry to CAN halt mode (during bus-off recovery period)"]
    _11 = 3,
}
impl From<Bom> for u8 {
    #[inline(always)]
    fn from(variant: Bom) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bom {
    type Ux = u8;
}
impl crate::IsEnum for Bom {}
#[doc = "Field `BOM` reader - Bus-Off Recovery Mode by a program request"]
pub type BomR = crate::FieldReader<Bom>;
impl BomR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bom {
        match self.bits {
            0 => Bom::_00,
            1 => Bom::_01,
            2 => Bom::_10,
            3 => Bom::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal mode (ISO11898-1 compliant)"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Bom::_00
    }
    #[doc = "Entry to CAN halt mode automatically at bus-off entry"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Bom::_01
    }
    #[doc = "Entry to CAN halt mode automatically at bus-off end"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Bom::_10
    }
    #[doc = "Entry to CAN halt mode (during bus-off recovery period)"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Bom::_11
    }
}
#[doc = "Field `BOM` writer - Bus-Off Recovery Mode by a program request"]
pub type BomW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bom, crate::Safe>;
impl<'a, REG> BomW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal mode (ISO11898-1 compliant)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Bom::_00)
    }
    #[doc = "Entry to CAN halt mode automatically at bus-off entry"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Bom::_01)
    }
    #[doc = "Entry to CAN halt mode automatically at bus-off end"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Bom::_10)
    }
    #[doc = "Entry to CAN halt mode (during bus-off recovery period)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Bom::_11)
    }
}
#[doc = "Forcible Return From Bus-Off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rboc {
    #[doc = "0: Nothing occurred"]
    _0 = 0,
    #[doc = "1: Forcible return from bus-off"]
    _1 = 1,
}
impl From<Rboc> for bool {
    #[inline(always)]
    fn from(variant: Rboc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBOC` reader - Forcible Return From Bus-Off"]
pub type RbocR = crate::BitReader<Rboc>;
impl RbocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rboc {
        match self.bits {
            false => Rboc::_0,
            true => Rboc::_1,
        }
    }
    #[doc = "Nothing occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rboc::_0
    }
    #[doc = "Forcible return from bus-off"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rboc::_1
    }
}
#[doc = "Field `RBOC` writer - Forcible Return From Bus-Off"]
pub type RbocW<'a, REG> = crate::BitWriter<'a, REG, Rboc>;
impl<'a, REG> RbocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Nothing occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rboc::_0)
    }
    #[doc = "Forcible return from bus-off"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rboc::_1)
    }
}
impl R {
    #[doc = "Bit 0 - CAN Mailbox Mode Select"]
    #[inline(always)]
    pub fn mbm(&self) -> MbmR {
        MbmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - ID Format Mode Select"]
    #[inline(always)]
    pub fn idfm(&self) -> IdfmR {
        IdfmR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Message Lost Mode Select"]
    #[inline(always)]
    pub fn mlm(&self) -> MlmR {
        MlmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmission Priority Mode Select"]
    #[inline(always)]
    pub fn tpm(&self) -> TpmR {
        TpmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Time Stamp Counter Reset Command"]
    #[inline(always)]
    pub fn tsrc(&self) -> TsrcR {
        TsrcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Time Stamp Prescaler Select"]
    #[inline(always)]
    pub fn tsps(&self) -> TspsR {
        TspsR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - CAN Operating Mode Select"]
    #[inline(always)]
    pub fn canm(&self) -> CanmR {
        CanmR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - CAN Sleep Mode"]
    #[inline(always)]
    pub fn slpm(&self) -> SlpmR {
        SlpmR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Bus-Off Recovery Mode by a program request"]
    #[inline(always)]
    pub fn bom(&self) -> BomR {
        BomR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Forcible Return From Bus-Off"]
    #[inline(always)]
    pub fn rboc(&self) -> RbocR {
        RbocR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTLR")
            .field("rboc", &self.rboc())
            .field("bom", &self.bom())
            .field("slpm", &self.slpm())
            .field("canm", &self.canm())
            .field("tsps", &self.tsps())
            .field("tsrc", &self.tsrc())
            .field("tpm", &self.tpm())
            .field("mlm", &self.mlm())
            .field("idfm", &self.idfm())
            .field("mbm", &self.mbm())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - CAN Mailbox Mode Select"]
    #[inline(always)]
    pub fn mbm(&mut self) -> MbmW<CtlrSpec> {
        MbmW::new(self, 0)
    }
    #[doc = "Bits 1:2 - ID Format Mode Select"]
    #[inline(always)]
    pub fn idfm(&mut self) -> IdfmW<CtlrSpec> {
        IdfmW::new(self, 1)
    }
    #[doc = "Bit 3 - Message Lost Mode Select"]
    #[inline(always)]
    pub fn mlm(&mut self) -> MlmW<CtlrSpec> {
        MlmW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmission Priority Mode Select"]
    #[inline(always)]
    pub fn tpm(&mut self) -> TpmW<CtlrSpec> {
        TpmW::new(self, 4)
    }
    #[doc = "Bit 5 - Time Stamp Counter Reset Command"]
    #[inline(always)]
    pub fn tsrc(&mut self) -> TsrcW<CtlrSpec> {
        TsrcW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Time Stamp Prescaler Select"]
    #[inline(always)]
    pub fn tsps(&mut self) -> TspsW<CtlrSpec> {
        TspsW::new(self, 6)
    }
    #[doc = "Bits 8:9 - CAN Operating Mode Select"]
    #[inline(always)]
    pub fn canm(&mut self) -> CanmW<CtlrSpec> {
        CanmW::new(self, 8)
    }
    #[doc = "Bit 10 - CAN Sleep Mode"]
    #[inline(always)]
    pub fn slpm(&mut self) -> SlpmW<CtlrSpec> {
        SlpmW::new(self, 10)
    }
    #[doc = "Bits 11:12 - Bus-Off Recovery Mode by a program request"]
    #[inline(always)]
    pub fn bom(&mut self) -> BomW<CtlrSpec> {
        BomW::new(self, 11)
    }
    #[doc = "Bit 13 - Forcible Return From Bus-Off"]
    #[inline(always)]
    pub fn rboc(&mut self) -> RbocW<CtlrSpec> {
        RbocW::new(self, 13)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlrSpec;
impl crate::RegisterSpec for CtlrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctlr::R`](R) reader structure"]
impl crate::Readable for CtlrSpec {}
#[doc = "`write(|w| ..)` method takes [`ctlr::W`](W) writer structure"]
impl crate::Writable for CtlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTLR to value 0x0500"]
impl crate::Resettable for CtlrSpec {
    const RESET_VALUE: u16 = 0x0500;
}
