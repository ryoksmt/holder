use crate::entity::healthcheck_entity::*;
pub trait HealthcheckUsecase {
    fn healthcheck(&self) -> Letter;
}