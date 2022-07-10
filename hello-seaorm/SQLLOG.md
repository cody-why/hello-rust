### seaorm 输出sql去掉文件名，sqlx输出sql去掉格式化

配置了tracing_subscriber输出文件名,但是seaorm输出的文件名是内部的文件名,不是我想要的

##### 1.输出sql

seaorm要想输出sql, features 加上 "debug-print"

在源码

~/.cargo/registry/src/[github.com-1ecc6299db9ec823/sea-orm-0.8.0/src/util.rs](http://github.com-1ecc6299db9ec823/sea-orm-0.8.0/src/util.rs)

19行

debug_print!

把tracing::debug! 改为 tracing::info3!

第4步改tracing的源码，增加info3!，不想要文件名


##### 2.如果不想输出select,只输出update、insert

~/.cargo/registry/src/[github.com-1ecc6299db9ec823/sea-orm-0.8.0/src/driver/sqlx_mysql.rs](http://github.com-1ecc6299db9ec823/sea-orm-0.8.0/src/driver/sqlx_mysql.rs)

src/driver/sqlx_mysql.rs

78行：

在方法query_one、query_all、

注释掉debug_print!

```rust
// 清掉指定的p

cargo clean -p sea-orm

//检查通过再build,节省时间

cargo check

cargo build
```


##### 3.sqlx 输出sql去掉格式化

执行cargo clean -p sqlx,再cargo build,按输出信息找到sea用到的sqlx的版本,因为logger用了sqlformat,

~/.cargo/registry/src/github.com-1ecc6299db9ec823/sqlx-core-5.1.13/src/logger.rs

屏蔽它，直接返回sql：

47行：

```rust
format!(

"\n{}",

self.sql

// sqlformat::format(

// &self.sql,

// &sqlformat::QueryParams::None,

// sqlformat::FormatOptions::default()

// )

)
```


82行，我只要2个单词返回，select XX等

parse_query_summary

.take(4) 改为 .take(2)

##### 4.tracing不想输出sea-orm的文件名，很长

因为ctracing的Metadata带有file信息，~/.cargo/registry/src/[github.com-1ecc6299db9ec823/tracing-core-0.1.28/src/lib.rs](http://github.com-1ecc6299db9ec823/tracing-core-0.1.28/src/lib.rs)

218行macro_rules! metadata里有生成代码,这里就不改了

修改后重新编译:

cargo clean -p tracing

增加一个新的macro代替info,debug

~/.cargo/registry/src/[github.com-1ecc6299db9ec823/tracing-0.1.35/src/macros.rs](http://github.com-1ecc6299db9ec823/tracing-0.1.35/src/macros.rs)

加入以下代码

```rust
/// info3! for sea-rom, no file name, no parent.

#[macro_export]

macro_rules! info3 {

($($arg:tt)+) => (

$crate::event3!(

$crate::Level::INFO,

$($arg)+

)

);

}

/// debug3! for sea-rom, no file name, no parent.

#[macro_export]

macro_rules! debug3 {

($($arg:tt)+) => (

$crate::event3!(

$crate::Level::Debug,

$($arg)+

)

);

}

/// event3! for sea-rom, no file name, no parent.

#[macro_export]

macro_rules! event3{

// (target: $target:expr,$lvl:expr, $($fields:tt)* )=> ({

( $lvl:expr, $($fields:tt)* )=> ({

//let lvl = $crate::Level::INFO;

use $crate::__macro_support::Callsite as _;

static CALLSITE: $crate::callsite::DefaultCallsite = $crate::callsite3! {

name: $crate::__macro_support::concat!(

"event ",

file!(),

":",

line!()

),

kind: $crate::metadata::Kind::EVENT,

target: module_path!(),//$target,

level: $lvl,

fields: $($fields)*

};

let enabled = $crate::level_enabled!($lvl) && {

let interest = CALLSITE.interest();

!interest.is_never() && $crate::__macro_support::__is_enabled(CALLSITE.metadata(), interest)

};

if enabled {

(|value_set: $crate::field::ValueSet| {

let meta = CALLSITE.metadata();

// event with contextual parent

$crate::Event::dispatch(

meta,

&value_set

);

$crate::__tracing_log!(

$lvl,

CALLSITE,

&value_set

);

})($crate::valueset!(CALLSITE.metadata().fields(), $($fields)*));

} else {

$crate::__tracing_log!(

$lvl,

CALLSITE,

&$crate::valueset!(CALLSITE.metadata().fields(), $($fields)*)

);

}

});

($lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => (

$crate::event3!(

$lvl,

{ message = format_args!($($arg)+), $($fields)* }

)

);

}

/// Constructs a new static callsite for a span or event.

/// no file name

#[doc(hidden)]

#[macro_export]

macro_rules! callsite3 {

(

name: $name:expr,

kind: $kind:expr,

target: $target:expr,

level: $lvl:expr,

fields: $($fields:tt)*

) => {{

static META: $crate::Metadata<'static> = {

$crate::metadata3! {

name: $name,

target: $target,

level: $lvl,

fields: $crate::fieldset!( $($fields)* ),

callsite: &CALLSITE,

kind: $kind,

}

};

$crate::callsite::DefaultCallsite::new(&META)

}};

}

/// no file name

#[macro_export]

macro_rules! metadata3 {

(

name: $name:expr,

target: $target:expr,

level: $level:expr,

fields: $fields:expr,

callsite: $callsite:expr,

kind: $kind:expr,

) => {

$crate::metadata::Metadata::new(

$name,

$target,

$level,

//Some(file!()),

Some(module_path!()),

Some(line!()),

Some(module_path!()),

// $crate::field::FieldSet::new($fields, $crate::identify_callsite!($callsite)),

$crate::field::FieldSet::new($fields, $crate::callsite::Identifier($callsite)),

$kind,

)

};

}
```
