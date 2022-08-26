use crate::builder::repr::builder_bdd::{BddPtr, PointerType};
use crate::repr::var_label::VarLabel;
use crate::builder::repr::builder_bdd::TableIndex;

mod test_graphviz {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct BddNode {
        pub low: BddPtr,
        pub high: BddPtr,
        pub var: VarLabel,
    }

    impl BddNode {
        pub fn new(low: BddPtr, high: BddPtr, var: VarLabel) -> BddNode {
            BddNode {
                low: low,
                high: high,
                var: var,
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Bdd {
        Node(BddNode),
        BddTrue,
        BddFalse,
    }

    impl Bdd {
        pub fn new_node(low: BddPtr, high: BddPtr, var: VarLabel) -> Bdd {
            Bdd::Node(BddNode {low: low, high: high, var: var,})
        }

        pub fn into_node(&self) -> BddNode {
            match self {
                &Bdd::Node(ref n) => n.clone(),
                _ => panic!("called into-node on non-node BDD"),
            }
        }
    }


    pub struct MyBddManager {
        compute_table: Vec<Vec<BddNode>>,
        order: Vec<VarLabel>,
    }
    impl MyBddManager {
        pub fn new(order: Vec<VarLabel>) -> MyBddManager {
            let mut compute_table = Vec::with_capacity(order.len());
            for _ in 0..order.len() {
                compute_table.push(vec![]);
            }
            MyBddManager {compute_table, order,}
        }
        pub fn var(&mut self, lbl: VarLabel) -> BddPtr {
            let bdd = BddNode::new(BddPtr::false_node(), BddPtr::true_node(), lbl);
            let r = self.get_or_insert(bdd);
            r
        }
        pub fn deref(&self, ptr: BddPtr) -> Bdd {
            match ptr.ptr_type() {
                PointerType::PtrFalse => Bdd::BddFalse,
                PointerType::PtrTrue => Bdd::BddTrue,
                PointerType::PtrNode => {
                    let bddnode = &self.compute_table[ptr.var() as usize][0];
                    Bdd::new_node(bddnode.low, bddnode.high, VarLabel::new(ptr.var()))

                }
            }
        }
        pub fn get_or_insert(&mut self, bdd: BddNode) -> BddPtr {
           let bdd = Bdd::new_node(bdd.low, bdd.high, bdd.var);
            match bdd {
                Bdd::BddFalse => BddPtr::false_node(),
                Bdd::BddTrue => BddPtr::true_node(),
                Bdd::Node(n) => {
                    let var = n.var.value();
                    let elem = BddNode::new(n.low, n.high, VarLabel::new(var.clone()));
                    let vartable = &self.compute_table[var as usize];
                    match vartable.iter().filter(|e| *e == &elem).next() {
                        None => {
                            self.compute_table[var as usize].push(elem.clone());
                        }
                        Some(ptr) => {
                        },
                    };

                    BddPtr::new(VarLabel::new(var), TableIndex::new(0 as u64))
                }
            }
        }
        pub fn low(&self, ptr: BddPtr) -> BddPtr {
            assert!(!ptr.is_const(), "Attempting to get low pointer of constant BDD");
            let b = self.deref(ptr).into_node();
            b.low
        }
        pub fn high(&self, ptr: BddPtr) -> BddPtr {
            assert!(!ptr.is_const(), "Attempting to get high pointer of constant BDD");
            let b = self.deref(ptr).into_node();
            b.high
        }
        pub fn and(&mut self, f: BddPtr, g: BddPtr) -> BddPtr {
            println!("and!");
            let reg_f = f.regular();
            let reg_g = g.regular();

            // we only deal with true in this example.
            if reg_f.is_true() {
                if f.is_true() {
                    return g;
                } else {
                    return f;
                }
            }

            let index = f.label();
            let fv = self.high(reg_f);
            let fnv = self.low(reg_f);
            let gv = g;
            let gnv = g;

            // now recurse
            let new_h = self.and(fv, gv);
            let new_l = self.and(fnv, gnv);
            let n = BddNode {
                low: new_l,
                high: new_h,
                var: index,
            };
            let r = self.get_or_insert(n);
            return r;
        }
    }

    fn my_get_label(mgr: &MyBddManager, ptr: BddPtr) -> (String, PointerType) {
        let x = ptr.ptr_type().clone();
        match x {
           PtrTrue  => ("PtrTrue".to_string(), x),
           PtrFalse => ("PtrFalse".to_string(), x),
           PtrNode  => ("PtrNode".to_string(), x),
       }
    }
    type MyReturn = Vec<((BddPtr, (String, PointerType)), (BddPtr, (String, PointerType)))>;
    pub fn call_recursive_bfs(mgr: &MyBddManager, ptr: BddPtr) -> MyReturn {
        fn bfs_recursively(
            mgr: &MyBddManager,
            optr: Option<BddPtr>,
            queue: &mut Vec<BddPtr>,
            edges: &mut MyReturn,
        ) -> MyReturn {
            match optr {
                None => edges.to_vec(),
                Some(ptr) => {
                    match ptr.ptr_type() {
                        PointerType::PtrTrue  => bfs_recursively(mgr, queue.pop(), queue, edges),
                        PointerType::PtrFalse => bfs_recursively(mgr, queue.pop(), queue, edges),
                        PointerType::PtrNode => {
                            let lp = mgr.low(ptr);
                            let hp = mgr.high(ptr);
                            let parent = (ptr, my_get_label(mgr, ptr));
                            let c1 = (lp, my_get_label(mgr, lp));
                            let c2 = (hp, my_get_label(mgr, hp));
                            edges.extend(vec![
                                (parent.clone(), c1),
                                (parent.clone(), c2),
                            ]);
                            queue.extend(vec![hp]);
                            bfs_recursively(mgr, Some(lp), queue, edges)
                        }
                    }
                }
            }
        }
        bfs_recursively(mgr, Some(ptr), &mut vec![], &mut vec![],)
    }


    #[test]
    fn test_dot_rust_bug() {
        let va = VarLabel::new(0);
        let vb = VarLabel::new(1);
        let mut mgr = MyBddManager::new(vec![va, vb]);
        let a = mgr.var(va);
        let b = mgr.var(vb);
        let ab = mgr.and(a, b);

        println!("{:#?}", call_recursive_bfs(&mgr, a));
        todo!();
    }
}
