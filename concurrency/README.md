
### 并发与异步 
添加依赖


#### 创建线程

```Rust 
pub fn spawn<F, T>(&'scope self, f: F) -> ScopedJoinHandle<'scope, T>
    where
        F: FnOnce() -> T + Send + 'scope,
        T: Send + 'scope,
    {
        Builder::new().spawn_scoped(self, f).expect("failed to spawn thread")
    }
```
F: 被约束为 FnOnce 实现 T Send ‘scope   

#### 等待线程


#### 线程同步

- 共享内存
- csp
- Actor

##### 共享内存

##### CSP
- channel
- mpsc
- oneshot



##### 


