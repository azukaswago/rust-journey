// You're building a **trade history analyzer** for a crypto portfolio tracker.

// The system needs to store and compare trade records efficiently using references where appropriate.

// **Requirements:**

// 1. Define a struct `TradeRecord<'a>` with fields: `pair: &'a str`, `side: &'a str`, and `amount: f64`

// 2. Implement a method `summary(&self)` on `TradeRecord` that prints all three fields in a clean format
// — no lifetime annotations needed on the method (elision handles it)

// 3. Write a function `larger_trade<'a>` that takes two `&'a TradeRecord` references and returns the one with the higher `amount`

// 4. Define an enum `TradeStatus` with variants `Open` and `Closed(f64)` — the `f64` is the profit/loss

// 5. Implement `Display` for `TradeStatus` — `Open` prints `"Trade is open"`, `Closed(x)` prints `"Trade closed with P&L: $x"`

// 6. Implement `PartialEq` for `TradeRecord` — two trades are equal if `pair` and `side` match

// 7. In `main`, create two `TradeRecord` instances, call `larger_trade`, print the summary of the larger one, test equality,
// and print a `TradeStatus`

// No syntax hints. Build it.

use std::fmt::Display;
use std::io;
struct TradeRecord<'a> {
    pair: &'a str,
    side: &'a str,
    amount: f64,
}

impl TradeRecord<'_> {
    fn summary(&self) {
        println!(
            "pair: {}, side: {}, amount: {}",
            self.pair, self.side, self.amount
        )
    }
}

fn larger_trade<'a>(
    trade: &'a TradeRecord<'a>,
    trade2: &'a TradeRecord<'a>,
) -> &'a TradeRecord<'a> {
    if trade.amount > trade2.amount {
        trade
    } else {
        trade2
    }
}

enum TradeStatus {
    Open,
    Closed(f64),
}

impl Display for TradeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradeStatus::Open => write!(f, "Trade is Open"),
            TradeStatus::Closed(x) => write!(f, "Trade closed with PnL: ${}", x),
        }
    }
}

impl PartialEq for TradeRecord<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.pair == other.pair && self.side == other.side
    }
}
fn main() {
    let trade_record = TradeRecord {
        pair: "BTC/USD",
        side: "BUY",
        amount: 6.0,
    };

    let trade_record2 = TradeRecord {
        pair: "BTC/USD",
        side: "BUY",
        amount: 7.0,
    };

    larger_trade(&trade_record, &trade_record2).summary();

    println!("{}", TradeStatus::Open);
    println!("{}", TradeStatus::Closed(78.0));
}


