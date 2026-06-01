

// pub trait TradableAsset {
//     fn max_leverage (&self)-> u32;
//     fn ticker(&self) -> String;
//     fn cost_of_asset(&self) -> f64;
//     fn risk_profile(&self) -> String {
//         format!("{} with max leverage {}x at  ${}", self.ticker(), self.max_leverage(), self.cost_of_asset())
//     }
// }

// pub struct CryptoAsset {
//    pub asset: String,
//    pub price: f64,
//    pub max_leverage: u32,
// }

// pub struct ForexPair {
//    pub pair: String,
//    pub price: f64,
//    pub max_leverage: u32,
// }

// impl TradableAsset for CryptoAsset {
//     fn max_leverage (&self)-> u32 {
//         self.max_leverage
//     } 

//     fn cost_of_asset(&self) -> f64 {
//         self.price
//     }

//     fn ticker(&self) -> String {
//         self.asset.clone()
//     }
//     }



// impl TradableAsset for ForexPair {
//     fn max_leverage (&self)-> u32 {
//         self.max_leverage
//     } 
//     fn cost_of_asset(&self) -> f64 {
//         self.price
//     }

//     fn ticker(&self) -> String {
//         self.pair.clone()
//     }
// }


