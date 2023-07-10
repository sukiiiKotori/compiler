use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use crate::riscv_gen::liveness::*;
use crate::riscv_gen::reg::*;
use crate::structures::riscv_struct::*;

#[derive(Clone, Eq, PartialEq)]
pub struct LinearActiveNode {
    pub virt: Option<String>,
    pub live: Option<Interval>,
    pub phy: Option<&'static str>,
}

impl Ord for LinearActiveNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.live.as_ref().unwrap().cmp_farther(
            &other.live.as_ref().unwrap()
        )
    }
}

impl PartialOrd for LinearActiveNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct LinearScan {
    pub intervals: Vec<(String, Interval)>,
    pub resource: RegisterResource,
    pub active: Vec<LinearActiveNode>,
    pub inactive: HashMap<String, &'static str>, 
    pub spilled: HashSet<String>,
}

impl LinearActiveNode {
    pub fn new(virt: String, live: Interval, phy: &'static str) -> Self {
        LinearActiveNode {
            virt: Some(virt),
            live: Some(live),
            phy: Some(phy),
        }
    }

    pub fn take_all(&mut self) -> (String, Interval, &'static str) {
        (self.virt.take().unwrap(), self.live.take().unwrap(), self.phy.take().unwrap())
    }
}

impl LinearScan {
    fn load_free_regs(&mut self) {
        self.resource.load_free_regs();
        self.resource.evict_regs(|r| PRESERVED_SET.contains(r) || FLOAT_PRESERVED_SET.contains(r));
    }

    fn eviction(&mut self, now: &Interval, func: &mut AsmFunc) {
        let evicted = self.active.iter()
            .enumerate()
            .filter_map(|(idx, item)|{
                if item.live.as_ref().unwrap().is_inactive(now) {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        
        let mut evicted_regs = Vec::new();
        for pos in evicted.into_iter().rev() {
            let (virt, _, phy) = self.active.remove(pos).take_all();
            evicted_regs.push((virt, phy));
        }
        for (virt, phy) in evicted_regs.into_iter() {
            if SAVED_SET.contains(phy) || FLOAT_SAVED_SET.contains(phy) {
                func.used_saved(phy);
            }
            self.resource.push_register(phy);
            self.inactive.insert(virt, phy);
        }
    }

    fn filter_reg(r: &str, ty: &RegType) -> bool {
        match ty {
            RegType::TempInt => &r[0..1] != "f",
            RegType::SavedInt => SAVED_SET.contains(r),
            RegType::TempFloat => &r[0..1] == "f",
            RegType::SavedFloat => FLOAT_SAVED_SET.contains(r),

            /*RegType::TempInt => &r[0..1] != "f" && !SAVED_SET.contains(r),
            RegType::SavedInt => SAVED_SET.contains(r),
            RegType::TempFloat => &r[0..1] == "f" && !FLOAT_SAVED_SET.contains(r),
            RegType::SavedFloat => FLOAT_SAVED_SET.contains(r),*/
        }
    }

    fn try_spill(&mut self, this_virt: String, live: Interval, ty: RegType) {
        let (max_id, max_live) = self.active.iter()
            .enumerate()
            .filter(|(_, n)| Self::filter_reg(n.phy.as_ref().unwrap(), &ty))
            .max_by(|(_, n0), (_, n1)| n0.cmp(&n1))
            .map(|(i, n)| (i, n.live.as_ref().unwrap()))
            .unwrap();
        
        if live.cmp_farther(max_live) == Ordering::Greater {
            self.spilled.insert(this_virt);
        } else {
            let (virt, _, phy) = self.active.remove(max_id).take_all();
            self.spilled.insert(virt);
            self.active.push(LinearActiveNode::new(this_virt, live, phy));
        }
    }
}
impl RegisterAllocator for LinearScan {
    fn new() -> Self {
        LinearScan {
            intervals: Vec::new(),
            resource: RegisterResource::new(),
            active: Vec::new(),
            inactive: HashMap::new(),
            spilled: HashSet::new(),
        }
    }

    fn alloc_regs(&mut self, func: &mut AsmFunc) {
        {
            let mut liveness = Liveness::new(func);
            liveness.compute_local_live_sets();
            liveness.compute_global_live_sets();
            liveness.build_intervals();
            let (virt_regs, _) = liveness.dump_intervals();
            self.intervals = virt_regs.into_iter()
                .map(|(virt_reg, live)| (String::from(virt_reg), live))
                .collect();

            self.intervals.sort_by(|(_, i0), (_, i1)| {
                i1.cmp_earlier(&i0)
            });

            liveness.compute_depth_first_instr();
            func.update_call_pos(&liveness.old2new);
        }
        self.load_free_regs();

        while !self.intervals.is_empty() {
            let (this_virt, live) = self.intervals.pop().unwrap();
            self.eviction(&live, func);

            let reg_ty = RegType::classify_label(func.is_float(this_virt.as_str()), func.interval_cross_call(&live, this_virt.as_str()));
            let new_reg = self.resource.pop_register(&reg_ty, |r| Self::filter_reg(r, &reg_ty));

            if let Some(new_reg) = new_reg {
                self.active.push(LinearActiveNode::new(this_virt, live, new_reg));
            } else { // spill one
                self.try_spill(this_virt, live, reg_ty);
            }
        }

        for LinearActiveNode{virt, live: _, phy} in self.active.iter() {
            if SAVED_SET.contains(phy.as_ref().unwrap()) || FLOAT_SAVED_SET.contains(phy.as_ref().unwrap()) {
                func.used_saved(phy.as_ref().unwrap());
            }
                self.inactive.insert(String::from(virt.as_ref().unwrap()), phy.as_ref().unwrap());
        }
    }
    
    fn get_spilled(&self) -> &HashSet<String> {
        &self.spilled
    }

    fn get_alloc_res(&self) -> &HashMap<String, &'static str> {
        &self.inactive
    }
}

