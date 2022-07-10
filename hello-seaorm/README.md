##### sea orm 是一个支持异步的数据库orm

[https://www.sea-ql.org/SeaORM/docs/index]()

##### 数据库迁移(把表同步到数据库)

```
* cargo install sea-orm-cli
* sea-orm-cli migrate -h
* sea-orm-cli migrate init
```

##### [从数据库生成实体](https://www.sea-ql.org/sea-orm-tutorial/ch01-04-entity-generation.html#generate-entity-from-database)(把数据库的表同步到项目)

```
* sea-orm-cli generate entity \
    -u mysql://root:root@localhost:3306/hello \
    -o src/entities
// 或者已经.env设置了url
* sea-orm-cli generate entity -o src/entities
```
