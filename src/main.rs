extern crate rug;
use crate::rug::Assign;
use substring::Substring;


#[derive(Clone,Debug)]
struct Typ {
    val: rug::Rational,
    str: String,
}


fn op_plus(x: &Typ, y: &Typ) -> Typ {
    Typ { val: x.val.clone() + y.val.clone(), str: "(".to_owned() + &x.str + "+" + &y.str + ")"}
}

fn op_prod(x: &Typ, y: &Typ) -> Typ {
    Typ { val: x.val.clone() * y.val.clone(), str: "(".to_owned() + &x.str + "*" + &y.str + ")"}
}

fn op_minus(x: &Typ, y: &Typ) -> Typ {
    Typ { val: x.val.clone() - y.val.clone(), str: "(".to_owned() + &x.str + "-" + &y.str + ")"}
}

fn op_div(x: &Typ, y: &Typ) -> Typ {
    Typ { val: x.val.clone() / y.val.clone(), str: "(".to_owned() + &x.str + "/" + &y.str + ")"}
}

fn compute_all_operations(x: &Typ, y: &Typ) -> Vec<Typ> {
    let mut v_ret = vec![ op_plus(x, y), op_prod(x, y), op_minus(x, y), op_minus(x, y) ];
    if y.val != 0 {
        v_ret.push(op_div(x, y))
    }
    if x.val != 0 {
        v_ret.push(op_div(y, x))
    }
    v_ret
}


fn spann_all_expression(l_val: Vec<Typ>) -> Vec<Vec<Typ>> {
    let len = l_val.len();
    let mut l_ret = Vec::<Vec<Typ>>::new();
    for i in 0..len {
        for j in 0..len {
            if i < j {
//                println!("i={} j={}", i, j);
                let mut l_concat = Vec::<Typ>::new();
                for k in 0..len {
                    if k != i && k != j {
                        l_concat.push(l_val[k].clone())
                    }
                }
                let l_merge = compute_all_operations(&l_val[i], &l_val[j]);
                for x_m in l_merge {
                    let mut l_ent = l_concat.clone();
                    l_ent.push(x_m);
//                    println!("l_ent={:?}", l_ent);
                    l_ret.push(l_ent);
                }
            }
        }
    }
    l_ret
}


fn test_solution(l_val: &Vec<Typ>, target: &rug::Rational) -> Option<String> {
    for e_val in l_val {
        if e_val.val == target.clone() {
            return Some(e_val.str.clone());
        }
    }
    None
}


fn compute_expression(l_val: Vec<Typ>, target: rug::Rational) -> Option<String> {
    let mut l_ret = Vec::<Vec<Typ>>::new();
    let len = l_val.len();
    l_ret.push(l_val);
    for _ in 0..len {
        let mut l_new = Vec::<Vec<Typ>>::new();
        for e_ent in l_ret {
            for f_new in spann_all_expression(e_ent) {
                match test_solution(&f_new, &target) {
                    Some(estr) => return Some(estr),
                    None => l_new.push(f_new)
                }
            }
        }
        println!("|l_new|={}", l_new.len());
        l_ret = l_new.clone()
            
    }
    None
}


fn get_rational(estr: &String) -> rug::Rational {
    let my_int = estr.parse::<i32>().unwrap();
    let mut num = rug::Integer::new();
    let mut den = rug::Integer::new();
    num.assign(my_int);
    den.assign(1);
    rug::Rational::from((num,den))
}



fn insert_val(l_val: &mut Vec<Typ>, estr: String) {
    let r = get_rational(&estr);
    let val = Typ { val: r, str: estr };
    l_val.push(val)
}



fn main() {
    let target = std::env::args().nth(1).expect("no pattern given");
    let units  = std::env::args().nth(2).expect("no pattern given");
    let mid1   = std::env::args().nth(3).expect("no pattern given");
    let mid2   = std::env::args().nth(4).expect("no pattern given");
    //
    let mut l_val = Vec::<Typ>::new();
    let target = get_rational(&target);
    for i in 0..4 {
        let substr = units.substring(i, i+1);
        insert_val(&mut l_val, substr.to_string());
    }
    insert_val(&mut l_val, mid1);
    insert_val(&mut l_val, mid2);
    match compute_expression(l_val, target) {
        Some(estr) => println!("Found expression {}", estr),
        None => println!("No expression found")
    }
}

