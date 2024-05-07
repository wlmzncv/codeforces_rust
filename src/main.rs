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
        //从字符串中解析出整数 以空格分隔
        fn help_1(t_1: &mut Vec<i64>, t_2: &String) {
            let mut i = 0;
            for item in t_2.split_whitespace() {
                t_1[i] = item.parse::<i64>().expect("parse i64");
                i += 1;
            }
        }
        //计算两城市的距离
        fn help_2(t_1: &[i64;2], t_2: &[i64;2]) -> i64 {
            (t_1[0] - t_2[0]).abs() + (t_1[1] - t_2[1]).abs()
        }
        let mut line = String::new();
        stdin().read_line(&mut line).expect("read line error");
        let t = line.trim().parse::<i64>().expect("line parse i64");
        for _ in 1..=t {
            line.clear();
            stdin().read_line(&mut line).expect("read line error");
            let mut nkab: Vec<i64> = vec![0, 0, 0, 0];
            let mut city_pos: Vec<[i64;2]> = Vec::new();
            help_1(&mut nkab, &line);
            for _ in 0..nkab[0] {
                line.clear();
                stdin().read_line(&mut line).expect("read line error");
                let mut c_pos:[i64;2]=[0;2];
                {
                    let mut i=0;
                    for item in line.split_whitespace() {
                        c_pos[i] = item.parse::<i64>().expect("parse i64");
                        i+=1;
                    }
                }
                city_pos.push(c_pos);
            }
            let mut mini_cost: i64;
            mini_cost = help_2(
                &city_pos[(nkab[2] - 1) as usize],
                &city_pos[(nkab[3] - 1) as usize],
            );
            let mut min_s:i64 = i64::MAX/2;
            let mut min_t:i64 = i64::MAX/2;
            for i in 0..nkab[1] {
                let c_1 = help_2(&city_pos[(i) as usize], &city_pos[(nkab[3] - 1) as usize]);
                if c_1<min_s{
                    min_s = c_1;
                }
                let c_2 = help_2(&city_pos[(i) as usize], &city_pos[(nkab[2] - 1) as usize]);
                if c_2<min_t{
                    min_t = c_2;
                }
                if min_s + min_t < mini_cost {
                    mini_cost = min_s + min_t;
                }
            }
            println!("{}", mini_cost);
        }
    }
    pub fn fun_c(){
        //生成长度为m的全排列作为矩阵的一行 i指示0值的下标
        fn help_1(m:u32, i:u32){
            let mut t0 = m-i;
            if t0==m{
                t0 = 0;
            }
            for i in 0..m-1{
                let mut t1 = t0+i;
                if t1>=m{
                    t1 = t1-m;
                }
                print!("{} ",t1);
            }
            t0 = t0+m-1;
            if t0>=m{
                t0 = t0-m;
            }
            println!("{}",t0);
        }
        let mut line = String::new();
        stdin().read_line(&mut line).expect("read line error");
        let t = line.trim().parse::<u32>().expect("line parse i64");
        for _ in 1..=t {
            line.clear();
            stdin().read_line(&mut line).expect("read line error");
            let mut i=0;
            let mut n = 1u32;
            let mut m = 1u32;
            for item in line.split_whitespace() {
                let t0 = item.parse::<u32>().expect("parse i64");
                if i==0 {n=t0;}
                else {m=t0;}
                i += 1;
            }
            if m==1{
                println!("{}",0);
                for _ in 1..=n{
                    println!("{}",0);
                }
            }
            else if m>n{
                println!("{}",n+1);
                for i in 0..n{
                    help_1(m, i);
                }
            }
            else{
                println!("{}",m);
                for i in 0..m-1{
                    help_1(m, i);
                }
                for _ in m-1..n{
                    help_1(m, m-2);
                }
            }
        }
    }
}

fn main() {
    round_896::fun_c()
}
