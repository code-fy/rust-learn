use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

pub struct ObjectPool<T> {
    pool: Arc<Mutex<VecDeque<T>>>,
    create_fn: Box<dyn Fn() -> T>,
    max_size: usize,
}

impl<T> ObjectPool<T> {
    pub fn new<F>(create_fn: F, max_size: usize) -> Self
    where
        F: Fn() -> T + 'static,
    {
        ObjectPool {
            pool: Arc::new(Mutex::new(VecDeque::new())),
            create_fn: Box::new(create_fn),
            max_size,
        }
    }

    pub fn get(&self) -> T {
        let mut pool = self.pool.lock().unwrap();
        if let Some(object) = pool.pop_front() {
            object // 如果池中有可用对象，直接返回
        } else {
            (self.create_fn)() // 否则创建一个新的对象
        }
    }

    pub fn put(&self, object: T) {
        let mut pool = self.pool.lock().unwrap();
        if pool.len() < self.max_size {
            pool.push_back(object); // 将对象放回池中
        }
        // 如果池已经满了，可以选择丢弃对象或者其他处理
    }
}

fn apply<F>(f:F)
where
    F:Fn(){
        f();
    }


// 使用示例
struct MyObject {
    value: i32,
}

fn make_incrementer(increment:i32)->Box<dyn Fn(i32) -> i32>{
    Box::new(move |x| x+increment)
}

fn fake_closer0(n:i32)->i32{
    n
}
fn fake_closer(n:i32)->i32{
    let s = n + fake_closer0(10);
    s

}

fn main() {
    let demo = fake_closer(5);
    println!("{}",demo);
    let increment_by_5 = make_incrementer(5);
    println!("{}",increment_by_5(10));
    let closure = || println!("d s b ");
    apply(closure);
    let pool = ObjectPool::new(|| MyObject { value: 0 }, 10);

    // 从池中获取对象
    let mut obj1 = pool.get();
    // 修改对象
    obj1.value = 42;

    // 使用完后将对象放回池中
    pool.put(obj1);

    // 再次获取对象
    let mut obj2 = pool.get();

    obj2.value = 90;
    pool.put(obj2);
    // println!("{}",obj1.value);
    println!("{}",pool.get().value);
    // println!("{}",pool.len());

    // println!("Object value: {}", obj2.value); // 可能是 0，因为 obj1 被放回池中
}

