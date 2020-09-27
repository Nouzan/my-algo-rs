use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    n: usize,
    x: f64,
}

fn _fun_recur(n: usize, x: f64) -> f64 {
    match n {
        0 => 1.0,
        1 => 2.0 * x,
        n => 2.0 * x * _fun_recur(n - 1, x) - 2.0 * (n as f64 - 1.0) * _fun_recur(n - 2, x),
    }
}

fn fun(n: usize, x: f64) -> f64 {
    let mut stack = Vec::new();
    match n {
        0 => 1.0,
        1 => 2.0 * x,
        n => {
            stack.push(1.0);
            stack.push(2.0 * x);
            for idx in 2..=n {
                let f2 = stack.pop().unwrap();
                let f1 = stack.pop().unwrap();
                let ans = 2.0 * x * f2 - 2.0 * (idx as f64 - 1.0) * f1;
                stack.push(f2);
                stack.push(ans);
            }
            stack.pop().unwrap()
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    println!("{}", fun(opt.n, opt.x));
}
