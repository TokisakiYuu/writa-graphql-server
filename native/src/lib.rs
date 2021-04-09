pub mod db;

use mongodb::bson::doc;
use neon::prelude::*;
use db::WritaDB;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

// fn test_mongodb(mut cx: FunctionContext) -> JsResult<JsValue> {
//     let uri = cx.argument::<JsString>(0)?.value();
//     match db::test_mongodb(&uri) {
//         Ok(_) => Ok(cx.boolean(true).upcast()),
//         Err(msg) => cx.throw_error(msg)
//     }
// }


declare_types! {
    pub class JsWritaDB for WritaDB {
        init(mut cx) {
            let uri = cx.argument::<JsString>(0)?.value();
            let instance = WritaDB::new(&uri);
            Ok(instance)
        }

        method use_db(mut cx) {
            let db_name = cx.argument::<JsString>(0)?.value();
            let mut this = cx.this();
            {
                let guard = cx.lock();
                let mut wdb = this.borrow_mut(&guard);
                wdb.db = Some(wdb.client.database(&db_name));
            }
            Ok(cx.undefined().upcast())
        }

        method get_collection_names(mut cx) {
            let this = cx.this();
            let mut names: Vec<String> = vec![];
            let mut error_msg: Option<&str> = None;
            {
                let guard = cx.lock();
                let wdb = this.borrow(&guard);
                if let Some(db) = &wdb.db {
                    let result = db.list_collection_names(doc! {});
                    if let Ok(list) = result {
                        names = list;
                    }
                } else {
                    error_msg = Some("no database selected yet, please call \"use_db(name)\" select a database");
                }
            }
            if let Some(msg) = error_msg {
                return cx.throw_error(msg);
            }
            let arr = JsArray::new(&mut cx, names.len() as u32);
            for (i, name) in names.iter().enumerate() {
                let val = cx.string(&name);
                arr.set(&mut cx, i as u32, val)
                    .expect("out of array index range");
            }
            Ok(arr.upcast())
        }
    }
}

register_module!(mut cx, {
    cx.export_function("hello", hello)?;
    cx.export_class::<JsWritaDB>("WritaDB")?;
    Ok(())
});
