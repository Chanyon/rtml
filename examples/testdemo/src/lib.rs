use std::ops::Add;

use rtml::EventKind::{self,Click};
use rtml::{attr, mount_body, style, tags::*};

use wasm_bindgen::prelude::*;
use web_sys::Event;

use js_sys::Reflect;

#[wasm_bindgen]
extern  "C" {
    #[wasm_bindgen(js_namespace= console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace= console)]
    fn log_vec(s: String);
}

#[derive(Clone)]
struct Item {
    id:usize, 
    msg: String,
    done: bool,
}

impl Item {
    fn new<S:Into<String>>(id: usize,msg:S) -> Self {
        Self {
            id,
            msg:msg.into(),
            done: false,
        }
    }
    fn toggle(&mut self){
        self.done = !self.done;
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    tracing_wasm::set_as_global_default();

    let items = vec![
        Item::new(1,"fun"),
        Item::new(2,"study"),
        Item::new(3,"running")
    ].reactive();
    let msg = String::new().reactive();
    let change_handler = msg.evt(|evt,msg| {
        let evt:Event = evt.into();
        // 拿到chang事件的对象的target
        if let Some(target) = evt.target() {
            let value = Reflect::get(&target, &JsValue::from_str("value")).unwrap();
            if let Some(val) = value.as_string(){
                *msg.val_mut() = val;
                log("change evt.");
            }
        }
    });
    let click_handler = msg.add(items.clone()).evt(|evt,(msg,items)| {
        let evt:Event = evt.into();
        if msg.val().is_empty(){

        }else{
            let text = msg.val().clone();
            items.val_mut().push(Item::new(100,text));
            items.update();
        }
        evt.prevent_default();
    });
    let new_item = form((
        h2("New Record"),

        input(attr! {type="text",id="xpr"}).on(EventKind::Change,change_handler),

        button("+").on(Click,click_handler),
    ));
    let cards = div((
        attr! {class="out-most"},
        items.view(|cards| {
            cards
                .val()
                .iter()
                .enumerate()
                .map(|(idx, data)| {
                    div((
                        attr! {class="container"},
                        (
                            p(format!("ID: {}", data.id)),
                            p(format!("Content: {}", data.msg)),
                            p(format!("status: {}", data.done)),
                            button((
                                style! {
                                    margin: "0 5px 0 0"
                                },
                                "toggle",
                            ))
                            .on(
                                Click,
                                cards.change(move |records| {
                                    if let Some(item) = records.val_mut().get_mut(idx) {
                                        item.toggle();
                                    }
                                }),
                            ),
                            button((
                                style! {
                                    margin: "0 5px 0 0"
                                },
                                "delete",
                            ))
                            .on(
                                Click,
                                cards.change(move |records| {
                                    records.val_mut().remove(idx);
                                }),
                            ),
                            hr(()),
                        ),
                    ))
                })
                .collect::<Vec<_>>()
                .into()
        }),
    ));
    mount_body(div((new_item,hr(()), cards))).unwrap();
}