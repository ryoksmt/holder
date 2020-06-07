use crate::usecase::healthcheck_usecase::*;

pub struct HealthcheckInteractor;

pub fn new_healthcheck_usecase(){
    HealthcheckUseca
}

impl HealthcheckUsecase for HealthcheckInteractor  {
    fn healthcheck(&self) -> Letter {
        Letter{ title:"hoge" ,message:"hoge"}
    }
}