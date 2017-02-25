/// 手动实现：
/// * 开平方
/// * 最小公倍数
/// * 最大公约数
/// * 判断素数
/// * 求n以内的所有素数
///
/// 其实Rust已经有相关实现的函数，这里只是复习算法
/// 作者: @yfgeek
// 开平方
fn sqrt(n: i32) -> f32 {
    let a: f32 = n as f32;
    let mut x: f32 = 1.0;
    let mut i = 0;
    while i < n {
        x = 0.5 * ( x + a / x );
        i = i+ 1;
    }
    x
}

// 两数交换
fn swap(x: &mut i32, y: &mut i32){
   let tmp = *x;
    *x = *y;
    *y = tmp;
}

// 最小公倍数
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
fn gcd(x : i32, y: i32) ->i32{
    if y==0{ x }
    else{gcd(y,x%y)}
}

// 判断素数
fn prime(x : i32) ->bool{
    let mut result = true;
    let b = sqrt(x).trunc() as i32;
    for i in 2..b{
        if x%i==0{
        result = false
        }
    }
    result
}

// 求n以内的素数
fn calc_prime(n : i32) ->Vec<i32>{
    let mut v = vec![];
    let mut i = 3;
    if n>2{
       v.push(2);
    }
    while i <=n{
        if prime(i){
            v.push(i);
        }
        i=i+2;
    }
    v
}
fn main() {
    let mut m = 24;
    let mut n = 36;
    let p = 100;
    swap(&mut m,&mut n);
    println!("{}和{}交换结果是 :{:?}",m,n,(m,n));
    println!("{}和{}的最大公约数是 :{}",m,n,gcd(m,n));
    println!("{}和{}的最小公倍数是 :{}",m,n,lcm(m,n));
    println!("{}的开根号结果是 :{}",m,sqrt(m));
    println!("{}是素数吗？ {}",m,prime(m));
    println!("{}是素数吗？ {}",3,prime(3));
    println!("{}以内的素数如下：",p);
    let array = calc_prime(p);
    for i in &array{
        print!("{} ",i);
    }

}
