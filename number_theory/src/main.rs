

///手动实现 最小公倍数、最大公约数 等功能
///其实Rust已经有相关实现的std::num
/// 作者:yfgeek

// 两数交换
// mem::swap(&mut x, &mut y);
fn swap(x: i32, y: i32)->(i32,i32){
    (y,x)
}

// 最小公倍数
// lcm<T>(x: T, y: T) -> T
fn lcm(x : i32, y: i32) ->i32{
    if x<y{
        swap(x,y);
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
    println!("24和36交换结果是 :{:?}",swap(24,36));
    println!("24和36的最大公约数是 :{}",gcd(24,36));
    println!("24和36的最小公倍数是 :{}",lcm(24,36));
}
