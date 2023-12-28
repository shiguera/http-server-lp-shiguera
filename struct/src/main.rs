#[derive(Debug, PartialEq, Clone)]
enum Lang {
  English,
  Spanish,
  Chinese,
  Texan,
  French
}

#[derive(Clone)]
struct Greeting {
    message: String,
    lang: Lang,
}

fn main() {
  let mut v :Vec<Greeting> = Vec::new();
  let g : Greeting = Greeting { lang: Lang::English, message: String::from("Hello WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Spanish, message: String::from("Hola WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Texan, message: String::from("Howdy WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Chinese, message: String::from("WasmEdge 你好!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::French, message: String::from("¡Bonjour, WasmEdge!") };
  v.push(g);

  //for e in &v {
  //   println!("{:?} {}", e.lang, e.message);
  //}

  let language = Lang::Spanish;
  let w: Greeting = v.clone().into_iter().find(|g| g.lang==language).unwrap();
  println!("{} {:?}", w.message, w.lang);
  let language = Lang::French;
  let w: Vec<Greeting> = v.clone().into_iter().filter(|g| g.lang==language).collect();
  println!("{:?} {}", w[0].lang, w[0].message);
  if let Some(chinese) = v.iter().find(|g| g.lang == Lang::Chinese) {
      println!("Chinese greeting is {}", chinese.message);
  }
}
