use crate::{
  graph::AdjList,
  instr::{Op, Relation, Event, Locality},
  mem::MemOp,
  arch::{Arch, Visibility, MicroOrdering},
  micro::{MicroOp},
};
impl Arch {
  pub fn create_micro_graph(&self, instr: &Vec<Vec<MemOp>>) -> AdjList<MicroOp, Relation> {
    let mut out = AdjList::new();
    use Visibility::*;
    let stages = self.stages_of(
      vec!(GlobalRead, GlobalWrite, LocalWrite, RetireRead, RetireWrite)
    );
    let (g_read, g_write, l_write, ret_read, ret_write) =
      (stages[0], stages[1], stages[2], stages[3], stages[4]);

    let (mut l_write_stages, mut g_write_stages, mut g_read_stages) = (vec!(), vec!(), vec!());

    instr.iter().enumerate().for_each(|(thread, mem_ops)| {
      let mut prev : Option<Vec<usize>> = None;

      // Creating uops
      mem_ops.iter().enumerate().for_each(|(pc, &mem)| {
        let op = Op{core: 0, thread, pc, mem};
        let stages = self.desc.stages.iter()
          .take(1 + match mem {
            MemOp::Write(_,_,_) => ret_write,
            MemOp::Read(_,_,_) | MemOp::ReadInit(_,_) => ret_read,
            MemOp::Init => unreachable!(),
            MemOp::Fence(_) => unimplemented!(),
          })
          .enumerate()
          .map(|(stage, arch_stage)| {
            let uop = MicroOp{stage, op};
            let index = out.push_node(uop);
            if stage != 0 { out.push_edge(index - 1, index, Relation::ProgramOrder) }
            if op.is_write() {
              if stage == g_write { g_write_stages.push((index, uop)) }
              if stage == l_write { l_write_stages.push((index, uop)) }
            } else if op.is_read() {
              if stage == g_read { g_read_stages.push((index, uop)) }
            }
            match arch_stage.ord {
              MicroOrdering::Queue => prev.as_ref().iter()
                .for_each(|s| s.get(stage).iter().for_each(|&prev|
                  // below should be stage order but that draws a weird graph
                  out.push_edge(*prev, index, Relation::StageOrder))),
              _ => (),
            };
            index
          })
          .collect();
          prev = Some(stages);
      });
    });

    g_read_stages.iter().for_each(|&(uop_i, curr_uop)| {
      assert!(curr_uop.is_read());
      assert_eq!(curr_uop.stage, g_read);
      let _local_writes = l_write_stages.iter()
        .filter(|(_, l_w_uop)| l_w_uop.op.same_core(&curr_uop.op))
        .filter(|(_, l_w_uop)| l_w_uop.op.same_thread(&curr_uop.op))
        .filter(|(_, l_w_uop)| l_w_uop.same_access_address(&curr_uop))
        .filter(|(_, l_w_uop)| l_w_uop.same_data(&curr_uop))
        .for_each(|&(uop_j,_)| out.push_edge(uop_j, uop_i, Relation::ReadFrom(Locality::Local)));

      let _global_writes = g_write_stages.iter()
        .filter(|(_, l_w_uop)| l_w_uop.same_access_address(&curr_uop))
        .filter(|(_, l_w_uop)| l_w_uop.same_data(&curr_uop))
        .for_each(|&(uop_j,_)| out.push_edge(uop_j,uop_i,Relation::ReadFrom(Locality::Global)));
    });

    out
  }
}









