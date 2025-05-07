use borsh::{BorshDeserialize, BorshSerialize};
use crypto_bigint::{Encoding, U256};

pub struct Token {
    pub denom: String,
    pub amount: U256,
}

impl BorshSerialize for Token {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.denom.serialize(writer)?;
        self.amount.to_le_bytes().serialize(writer)?;
        Ok(())
    }
}

impl BorshDeserialize for Token {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let denom = String::deserialize_reader(reader)?;
        let amount_bytes = <U256 as Encoding>::Repr::deserialize_reader(reader)?;
        let amount = U256::from_le_bytes(amount_bytes);
        Ok(Token { denom, amount })
    }
}
