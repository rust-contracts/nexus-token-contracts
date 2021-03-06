use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use std::env::current_dir;
use std::fs::create_dir_all;

use services::governance::{
    AnyoneMsg, ConfigResponse, Cw20HookMsg, ExecuteMsg, GovernanceMsg, InstantiateMsg, MigrateMsg,
    PollCountResponse, PollExecuteMsg, PollResponse, PollStatus, PollsResponse, QueryMsg,
    StakerResponse, StateResponse, VoteOption, VoterInfo, VotersResponse, VotersResponseItem,
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(AnyoneMsg), &out_dir);
    export_schema(&schema_for!(GovernanceMsg), &out_dir);
    export_schema(&schema_for!(Cw20HookMsg), &out_dir);
    export_schema(&schema_for!(PollExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(ConfigResponse), &out_dir);
    export_schema(&schema_for!(StateResponse), &out_dir);
    export_schema(&schema_for!(PollResponse), &out_dir);
    export_schema(&schema_for!(PollsResponse), &out_dir);
    export_schema(&schema_for!(PollCountResponse), &out_dir);
    export_schema(&schema_for!(StakerResponse), &out_dir);
    export_schema(&schema_for!(VotersResponseItem), &out_dir);
    export_schema(&schema_for!(VotersResponse), &out_dir);
    export_schema(&schema_for!(VoterInfo), &out_dir);
    export_schema(&schema_for!(PollStatus), &out_dir);
    export_schema(&schema_for!(VoteOption), &out_dir);
    export_schema(&schema_for!(MigrateMsg), &out_dir);
}
