use serenity::client::{bridge::gateway::ShardId, Context};
use crate::{
    structures::{
        CommitResponse,
        SysInfo,
        cmd_data::{ShardManagerContainer, ReqwestClient}, 
    }
};
use tokio::process::Command;

pub async fn get_last_commit(ctx: &Context) -> Result<CommitResponse, Box<dyn std::error::Error + Send + Sync>> {
    let data = ctx.data.read().await;
    let reqwest_client = data.get::<ReqwestClient>().unwrap();

    let resp = reqwest_client
        .get("https://api.github.com/repos/bdashore3/courtjester/commits/serenity")
        .send().await?
        .json::<CommitResponse>().await?;
    
    Ok(resp)
}

pub async fn get_system_info(ctx: &Context) -> SysInfo {
    let data = ctx.data.read().await;
    let mut sys_info = SysInfo::default();

    sys_info.shard_latency = {
        let shard_manager = data.get::<ShardManagerContainer>().unwrap();

        let manager = shard_manager.lock().await;
        let runners = manager.runners.lock().await;
    
        let runner_raw = runners.get(&ShardId(ctx.shard_id));
        match runner_raw {
            Some(runner) => {
                match runner.latency {
                    Some(ms) => format!("{}ms", ms.as_millis()),
                    None => "?ms".to_string()
                }
            },
            None => "?ms".to_string()
        }
    };

    let pid = std::process::id().to_string();

    let mem_stdout = Command::new("sh")
            .arg("-c")
            .arg(format!("pmap {} | head -n 3 | tail -n 1 | awk '/[0-9]K/{{print $2}}'", &pid).as_str())
            .output()
            .await
            .expect("failed to execute process");

    let mem_used = String::from_utf8(mem_stdout.stdout).unwrap();

    sys_info.memory = &mem_used[..mem_used.len() - 2].parse::<f32>().unwrap()/1000f32;

    sys_info
}