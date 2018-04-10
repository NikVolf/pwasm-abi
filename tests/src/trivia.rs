#![allow(dead_code)]
/// The eth_abi marco generates code uses
/// `bigint`, `parity_hash`, `pwasm_ethereum` and `pwasm_abi` crates
/// The following code demonstrates that user dont have to import these crates
/// unless he doesn't use one of those directly

use pwasm_abi_derive::eth_abi;

#[eth_abi(Endpoint, Client)]
pub trait Contract {
	fn constructor(&mut self, _p: bool);
	fn sam(&mut self, _p1: Vec<u8>) -> u32;

	#[event]
	fn baz_fired(&mut self, indexed_p1: u32, p2: u32);
}
