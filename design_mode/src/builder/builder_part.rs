#[derive(Debug,Clone)]
pub struct Product {
   pub parts: Vec<String>,
}

impl Product {
    pub fn new() -> Product {
        Product {
            parts: Vec::new(),
        }
    }

    pub fn list_parts(&self){
        let parts_list = String::from(" parts ");
        println!("{0}{1}{0}", "*".repeat(10), parts_list);
        for v in &self.parts {
            println!("part is {:?}",v);
        }
        println!("{0}{1}{0}", "*".repeat(10), "*".repeat(parts_list.len()));
    }
}

// 抽象共有的构建方法
pub trait Builder {

     fn create_product_part1(&mut self);
     fn create_product_part2(&mut self);
     fn create_product_part3(&mut self);
     fn get_product(&mut self) -> Product;
  
}
// 构建Product方式1
pub struct CreateBuilder1{
    pub product: Product,
}

impl CreateBuilder1{
    pub fn new() -> CreateBuilder1 {
        CreateBuilder1{
            product: Product::new(),
        }
    }
}

impl Builder for CreateBuilder1 {

    fn create_product_part1(&mut self){
        self.product.parts.push("builder1 add part 1".to_string());
    }

    fn create_product_part2(&mut self){
        self.product.parts.push("builder1 add part 2".to_string());
    }

    fn create_product_part3(&mut self){
        self.product.parts.push("builder1 add part 3".to_string());
    }

    fn get_product(&mut self) -> Product{
        let product = self.product.clone();
        self.product = Product::new();
        product
    }
}

// 构建Product方式2
pub struct CreateBuilder2 {
    pub product: Product,
}

impl CreateBuilder2 {
    pub fn new() -> CreateBuilder2 {
        CreateBuilder2{
            product: Product::new(),
        }
    }
}

impl Builder for CreateBuilder2 {

    fn create_product_part1(&mut self){
        self.product.parts.push("builder2 add part 1".to_string());
    }

    fn create_product_part2(&mut self){
        self.product.parts.push("builder2 add part 2".to_string());
    }

    fn create_product_part3(&mut self){
        self.product.parts.push("builder2 add part 3".to_string());
    }

    fn get_product(&mut self) -> Product{
        let product = self.product.clone();
        self.product = Product::new();
        product
    }
}

/**
 * The Director is only responsible for executing the building steps in a
 * particular sequence. It is helpful when producing products according to a
 * specific order or configuration. Strictly speaking, the Director class is
 * optional, since the client can control builders directly.
 */
pub struct Direct {
  pub builder: Box<dyn Builder>
}

impl Direct {

  pub fn new(builder: Box<dyn Builder>) -> Direct {
        Direct {
            builder
        }
    }

  pub fn construct(&mut self) {
        self.builder.create_product_part1();
        self.builder.create_product_part2();
        self.builder.create_product_part3();
    }
}

