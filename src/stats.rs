#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct GlobalStats {
	/// The number of bitcoin nodes found
	btc_nodes_count: usize,
	/// USD Market cap of bitcoin
	btc_mkt_cap: f64,
	/// Percentage market dominance of bitcoin
	btc_dom: f64,

}