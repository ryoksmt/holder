use crate::repository::healthcheck_repository::*;
use crate::usecase::healthcheck_usecase::HealthcheckUsecase;
use crate::entity::healthcheck_entity::Letter;

pub struct HealthcheckInteractor{
    repository: HealthcheckRepository
}

pub fn new_healthcheck_usecase(repository:HealthcheckRepository) -> HealthcheckInteractor {
    HealthcheckInteractor{
        repository:repository,
    }
}

impl HealthcheckUsecase for HealthcheckInteractor  {
    fn healthcheck() -> Letter {
        Letter{ title:"hoge".to_string() ,message:"hoge".to_string()}
    }
}