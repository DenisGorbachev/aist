use crate::{Check, CheckError, panic_message};
use errgonomic::{ErrorDisplayer, exit_result, handle};
use std::error::Error;
use std::fmt::Debug;
use std::io::{self, Write, stdout};
use std::panic::{AssertUnwindSafe, UnwindSafe, catch_unwind};
use std::process::ExitCode;
use thiserror::Error as ThisError;

#[derive(Default, Debug)]
pub struct ContractReport {
    pub checks: Vec<Check>,
}

impl ContractReport {
    pub fn require(&mut self, name: impl Into<String>, predicate: bool) {
        let name = name.into();
        let result = if predicate {
            Ok(())
        } else {
            Err(CheckError::PredicateInvalid {
                predicate: name.clone(),
            })
        };
        self.checks.push(Check {
            name,
            result,
        });
    }

    pub fn equal<T: PartialEq + Debug>(&mut self, name: impl Into<String>, actual: T, expected: T) {
        let result = if actual == expected {
            Ok(())
        } else {
            Err(CheckError::EqualityInvalid {
                expected: format!("{expected:?}"),
                actual: format!("{actual:?}"),
            })
        };
        self.checks.push(Check {
            name: name.into(),
            result,
        });
    }

    pub fn error(&mut self, name: impl Into<String>, source: impl Error + 'static) {
        self.checks.push(Check {
            name: name.into(),
            result: Err(CheckError::OperationFailed {
                source: Box::new(source),
            }),
        });
    }

    pub fn unverified(&mut self, name: impl Into<String>, predicate: impl Into<String>) {
        self.checks.push(Check {
            name: name.into(),
            result: Err(CheckError::ProofNotFound {
                predicate: predicate.into(),
            }),
        });
    }

    pub fn run(&mut self, name: &str, contract: impl FnOnce(&mut Self) + UnwindSafe) {
        let mut report = Self::default();
        let result = catch_unwind(AssertUnwindSafe(|| contract(&mut report)));
        match result {
            Ok(()) if report.checks.is_empty() => report.error("completion", CheckError::ChecksNotFound {}),
            Ok(()) => (),
            Err(payload) => report.error(
                "completion",
                CheckError::PanickedFailed {
                    message: panic_message(payload.as_ref()).to_owned(),
                },
            ),
        }
        self.checks
            .extend(report.checks.into_iter().map(|check| Check {
                name: format!("{name}/{}", check.name),
                ..check
            }));
    }

    pub fn passed(&self) -> bool {
        !self.checks.is_empty() && self.checks.iter().all(|check| check.result.is_ok())
    }

    pub fn print(&self) -> ExitCode {
        exit_result(
            self.write(stdout().lock())
                .map(|()| if self.passed() { ExitCode::SUCCESS } else { ExitCode::FAILURE }),
        )
    }

    pub fn write(&self, mut writer: impl Write) -> Result<(), ContractReportWriteError> {
        use ContractReportWriteError::*;
        let result = self
            .checks
            .iter()
            .try_for_each(|check| match &check.result {
                Ok(()) => writeln!(writer, "PASS {}", check.name),
                Err(error) => writeln!(writer, "FAIL {}\n{}", check.name, ErrorDisplayer(error)),
            });
        handle!(result, WriteFailed);
        Ok(())
    }
}

#[derive(ThisError, Debug)]
pub enum ContractReportWriteError {
    #[error("failed to write contract report")]
    WriteFailed { source: io::Error },
}
