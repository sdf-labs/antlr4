use std::cell::RefCell;
use std::fmt::{Debug, Error, Formatter};
use std::ops::Deref;
use std::rc::Rc;

use crate::atn::ATN;
use crate::dfa::DFA;
use crate::prediction_context::PredictionContextCache;

pub trait IATNSimulator {
    fn shared_context_cache(&self) -> &PredictionContextCache;
    fn atn(&self) -> &ATN;
    fn decision_to_dfa(&self) -> &Vec<RefCell<DFA>>;
}

pub struct BaseATNSimulator {
    pub atn: Rc<ATN>,
    pub shared_context_cache: Rc<PredictionContextCache>,
    pub decision_to_dfa: Rc<Vec<RefCell<DFA>>>,
}

impl Debug for BaseATNSimulator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str("BaseATNSimulator { .. }")
    }
}

impl BaseATNSimulator {
    pub fn new_base_atnsimulator(
        atn: Rc<ATN>,
        decision_to_dfa: Rc<Vec<RefCell<DFA>>>,
        shared_context_cache: Rc<PredictionContextCache>,
    ) -> BaseATNSimulator {
        BaseATNSimulator {
            atn,
            shared_context_cache,
            decision_to_dfa,
        }
    }
}

impl IATNSimulator for BaseATNSimulator {
    fn shared_context_cache(&self) -> &PredictionContextCache {
        self.shared_context_cache.deref()
    }

    fn atn(&self) -> &ATN {
        self.atn.as_ref()
    }

    fn decision_to_dfa(&self) -> &Vec<RefCell<DFA>> {
        self.decision_to_dfa.as_ref()
    }
}
