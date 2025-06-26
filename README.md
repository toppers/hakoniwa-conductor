[![build](https://github.com/toppers/hakoniwa-conductor/actions/workflows/build.yml/badge.svg)](https://github.com/toppers/hakoniwa-conductor/actions/workflows/build.yml)



箱庭コンダクタは、分散配置されているマシン間で、箱庭上にいるアセット間のシミュレーション調停する役割をするデーモンプロセスです。

![image](https://github.com/toppers/hakoniwa-conductor/assets/164193/80675925-270e-4adc-b71b-68db36e30c71)

箱庭コンダクタは、サーバーとクライアントに分かれており、本リポジトリでは、Rust版の箱庭コンダクタ（サーバー/クライアント）機能を提供しています。


サーバー/クライアント間の調停(シミュレーションの開始/停止等)は、gRPC で実現しています。
ただし、PDU間のデータ通信は用途に応じて、UDP or MQTT いずれかを選択できます。


なお、箱庭コンダクタは、箱庭シミュレーション環境に同梱されるモジュールであるため、本リポジトリ単体で箱庭シミュレーションを実施することはできません。

## 箱庭コンダクタをホストPCに直接インストールする手順

箱庭コンダクタをインストールするには、以下の手順で実施ます。

1. C++版箱庭コア機能をインストール
2. 箱庭コンダクタをインストール

## C++版箱庭コア機能をインストール

C++版箱庭コア機能をインストールします。

```
cd project
git clone --recursive https://github.com/toppers/hakoniwa-core-cpp-client.git
cd hakoniwa-core-cpp-client
bash build.bash
bash install.bash
```

## 箱庭コンダクタをインストール

事前に、rust のインストールが必要です。
rust のインストール方法は、[Rustのインストール](https://www.rust-lang.org/ja/tools/install)を参照してください。

箱庭コンダクタをビルドします。

```
cd project
git clone --recursive https://github.com/toppers/hakoniwa-conductor.git
cd hakoniwa-conductor/main
bash build.bash
bash install.bash
```

# 箱庭コンダクタの仕様

## 箱庭コンダクタのポート構成

### 箱庭コンダクタのサーバー側

* gRPC ポート番号: `50051` (デフォルト)
  * 引数 <ipaddr>:<port> の <port> に設定
* UDP 受信ポート番号: `54001` (デフォルト)
  * 引数 <udp_server_port> の値に設定
* UDP 送信ポート番号(自分側): `54002` (デフォルト)
  * 引数 <udp_sender_port> の値に設定

### 箱庭コンダクタのクライアント側

* UDP 受信ポート番号: `51001` (デフォルト)
  * conductor_config.json の <udp_server_port> に設定
* UDP 送信ポート番号(自分側): `51002` (デフォルト)
  * conductor_config.json の <udp_sender_port> に設定

## 箱庭コンダクタのコマンド仕様

### 箱庭コンダクタのサーバー側

サーバーを起動するには、以下の形式でコマンドを実行します：

```bash
hako-master-rust <delta_msec> <max_delay_msec> <ipaddr>:<port> [<udp_server_port> <udp_sender_port> [<mqtt_portno>]]
```

---

### 引数の説明

| 引数                  | 説明                                                 |
| ------------------- | -------------------------------------------------- |
| `<delta_msec>`      | シミュレーションの1ステップあたりの時間幅（ミリ秒単位）。タイミング制御周期を表します。例：`20` |
| `<max_delay_msec>`  | 最大許容遅延時間（ミリ秒単位）。PDU配信の最大許容遅延として使用されます。例：`100`      |
| `<ipaddr>:<port>`   | gRPCの待受アドレスとポート番号。例：`127.0.0.1:50051`              |
| `<udp_server_port>` | UDP受信用ポート番号（任意）。UDP通信を行う場合に指定します。例：`54001`         |
| `<udp_sender_port>` | UDP送信用ポート番号（任意）。UDP通信を行う場合に指定します。例：`54002`         |
| `<mqtt_portno>`     | MQTTブローカーのポート番号（任意）。MQTT通信を行う場合に指定します。例：`1883`     |

---

### 実行例

#### PDU通信でUDPを使用する場合：

```bash
hako-master-rust 20 100 172.20.0.10:50051 54001 54002
```

#### PDU通信でMQTTも使用する場合(UDP指定は必須です)：

```bash
hako-master-rust 20 100 172.20.0.10:50051 54001 54002 1883
```

#### 最小構成（gRPCのみ）：

```bash
hako-master-rust 20 100 172.20.0.10:50051
```

### 箱庭コンダクタのクライアント側

クライアントは、**JSON形式の設定ファイル**を用いて箱庭コンダクタサーバーと接続・通信します。クライアントは、以下のように起動します。

### 起動コマンド

```bash
hako-conductor-client <conductor-config> <robot-config>
```

* `<conductor-config>` : クライアント用の設定ファイル（JSON形式）
* `<robot-config>` : 操作対象ロボットの設定ファイル（JSON形式）

---

### クライアント設定ファイル (`conductor_config.json`)

以下のような形式で記述します：

```json
{
  "asset_name": "ConductorClient-01",
  "core_ipaddr": "172.20.0.10",
  "core_portno": 50051,
  "delta_msec": 20,
  "max_delay_msec": 20,
  "udp_server_ip_port": "172.20.0.11:51001",
  "udp_sender_ip_port": "172.20.0.11:51002",
  "mqtt_portno": -1,
  "mqtt_pub_client_id": "hako-mqtt-publisher-client-01",
  "mqtt_sub_client_id": "hako-mqtt-subscriber-client-01"
}
```

### 各項目の説明

| フィールド名               | 説明                                           |
| -------------------- | -------------------------------------------- |
| `asset_name`         | クライアントの識別名。サーバーへの登録名として使われます。                |
| `core_ipaddr`        | 箱庭コンダクタサーバーのIPアドレス。                          |
| `core_portno`        | サーバーのgRPCポート番号（通常 `50051`）。                  |
| `delta_msec`         | シミュレーションのステップ間隔（ミリ秒単位）。                      |
| `max_delay_msec`     | 最大許容遅延時間（ミリ秒単位）。                             |
| `udp_server_ip_port` | UDP受信用のIPアドレス＋ポート番号（例: `172.20.0.11:51001`）。 |
| `udp_sender_ip_port` | UDP送信用のIPアドレス＋ポート番号（例: `172.20.0.11:51002`）。 |
| `mqtt_portno`        | MQTTブローカーのポート番号。使用しない場合は `-1` を指定。           |
| `mqtt_pub_client_id` | MQTTパブリッシュ時のクライアントID（任意、重複不可）。               |
| `mqtt_sub_client_id` | MQTTサブスクライブ時のクライアントID（任意、重複不可）。              |

---

### 実行例

```bash
hako-conductor-client conductor_config.json robot_config.json
```

---

### 備考

* UDPとMQTTの両方の通信方式をサポートしています。

  * UDPのみ使用する場合は、`mqtt_portno: -1` とします。
  * MQTTを有効にする場合は、正しいポート番号（通常は `1883`）とクライアントIDを設定してください。
* `robot_config.json` には、使用する箱庭アセットのPDU定義や通信チャネルの情報が記述されます（内容はプロジェクトごとに異なります）。



# 箱庭コンダクタの動作確認手順

本リポジトリでは、箱庭コンダクタの動作確認用のテスト環境を用意しています。

## テスト環境

箱庭コンダクタの動作確認環境は以下の通りです。

* OS: 
  * Windows 10/11 WSL2 && Docker Compose
  * MacOS && Docker Compose

## テスト構成

* クライアント側の箱庭アセット
  * hakoniwa-conductor/test/workspace/client/asset-client-tester.py
* サーバー側の箱庭アセット
  * hakoniwa-conductor/test/workspace/server/asset-srv-tester.py 

## docker 環境

本テストでは、手軽にテストできるように、docker 環境を用意しています。

docker 環境は、docker-compose を使って、箱庭コンダクタのサーバーとクライアントを起動します。

### docker compose 環境の作成方法

docker-compose でビルドします。

```
cd hakoniwa-conductor/test
```

```
docker-compose build
```


### docker compose 環境の起動方法

docker-compose で箱庭コンダクタのサーバーとクライアントを起動します。

```
cd hakoniwa-conductor/test
```

```
docker-compose up -d
```

成功すると、こうなります。

```
[+] Running 2/0
 ✔ Container hakoniwa-server  Running                                            0.0s 
 ✔ Container hakoniwa-client  Running                                            0.0s
 ```

### docker コンテナへの接続方法

docker-compose で起動した箱庭コンダクタのサーバーとクライアントに接続します。

```
cd hakoniwa-conductor/test
```

サーバーに接続する場合：

```
docker exec -it hakoniwa-server /bin/bash
```

クライアントに接続する場合：

```
docker exec -it hakoniwa-client /bin/bash
```

### docker コンテナの再起動方法

docker-compose で起動した箱庭コンダクタのサーバーとクライアントを再起動します。

```
cd hakoniwa-conductor/test
```

```
docker-compose restart
```

### docker コンテナの停止方法

docker-compose で起動した箱庭コンダクタのサーバーとクライアントを停止します。

```
cd hakoniwa-conductor/test
```

```
docker-compose down
```

## 通信構成

![スクリーンショット 2025-06-26 10 33 48](https://github.com/user-attachments/assets/c7b65f2d-9d6c-4f51-8433-ff9bf7f04f4e)


* UDP 通信
  * hakoniwa-conductor/test/workspace/spec/custom.json
* MQTT 通信
  * hakoniwa-conductor/test/workspace/spec/custom_mqtt.json
* 箱庭コンダクタのクライアントのコンフィグ
  * hakoniwa-conductor/test/workspace/client/conductor_config.json
    * IPアドレスは、WSL2上の eth0 のIPアドレスを設定してください。
    * MQTTのテストをする場合は、mqtt_portnoの値を `1883` にしてください。

## テストアプリケーションの説明

本テストでは、サーバー側のノードに箱庭アセット `asset-srv-tester.py` を配置し、クライアント側のノードに箱庭アセット `asset-client-tester.py` を配置します。

- [asset-srv-tester.py](./test/workspace/server/asset-srv-tester.py)
  - サーバー側の箱庭アセット
  - クライアント側のノードの箱庭アセットが読み込むPDUデータへデータを書き込みします。
  - PDUデータは、`std_msgs/String` 型のデータを送信します。
  - また、クライアント側のノードの箱庭アセットが書き込みしたPDUデータを読み込みします。
  - 読み込んだデータは、デバッグ用にログ出力されます。
- [asset-client-tester.py](./test/workspace/client/asset-client-tester.py)
  - クライアント側の箱庭アセット
  - サーバー側のノードの箱庭アセットが読み込むPDUデータからデータを読み込みます。
  - PDUデータは、`std_msgs/String` 型のデータを受信します。
  - また、サーバー側のノードの箱庭アセットが書き込みしたPDUデータへデータを読み込みします。
  - 読み込んだデータは、デバッグ用にログ出力されます。

## テスト実行方法

端末を３個用意します。

* 端末A：箱庭コンダクタのサーバー側
* 端末B：箱庭コンダクタのクライアント側
* 端末C：箱庭コンダクタのサーバー側（コマンド実行用）

### 端末A： docker compose を起動します。

```
cd hakoniwa-conductor/test
```

```
docker-compose up -d
```

### 端末A, B, C： docker コンテナに接続します。

端末A:
```
cd hakoniwa-conductor/test
```
```
docker exec -it hakoniwa-server /bin/bash
```


端末B:
```
cd hakoniwa-conductor/test
```
```
docker exec -it hakoniwa-client /bin/bash
```


端末C:
```
cd hakoniwa-conductor/test
```
```
docker exec -it hakoniwa-server /bin/bash
```

### 端末A：箱庭コンダクタのサーバーを起動します。

UDPの場合：
```
bash server/run.bash
```

MQTTの場合：
```
bash server/run.bash mqtt
```

実行例(UDPの場合)：
```
root@f9b5698b3fdc:~/workspace# bash server/run.bash
OPEN RECIEVER UDP PORT=172.20.0.10:54001
OPEN SENDER UDP PORT=172.20.0.10:54002
delta_msec = 20
max_delay_msec = 20
Server Start: 172.20.0.10:50051
INFO: ACTIVATING :server/asset-srv-tester.py
Robot: TB3RoboModel, PduWriter: TB3RoboModel_ch2
channel_id: 2 pdu_size: 256
INFO: TB3RoboModel create_lchannel: logical_id=2 real_id=0 size=256
Robot: TB3RoboModel, PduWriter: TB3RoboModel_ch1
channel_id: 1 pdu_size: 256
INFO: TB3RoboModel create_lchannel: logical_id=1 real_id=1 size=256
INFO: asset(Server) is registered.
WAIT START
```

### 端末B：箱庭コンダクタのクライアントを起動します。

UDPの場合：
```
bash client/run.bash
```

MQTTの場合：
```
bash client/run.bash mqtt
```

実行例(UDPの場合)：
```
root@699e6354f7c0:~/workspace# bash client/run.bash 
ACTIVATING CONDUCTOR(CLIENT)
Conductor asset_name: ConductorClient-01
Conductor core_ipaddr: 172.20.0.10
Conductor core_portno: 50051
Conductor delta_msec: 20
Conductor max_delay_msec: 20
Conductor udp_server_port: 172.20.0.11:51001
Conductor udp_sender_port: 172.20.0.11:51002
Conductor mqtt_portno: -1
Conductor mqtt_pub_client_id: hako-mqtt-publisher-client-01
Conductor mqtt_sub_client_id: hako-mqtt-subscriber-client-01
-------------------
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
OPEN SENDER UDP PORT=172.20.0.11:51002
OPEN RECIEVER UDP PORT=172.20.0.11:51001
ACTIVATING PYTHON PROG
Robot: TB3RoboModel, PduWriter: TB3RoboModel_ch2
channel_id: 2 pdu_size: 256
Robot: TB3RoboModel, PduWriter: TB3RoboModel_ch1
channel_id: 1 pdu_size: 256
INFO: TB3RoboModel create_lchannel: logical_id=1 real_id=1 size=256
INFO: asset(Client) is registered.
WAIT START
```

### 端末C：シミュレーション開始します。

```
hako-cmd start
```

成功するとそれぞれの端末A/Bで以下のようにログ出力されます。

端末A：
```
WAIT START
register: Got a request: Request { metadata: MetadataMap { headers: {"te": "trailers", "content-type": "application/grpc", "user-agent": "tonic/0.8.3"} }, message: AssetInfo { name: "ConductorClient-01" }, extensions: Extensions }
subscribe_pdu_channel: Got a request: Request { metadata: MetadataMap { headers: {"te": "trailers", "content-type": "application/grpc", "user-agent": "tonic/0.8.3"} }, message: SubscribePduChannelRequest { asset_name: "ConductorClient-01", channel_id: 2, pdu_size: 256, listen_udp_ip_port: "172.20.0.11:51001", method_type: "UDP", robo_name: "TB3RoboModel" }, extensions: Extensions }
create_asset_sub_pdu
create_pdu_channel: Got a request: Request { metadata: MetadataMap { headers: {"te": "trailers", "content-type": "application/grpc", "user-agent": "tonic/0.8.3"} }, message: CreatePduChannelRequest { asset_name: "ConductorClient-01", channel_id: 1, pdu_size: 256, method_type: "UDP", robo_name: "TB3RoboModel" }, extensions: Extensions }
create_asset_pub_pdu: robo_name=TB3RoboModel channel_id=1 real_id=1
create_asset_pub_pdu: channel_ID=1
asset_notification_start: Got a request: Request { metadata: MetadataMap { headers: {"te": "trailers", "content-type": "application/grpc", "user-agent": "tonic/0.8.3"} }, message: AssetInfo { name: "ConductorClient-01" }, extensions: Extensions }
WAIT RUNNING
## SimulationAssetEvent START
asset_notification_feedback: Got a request: Request { metadata: MetadataMap { headers: {"te": "trailers", "content-type": "application/grpc", "user-agent": "tonic/0.8.3"} }, message: AssetNotificationReply { event: Start, asset: Some(AssetInfo { name: "ConductorClient-01" }), ercd: Ok }, extensions: Extensions }
START CREATE PDU DATA: total_size= 512
PDU CREATED
PDU DATA CREATED
CREATED ADDR=0x405539a00c
LOADED: PDU DATA
INFO: start simulation
INFO: on_manual_timing_control enter
20000: pdu_ch1_data={'data': 'CLIENT DATA:  0'}
60000: pdu_ch1_data={'data': 'CLIENT DATA:  1'}
100000: pdu_ch1_data={'data': 'CLIENT DATA:  2'}
140000: pdu_ch1_data={'data': 'CLIENT DATA:  3'}
```

端末B：
```
WAIT START
^[[1;2Dserver_event_handling:Start
wait_asset_callback_done(): prev_state=Runnable next_state=Running
WAIT RUNNING
START CREATE PDU DATA: total_size= 512
PDU CREATED
PDU DATA CREATED
CREATED ADDR=0x404c8c900c
asset_notification_feedback:request=AssetNotificationReply { event: Start, asset: Some(AssetInfo { name: "ConductorClient-01" }), ercd: Ok }
LOADED: PDU DATA
INFO: start simulation
INFO: on_manual_timing_control enter
asset_notification_feedback:response=Response { metadata: MetadataMap { headers: {"content-type": "application/grpc", "date": "Wed, 25 Jun 2025 23:20:03 GMT", "grpc-status": "0"} }, message: NormalReply { ercd: Ok }, extensions: Extensions }
asset_notify_write_pdu_done() asset_name="ConductorClient-01" 
INFO: conductor execute():status.is_pdu_created is false: false
20000: pdu_ch2_data={'data': ''}
60000: pdu_ch2_data={'data': 'SERVER DATA: 0'}
100000: pdu_ch2_data={'data': 'SERVER DATA: 1'}
140000: pdu_ch2_data={'data': 'SERVER DATA: 2'}
180000: pdu_ch2_data={'data': 'SERVER DATA: 3'}
```
