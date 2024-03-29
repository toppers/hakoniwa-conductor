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
    CreatePduChannelRequest, CreatePduChannelReply,
    SubscribePduChannelRequest, SubscribePduChannelReply
};
fn get_status() -> SimulationStatus
{
    let state = hako::api::simevent_get_state();
    match state {
        hako::api::SimulationStateType::Runnable => {
            return SimulationStatus::StatusRunnable;
        },
        hako::api::SimulationStateType::Running => {
            return SimulationStatus::StatusRunning;
        },
        hako::api::SimulationStateType::Stopped => {
            return SimulationStatus::StatusStopped;
        },
        hako::api::SimulationStateType::Stopping => {
            return SimulationStatus::StatusStopping;
        },
        hako::api::SimulationStateType::Error => {
            return SimulationStatus::StatusTerminated;
        },
        _ => {
            return SimulationStatus::StatusTerminated;
        },
    }
}

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
        let result = hako::api::asset_register_polling(req.name);
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
        let result = hako::api::asset_unregister(req.name);
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

        let result = hako::api::simevent_start();
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
    /// シミュレーションを終了する
    async fn stop_simulation(
        &self,
        request: Request<()>,
    ) -> Result<Response<NormalReply>, Status>
    {
        println!("stop_simulation: Got a request: {:?}", request);

        let result = hako::api::simevent_stop();
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
    /// シミュレーション実行状況を取得する
    async fn get_sim_status(
        &self,
        request: Request<()>,
    ) -> Result<Response<SimStatReply>, Status>
    {
        println!("reset_simulation: Got a request: {:?}", request);
 
        let state = get_status();
        let reply = hakoniwa::SimStatReply {
            ercd: ErrorCode::Ok as i32,
            status: state as i32,
        };
        Ok(Response::new(reply))
    }
    /// シミュレーションを実行開始状態に戻す
    async fn reset_simulation(
        &self,
        request: Request<()>,
    ) -> Result<Response<NormalReply>, Status>
    {
        println!("reset_simulation: Got a request: {:?}", request);

        let result = hako::api::simevent_reset();
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
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                let ev = hako::api::asset_get_event(req.name.clone());
                match ev {
                    hako::api::SimulationAssetEventType::None => {
                        let ev = hakoniwa::AssetNotification {
                            event: AssetNotificationEvent::Heartbeat as i32,
                        };
                        //println!("## SimulationAssetEvent NONE");
                        tx.send(Ok(ev)).await.unwrap();
                    },
                    hako::api::SimulationAssetEventType::Start => {
                        let ev = hakoniwa::AssetNotification {
                            event: AssetNotificationEvent::Start as i32,
                        };
                        if hako::method::mqtt::is_enabled() {
                            hako::method::mqtt::activate_server("hako-mqtt-subscriber-server");
                        }
                        println!("## SimulationAssetEvent START");
                        tx.send(Ok(ev)).await.unwrap();
                    },
                    hako::api::SimulationAssetEventType::Stop => {
                        let ev = hakoniwa::AssetNotification {
                            event: AssetNotificationEvent::Stop as i32,
                        };
                        println!("## SimulationAssetEvent STOP");
                        tx.send(Ok(ev)).await.unwrap();
                    },
                    hako::api::SimulationAssetEventType::Reset => {
                        let ev = hakoniwa::AssetNotification {
                            event: AssetNotificationEvent::Reset as i32,
                        };
                        println!("## SimulationAssetEvent RESET");
                        tx.send(Ok(ev)).await.unwrap();
                    },
                    hako::api::SimulationAssetEventType::Error => {
                        let ev = hakoniwa::AssetNotification {
                            event: AssetNotificationEvent::Error as i32,
                        };
                        println!("## SimulationAssetEvent ERROR");
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
            hako::api::asset_start_feedback(asset_info.name, result);
        }
        else if event == AssetNotificationEvent::Stop as i32 {
            hako::api::asset_stop_feedback(asset_info.name, result);
        }
        else if event == AssetNotificationEvent::Reset as i32 {
            hako::api::asset_reset_feedback(asset_info.name, result);
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
        //println!("notify_simtime: Got a request: {:?}", request);

        let req = request.into_inner();
        let asset_info = req.asset.unwrap();
        hako::api::asset_notify_simtime(asset_info.name.clone(), req.asset_time);

        if req.is_read_pdu_done {
            hako::api::asset_notify_read_pdu_done(asset_info.name.clone());
        }
        if req.is_write_pdu_done {
            hako::api::asset_notify_write_pdu_done(asset_info.name.clone());
        }

        let master_time: i64 = hako::api::asset_get_worldtime();
        //println!("master_time={}", master_time);
        let reply = hakoniwa::NotifySimtimeReply {
            ercd: ErrorCode::Ok as i32,
            master_time: master_time as i64,
            is_pdu_created: hako::api::asset_is_pdu_created(),
            is_pdu_sync_mode: hako::api::asset_is_pdu_sync_mode(asset_info.name.clone()),
            is_simulation_mode: hako::api::asset_is_simulation_mode(),
            status: get_status() as i32
        };
        Ok(Response::new(reply))
    }
    /// 箱庭PDUチャネル作成
    async fn create_pdu_channel(
        &self,
        request: Request<CreatePduChannelRequest>,
    ) -> Result<Response<CreatePduChannelReply>, Status> {
        println!("create_pdu_channel: Got a request: {:?}", request);

        let req = request.into_inner();

        let method_type: String = req.method_type;
        let result = hako::asset_create_pdu_channel(req.asset_name, req.robo_name, req.channel_id, req.pdu_size, method_type.clone());
        if result {
            if method_type == "UDP" {
                let reply = hakoniwa::CreatePduChannelReply {
                    ercd: ErrorCode::Ok as i32,
                    port: hako::method::udp::get_server_port() as i32
                };
                Ok(Response::new(reply))
            }
            else {
                let reply = hakoniwa::CreatePduChannelReply {
                    ercd: ErrorCode::Ok as i32,
                    port: hako::method::mqtt::get_mqtt_port() as i32
                };    
                Ok(Response::new(reply))
            }
        }
        else {
            let reply = hakoniwa::CreatePduChannelReply {
                ercd: ErrorCode::Inval as i32,
                port: -1 as i32
            };
            Ok(Response::new(reply))
        }
    }
    /// 箱庭PDUチャネル購読
    async fn subscribe_pdu_channel(
        &self,
        request: Request<SubscribePduChannelRequest>,
    ) -> Result<Response<SubscribePduChannelReply>, Status> {
        println!("subscribe_pdu_channel: Got a request: {:?}", request);

        let req = request.into_inner();
        let method_type: String = req.method_type;
        let result = hako::pdu::create_asset_sub_pdu(req.asset_name, req.robo_name, req.channel_id, req.pdu_size, req.listen_udp_ip_port, method_type);
        if result {
            let reply = hakoniwa::SubscribePduChannelReply {
                ercd: ErrorCode::Ok as i32
            };
            Ok(Response::new(reply))
        }
        else {
            let reply = hakoniwa::SubscribePduChannelReply {
                ercd: ErrorCode::Inval as i32
            };
            Ok(Response::new(reply))
        }
    }
}

pub async fn start_service(ip_port: &String) -> Result<(), Box<dyn std::error::Error>>
{
    let addr = ip_port.parse().unwrap();
    let service = HakoCoreService::default();

    hako::api::asset_init();

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