use crate::{ReviewEvidence, ReviewSnapshot, ReviewStatus};
use errgonomic::handle_bool;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ReviewVerdict {
    pub status: ReviewStatus,
    pub rationale: String,
    pub evidence: Vec<ReviewEvidence>,
}

impl ReviewVerdict {
    pub fn into_result(self) -> Result<(), ReviewVerdictIntoResultError> {
        use ReviewVerdictIntoResultError::*;
        handle_bool!(self.status != ReviewStatus::Satisfied, RequirementInvalid, verdict: self);
        Ok(())
    }

    pub fn validate(&self, snapshot: &ReviewSnapshot) -> Result<(), ReviewVerdictValidateError> {
        use ReviewVerdictValidateError::*;
        handle_bool!(self.rationale.trim().is_empty(), RationaleInvalid, verdict: self.to_owned());
        handle_bool!(self.status != ReviewStatus::Inconclusive && self.evidence.is_empty(), EvidenceInvalid, verdict: self.to_owned());
        handle_bool!(self.evidence.iter().any(|evidence| !snapshot.files().get(&evidence.path).is_some_and(|source| evidence.line.get() <= source.lines) || evidence.explanation.trim().is_empty()), EvidenceInvalid, verdict: self.to_owned());
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ReviewVerdictValidateError {
    #[error("review rationale is empty")]
    RationaleInvalid { verdict: Box<ReviewVerdict> },
    #[error("review evidence is missing or references an uncaptured path or nonexistent line")]
    EvidenceInvalid { verdict: Box<ReviewVerdict> },
}

#[derive(Error, Debug)]
pub enum ReviewVerdictIntoResultError {
    #[error("requirement was not satisfied: {rationale}", rationale = verdict.rationale)]
    RequirementInvalid { verdict: Box<ReviewVerdict> },
}
