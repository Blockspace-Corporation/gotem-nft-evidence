#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::evidence::{
    Evidence,
    EvidenceRef,
};

#[ink::contract]
pub mod evidence {
    use ink_prelude:: {
        string::String,
        vec::Vec,
        collections::BTreeMap,
    };
    use scale::{
        Decode,
        Encode,
    };

    pub type EvidenceId = u32;

    #[ink(storage)]
    pub struct Evidence {
        pub evidence: BTreeMap<EvidenceId, EvidenceNFT>,
    }

    #[derive(Encode, Decode, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct EvidenceNFT {
        pub description: String,
        pub owner: AccountId,
        pub file: Hash,
        pub case_id: u32,
        status: Status,
    }

    #[derive(Encode, Decode, Debug, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    enum Status {
        New,
        Voted,
        Close,
    }

    impl EvidenceNFT {
        fn get_evidence(evidence: &EvidenceNFT) -> EvidenceNFT {
            EvidenceNFT {
                description: evidence.description.clone(),
                owner: evidence.owner.clone(),
                file: evidence.file.clone(),
                case_id: evidence.case_id.clone(),
                status: evidence.status.clone(),
            }
        }
    }

    impl Evidence {
        #[ink(constructor, payable)]
        pub fn new() -> Self {
            Self {
                evidence: BTreeMap::new(),
            }
        }

        #[ink(message)]
        pub fn set_evidence(&mut self, evidence: EvidenceNFT) {
            let length = (self.evidence.len() as u32).checked_add(1).unwrap();
            self.evidence.insert(length, evidence);
        }

        #[ink(message)]
        pub fn get_evidence_by_id(&self, evidence_id: EvidenceId) -> Option<EvidenceNFT> {
            if let Some(evidence) = self.evidence.get(&evidence_id) {
                let evidence = EvidenceNFT::get_evidence(evidence);
                Some(evidence)
            } else {
                None
            }
        }

        #[ink(message)]
        pub fn get_all_evidence(&self) -> Vec<EvidenceNFT> {
            let evidence = self
                .evidence
                .iter()
                .map(|(_id, evidence)| EvidenceNFT::get_evidence(evidence))
                .collect();
            evidence
        }

        #[ink(message)]
        pub fn get_evidence_id(&self, evidence_id: EvidenceId) -> EvidenceId {
            if self.evidence.get(&evidence_id).is_some() {
                evidence_id
            } else {
                0 as u32
            }
        }

        #[ink(message)]
        pub fn set_code(&mut self, code_hash: Hash) {
            self.env().set_code_hash(&code_hash).unwrap_or_else(|err| {
                panic!("Failed to `set_code_hash` to {code_hash:?} due to {err:?}")
            });
            ink::env::debug_println!("Switched code hash to {:?}.", code_hash);
        }
    }
}
