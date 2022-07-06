/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use dubbo_rust_protocol::jsonrpc::server::{JsonRpcServer, JsonRpcService};
use log::info;
use std::{net::SocketAddr, str::FromStr, sync::Mutex};

mod addservice;

use addservice::{add_service::AddServer, AddReq, AddResp, AddService, StdError};
struct MyAdd {
    times: Mutex<usize>,
}

#[async_trait::async_trait]
impl AddService for MyAdd {
    async fn add(&self, req: AddReq) -> Result<AddResp, StdError> {
        let times = {
            if let Ok(mut v) = self.times.lock() {
                *v += 1;
                *v
            } else {
                0
            }
        };
        info!("get request {:?} this is no.{} call", req, times);
        Ok(req.numbers.iter().sum())
    }
}

#[tokio::main]
async fn main() {
    // log
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let addr = SocketAddr::from_str("0.0.0.0:40021").unwrap();
    let rt = tokio::runtime::Handle::current();

    let service_impl = JsonRpcService::new(AddServer::new(MyAdd {
        times: Mutex::new(0),
    }));

    let server = JsonRpcServer::new(&addr, rt, service_impl);

    info!("Server start at {}", addr.to_string());

    server.await.unwrap();
}
