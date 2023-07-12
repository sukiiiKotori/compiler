use std::collections::{HashMap, HashSet, VecDeque};
use crate::llvm_opt::flow::FlowItem;
use super::*;

/// 映射标号的关联标号
type RelateMap = HashMap<String, HashSet<String>>;

/// 从FlowItem中建立映射关系<br>
/// 返回两个HashMap，存储值均为集合，集合中的值为键值的关联类型<br>
/// 第一个集合包含自身决定其活跃性的标识<br>
/// 第二个集合包含决定自身活跃性的标识<br>
pub fn build_map(items: Vec<&impl FlowItem>) -> (RelateMap, RelateMap) {
    let mut succs = HashMap::new();
    let mut preds = HashMap::new();
    for item in items.iter() {
        let (self_label, associate_labels) = item.flow_info();
        let self_id = self_label.unwrap_or("");

        if !succs.contains_key(self_id) {
            succs.insert(String::from(self_id), HashSet::new());
        }
        if !preds.contains_key(self_id) {
            preds.insert(String::from(self_id), HashSet::new());
        }
        for associate in associate_labels.into_iter() {
            succs
                .get_mut(self_id)
                .unwrap()
                .insert(String::from(associate));

            if !preds.contains_key(associate) {
                preds.insert(String::from(associate), HashSet::new());
            }

            preds
                .get_mut(associate)
                .unwrap()
                .insert(String::from(self_id));
        }
    }
    (succs, preds)
}

/// 根据映射关系计算活跃的标识
pub fn calc_active(succs: &RelateMap, mut preds: RelateMap) -> HashSet<String> {
    let mut deque: VecDeque<String> = preds
        .iter()
        .filter(|(_, v)| v.is_empty())
        .map(|(k, _)| String::from(k))
        .collect();
    let mut traversed: HashSet<String> = deque.iter().map(|x| String::from(x)).collect();

    while !deque.is_empty() {
        let this_label = deque.pop_front().unwrap();
        let succ_set = succs.get(&this_label);
        if succ_set.is_none() {
            continue;
        }
        let succ_set = succ_set.unwrap();

        for i in succ_set.iter() {
            let pred_set = preds.get_mut(i);
            if pred_set.is_none() {
                continue;
            }
            let pred_set = pred_set.unwrap();
            pred_set.remove(&this_label);
            if pred_set.is_empty() && !traversed.contains(i) {
                traversed.insert(String::from(i));
                deque.push_back(String::from(i));
            }
        }
    }

    preds
        .into_iter()
        .filter(|(_, v)| !v.is_empty())
        .map(|(k, _)| k)
        .collect()
} // calc_active