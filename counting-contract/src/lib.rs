use cosmwasm_std::{Deps, DepsMut, Empty, entry_point, Env, MessageInfo, QueryResponse, Response, StdResult};

#[entry_point]
pub fn instantiate(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[derive(Serialize, Deserialize)]
struct QueryMsg {
}
impl<T: de::DeserializeOwned> de::DeserializeOwned for QueryMsg {}

#[entry_point]
pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> StdResult<QueryResponse> {
    Result::Ok(QueryResponse::default())
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
