use tonic::{ transport::Server, Request, Response, Status};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub mod hakoniwa {
    tonic::include_proto!("hakoniwa");
}
use crate::hako;

use hakoniwa::{
    core_service_server:: { CoreService, CoreServiceServer },
    ErrorCode, AssetInfo, NormalReply,
    AssetInfoList, SimStatReply, SimulationStatus,
    SimulationTimeSyncOutputFile,
    AssetNotification, AssetNotificationReply,
    AssetNotificationEvent,
    NotifySimtimeRequest, NotifySimtimeReply,
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
        let req = request.into_inner();
        let result = hako::asset_register_polling(req.name);
        if result {
            let reply = hakoniwa::NormalReply {
                ercd: ErrorCode::Ok as i32,
            };
            Ok(Response::new(reply))
        }
        else {
            let reply = hakoniwa::NormalReply {
                ercd: ErrorCode::Exist as i32,
            };
            Ok(Response::new(reply))
        }
    }

    async fn unregister(
        &self,
        request: Request<AssetInfo>,
    ) -> Result<Response<NormalReply>, Status>
    {
        println!("unregister: Got a request: {:?}", request);

        let req = request.into_inner();
        let result = hako::asset_unregister(req.name);
        if result {
            let reply = hakoniwa::NormalReply {
                ercd: ErrorCode::Ok as i32,
            };
            Ok(Response::new(reply))
        }
        else {
            let reply = hakoniwa::NormalReply {
                ercd: ErrorCode::Inval as i32,
            };
            Ok(Response::new(reply))
        }
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
        let req = request.into_inner();

        tokio::spawn(async move {
            loop {
                let ev = hako::asset_get_event(req.name.clone());
                match ev {
                    hako::SimulationAssetEventType::None => {
                        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                        let ev = hakoniwa::AssetNotification {
                            event: AssetNotificationEvent::Heartbeat as i32,
                        };
                        tx.send(Ok(ev)).await.unwrap();
                    },
                    hako::SimulationAssetEventType::Start => {
                        let ev = hakoniwa::AssetNotification {
                            event: AssetNotificationEvent::Start as i32,
                        };
                        tx.send(Ok(ev)).await.unwrap();
                    },
                    hako::SimulationAssetEventType::Stop => {
                        let ev = hakoniwa::AssetNotification {
                            event: AssetNotificationEvent::End as i32,
                        };
                        tx.send(Ok(ev)).await.unwrap();
                    },
                    _ => {
                        println!("Invalid event {:?}", ev);
                    },
                }

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
        let req = request.into_inner();
        let asset_info = req.asset.unwrap();
        let ercd = req.ercd;
        let event = req.event;
        let mut result = true;
        if ercd != ErrorCode::Ok as i32 {
            result = false;
        }
        if event == AssetNotificationEvent::Start as i32 {
            hako::asset_start_feedback(asset_info.name, result);
        }
        else if event == AssetNotificationEvent::End as i32 {
            hako::asset_stop_feedback(asset_info.name, result);
        }
        else if event == AssetNotificationEvent::Heartbeat as i32 {
            //nothing to do
        }
        let reply = hakoniwa::NormalReply {
            ercd: ErrorCode::Ok as i32,
        };
        Ok(Response::new(reply))
    }
    /// 箱庭シミュレーション時間取得
    async fn notify_simtime(
        &self,
        request: Request<NotifySimtimeRequest>,
    ) -> Result<Response<NotifySimtimeReply>, Status>
    {
        println!("notify_simtime: Got a request: {:?}", request);

        let req = request.into_inner();
        let asset_info = req.asset.unwrap();
        hako::asset_notify_simtime(asset_info.name, req.asset_time);
        let master_time: i64 = hako::asset_get_worldtime();
        //println!("master_time={}", master_time);
        let reply = hakoniwa::NotifySimtimeReply {
            ercd: ErrorCode::Ok as i32,
            master_time: master_time as i64
        };
        Ok(Response::new(reply))
    }

}

pub async fn start_service() -> Result<(), Box<dyn std::error::Error>>
{
    println!("hello world");
    let addr = "[::1]:50051".parse().unwrap();
    let service = HakoCoreService::default();

    hako::asset_init();

    println!("Server Start: {:?}", addr);
    Server::builder()
    .add_service(CoreServiceServer::new(service))
    .serve(addr)
    .await?;

    Ok(())
}

pub fn stop_service()
{
    //TODO
}