use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Node {
    Dir { children: HashMap<String, Node> },
    File { size: u64 },
}
