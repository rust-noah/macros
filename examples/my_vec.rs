use anyhow::Result;

// NOTE: Macro
// $(), * 是一个组合, 表示重复0次或多次, 匹配的是逗号分隔的表达式
// $x:expr 表示一个表达式, $x 表示分隔出来的表达式

#[macro_export]
macro_rules! my_vec {
    ($($x:expr), *) => {
        {
            // let mut temp_vec = Vec::new();
            // $(
            //     temp_vec.push($x);
            // )*
            // temp_vec
            vec![$($x), *]
        }
    }
}

// NOTE:
// ($($x:expr), + $(,)?) => {{
//     <[_]>::into_vec(Box::new([$($x), +]))
// }};
// 1. $($x:expr), + 匹配一个或多个表达式, 用逗号分隔
// 2. $(,)? 匹配一个逗号, 问号表示可选


#[macro_export]
macro_rules! my_vec2 {
    () => {
        Vec::new()
    };
    ($elem:expr; $n:expr) => {
        std::vec::from_elem($elem, $n)
    };
    ($($x:expr), + $(,)?) => {{
        <[_]>::into_vec(Box::new([$($x), +]))
    }};
}
fn main() -> Result<()> {
    let v = my_vec!(1, 2, 3);
    println!("{:?}", v);
    Ok(())
}
