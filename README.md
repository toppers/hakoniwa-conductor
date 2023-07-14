[![Build](https://github.com/toppers/hakoniwa-conductor/actions/workflows/build.yml/badge.svg)](https://github.com/toppers/hakoniwa-conductor/actions/workflows/build.yml)

箱庭コンダクタは、分散配置されているマシン間で、箱庭上にいるアセット間のシミュレーション調停する役割をするデーモンプロセスです。

![image](https://github.com/toppers/hakoniwa-conductor/assets/164193/80675925-270e-4adc-b71b-68db36e30c71)

箱庭コンダクタは、サーバーとクライアントに分かれており、本リポジトリでは、Rust版の箱庭コンダクタ（サーバー/クライアント）機能を提供しています。


サーバー/クライアント間の調停(シミュレーションの開始/停止等)は、gRPC で実現しています。
ただし、PDU間のデータ通信は用途に応じて、UDP or MQTT いずれかを選択できます。


なお、箱庭コンダクタは、箱庭シミュレーション環境に同梱されるモジュールであるため、本リポジトリ単体で箱庭シミュレーションを実施することはできません。

# 箱庭コンダクタの動作確認手順

本リポジトリでは、箱庭コンダクタの動作確認用のテスト環境を用意しています。

## テスト環境

箱庭コンダクタの動作確認環境は以下の通りです。

* OS: Windows 10/11 WSL2 && Docker
* Python3

## テスト構成

* クライアント側の箱庭アセット
  * hakoniwa-conductor/test/workspace/client/asset-client-tester.py
* サーバー側の箱庭アセット
  * hakoniwa-conductor/test/workspace/server/asset-srv-tester.py 

## 通信構成

![image](https://github.com/toppers/hakoniwa-conductor/assets/164193/1a8d3b77-5738-4dae-a0f9-ae290e99e352)


* UDP 通信
  * hakoniwa-conductor/test/workspace/spec/custom.json
* MQTT 通信
  * hakoniwa-conductor/test/workspace/spec/custom_mqtt.json
* 箱庭コンダクタのクライアントのコンフィグ
  * hakoniwa-conductor/test/workspace/client/conductor_config.json
    * IPアドレスは、WSL2上の eth0 のIPアドレスを設定してください。
    * MQTTのテストをする場合は、mqtt_portnoの値を `1883` にしてください。

## インストール手順

ここでは、project というディレクトリが /mnt/c 直下に存在することを前提とします（ご自身の環境に応じて読み替えてください）。


## C++版箱庭コア機能側

C++版箱庭コア機能をインストールします。

```
cd project
git clone --recursive https://github.com/toppers/hakoniwa-core-cpp-client.git
cd hakoniwa-core-cpp-client
bash build.bash
bash install.bash
```

## 箱庭コンダクタ側


箱庭コンダクタをビルドします。

```
cd project
git clone --recursive https://github.com/toppers/hakoniwa-conductor.git
cd hakoniwa-conductor/main
bash build.bash
```

dockerイメージを作成します。

```
cd ../test
bash docker/create-image.bash 
```

最後に、`hakoniwa-conductor/test/workspace/client/conductor_config.json`のIPアドレスを、WSL2上の eth0 のIPアドレスを設定してください。

## 動作確認手順

端末を３個用意します。

* 端末A：箱庭コンダクタのサーバー側
* 端末B：箱庭コンダクタのクライアント側
* 端末C：箱庭コンダクタのサーバー側（コマンド実行用）

端末A：箱庭コンダクタのサーバーとサーバー側のアセットを起動します。

```
cd test
bash docker/run.bash
```

成功するとこうなります。

```
OPEN RECIEVER UDP PORT=172.25.195.216:54001
OPEN SENDER UDP PORT=172.25.195.216:54002
delta_msec = 20
max_delay_msec = 100
INFO: shmget() key=255 size=80768
Server Start: 172.25.195.216:50051
INFO: ACTIVATING :server/asset-srv-tester.py
START TEST
LOADED: TB3RoboModel
INFO: TB3RoboModel create_lchannel: logical_id=2 real_id=0 size=256
subscribe:channel_id=1
subscribe:typename=String
subscribe:pdu_size=256
WAIT START:
```

端末B：箱庭コンダクタのクライアントとクライアント側のアセットを起動します。

```
cd test/workspace
bash client/activate-client.bash  spec/custom_mqtt.json
```

成功するとこうなります。

```
ACTIVATING CONDUCTOR(CLIENT)
Conductor asset_name: ConductorClient-01
Conductor core_ipaddr: 172.25.195.216
Conductor core_portno: 50051
Conductor delta_msec: 20
Conductor max_delay_msec: 100
Conductor udp_server_port: 172.25.195.216:51001
Conductor udp_sender_port: 172.25.195.216:51002
Conductor mqtt_portno: -1
Conductor mqtt_pub_client_id: hako-mqtt-publisher-client-01
Conductor mqtt_sub_client_id: hako-mqtt-subscriber-client-01
-------------------
INFO: shmget() key=255 size=80768
INFO: asset_register_polling() success
Register response: NormalReply { ercd: Ok }
Robot Name: TB3RoboModel
RPC PDU Readers:
  - Type: std_msgs/String
    Org Name: ch2
    Name: TB3RoboModel_ch2
RPC PDU Writers:
  - Type: std_msgs/String
    Org Name: ch1
    Name: TB3RoboModel_ch1
-------------------
INFO: TB3RoboModel create_lchannel: logical_id=2 real_id=0 size=256
Subscribe Pdu Channel Robot Name: TB3RoboModel Channel: 2 real_id: 0
SubscribePduChannel response: SubscribePduChannelReply { ercd: Ok }
create_asset_pub_pdu: robo_name=TB3RoboModel channel_id=2 real_id=0
create_asset_pub_pdu: channel_ID=2
CreatePduChannel response: CreatePduChannelReply { ercd: Ok, port: 54001 }
create_asset_sub_pdu
OPEN SENDER UDP PORT=172.25.195.216:51002
OPEN RECIEVER UDP PORT=172.25.195.216:51001
ACTIVATING PYTHON PROG
START TEST
LOADED: TB3RoboModel
INFO: TB3RoboModel create_lchannel: logical_id=1 real_id=1 size=256
subscribe:channel_id=2
subscribe:typename=String
subscribe:pdu_size=256
WAIT START:
```

端末C：シミュレーション開始します。

```
cd test
bash docker/attach.bash
```

docker コンテナ起動後、以下のコマンドでシミュレーション開始します。

```
hako-cmd start
```

成功するとそれぞれの端末A/Bで以下のようにログ出力されます。

端末A：
```
register: Got a request: Request { metadata: MetadataMap { headers: {"te": "trailers", "content-type": "application/grpc", "user-agent": "tonic/0.8.3"} }, message: AssetInfo { name: "ConductorClient-01" }, extensions: Extensions }
subscribe_pdu_channel: Got a request: Request { metadata: MetadataMap { headers: {"te": "trailers", "content-type": "application/grpc", "user-agent": "tonic/0.8.3"} }, message: SubscribePduChannelRequest { asset_name: "ConductorClient-01", channel_id: 2, pdu_size: 256, listen_udp_ip_port: "172.25.195.216:51001", method_type: "UDP", robo_name: "TB3RoboModel" }, extensions: Extensions }
create_asset_sub_pdu
create_pdu_channel: Got a request: Request { metadata: MetadataMap { headers: {"te": "trailers", "content-type": "application/grpc", "user-agent": "tonic/0.8.3"} }, message: CreatePduChannelRequest { asset_name: "ConductorClient-01", channel_id: 1, pdu_size: 256, method_type: "UDP", robo_name: "TB3RoboModel" }, extensions: Extensions }
INFO: TB3RoboModel create_lchannel: logical_id=1 real_id=1 size=256
create_asset_pub_pdu: robo_name=TB3RoboModel channel_id=1 real_id=1
create_asset_pub_pdu: channel_ID=1
asset_notification_start: Got a request: Request { metadata: MetadataMap { headers: {"te": "trailers", "content-type": "application/grpc", "user-agent": "tonic/0.8.3"} }, message: AssetInfo { name: "ConductorClient-01" }, extensions: Extensions }
WAIT RUNNING:
## SimulationAssetEvent START
asset_notification_feedback: Got a request: Request { metadata: MetadataMap { headers: {"te": "trailers", "content-type": "application/grpc", "user-agent": "tonic/0.8.3"} }, message: AssetNotificationReply { event: Start, asset: Some(AssetInfo { name: "ConductorClient-01" }), ercd: Ok }, extensions: Extensions }
START CREATE PDU DATA: total_size= 512
INFO: shmget() key=256 size=512
PDU DATA CREATED
CREATED ADDR=0x7fe9a4dcd00c
WAIT PDU CREATED:
LOADED: PDU DATA
sync_mode: true
simulation mode: false
SLEEP START: 1000msec
GO:
world_time=1020000 YOUR DATA: ch1_data:HELLO_CLIENT_0
world_time=2020000 YOUR DATA: ch1_data:HELLO_CLIENT_0
world_time=3020000 YOUR DATA: ch1_data:HELLO_CLIENT_1
world_time=4020000 YOUR DATA: ch1_data:HELLO_CLIENT_2
world_time=5020000 YOUR DATA: ch1_data:HELLO_CLIENT_3
world_time=6020000 YOUR DATA: ch1_data:HELLO_CLIENT_4
world_time=7020000 YOUR DATA: ch1_data:HELLO_CLIENT_5
```

端末B：
```
server_event_handling:Start
wait_asset_callback_done(): prev_state=Runnable next_state=Running
WAIT RUNNING:
START CREATE PDU DATA: total_size= 512
INFO: shmget() key=256 size=512
WAIT PDU CREATED:
PDU DATA CREATED
CREATED ADDR=0x7f6d5d20f00c
asset_notification_feedback:request=AssetNotificationReply { event: Start, asset: Some(AssetInfo { name: "ConductorClient-01" }), ercd: Ok }
asset_notification_feedback:response=Response { metadata: MetadataMap { headers: {"content-type": "application/grpc", "date": "Fri, 14 Jul 2023 05:50:02 GMT", "grpc-status": "0"} }, message: NormalReply { ercd: Ok }, extensions: Extensions }
asset_notify_write_pdu_done() asset_name="ConductorClient-01"
LOADED: PDU DATA
OK PDU CREATED:
DO EXECUTE():
sync_mode: true
SLEEP START: 1000msec
GO:
world_time=1020000  YOUR DATA: ch2_data:HELLO_SERVER_0
world_time=2020000  YOUR DATA: ch2_data:HELLO_SERVER_1
world_time=3020000  YOUR DATA: ch2_data:HELLO_SERVER_2
world_time=4020000  YOUR DATA: ch2_data:HELLO_SERVER_3
world_time=5020000  YOUR DATA: ch2_data:HELLO_SERVER_4
world_time=6020000  YOUR DATA: ch2_data:HELLO_SERVER_5
```
