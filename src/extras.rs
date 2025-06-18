use crate::repr::{BddPtr, DDNNFPtr};
use crate::builder::bdd::RobddBuilder;
use crate::builder::*;
use crate::builder::cache::IteTable;
use itertools::Itertools;

pub fn all_models<'a, T: IteTable<'a, BddPtr<'a>> + Default> (builder: &'a RobddBuilder<'a, T>, bdd: BddPtr<'a>) -> Vec<Vec<bool>>{
    let polarity = [false, true];
    let nvars = builder.order().num_vars();
    let combinations : Vec<Vec<_>> = (0..nvars).map(|_| polarity)
        .multi_cartesian_product()
        .filter(|xs| eval_assignment(builder, bdd, &xs).expect("assignments are total"))
        .collect();
    combinations
}

pub fn all_models_flat<'a, T: IteTable<'a, BddPtr<'a>> + Default> (builder: &'a RobddBuilder<'a, T>, bdd: BddPtr<'a>) -> Vec<bool> {
    all_models(builder, bdd).into_iter().flatten().collect()
}



pub fn eval_assignment<'a, T: IteTable<'a, BddPtr<'a>> + Default> (builder: &'a RobddBuilder<'a, T>, bdd: BddPtr<'a>, xs:&[bool]) -> Option<bool> {
    let vo = builder.order();
    let mut ev = bdd;
    for (ix, pol) in xs.iter().enumerate() {
        ev = builder.condition(ev, vo.var_at_level(ix), *pol)
    }
    if ev.is_const() {
        Some(ev.is_true())
    } else {
        None
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::{quickcheck,TestResult};
    use crate::plan::{BottomUpPlan};
    use crate::repr::{DTree, Cnf};
    use crate::builder::cache::*;

    #[test]
    fn getting_combos() {
        let combinations : Vec<Vec<_>> = (0..2).map(|i| (i * 2)..(i * 2 + 2))
            .multi_cartesian_product()
            .collect();
        let expected = vec![
            vec![0, 2],
            vec![0, 3],
            vec![1, 2],
            vec![1, 3],
        ];
        for c in combinations.iter() {
            println!("{:?}", c);
        }
        assert!(combinations == expected)
    }
    #[test]
    fn getting_all_assignments() {
        let polarity = [false, true];
        let nvars = 3;
        let combinations : Vec<Vec<_>> = (0..nvars).map(|_| polarity)
            .multi_cartesian_product()
            .collect();
        let expected = vec![
            vec![false, false, false],
            vec![false, false, true ],
            vec![false, true , false],
            vec![false, true , true ],
            vec![true , false, false],
            vec![true , false, true ],
            vec![true , true , false],
            vec![true , true , true ],
        ];
        for c in combinations.iter() {
            println!("{:?}", c);
        }
        assert!(combinations == expected)
    }
    quickcheck! {
        fn prop_validate_all_models(cnf: Cnf) -> TestResult {
            let vo = Cnf::linear_order(&cnf);
            let dtree = DTree::from_cnf(&cnf, &vo);
            let plan = BottomUpPlan::from_dtree(&dtree);
            let builder = RobddBuilder::<LruIteTable<BddPtr>>::new(vo.clone());
            let bdd = builder.compile_plan(&plan);
            if bdd.is_const() {
                return TestResult::discard();
            }
            let assignments = all_models(&builder, bdd);
            if assignments.len() == 0 {
                return TestResult::failed();
            }
            println!("Bdd: {}", bdd.print_bdd());
            println!("{:?}", vo);
            for xs in &assignments {
                println!("{:?}", xs);
            }
            let all_valid = assignments
                .into_iter()
                .all(|xs| eval_assignment(&builder, bdd, &xs).expect("assignments are total"));
            TestResult::from_bool(all_valid)
        }
  }

}
