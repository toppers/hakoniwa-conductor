use tonic::{ Request, Response, Status};
//use futures_core::Stream;
//use std::pin::Pin;
//use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub mod hakoniwa {
    tonic::include_proto!("hakoniwa");
}
use hakoniwa::{
    core_service_server::CoreService,
    ErrorCode, AssetInfo, NormalReply,
    AssetInfoList, SimStatReply, SimulationStatus,
    SimulationTimeSyncOutputFile,
    AssetNotification, AssetNotificationReply,
    AssetNotificationEvent
};

#[derive(Debug, Default)]
pub struct HakoCoreService {}

#[tonic::async_trait]
impl CoreService for HakoCoreService {
    async fn register(
        &self,
        request: Request<AssetInfo>,
    ) -> Result<Response<NormalReply>, Status>
    {
        println!("register: Got a request: {:?}", request);

        let reply = hakoniwa::NormalReply {
            ercd: ErrorCode::Ok as i32,
        };
        //TODO
        Ok(Response::new(reply))
    }

    async fn unregister(
        &self,
        request: Request<AssetInfo>,
    ) -> Result<Response<NormalReply>, Status>
    {
        println!("unregister: Got a request: {:?}", request);

        let reply = hakoniwa::NormalReply {
            ercd: ErrorCode::Ok as i32,
        };
        //TODO
        Ok(Response::new(reply))   
    }
    async fn get_asset_list(
        &self,
        request: Request<()>,
    ) -> Result<Response<AssetInfoList>, Status>
    {
        println!("get_asset_list: Got a request: {:?}", request);
        let mut assets = prost::alloc::vec::Vec::<AssetInfo>::new();
        let asset = AssetInfo {
            name: String::from("TestAsset")
        };
        assets.push(asset);

        let reply = hakoniwa::AssetInfoList {
            assets: assets
        };
        //TODO
        Ok(Response::new(reply))
    }
    /// シミュレーションを開始する
    async fn start_simulation(
        &self,
        request: Request<()>,
    ) -> Result<Response<NormalReply>, Status>
    {
        println!("start_simulation: Got a request: {:?}", request);

        let reply = hakoniwa::NormalReply {
            ercd: ErrorCode::Ok as i32,
        };
        //TODO
        Ok(Response::new(reply))

    }
    /// シミュレーションを終了する
    async fn stop_simulation(
        &self,
        request: Request<()>,
    ) -> Result<Response<NormalReply>, Status>
    {
        println!("stop_simulation: Got a request: {:?}", request);

        let reply = hakoniwa::NormalReply {
            ercd: ErrorCode::Ok as i32,
        };
        //TODO
        Ok(Response::new(reply))   
    }
    /// シミュレーション実行状況を取得する
    async fn get_sim_status(
        &self,
        request: Request<()>,
    ) -> Result<Response<SimStatReply>, Status>
    {
        println!("reset_simulation: Got a request: {:?}", request);

        let reply = hakoniwa::SimStatReply {
            ercd: ErrorCode::Ok as i32,
            status: SimulationStatus::StatusStopped as i32
        };
        //TODO
        Ok(Response::new(reply))
    }
    /// シミュレーションを実行開始状態に戻す
    async fn reset_simulation(
        &self,
        request: Request<()>,
    ) -> Result<Response<NormalReply>, Status>
    {
        println!("reset_simulation: Got a request: {:?}", request);

        let reply = hakoniwa::NormalReply {
            ercd: ErrorCode::Ok as i32,
        };
        //TODO
        Ok(Response::new(reply))
    }
    /// シミュレーション時間同期度合いを取得する
    async fn flush_simulation_time_sync_info(
        &self,
        request: Request<SimulationTimeSyncOutputFile>,
    ) -> Result<Response<NormalReply>, Status>
    {
        println!("flush_simulation_time_sync_info: Got a request: {:?}", request);

        let reply = hakoniwa::NormalReply {
            ercd: ErrorCode::Ok as i32,
        };
        //TODO
        Ok(Response::new(reply))
    }
    //type AssetNotificationStartStream = Pin<Box<dyn Stream<Item = Result<AssetNotification, Status>> + Send  + 'static>>;
    type AssetNotificationStartStream = ReceiverStream<Result<AssetNotification, Status>>;
    /// 箱庭アセット非同期通知
    async fn asset_notification_start(
        &self,
        request: Request<AssetInfo>,
    ) -> Result<Response<Self::AssetNotificationStartStream>, Status>
    {
        println!("asset_notification_start: Got a request: {:?}", request);
        let (tx, rx) = mpsc::channel::<Result<AssetNotification, Status>>(4);

        tokio::spawn(async move {
            loop {
                //TODO get event
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                let ev = hakoniwa::AssetNotification {
                    event: AssetNotificationEvent::Heartbeat as i32,
                };
                tx.send(Ok(ev)).await.unwrap();
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }
    async fn asset_notification_feedback(
        &self,
        request: Request<AssetNotificationReply>,
    ) -> Result<tonic::Response<NormalReply>, tonic::Status>
    {
        println!("asset_notification_feedback: Got a request: {:?}", request);

        let reply = hakoniwa::NormalReply {
            ercd: ErrorCode::Ok as i32,
        };
        //TODO
        Ok(Response::new(reply))
    }

}
