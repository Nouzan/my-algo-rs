use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    is_softs: Vec<bool>,
}

/// 车厢调度: 将软座车厢调度至硬座车厢之前.
///
/// 输入: N节车厢, 可能为硬座或软座, 表示为一个布尔列表`is_softs`, `true`表示该位置上的车厢为软座, 否则为硬座.
///
/// 输出: 一个字符列表, `I`表示入栈, `O`表示出栈. 入栈对应于让车厢进入暂存区, 出栈对应于让暂存区的车厢离开.
/// 通过该操作序列实现让软座车厢调度至硬座车厢之前.
///
/// 思路: 遇到硬座则入栈, 遇到软座则入栈后立马出栈.
fn main() {
    let opt = Opt::from_args();
    let mut ops = vec![];
    let len = opt.is_softs.len();
    for tran in opt.is_softs {
        ops.push(true);
        if tran {
            ops.push(false);
        }
    }
    ops.resize(2 * len, false);
    let ops: Vec<_> = ops
        .iter()
        .copied()
        .map(|push| if push { "I" } else { "O" })
        .collect();

    println!("{}", ops.join(" "));
}
