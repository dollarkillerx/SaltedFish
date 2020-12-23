# SaltedFish

### 任务
- [ ] 文件区块设计(文件系统设计)
- [ ] 索引设计(mem + desk)
- [ ] 写引擎
- [ ] 全表搜索引擎
- [ ] 修改(注意 写区块 当写指针和修改指针 同时存在时需加锁)

### 文件区块设计
- 数据区块
    - 5M ~ 20M  可选配置
- 文件排布
```
- 存储根目录
    - database
        - collection
            - document
```

### 需要实现的
- [ ] 持久化 database collection 关系