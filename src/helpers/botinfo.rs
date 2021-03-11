use serenity::{
    client::{bridge::gateway::ShardId, Context},
    framework::standard::CommandResult,
};
use std::{env, process};
use tokio::process::Command;

use crate::structures::{
    cmd_data::{ReqwestClient, ShardManagerContainer},
    CommitResponse, SysInfo,
};

pub async fn get_last_commit(
    ctx: &Context,
) -> Result<CommitResponse, Box<dyn std::error::Error + Send + Sync>> {
    let reqwest_client = ctx
        .data
        .read()
        .await
        .get::<ReqwestClient>()
        .cloned()
        .unwrap();

    let resp = reqwest_client
        .get("https://api.github.com/repos/bdashore3/courtjester/commits/serenity")
        .send()
        .await?
        .json::<CommitResponse>()
        .await?;

    Ok(resp)
}

pub async fn get_system_info(ctx: &Context) -> CommandResult<SysInfo> {
    let shard_manager = ctx
        .data
        .read()
        .await
        .get::<ShardManagerContainer>()
        .cloned()
        .unwrap();
    let mut sys_info = SysInfo::default();

    sys_info.shard_latency = {
        let manager = shard_manager.lock().await;
        let runners = manager.runners.lock().await;

        let runner_raw = runners.get(&ShardId(ctx.shard_id));
        match runner_raw {
            Some(runner) => match runner.latency {
                Some(ms) => format!("{}ms", ms.as_millis()),
                None => "?ms".to_string(),
            },
            None => "?ms".to_string(),
        }
    };

    let pid = process::id();

    let raw_bin_path = env::current_exe()?;
    let bin_path = raw_bin_path.to_string_lossy();
    let bin_str = bin_path.rsplit('/').next().unwrap();

    let mem_stdout = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "pmap {} | grep {} | awk 'NR>1 {{sum+=substr($2, 1, length($2)-1)}} END {{print sum}}'",
            pid, bin_str
        ))
        .output()
        .await
        .expect("failed to execute process");

    let mem_used = String::from_utf8(mem_stdout.stdout).unwrap();

    sys_info.memory = &mem_used[..mem_used.len() - 1].parse::<f32>().unwrap() / 1000f32;

    Ok(sys_info)
}
