use std::io::BufReader;
use market_common::good::good_kind::GoodKind::{EUR, USD, YEN, YUAN};
use market_common::market::SellError;
use trader::trader::MarketKind::{BFB, BOSE, DOGE};
use trader::trader::Trader;
use trader::trader::trader_errors::{TraderDemandError, TraderSupplyError};
use colored::Colorize;
use gtk_plotter;

//My implementation of "a struct function that is dynamically defined" (achieved using the closoure field inside the struct) is definitely
//not a common one, therefore I had to come out for an acrobatic way of dealing with states. I was scared of doing this:
//https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=12360dab5f942f84938867448f2b012e
//..because I thought I was scared I would modify the function *while* it was running. It appears that this is not the case.
//it appears that this wasn't the case: everything worked fine.
//The problem that appeared is that, after stealing the closure from inside the trader (in run()), I couldn't put it back anymore
//I could fix this problem easily with function pointers instead of dyn Fn, in order to be able to clone them, but I would need to rewrite
//the whole thing.
//I need to check if the closure was modified before "putting it back": I used an external variable. It's awful and not rusty, but I sticked to it.

fn BFB_YEN_DEPLETE(trader : &mut Trader) {

    loop {
        match trader.buy(BFB, YEN, 10_000.0){
            Ok(_) => {}
            Err(e) => {
                match e {
                    TraderSupplyError::MarketInsufficientSupply => {
                        trader.buy(BFB, YEN, trader.get_good_qty(BFB, YEN)).ok();

                        trader.set_strategy(BFB_FINAL_RUN);

                        return;
                    }
                    TraderSupplyError::TraderInsufficientFunds => {
                        trader.set_strategy(BFB_YUAN_SELL_EXPLOIT);
                        return;
                    }
                    _ => panic!("{:?}", e),
                }

            }
        }
    }
}

fn BFB_USD_DEPLETE(trader : &mut Trader) {
    loop {
        match trader.buy(BFB, USD, 10_000.0){
            Ok(_) => {}
            Err(e) => {
                match e {
                    TraderSupplyError::MarketInsufficientSupply => {
                        trader.buy(BFB, USD, trader.get_good_qty(BFB, USD)).ok();
                        trader.set_strategy(BFB_YEN_DEPLETE);
                        return;
                    }
                    TraderSupplyError::TraderInsufficientFunds => {
                        trader.set_strategy(BFB_YUAN_SELL_EXPLOIT);
                        return;
                    }
                    _ => panic!("{:?}", e),
                }

            }
        }
    }
}

fn BFB_YUAN_SELL_EXPLOIT(trader : &mut Trader) {
    loop {

        println!("{}", "\nStatus: exploiting BFB sell".green());
        trader.print_goods();

        //exploit
        while trader.get_demand_price(BFB, YUAN) < 400.0 {
            trader.sell(BFB, YUAN, 0.00001).unwrap_or_else(|_| {
                trader.buy(BOSE, YUAN, 1.0).unwrap();
                0.0
            });
        }

        match trader.sell(BFB, YUAN, 10_000.0) {
            Ok(_) => {}
            Err(e) => {
                match e {
                    TraderDemandError::MarketInsufficientFunds => {
                        println!("BFB no more euros.");
                        trader.set_strategy(BFB_USD_DEPLETE);
                        return;
                    }
                    TraderDemandError::TraderInsufficientGoods => {
                        trader.set_strategy(BFB_YUAN_BUY_EXPLOIT);
                        return;
                    }
                    _ => panic!("{:?}", e),
                }

            }
        }

    }
}

fn BFB_YUAN_BUY_EXPLOIT(trader : &mut Trader) {
    loop {

        println!("\nStatus: exploiting BFB buy");
        trader.print_goods();

        //exploiting a bug. A side effect is that it breaks the supply price function
        while trader.get_supply_price(BFB, YUAN).is_err() {
            //println!("no good");
            //println!("price: {}", trader.get_demand_price(BFB, YUAN));
            //println!("asd: {}", trader.get_good_qty(BFB, YUAN));
            trader.sell(BFB, YUAN, 0.001).unwrap();
        }

        match trader.buy(BFB, YUAN, 10_000.0) {
            Ok(_) => {}
            Err(e) => {
                match e {
                    TraderSupplyError::MarketInsufficientSupply => {
                        trader.set_strategy(BFB_YUAN_SELL_EXPLOIT);
                        return;
                    }
                    TraderSupplyError::TraderInsufficientFunds => {
                        trader.set_strategy(BFB_YUAN_SELL_EXPLOIT);
                        return;
                    }
                    _ => panic!("{:?}", e),
                }
            }
        }


    }
}

fn BFB_FINAL_RUN(trader : &mut Trader) {

    println!("\n\n\n\n\n\n\n\n\n-----------------\nBailing out...\n-----------------\n\n");
    trader.bailout();
    trader.print_liquidity();
    trader.print_goods();
    println!("\n");
    return;

}

pub fn strategy(trader : &mut Trader) {

    trader.buy(BOSE, YUAN, 10_000.0).unwrap();
    trader.set_strategy(BFB_YUAN_BUY_EXPLOIT);
    return;

}

pub fn test_exploit(trader : &mut Trader) {
    trader.buy(BOSE, YUAN, 10_000.0).unwrap();
    for i in 0..1000 {
        trader.sell(BFB, YUAN, 0.01).unwrap();
        println!("{}. Sell to BFB for: {}", i, trader.get_demand_price(BFB, YUAN));
        println!("{}. Buy from BFB for: {}", i, trader.get_demand_price(BFB, YUAN));
    }
    return;
}
