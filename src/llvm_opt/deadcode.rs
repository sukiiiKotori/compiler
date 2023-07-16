use std::collections::HashMap;

use crate::utils::check::*;
use crate::structures::scopes::*;

/// 根据输入的旧标签更新标签映射表，并返回新的标签。
pub fn update_label(labels: &mut Labels, label_map: &mut HashMap<String, String>, old_label: &str) -> String {
    
    // 如果旧标签不包含 "%"，则为非局部标号，直接返回旧标签
    if !old_label.contains("%") {
        String::from(old_label)
    // 如果旧标签是局部非数字标号
    } else if !is_num_label(old_label) {
        // 尝试从标签映射中获取新标签，如果不存在则返回旧标签
        label_map.get(old_label).map_or(String::from(old_label), |x| String::from(x))
    // 如果旧标签是局部数字标号
    } else {
        // 尝试从标签映射中获取新标签
        if let Some(new_label) = label_map.get(old_label) {
            // 如果存在新标签，则返回新标签
            String::from(new_label)
        // 如果不存在新标签
        } else {
            // 从标签列表中获取一个新的数字标号
            let new_label = labels.pop_num_str();
            // 将旧标签和新标签添加到标签映射中
            label_map.insert(String::from(old_label), String::from(&new_label));
            // 返回新标签
            new_label
        }
    }
}



