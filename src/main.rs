use alfred::{Item, ItemBuilder};
use std::{borrow::Cow, env::args, io};

fn main() {
    let args = args().into_iter().collect::<Vec<_>>();
    let param = args
        .iter()
        .enumerate()
        .filter(|(i, _)| *i >= 1)
        .map(|(_, s)| s)
        .fold("".to_string(), |prev, s| prev + s);

    let mut alfred = alfred::json::Builder::new();
    let mut items = Vec::<Item>::new();

    let enc = base64::encode(param.clone());
    items.push(
        ItemBuilder::new("Encode")
            .subtitle(enc.clone())
            .arg(enc)
            .into_item(),
    );

    let dec = base64::decode(param);
    if let Ok(v) = dec {
        let str = String::from_utf8(v.clone());
        if let Ok(s) = str {
            let s = Cow::from(s);
            items.push(
                ItemBuilder::new("Decode Text".to_string())
                    .subtitle(s.clone())
                    .arg(s)
                    .into_item(),
            );
        }
        items.push(
            ItemBuilder::new("Decode Hex".to_string())
                .subtitle(hex::encode(&v))
                .arg(hex::encode(&v))
                .into_item(),
        )
    }
    alfred.set_items(&items);
    alfred.write(io::stdout()).unwrap();
    // let s = Rc::new(String::from("whoami"));
}
