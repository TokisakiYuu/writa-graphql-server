var addon = require('../native');

console.log(addon.hello());

let wdb = new addon.WritaDB("mongodb uri...");
wdb.use_db("yuulog");

console.log(wdb.get_collection_names());
