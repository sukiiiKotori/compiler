use std::collections::{HashMap, HashSet, VecDeque};
use crate::llvm_opt::flow::FlowItem;

/// 从FlowItem中建立映射关系
/// 返回两个HashMap，存储值均为集合，集合中的值为键值的关联类型
/// 第一个集合包含自身决定其活跃性的标识
/// 第二个集合包含决定自身活跃性的标识
pub fn build_map(items: Vec<&impl FlowItem>) -> (HashMap<String, HashSet<String>>, HashMap<String, HashSet<String>>) {
    let mut succs = HashMap::new();
    let mut preds = HashMap::new();

    for item in items {
        let (self_label, associate_labels) = item.flow_info();
        let self_id = self_label.unwrap_or("");

        succs.entry(String::from(self_id)).or_insert_with(HashSet::new);
        preds.entry(String::from(self_id)).or_insert_with(HashSet::new);

        for associate in associate_labels {
            let self_set = succs.entry(String::from(self_id)).or_insert_with(HashSet::new);
            self_set.insert(String::from(associate));

            let assoc_set = preds.entry(String::from(associate)).or_insert_with(HashSet::new);
            assoc_set.insert(String::from(self_id));
        }
    }

    (succs, preds)
}

/// 根据映射关系计算活跃的标识
pub fn calc_active(succs: &HashMap<String, HashSet<String>>, mut preds: HashMap<String, HashSet<String>>) -> HashSet<String> {
    let mut deque = preds
        .iter()
        .filter(|(_, v)| v.is_empty())
        .map(|(k, _)| k.clone())
        .collect::<VecDeque<String>>();

    let mut traversed = deque.iter().cloned().collect::<HashSet<String>>();

    while let Some(this_label) = deque.pop_front() {
        if let Some(succ_set) = succs.get(&this_label) {
            for i in succ_set.iter().cloned() {
                if let Some(pred_set) = preds.get_mut(&i) {
                    pred_set.remove(&this_label);
                    if pred_set.is_empty() && !traversed.contains(&i) {
                        traversed.insert(i.clone());
                        deque.push_back(i);
                    }
                }
            }
        }
    }

    preds.into_iter()
        .filter(|(_, v)| !v.is_empty())
        .map(|(k, _)| k)
        .collect()
}