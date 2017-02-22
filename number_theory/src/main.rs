///手动实现 最小公倍数、最大公约数 等功能
///其实Rust已经有相关实现的std::num
/// 作者:yfgeek

// 两数交换
// mem::swap(&mut x, &mut y);
fn swap(x: &mut i32, y: &mut i32){
   let tmp = *x;
    *x = *y;
    *y = tmp;
}

// 最小公倍数
// lcm<T>(x: T, y: T) -> T
fn lcm(mut x : i32, mut y: i32) ->i32{
    if x<y{
        swap(&mut x,&mut y);
    }
    let mut max = x;
    while max % y != 0{
        max = max+1;
    }
    max
}

// 最大公约数
// gcd<T>(x: T, y: T) -> T
fn gcd(x : i32, y: i32) ->i32{
    if y==0{ x }
    else{gcd(y,x%y)}
}

fn main() {
    let mut m = 24;
    let mut n = 36;
    swap(&mut m,&mut n);
    println!("{}和{}交换结果是 :{:?}",m,n,(m,n));
    println!("{}和{}的最大公约数是 :{}",m,n,gcd(m,n));
    println!("{}和{}的最小公倍数是 :{}",m,n,lcm(m,n));
}
