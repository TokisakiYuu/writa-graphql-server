use mongodb::{
    error::Result,
    sync::Client,
    bson::{doc, Bson}
};

pub fn test_mongodb(mongodb_uri: &str) -> std::result::Result<(), String> {
    let client = if_err(Client::with_uri_str(mongodb_uri), "MongoDB连接失败")?;
    let db_names = if_err(client.list_database_names(doc!{}, None), "数据库列表获取失败")?;
    println!("数据库: {:?}", db_names);

    let db = client.database("yuulog");
    let collection_names = if_err(db.list_collection_names(doc!{}), "集合列表获取失败")?;
    println!("yuulog库中的集合: {:?}", collection_names);

    let collection = db.collection("_rust_temp");

    // 删除集合
    if_err(collection.drop(None), "删除集合失败")?;
    
    // TODO
    // 创建集合
    if_err(db.create_collection("_rust_temp", None), "创建集合失败")?;

    // 写入文档
    if_err(collection.insert_one(doc! {
        "name": "张三",
        "age": "18",
        "firends": vec!["李四", "王五"]
    }, None), "文档写入失败")?;

    // 更新文档
    if_err(collection.update_one(
        doc! { "name": "张三" },
        doc! {
            "$push": {
                "firends": "赵六"
            }
        },
        None
    ), "更新文档失败")?;

    // 条件查询文档
    let cursor = if_err(collection.find(doc! { "name": "张三" }, None), "查询文档失败")?;
    for result in cursor {
        if let Ok(document) = result {
            let age = document.get("age").and_then(Bson::as_str).unwrap();
            println!("年龄: {}", age);
            let firends = document.get("firends").and_then(Bson::as_array).unwrap();
            print!("朋友：");
            for firend in firends {
                print!("{}、", firend.as_str().unwrap());
            }
            println!();
        }
    }

    // 删除文档
    if_err(collection.delete_one(doc! { "name": "张三" }, None), "删除文档失败")?;

    // 删除集合
    if_err(collection.drop(None), "删除集合失败")?;

    Ok(())
}

fn if_err<T>(res: Result<T>, msg: &str) -> std::result::Result<T, String> {
    match res {
        Ok(ret) => Ok(ret),
        Err(_) => Err(msg.to_string())
    }
}
