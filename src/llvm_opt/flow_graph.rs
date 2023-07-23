use std::collections::{HashMap, HashSet, VecDeque};
use crate::llvm_opt::flow::FlowItem;

/// 从FlowItem中建立映射关系
/// 返回两个HashMap，存储值均为集合，集合中的值为键值的关联类型
/// 第一个集合包含自身决定其活跃性的标识
/// 第二个集合包含决定自身活跃性的标识
pub fn build_map(items: Vec<&impl FlowItem>) -> (HashMap<String, HashSet<String>>, HashMap<String, HashSet<String>>) {
    let mut succs = HashMap::new();
    let mut preds = HashMap::new();
    items.iter().for_each(|item| {
        let (self_label, associate_labels) = item.flow_info();
        let self_id = self_label.unwrap_or("");
        succs.entry(self_id.to_string()).or_insert_with(HashSet::new);
        preds.entry(self_id.to_string()).or_insert_with(HashSet::new);
        associate_labels.iter().for_each(|associate| {
            succs.entry(self_id.to_string()).or_insert_with(HashSet::new).insert(associate.to_string());
            preds.entry(associate.to_string()).or_insert_with(HashSet::new).insert(self_id.to_string());
        });
    });
    (succs, preds)
}

/// 根据映射关系计算活跃的标识
pub fn calc_active(succs: &HashMap<String, HashSet<String>>,mut preds: HashMap<String, HashSet<String>>) -> HashSet<String> {
    let mut deque: VecDeque<String> = preds.iter().filter_map(|(k, v)| {
        if v.is_empty() {
            Some(k.to_string())
        } else {
            None
        }
    }).collect();
    let mut traversed: HashSet<String> = deque.iter().cloned().collect();
    while let Some(this_label) = deque.pop_front() {
        if let Some(succ_set) = succs.get(&this_label) {
            succ_set.iter().cloned().for_each(|i| {
                if let Some(pred_set) = preds.get_mut(&i) {
                    pred_set.remove(&this_label);
                    if pred_set.is_empty() && !traversed.contains(&i) {
                        traversed.insert(i.clone());
                        deque.push_back(i);
                    }
                }
            });
        }
    }
    preds.iter().filter_map(|(k, v)| {
        if v.is_empty() {
            None
        } else {
            Some(k.to_string())
        }
    }).collect()
}