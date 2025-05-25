/// Represents a 32-byte address used throughout the InterLiquid SDK.
/// This is typically used to identify accounts, contracts, or other entities.
pub type Address = [u8; 32];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_creation() {
        let addr: Address = [0u8; 32];
        assert_eq!(addr.len(), 32);
        assert!(addr.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_address_with_values() {
        let mut addr: Address = [0u8; 32];
        addr[0] = 1;
        addr[31] = 255;
        
        assert_eq!(addr[0], 1);
        assert_eq!(addr[31], 255);
        assert_eq!(addr[1], 0);
    }

    #[test]
    fn test_address_comparison() {
        let addr1: Address = [1u8; 32];
        let addr2: Address = [1u8; 32];
        let addr3: Address = [2u8; 32];
        
        assert_eq!(addr1, addr2);
        assert_ne!(addr1, addr3);
    }
}
