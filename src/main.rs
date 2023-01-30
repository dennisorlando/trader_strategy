mod strategy;

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::io::BufReader;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use trader::trader::MarketKind::{BFB, BOSE, DOGE, TASE};
use trader::trader::Trader;
use bfb::bfb_market::Bfb;
use dogemarket::dogemarket::DogeMarket;
use bose::market::BoseMarket;
use colored::Colorize;
use market_common::good::good_kind::GoodKind;
use market_common::good::good_kind::GoodKind::{EUR, YUAN};
use market_common::market::Market;

fn main() {


    let tase = tase::TASE::new_random();
    let bose = BoseMarket::new_random();
    let doge = DogeMarket::new_random();
    let bfb = Bfb::new_random();

    let mut trader = Trader::new()
        .with_market(BOSE, Rc::clone(&bose))
        .with_market(DOGE, Rc::clone(&doge))
        .with_market(BFB, Rc::clone(&bfb))
        .with_market(TASE, Rc::clone(&tase))
        .with_initial_money(100000.0);

    println!("{}", trader.get_supply_price(BFB, YUAN).unwrap());

    trader.set_strategy(crate::strategy::strategy);
    trader.run(1);

}
