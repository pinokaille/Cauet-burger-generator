use figlet_rs::FIGfont;
use loading::Loading;
use std::thread;
use std::time::Duration;

fn main(){
    let standard_font = FIGfont::standand().unwrap();
    let figure = standard_font.convert("CAUET BURGER");
    let figure2 = standard_font.convert("GENERATOR");
    assert!(figure.is_some());
    assert!(figure2.is_some());
    println!("{}",figure.unwrap());
    println!("{}",figure2.unwrap());
    let mut loading = Loading::new();

    loading.start();

    for i in 0..100 {
        loading.text(format!("🍞 Génération du pain {}", i));
        thread::sleep(Duration::from_millis(50));
    }

    loading.success("Pain : OK");


    for i in 0..100 {
        loading.text(format!("🧀 Génération du fromage {}", i));
        thread::sleep(Duration::from_millis(50));
    }

    loading.success("fromage : OK");

    for i in 0..100 {
        loading.text(format!("🍅 Génération des tomates {}", i));
        thread::sleep(Duration::from_millis(50));
    }

    loading.success("tomates : OK");


    for i in 0..100 {
        loading.text(format!("🥩 Génération du steak {}", i));
        thread::sleep(Duration::from_millis(50));
    }

    loading.success("steak : OK");

    for i in 0..100 {
        loading.text(format!("🥬 Génération de la salade {}", i));
        thread::sleep(Duration::from_millis(50));
    }

    loading.success("salade : OK");

    loading.info(format!("Le cauet burger est maintenant fini vous allez pouvoir goûter ce burger délicieux"));
    thread::sleep(Duration::from_millis(80));
    
    while 1 == 1{
    loading.success("UN CAUET BURGER");
    }

  


    loading.end();
    
}