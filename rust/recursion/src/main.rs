use lazy_static::lazy_static;
use std::sync::Mutex;

fn tree_recursion(n: u16, mut depth: Option<usize>) {
    if n > 0 {
        depth = match depth {
            Some(val) => {
                println!("{:indent$}└─ n={}", "", n, indent = val);
                Some(val + 1)
            }
            None => None,
        };
        tree_recursion(n - 1, depth);
        tree_recursion(n - 1, depth);
    }
}

fn sum(n: u16, mut depth: Option<usize>) -> u16 {
    depth = match depth {
        Some(val) => {
            println!("{:indent$}└─ sum({})", "", n, indent = val);
            Some(val + 1)
        }
        None => None,
    };

    if n == 0 {
        return 0;
    }

    let out: u16 = sum(n - 1, depth) + n;

    match depth {
        Some(_) => {
            println!(
                "{:indent$}└─ sum({})={} + {} = {}",
                "",
                n - 1,
                out - 1,
                n,
                out,
                indent = depth.unwrap()
            );
        }
        None => (),
    };

    return out;
}

fn factorial(n: u16, mut depth: Option<usize>) -> u16 {
    depth = match depth {
        Some(val) => {
            println!("{:indent$}└─ factorial({})", "", n, indent = val);
            Some(val + 1)
        }
        None => None,
    };

    if n == 0 {
        return 1;
    }

    let out: u16 = factorial(n - 1, depth) * n;

    match depth {
        Some(_) => {
            println!(
                "{:indent$}└─ factorial({})={} * {} = {}",
                "",
                n - 1,
                out / n,
                n,
                out,
                indent = depth.unwrap()
            );
        }
        None => (),
    };

    return out;
}

fn power(m: u16, n: u16, mut depth: Option<usize>) -> u16 {
    depth = match depth {
        Some(val) => {
            println!("{:indent$}└─ power({}, {})", "", m, n, indent = val);
            Some(val + 1)
        }
        None => None,
    };

    if n == 0 {
        return 1;
    }

    let out: u16 = power(m, n - 1, depth) * m;

    match depth {
        Some(_) => {
            println!(
                "{:indent$}└─ power({}, {})={} * {} = {}",
                "",
                m,
                n - 1,
                out / m,
                m,
                out,
                indent = depth.unwrap()
            );
        }
        None => (),
    };

    return out;
}

fn taylors_series(
    x: u16,
    n: u16,
    power: &Mutex<f64>,
    factorial: &Mutex<f64>,
    mut depth: Option<usize>,
) -> f64 {
    let r_out: f64;
    let out: f64;

    depth = match depth {
        Some(val) => {
            println!(
                "{:indent$}└─ taylors_series({}, {})",
                "",
                x,
                n,
                indent = val
            );
            Some(val + 1)
        }
        None => None,
    };

    if n == 0 {
        return 1.0;
    }

    r_out = taylors_series(x, n - 1, power, factorial, depth);
    let mut p = power.lock().unwrap();
    let mut f = factorial.lock().unwrap();
    *p = *p * x as f64;
    *f = *f * n as f64;
    out = r_out + *p / *f;

    match depth {
        Some(_) => {
            println!(
                "{:indent$}└─ taylors_series({}, {})={} + {}/{} = {}",
                "",
                x,
                n - 1,
                r_out,
                p,
                f,
                out,
                indent = depth.unwrap()
            );
        }
        None => (),
    };

    return out;
}

fn taylors_series_horner_rule(x: u16, n: u16, sum: &Mutex<f64>, mut depth: Option<usize>) -> f64 {
    let out: f64;

    depth = match depth {
        Some(val) => {
            println!(
                "{:indent$}└─ taylors_series_horner_rule({}, {})",
                "",
                x,
                n,
                indent = val
            );
            Some(val + 1)
        }
        None => None,
    };

    {
        let mut s = sum.lock().unwrap();
        if n == 0 {
            return *s;
        }
        match depth {
            Some(_) => {
                println!(
                    "{:indent$}└─ s = 1 + {}*{}/{} = {}",
                    "",
                    x,
                    *s,
                    n,
                    1.0 + (x as f64 * *s / n as f64),
                    indent = depth.unwrap()
                );
            }
            None => (),
        };

        *s = 1.0 + (x as f64 * *s / n as f64);
    }

    out = taylors_series_horner_rule(x, n - 1, sum, depth);

    match depth {
        Some(_) => {
            println!(
                "{:indent$}└─ taylors_series_horner_rule({}, {}) = {}",
                "",
                x,
                n - 1,
                out,
                indent = depth.unwrap()
            );
        }
        None => (),
    };

    return out;
}

fn main() {
    let visualize: bool = true;
    let depth: Option<usize>;

    if visualize == true {
        depth = Some(0)
    } else {
        depth = None
    }

    let tree_n: u16 = 4;
    println!("Visualizing Tree Recursion where n={}!", tree_n);
    tree_recursion(tree_n, depth);
    println!("----");

    let sum_n: u16 = 5;
    println!(
        "Calculating sum of n numbers using Recursion where n={}!",
        sum_n
    );
    let sum: u16 = sum(sum_n, depth);
    println!("Sum of n numbers where n={} is {}", sum_n, sum);
    println!("----");

    let fact_n: u16 = 5;
    println!(
        "Calculating factorial of n using Recursion where n={}!",
        fact_n
    );
    let fact: u16 = factorial(fact_n, depth);
    println!("Factorial of n where n={} is {}", fact_n, fact);
    println!("----");

    let power_m: u16 = 2;
    let power_n: u16 = 3;
    println!(
        "Calculating m power of n using Recursion where m={} and n={}!",
        power_m, power_n
    );
    let power: u16 = power(power_m, power_n, depth);
    println!(
        "m power of n where m={} and n={} is {}",
        power_m, power_n, power
    );
    println!("----");

    let ts_x: u16 = 1;
    let ts_n: u16 = 3;
    // Additional pieces to implement C++ static variables like behaviour in Rust for Taylor's Series
    lazy_static! {
        static ref TS_POWER: Mutex<f64> = Mutex::new(1.0);
        static ref TS_FACTORIAL: Mutex<f64> = Mutex::new(1.0);
    }
    println!(
        "Calculating Taylor's Series using Recursion where x={} and n={}!",
        ts_x, ts_n
    );
    let ts: f64 = taylors_series(ts_x, ts_n, &TS_POWER, &TS_FACTORIAL, depth);
    println!("Taylor's Series where x={} and n={} is {}", ts_x, ts_n, ts);
    println!("----");

    // Additional pieces to implement C++ static variables like behaviour in Rust for Taylor's Series
    lazy_static! {
        static ref TS_HORNER_SUM: Mutex<f64> = Mutex::new(1.0);
    }
    println!(
        "Calculating Taylor's Series using Horner's Rule where x={} and n={}!",
        ts_x, ts_n
    );
    let ts_horner: f64 = taylors_series_horner_rule(ts_x, ts_n, &TS_HORNER_SUM, depth);
    println!(
        "Taylor's Series using Horner's Rule where x={} and n={} is {}",
        ts_x, ts_n, ts_horner
    );
    println!("----");
}
