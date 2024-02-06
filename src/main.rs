use once_cell::sync::Lazy;


trait Animal: Send + Sync {
    fn meow(&self, num:i32)  -> std::pin::Pin<Box<dyn '_ + std::future::Future<Output = i32>>>;
}

struct Cat{
    name: String
}
struct Dog{
    name: String
}


impl Cat{
    fn introduce(&self) {
        println!("I am {}", self.name);
    }
}

impl Dog{
    fn introduce(&self) {
        println!("I am {}", self.name);
    }
}

impl Animal for Cat {

    fn meow(&self, num:i32)  -> std::pin::Pin<Box<dyn '_ + std::future::Future<Output = i32>>>{
        println!("Cat meow");
        Box::pin(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;
            println!("Cat meow2");
            self.introduce();
            100 + num
        })
    }
}

impl Animal for Dog {
    fn meow(&self, num:i32)  -> std::pin::Pin<Box<dyn '_ + std::future::Future<Output = i32>>>{
        println!("Dog meow");
        Box::pin(async move{
            tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;
            println!("Dog meow2");
            self.introduce();
            200 + num
        })
    }
}


#[cfg(feature = "cat")]
static ANIMAL: Lazy<Box<dyn Animal>> = Lazy::new(|| {
    Box::new(Cat{name: "Kitty".to_string()})
});

#[cfg(feature = "dog")]
static ANIMAL: Lazy<Box<dyn Animal>> = Lazy::new(|| {
    Box::new(Dog{name: "Doggy".to_string()})
});


#[tokio::main]
async fn main() {

    println!("Hello, world!");
    ANIMAL.meow(10).await;
}
