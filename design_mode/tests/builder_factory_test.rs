include!("../src/main.rs");

use factory::singleton;
use builder::builder_mode::Object;
use builder::builder_part::{Direct,CreateBuilder1,CreateBuilder2};
use split::split_mode::{SplitObject,SplitObjectMore};
use factory::abstract_factory::Application;
use factory::factory::{ShapeFactory,ShapType};

#[cfg(test)]
mod tests {

    // 创建型模式-工厂模式
    // ==============================builder模式=============================
    #[test]
    fn chain_builder_test(){
    
        let object = super::Object::new(String::from("构建object对象")).id(23).email(String::from("w1999wtw3537@sina.com")).build();
        let object2 = super::Object::new("构建object对象".to_string()).id(24).email("w1989wtw3537@sina.com".to_string()).build();
        println!("builder模式构建对象:{:?}",object);
        println!("builder模式构建对象2:{:?}",object2);

        assert_eq!(object.name,object2.name);

    }

    // ===============================split模式===================================
    #[test]
    fn split_builder_test(){
        //消耗掉Object，将其成员Move给ObjectMore，同时对data做进一步的细化转换，保持最小内存分配开销
        let name = String::from("split模式");
        let data = vec![10,20,30];
        let sobject = super::SplitObject{name,data};
        
        let (name,data) = sobject.split();
        let (id,email) = super::SplitObject::decode(data);
        let sobject_more = super::SplitObjectMore{name,id:Some(id),email:Some(email)};
        println!("split模式构建对象:{:?}",sobject_more);
    }


    // ==============================builder part模式========================
    #[test]
    fn part_builder_test(){
    
        // 方式1构造对象数据
        let mut direct1 = super::Direct::new(Box::new(super::CreateBuilder1::new()));
        direct1.construct();
        let product1 = direct1.builder.get_product();
        product1.list_parts();
        // 方式2构造对象数据
        let mut direct2 = super::Direct::new(Box::new(super::CreateBuilder2::new()));
        direct2.construct();
        let product2 = direct2.builder.get_product();
        product2.list_parts();
    }

   // ==============================abstract builder 模式========================
   #[test]
   fn abstract_factory_builder_test(){
        // WIN系统
        let win_factory = super::Application::new_gui_factory("WIN");
        let win_button = win_factory.create_button();
        let win_check_box = win_factory.create_check_box();
        win_button.paint();
        win_check_box.paint();
        let mac_factory = super::Application::new_gui_factory("MAC");
        let mac_button = mac_factory.create_button();
        let mac_check_box = mac_factory.create_check_box();
        mac_button.paint();
        mac_check_box.paint();
   }
   // ==============================简单工厂模式==================================
   #[test]
   fn sample_factory_test(){
    let rectangle_factory = super::ShapeFactory::new_shape(&super::ShapType::RecTangle);
    rectangle_factory.draw();
    let circle_factory = super::ShapeFactory::new_shape(&super::ShapType::Circle);
    circle_factory.draw();
   }
   //  ==============================单例工厂模式==================================
   #[test]
   fn singleton_factory_test(){
    let config1 = super::singleton::get_config();
    println!("config1{:?}",config1);
    // 对配置进行修改
    {
      let mut conf = config1.lock().unwrap();
      conf.db_connect_url = "修改后的db连接路径".to_string();
    }
    let config2 = super::singleton::get_config();
    println!("config2{:?}",config2);
    let conf2 = config2.lock().unwrap();
    assert_eq!(conf2.db_connect_url,"修改后的db连接路径".to_string());
   }
   

    #[test]
    fn adapter(){
        // assert_eq!(4,add_two(3));
    
    }
}
