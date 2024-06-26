// External uses
use serde::Deserialize;
// Workspace uses
use zksync_basic_types::Address;

/// Data about deployed contracts.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ContractsConfig {
    pub governance_addr: Address,
    pub verifier_addr: Address,
    pub default_upgrade_addr: Address,
    pub diamond_proxy_addr: Address,
    pub validator_timelock_addr: Address,
    pub l1_erc20_bridge_proxy_addr: Address,
    pub l2_erc20_bridge_addr: Address,
    pub l1_weth_bridge_proxy_addr: Option<Address>,
    pub l2_weth_bridge_addr: Option<Address>,
    pub l2_testnet_paymaster_addr: Option<Address>,
    pub l1_multicall3_addr: Address,
}

impl ContractsConfig {
    pub fn for_tests() -> Self {
        Self {
            verifier_addr: Address::repeat_byte(0x06),
            default_upgrade_addr: Address::repeat_byte(0x06),
            diamond_proxy_addr: Address::repeat_byte(0x09),
            validator_timelock_addr: Address::repeat_byte(0x0a),
            l1_erc20_bridge_proxy_addr: Address::repeat_byte(0x0b),
            l2_erc20_bridge_addr: Address::repeat_byte(0x0d),
            l1_weth_bridge_proxy_addr: Some(Address::repeat_byte(0x0e)),
            l2_weth_bridge_addr: Some(Address::repeat_byte(0x0f)),
            l2_testnet_paymaster_addr: Some(Address::repeat_byte(0x11)),
            l1_multicall3_addr: Address::repeat_byte(0x12),
            governance_addr: Address::repeat_byte(0x13),
        }
    }
}
