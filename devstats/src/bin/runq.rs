use std::time::SystemTime;
use std::{env, process};

use devstats::lib;

fn runq(sql_file: &String, params: &[String]) -> lib::Ctx {
    // xxx
    println!("file: {sql_file:?}");
    // xxx
    println!("params: {params:?}");
    let ctx = lib::Ctx::new();
    return ctx;
}

fn main() {
    let dt_start = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        // xxx
        println!("Required SQL file name [param1 value1 [param2 value2 ...]]");
        // xxx
        println!(
            "Special replace 'qr' 'period,from,to' is used for {{period.alias.name}} replacements"
        );
        process::exit(1);
    }
    let ctx = runq(&args[1], &args[2..]);
    // xxx
    println!("ctx is {ctx:?}");
    let res_elapsed = dt_start.elapsed();
    lib::fatal_no_log(&res_elapsed);
    if ctx.debug >= 0 {
        // xxx
        println!("Time: {:?}", res_elapsed.expect("cannot get elapsed time"));
    }
}
