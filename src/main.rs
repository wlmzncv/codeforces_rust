/// [Codeforces Round 896 (Div. 2)](https://codeforces.com/contest/1969)
///

mod round_896 {
    use std::io::stdin;
    pub fn fun_a() {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("read line error");
        let t = line.trim().parse::<i32>().expect("line parse i32");
        for _ in 1..=t {
            line.clear();
            stdin().read_line(&mut line).expect("read line error");
            let n = line.trim().parse::<i32>().expect("line parse i32");
            stdin().read_line(&mut line).expect("read line error");
            if n & 1 == 0 {
                println!("2\n1 {0}\n1 {0}", n);
            } else {
                println!("4\n1 {1}\n1 {1}\n{1} {0}\n{1} {0}", n, n - 1);
            }
        }
    }
    pub fn fun_b() {
        
    }
}

fn main() {
    round_896::fun_a()
}
