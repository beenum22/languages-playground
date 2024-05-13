use std::sync::Mutex;
use lazy_static::lazy_static;

fn tree_recursion(n: u16, depth: usize) {
    if n > 0 {
        println!("{:indent$}└─ n={}", "", n, indent=depth*4);
        tree_recursion(n-1, depth+1);
        tree_recursion(n-1, depth+1);
    }
}

fn sum(n: u16, mut depth: usize) -> u16 {
    if depth == 0 {
        println!("{:indent$}sum({})", "", n, indent=depth);
        depth = depth + 1;
    }
    if n == 0 {
        return 0;
    }
    depth = depth + 1;
    println!("{:indent$}└─ sum({}-1)", "", n, indent=depth);
    let out: u16 =  sum(n-1,depth) + n;
    println!("{:indent$}└─ sum({}-1) = {} + {} = {}", "", n, out - n, n, out, indent=depth);
    return out
}

fn factorial(n: u16, mut depth: usize) -> u16 {
    if depth == 0 {
        println!("{:indent$}factorial({})", "", n, indent=depth);
        depth = depth + 1;
    }
    if n == 0 {
        return 1;
    }
    depth = depth + 1;
    println!("{:indent$}└─ factorial({}-1)", "", n, indent=depth);
    let out: u16 = factorial(n-1, depth) * n;
    println!("{:indent$}└─ factorial({}-1) = {} * {} = {}", "", n, out / n, n, out, indent=depth);
    return out
}

fn power(m: u16, n: u16, mut depth: usize) -> u16 {
    if depth == 0 {
        println!("{:indent$}power({}, {})", "", m, n, indent=depth);
        depth = depth + 1;
    }
    if n == 0 {
        return 1;
    }
    depth = depth + 1;
    println!("{:indent$}└─ power({}, {}-1)", "", m, n, indent=depth);
    let out: u16 = crate::power(m, n-1, depth) * m;
    println!("{:indent$}└─ power({}, {}-1) = {} * {} = {}", "", m, n, out / m, m, out, indent=depth);
    return out
}

fn taylors_series(x: u16, n: u16, power: &Mutex<f64>, factorial: &Mutex<f64>, mut depth: usize) -> f64 {
    let r_out: f64;
    let out: f64;

    if depth == 0 {
        println!("{:indent$}taylors_series({}, {})", "", x, n, indent=depth);
        depth = depth + 1;
    }

    if n == 0 {
        return 1.0;
    }

    depth = depth + 1;
    println!("{:indent$}└─ taylors_series({}, {}-1)", "", x, n, indent=depth);

    r_out = taylors_series(x, n-1, power, factorial, depth);
    let mut p = power.lock().unwrap();
    let mut f = factorial.lock().unwrap();
    *p = *p * x as f64;
    *f = *f * n as f64;
    out = r_out + *p / *f;
    println!("{:indent$}└─ taylors_series({}, {}-1) = {} + {}/{} = {}", "", x, n, r_out, p, f, out, indent=depth);
    return out;
}

fn taylors_series_horner_rule(x: u16, n: u16, sum: &Mutex<f64>, mut depth: usize) -> f64 {
    let out: f64;

    if depth == 0 {
        println!("{:indent$}taylors_series_horner_rule({}, {})", "", x, n, indent=depth);
        depth = depth + 1;
    }

    depth = depth + 1;

    {
        let mut s = sum.lock().unwrap();
        if n == 0 {
            return *s;
        }
        println!("{:indent$}└─ s = 1 + {}*{}/{} = {}", "", x, *s, n, 1.0 + (x as f64 * *s / n as f64), indent=depth);
        *s = 1.0 + (x as f64 * *s / n as f64);
    }


    println!("{:indent$}└─ taylors_series_horner_rule({}, {}-1)", "", x, n, indent=depth);
    out = taylors_series_horner_rule(x, n-1, sum, depth);
    println!("{:indent$}└─ taylors_series_horner_rule({}, {}-1) = {}", "", x, n, out, indent=depth);
    return out
}

fn main() {
    let m: u16 = 2;
    let n: u16 = 3;

    println!("Visualizing Tree Recursion where n={}!", n);
    tree_recursion(n, 0);
    println!("----");

    println!("Calculating sum of n numbers using Recursion where n={}!", n);
    let sum: u16 = sum(n, 0);
    println!("Sum of n numbers where n={} is {}", n, sum);
    println!("----");

    println!("Calculating factorial of n using Recursion where n={}!", n);
    let fact: u16 = factorial(n, 0);
    println!("Factorial of n where n={} is {}", n, fact);
    println!("----");

    println!("Calculating m power of n using Recursion where m={} and n={}!", m, n);
    let power: u16 = power(m, n, 0);
    println!("m power of n where m={} and n={} is {}", m, n, power);
    println!("----");

    let x: u16 = 1;
    let n: u16 = 5;
    // Additional pieces to implement C++ static variables like behaviour in Rust for Taylor's Series
    lazy_static! {
        static ref TS_POWER: Mutex<f64> = Mutex::new(1.0);
        static ref TS_FACTORIAL: Mutex<f64> = Mutex::new(1.0);
    }
    println!("Calculating Taylor's Series using Recursion where x={} and n={}!", x, n);
    let ts: f64 = taylors_series(x, n, &TS_POWER, &TS_FACTORIAL, 0);
    println!("Taylor's Series where x={} and n={} is {}", x, n, ts);
    println!("----");

    // Additional pieces to implement C++ static variables like behaviour in Rust for Taylor's Series
    lazy_static! {
        static ref TS_HORNER_SUM: Mutex<f64> = Mutex::new(0.0);
    }
    println!("Calculating Taylor's Series using Horner's Rule where x={} and n={}!", x, n);
    let ts_horner: f64 = taylors_series_horner_rule(x, n, &TS_HORNER_SUM, 0);
    println!("Taylor's Series using Horner's Rule where x={} and n={} is {}", x, n, ts_horner);
    println!("----");
}
