# hello-diesel

diesel是一个rust数据库orm

```rust
// 已经创建了rust项目并且导入了diesel，在项目根目录

// 设置数据库rul

echo DATABASE_URL=mysql://root:newpassword@192.168.1.199:3306/hello .env

// 项目初始

diesel setup

// 创建一个同步项目，用于和数据库同步表

diesel migration generate create_posts

// 开始同步，根据up.sql的语句创建表，并同步数据库产生了一个描述文件schema.rs

diesel migration run

// 或者不同步数据库,只输出描述文件
diesel print-schema
```

1.普通连接

2.线程池连接
