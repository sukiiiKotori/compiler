use std::collections::{HashMap, HashSet};
use crate::llvm_opt::flow::*;
use crate::llvm_opt::flow_graph::*;

use crate::utils::check::*;
use crate::structures::llvm_struct::*;
use crate::llvm_gen::scopes::Labels;

pub fn update_label(labels: &mut Labels, label_map: &mut HashMap<String, String>, old_label: &str) -> String {
    if !old_label.contains("%") { // 跳过非局部标号
        String::from(old_label)
    } else if !is_num_label(old_label) {// 局部非数字标号
        label_map.get(old_label).map_or(String::from(old_label), |x| String::from(x))
    } else { // 局部数字标号
        if let Some(new_label) = label_map.get(old_label) {
            String::from(new_label)
        } else {
            let new_label = labels.pop_num_str();
            label_map.insert(String::from(old_label), String::from(&new_label));
            new_label
        }
    }
}



