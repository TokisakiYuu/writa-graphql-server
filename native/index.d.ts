declare namespace addon {
  function hello(): void;
  class WritaDB {
    constructor(uri: string);
    use_db(name: string): void;
    get_collection_names(): [string];
  }
}
export default addon;
