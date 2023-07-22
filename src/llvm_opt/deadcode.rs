use std::collections::HashMap;

use crate::utils::check::*;
use crate::structures::scopes::*;

/// 根据输入的旧标签更新标签映射表，并返回新的标签。
pub fn update_label(labels: &mut Labels, label_map: &mut HashMap<String, usize>, old_label: &str) -> String {
    if !old_label.contains("%") {
        return String::from(old_label);
    }
    if !is_num_label(old_label) {
        return label_map.get(old_label)
            .map_or(String::from(old_label), |x| String::from(x.to_string()));
    }
    if let Some(new_label) = label_map.get(old_label) {
        return String::from(new_label.to_string());
    }
    let new_label = labels.pop_num_str();
    label_map.insert(String::from(old_label), String::from(&new_label).parse().unwrap());
    new_label
}


