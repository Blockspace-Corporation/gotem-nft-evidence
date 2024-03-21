#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::evidence::{
    Evidence,
    EvidenceRef,
};

#[ink::contract]
pub mod evidence {
    use case::CaseRef;
    use ink_prelude:: {
        string::String,
        vec::Vec,
        collections::BTreeMap,
    };
    use scale::{
        Decode,
        Encode,
    };

    pub type Id = u32;

    #[ink(storage)]
    pub struct Evidence {
        pub evidence: BTreeMap<Id, EvidenceNFT>,
        pub case: CaseRef,
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

    #[derive(Encode, Decode, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct EvidenceNFTOutput {
        pub evidence_id: Id,
        pub description: String,
        pub owner: AccountId,
        pub file: Hash,
        pub case_id: u32,
        pub case_title: Option<String>,
        status: Status,
    }

    #[derive(Encode, Decode, Debug, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    enum Status {
        New,
        Voted,
        Close,
    }

    #[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        EvidenceNotFound,
    }

    impl EvidenceNFTOutput {
        fn get_evidence(
            evidence_id: Id, 
            case_title: Option<String>, 
            evidence: &EvidenceNFT
        ) -> EvidenceNFTOutput {
            EvidenceNFTOutput {
                evidence_id: evidence_id.clone(),
                description: evidence.description.clone(),
                owner: evidence.owner.clone(),
                file: evidence.file.clone(),
                case_id: evidence.case_id.clone(),
                case_title: case_title.clone(),
                status: evidence.status.clone(),
            }
        }
    }

    impl Evidence {
        #[ink(constructor, payable)]
        pub fn new(case: CaseRef) -> Self {
            Self {
                evidence: BTreeMap::new(),
                case,
            }
        }

        #[ink(message)]
        pub fn set_evidence(&mut self, evidence: EvidenceNFT) {
            let length: Id = (self.evidence.len() as Id).checked_add(1).unwrap();
            self.evidence.insert(length, evidence);
        }

        #[ink(message)]
        pub fn burn_evidence(&mut self, evidence_id: Id) -> Result<(), Error> {
            if !self.evidence.contains_key(&evidence_id) {
                return Err(Error::EvidenceNotFound)
            };
            self.evidence.remove(&evidence_id);
            Ok(())
        }

        #[ink(message)]
        pub fn update_evidence(&mut self, evidence_id: Id, new_evidence: EvidenceNFT) -> Result<(), Error> {
            let evidence: &mut EvidenceNFT = self
                .evidence
                .get_mut(&evidence_id)
                .ok_or(Error::EvidenceNotFound)?;
            *evidence = new_evidence;
            Ok(())
        }

        #[ink(message)]
        pub fn get_evidence_by_id(&self, evidence_id: Id) -> Option<EvidenceNFTOutput> {
            if let Some(evidence) = self.evidence.get(&evidence_id) {
                let case_title: Option<String> = self.case.get_case_title(evidence.case_id);
                let evidence: EvidenceNFTOutput = EvidenceNFTOutput::get_evidence(evidence_id, case_title, evidence);
                Some(evidence)
            } else {
                None
            }
        }

        #[ink(message)]
        pub fn get_all_evidence(&self) -> Vec<EvidenceNFTOutput> {
            let evidence: Vec<EvidenceNFTOutput> = self
                .evidence
                .iter()
                .map(|(evidence_id, evidence)| {
                    let case_title = self.case.get_case_title(evidence.case_id);
                    EvidenceNFTOutput::get_evidence(*evidence_id, case_title, evidence)
                })
                .collect();
            evidence
        }

        #[ink(message)]
        pub fn evidence_by_case_id(&self, case_id: Id) -> Vec<EvidenceNFTOutput> {
            let evidence: Vec<EvidenceNFTOutput> = self
                .evidence
                .iter()
                .filter_map(|(evidence_id, evidence)| {
                    if case_id == evidence.case_id {
                        let case_title = self.case.get_case_title(evidence.case_id);
                        Some(EvidenceNFTOutput::get_evidence(*evidence_id, case_title, evidence))
                    } else {
                        None
                    }
                })
                .collect();
            evidence
        }

        #[ink(message)]
        pub fn get_evidence_id(&self, evidence_id: Id) -> Id {
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
